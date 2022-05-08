use crate::{label, prelude::ShaderModule, target::Target};
use naga::{
    valid::{Capabilities, ValidationFlags, Validator},
    AddressSpace, ImageClass, ImageDimension, Module, ScalarKind, TypeInner,
};
use std::{collections::BTreeMap, num::NonZeroU64};
use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, PipelineLayoutDescriptor, SamplerBindingType, ShaderSource, ShaderStages,
    TextureSampleType, TextureViewDimension,
};

//

pub struct AutoLayout {
    group: BindGroupLayout,
}

pub struct AutoLayoutGetter<'a> {
    groups: [&'a BindGroupLayout; 1],
}

//

impl<'a> AutoLayoutGetter<'a> {
    pub fn get(&self) -> PipelineLayoutDescriptor {
        PipelineLayoutDescriptor {
            label: label!(),
            bind_group_layouts: &self.groups,
            push_constant_ranges: &[],
        }
    }
}

impl AutoLayout {
    pub fn new(
        target: &Target,
        (vs, vs_main): (&ShaderModule, &str),
        (fs, fs_main): (&ShaderModule, &str),
    ) -> Self {
        let mut validator = Validator::new(ValidationFlags::all(), Capabilities::all());
        let vs = Self::module(vs, vs_main, ShaderStages::VERTEX, &mut validator);
        let fs = Self::module(fs, fs_main, ShaderStages::FRAGMENT, &mut validator);

        let entries: Vec<BindGroupLayoutEntry> = Self::merge(vs, fs)
            .into_iter()
            .map(|(_, entry)| entry)
            .collect();

        let group = target
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: label!(),
                entries: &entries,
            });

        Self { group }
    }

    pub fn get(&self) -> AutoLayoutGetter {
        AutoLayoutGetter {
            groups: [&self.group],
        }
    }

    fn module(
        module: &ShaderModule,
        entry: &str,
        visibility: ShaderStages,
        validator: &mut Validator,
    ) -> Vec<BindGroupLayoutEntry> {
        let module = parse(&module.source);

        let i = module
            .entry_points
            .iter()
            .enumerate()
            .find(|(_, ep)| ep.name == entry)
            .unwrap()
            .0;

        let module_info = validator.validate(&module).unwrap();

        let entry_function = module_info.get_entry_point(i);

        let mut layouter = naga::proc::Layouter::default();
        layouter.update(&module.types, &module.constants).unwrap();

        module
            .global_variables
            .iter()
            .filter(|(handle, _)| !entry_function[*handle].is_empty())
            .filter_map(|(_, var)| Some((var.space, var.binding.clone()?, var.ty)))
            .filter_map(|(space, bind, ty)| {
                let size = layouter[ty];
                let ty = module.types.get_handle(ty).unwrap();

                match (&ty.inner, space) {
                    (TypeInner::Sampler { .. }, _) => Some(BindGroupLayoutEntry {
                        binding: bind.binding,
                        visibility,
                        ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                        count: None,
                    }),
                    (
                        TypeInner::Image {
                            dim,
                            arrayed,
                            class,
                        },
                        _,
                    ) => Some(BindGroupLayoutEntry {
                        binding: bind.binding,
                        visibility,
                        ty: BindingType::Texture {
                            sample_type: match class {
                                ImageClass::Sampled {
                                    kind: ScalarKind::Float,
                                    ..
                                } => TextureSampleType::Float { filterable: false },
                                ImageClass::Sampled {
                                    kind: ScalarKind::Sint,
                                    ..
                                } => TextureSampleType::Sint,
                                ImageClass::Sampled {
                                    kind: ScalarKind::Uint,
                                    ..
                                } => TextureSampleType::Uint,
                                ImageClass::Depth { .. } => TextureSampleType::Depth,
                                ImageClass::Storage { .. } => todo!(),
                                _ => todo!(),
                            },
                            view_dimension: match (dim, arrayed) {
                                (ImageDimension::D1, false) => TextureViewDimension::D1,
                                (ImageDimension::D2, false) => TextureViewDimension::D2,
                                (ImageDimension::D2, true) => TextureViewDimension::D2Array,
                                (ImageDimension::D3, false) => TextureViewDimension::D3,
                                (ImageDimension::Cube, false) => TextureViewDimension::Cube,
                                (ImageDimension::Cube, true) => TextureViewDimension::CubeArray,
                                _ => unimplemented!(),
                            },
                            multisampled: false,
                        },
                        count: None,
                    }),
                    (_, AddressSpace::Uniform) => Some(BindGroupLayoutEntry {
                        binding: bind.binding,
                        visibility,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: Some(NonZeroU64::new(size.size as _).unwrap()),
                        },
                        count: None,
                    }),
                    other => unimplemented!("Unimplemented: {other:?}"),
                }
            })
            .collect()
    }

    fn merge(
        vs: Vec<BindGroupLayoutEntry>,
        fs: Vec<BindGroupLayoutEntry>,
    ) -> BTreeMap<u32, BindGroupLayoutEntry> {
        let mut first: BTreeMap<u32, BindGroupLayoutEntry> =
            vs.into_iter().map(|entry| (entry.binding, entry)).collect();

        for mut entry in fs.into_iter() {
            if let Some(existing_entry) = first.get(&entry.binding) {
                entry.visibility |= existing_entry.visibility;
                first.insert(entry.binding, entry);
            } else {
                first.insert(entry.binding, entry);
            }
        }

        first
    }
}

//

fn parse(source: &ShaderSource) -> Module {
    match source {
        #[cfg(feature = "spirv")]
        ShaderSource::SpirV(spv) => {
            // source from wgpu repo to keep it somewhat similar:
            // Parse the given shader code and store its representation.
            let options = naga::front::spv::Options {
                adjust_coordinate_space: false, // we require NDC_Y_UP feature
                strict_capabilities: true,
                block_ctx_dump_prefix: None,
            };
            let parser = naga::front::spv::Parser::new(spv.iter().cloned(), &options);
            parser.parse().unwrap()
        }

        #[cfg(feature = "glsl")]
        ShaderSource::Glsl {
            shader,
            stage,
            defines,
        } => {
            // source from wgpu repo to keep it somewhat similar:
            // Parse the given shader code and store its representation.
            let options = naga::front::glsl::Options {
                stage: *stage,
                defines: defines.clone(),
            };
            let mut parser = naga::front::glsl::Parser::default();
            parser.parse(&options, shader).unwrap()
        }

        ShaderSource::Wgsl(source) => naga::front::wgsl::parse_str(source).unwrap(),

        _ => unimplemented!(),
    }
}

//

#[cfg(test)]
mod tests {
    use super::AutoLayout;
    use crate::{prelude::ShaderModule, Engine};
    use std::borrow::Cow;

    //

    const SOURCE: &str = include_str!("../../res/shader/test.wgsl");

    //

    #[test]
    fn main() {
        let engine = Engine::new();
        let target = pollster::block_on(engine.new_target_headless());
        let module = ShaderModule::new_wgsl_source(&target, Cow::Borrowed(SOURCE)).unwrap();
        AutoLayout::new(&target, (&module, "vs_main"), (&module, "fs_main"));
    }
}
