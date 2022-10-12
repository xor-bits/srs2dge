use components::{Collider, CollisionResolver, CustomPlugin, Player};
use legion::{component, IntoQuery};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::de::DeserializeSeed;
use std::ops::Deref;

use srs2dge::prelude::*;

//

mod components;

//

const CAM_FOLLOW_PLAYER: bool = true;

//

struct App {
    target: Target,

    ws: WindowState,
    ks: KeyboardState,
    gs: GamepadState,

    texture_atlas: TextureAtlasMap<u8>,

    ubo: UniformBuffer<Mat4>,
    shader: Texture2DShader,

    world: World,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();

        let ws = WindowState::new(&target.get_window().unwrap());
        let ks = KeyboardState::new();
        let gs = GamepadState::new();

        let texture_atlas = if false {
            // generate the texture atlas and print it
            let texture_atlas = TextureAtlasMap::builder()
                .with_bytes(0, res::texture::SPRITE)
                .unwrap()
                .with_bytes(1, res::texture::EMPTY)
                .unwrap()
                .build_serializeable();
            let s = to_string_pretty(&texture_atlas, PrettyConfig::default()).unwrap();
            println!("{s}",);
            texture_atlas.upload(&target)
        } else {
            // or load the texture atlas baked into this binary
            ron::de::from_str::<SerializeableTextureAtlasMap<_>>(include_str!("atlas.ron"))
                .unwrap()
                .upload(&target)
        };

        let ubo = UniformBuffer::new(&target, 1);
        let shader = Texture2DShader::new(&target);

        let mut world = World::new()
            .with_plugin(DefaultClientPlugins(&target))
            .with_plugin(CustomPlugin);

        if false {
            // generate the scene and print it
            world.push((
                RigidBody2D::default(),
                Transform2D {
                    translation: Vec2::ZERO,
                    scale: Vec2::ONE * 0.1,
                    ..Default::default()
                },
                Sprite {
                    // TODO: texture manager
                    sprite: texture_atlas.get(&0).unwrap(),
                    color: Color::WHITE,
                    ..Default::default()
                },
                Player::default(),
                Collider,
                CollisionResolver::default(),
            ));
            world.push((
                Transform2D {
                    translation: Vec2::new(0.0, -0.5),
                    scale: Vec2::new(1.5, 0.2),
                    ..Default::default()
                },
                Sprite {
                    sprite: texture_atlas.get(&1).unwrap(),
                    color: Color::ORANGE,
                    ..Default::default()
                },
                Collider,
            ));
            world.push((
                Transform2D {
                    translation: Vec2::new(0.4, -0.2),
                    scale: Vec2::new(0.3, 0.2),
                    ..Default::default()
                },
                Sprite {
                    sprite: texture_atlas.get(&1).unwrap(),
                    color: Color::CYAN,
                    ..Default::default()
                },
                Collider,
            ));
            world.push((
                Transform2D {
                    translation: Vec2::new(0.8, 0.0),
                    scale: Vec2::new(0.2, 0.4),
                    ..Default::default()
                },
                Sprite {
                    sprite: texture_atlas.get(&1).unwrap(),
                    color: Color::CHARTREUSE,
                    ..Default::default()
                },
                Collider,
            ));
            world.push((
                Transform2D {
                    translation: Vec2::new(-0.95, 1.0),
                    scale: Vec2::new(0.2, 3.4),
                    ..Default::default()
                },
                Sprite {
                    sprite: texture_atlas.get(&1).unwrap(),
                    color: Color::AZURE,
                    ..Default::default()
                },
                Collider,
            ));

            let s = to_string_pretty(
                &world
                    .serialize_builder::<String>()
                    .with_named_component::<RigidBody2D>()
                    .with_named_component::<Transform2D>()
                    .with_named_component::<Sprite>()
                    .with_named_component::<Player>()
                    .with_named_component::<Collider>()
                    .with_named_component::<CollisionResolver>()
                    .serialize(legion::passthrough()),
                PrettyConfig::default(),
            )
            .unwrap();
            println!("{s}");
        } else {
            // or load the scene
            world
                .deserialize_builder::<String>()
                .with_named_component::<RigidBody2D>()
                .with_named_component::<Transform2D>()
                .with_named_component::<Sprite>()
                .with_named_component::<Player>()
                .with_named_component::<Collider>()
                .with_named_component::<CollisionResolver>()
                .deserialize()
                .deserialize(
                    &mut ron::de::Deserializer::from_str(include_str!("scene.ron")).unwrap(),
                )
                .unwrap();
        }

        Self {
            target,

            ws,
            ks,
            gs,

            texture_atlas,

            ubo,
            shader,

            world,
        }
    }

    async fn event(&mut self, event: Event<'_>, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.ws.event(&event);
        self.ks.event(&event);
        self.gs.event(&event);

        if self.ws.should_close {
            *control = ControlFlow::Exit;
        }
    }

    async fn draw(&mut self) {
        self.world.resources.insert(self.ks.clone());
        self.world.resources.insert(self.gs.clone());
        if self.world.run() {
            self.ks.clear();
            self.gs.clear();
        }

        // frames
        let mut frame = self.target.get_frame();

        let player_pos = <&Sprite>::query()
            .filter(component::<Player>())
            .iter(self.world.deref())
            .next()
            .unwrap()
            .lerp_transform
            .translation;

        self.ubo.upload(
            &mut self.target,
            &mut frame,
            &[Mat4::orthographic_rh(
                -1.0 * self.ws.aspect,
                1.0 * self.ws.aspect,
                -1.0,
                1.0,
                -1.0,
                1.0,
            ) * if CAM_FOLLOW_PLAYER {
                Mat4::from_translation(Vec3::new(-player_pos.x, -player_pos.y, 0.0))
            } else {
                Mat4::IDENTITY
            }],
        );

        let mut batcher = self.world.get_batcher_mut();
        let (vbo, ibo, i) = batcher.generate(&mut self.target, &mut frame);

        frame
            .primary_render_pass()
            .bind_vbo(vbo)
            .bind_ibo(ibo)
            .bind_group(&self.shader.bind_group((&self.ubo, &self.texture_atlas)))
            .bind_shader(&self.shader)
            .draw_indexed(0..i, 0, 0..1);

        self.target.finish_frame(frame);
    }
}

//

fn main() {
    app!(App);
}
