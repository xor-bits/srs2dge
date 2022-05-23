use components::{Collider, CollisionResolver, CustomPlugin, Player};
use legion::{component, IntoQuery, Resources};
use std::ops::Deref;
use winit::event_loop::ControlFlow;

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

        let texture_atlas = TextureAtlasMapBuilder::new()
            .with(
                0,
                image::load_from_memory(res::texture::SPRITE)
                    .unwrap()
                    .to_rgba8(),
            )
            .with(
                1,
                image::load_from_memory(res::texture::EMPTY)
                    .unwrap()
                    .to_rgba8(),
            )
            .build(&target);

        let ubo = UniformBuffer::new(&target, 1);
        let shader = Texture2DShader::new(&target);

        let mut world = World::new(&target)
            .with_plugin(DefaultPlugins)
            .with_plugin(CustomPlugin);
        world.push((
            RigidBody2D::default(),
            Transform2D {
                translation: Vec2::ZERO,
                scale: Vec2::ONE * 0.1,
                ..Default::default()
            },
            Sprite {
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
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.ws.event(&event);
        self.ks.event(&event);

        if self.ws.should_close {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
        let mut resources = Resources::default();
        resources.insert(self.ks.clone());
        resources.insert(self.gs.clone());
        if self.world.run_with(resources, Default::default()) {
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
            &[Mat4::orthographic_lh(
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

        let (vbo, ibo) = self
            .world
            .get_batcher_mut()
            .generate(&mut self.target, &mut frame);

        frame
            .primary_render_pass()
            .bind_vbo(vbo)
            .bind_ibo(ibo)
            .bind_group(&self.shader.bind_group((&self.ubo, &self.texture_atlas)))
            .bind_shader(&self.shader)
            .draw_indexed(0..ibo.capacity() as _, 0, 0..1);

        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
