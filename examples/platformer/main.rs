use winit::event_loop::ControlFlow;

use srs2dge::{prelude::*, winit::event::VirtualKeyCode};

//

const CAM_FOLLOW_PLAYER: bool = true;

//

struct App {
    target: Target,

    ws: WindowState,
    ks: KeyboardState,
    ul: Option<UpdateLoop>,
    rate: f32,

    texture_atlas: TextureAtlasMap<u8>,

    batcher: BatchRenderer,
    ubo: UniformBuffer<Mat4>,
    shader: Texture2DShader,

    vel: Vec2,
    pos: Vec2,
    player: Idx,
    floor: Idx,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();

        let ws = WindowState::new(&target.get_window().unwrap());
        let ks = KeyboardState::new();
        let rate = UpdateRate::PerSecond(60);
        let ul = Some(UpdateLoop::new(rate));
        let rate = rate.to_interval().as_secs_f32();

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

        let mut batcher = BatchRenderer::new(&target);
        let ubo = UniformBuffer::new(&target, 1);
        let shader = Texture2DShader::new(&target);

        let player = batcher.push_with(QuadMesh {
            pos: Vec2::ZERO,
            size: Vec2::ONE * 0.1,
            col: Vec4::ONE,
            tex: texture_atlas.get(&0).unwrap(),
        });

        let floor = batcher.push_with(QuadMesh {
            pos: Vec2::new(0.0, -0.5),
            size: Vec2::new(1.5, 0.2),
            col: Vec4::ONE,
            tex: texture_atlas.get(&1).unwrap(),
        });

        Self {
            target,

            ws,
            ks,
            ul,
            rate,

            texture_atlas,

            batcher,
            ubo,
            shader,

            vel: Vec2::ZERO,
            pos: Vec2::ZERO,
            player,
            floor,
        }
    }

    fn update(&mut self) {
        if self.ks.pressed(VirtualKeyCode::A) {
            self.vel.x -= 0.1;
        }
        if self.ks.pressed(VirtualKeyCode::D) {
            self.vel.x += 0.1;
        }
        if self.ks.just_pressed(VirtualKeyCode::W) | self.ks.just_pressed(VirtualKeyCode::Space) {
            self.vel.y += 3.0;
        }
        self.ks.clear();
        self.vel += Vec2::new(0.0, -0.1);
        self.vel *= 0.95;
        self.pos += self.vel * self.rate;

        let a = self.batcher.get(self.player);
        let b = self.batcher.get(self.floor);
        let a_min = self.pos - a.size * 0.5;
        let a_max = self.pos + a.size * 0.5;
        let b_min = b.pos - b.size * 0.5;
        let b_max = b.pos + b.size * 0.5;
        if Self::aabb(a_min, a_max, b_min, b_max) {
            let res = Self::aabb_res(a_min, a_max, b_min, b_max);
            self.pos += res;
            if res.x.abs() >= f32::EPSILON {
                self.vel.x = 0.0;
            }
            if res.y.abs() >= f32::EPSILON {
                self.vel.y = 0.0;
            }
        }
    }

    fn aabb(a_min: Vec2, a_max: Vec2, b_min: Vec2, b_max: Vec2) -> bool {
        (a_min.x <= b_max.x && a_max.x >= b_min.x) && (a_min.y <= b_max.y && a_max.y >= b_min.y)
    }

    fn aabb_res(a_min: Vec2, a_max: Vec2, b_min: Vec2, b_max: Vec2) -> Vec2 {
        let xo_1 = a_max.x - b_min.x;
        let yo_1 = a_max.y - b_min.y;
        let xo_2 = a_min.x - b_max.x;
        let yo_2 = a_min.y - b_max.y;
        let smallest = |v: f32, a: &[f32]| -> bool { a.iter().all(|x| v.abs() < x.abs()) };

        if smallest(xo_1, &[yo_1, xo_2, yo_2]) {
            Vec2::new(-xo_1, 0.0)
        } else if smallest(yo_1, &[xo_1, xo_2, yo_2]) {
            Vec2::new(0.0, -yo_1)
        } else if smallest(xo_2, &[xo_1, yo_1, yo_2]) {
            Vec2::new(-xo_2, 0.0)
        } else if smallest(yo_2, &[xo_1, yo_1, xo_2]) {
            Vec2::new(0.0, -yo_2)
        } else {
            Vec2::ZERO
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
        // updates
        let mut update = self.ul.take().unwrap();
        let delta = update.update(|| {
            self.update();
        });
        self.ul = Some(update);

        // frames
        let mut frame = self.target.get_frame();

        let a = self.batcher.get_mut(self.player);
        a.pos = self.pos + self.vel * self.rate * delta;

        let player_pos = self.batcher.get(self.player).pos;
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

        let (vbo, ibo) = self.batcher.generate(&mut self.target, &mut frame);

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
