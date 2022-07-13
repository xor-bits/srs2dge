use components::{Collider, CollisionResolver, CustomPlugin, Player};
use legion::{component, serialize::Canon, IntoQuery};
use serde::de::DeserializeSeed;
use std::ops::{Deref, DerefMut};

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

        let texture_atlas = ron::de::from_str::<TextureAtlasMapFile<_>>(include_str!("atlas.ron"))
            .unwrap()
            .convert(&target);

        let ubo = UniformBuffer::new(&target, 1);
        let shader = Texture2DShader::new(&target);

        let mut world = World::new()
            .with_plugin(DefaultClientPlugins(&target))
            .with_plugin(CustomPlugin);

        // generate
        /* world.push((
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
        )); */

        // load
        let mut reg = legion::Registry::<String>::default();
        reg.register::<RigidBody2D>("RigidBody2D".to_owned());
        reg.register::<Transform2D>("Transform2D".to_owned());
        reg.register::<Sprite>("Sprite".to_owned());
        reg.register::<Collider>("Collider".to_owned());
        reg.register::<Player>("Player".to_owned());
        reg.register::<CollisionResolver>("CollisionResolver".to_owned());
        let entity_serializer = Canon::default();
        let w = world.deref_mut();
        *w = reg
            .as_deserialize(&entity_serializer)
            .deserialize(&mut ron::de::Deserializer::from_str(include_str!("scene.ron")).unwrap())
            .unwrap();

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
        self.gs.event(&event);

        if self.ws.should_close {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
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

async fn __main_run() {
    let target = main_game_loop::event::EventLoop::new();
    let app = App::init(&target).await;
    target.runnable(app);
}

fn main() {
    // main_game_loop::init_log();
    main_game_loop::as_async(__main_run());
}

// main_app!(async App);
