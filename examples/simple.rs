use glam::{Mat4, Vec2, Vec4};
use main_game_loop::{
    engine::AnyEngine,
    report::Reporter,
    state::{input::InputState, window::WindowState},
};
use rand::{thread_rng, Rng};
use srs2dge::{
    batch::{quad::QuadMesh, BatchRenderer},
    prelude::{vertex::ty::DefaultVertex, UniformBuffer},
    shader::presets::Colored2DShader,
    Engine,
};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

//

fn run(mut engine: Engine) {
    let mut target = engine.new_target().unwrap();

    let mut frame_reporter = Reporter::new();

    let mut ws = WindowState::new(&target.get_window());
    let mut old_aspect = ws.aspect;

    let mut is = InputState::new();

    let mut batcher = BatchRenderer::<QuadMesh, DefaultVertex>::new(&target);
    let ubo = UniformBuffer::new_single(&target, gen_mvp(ws.aspect));
    let shader = Colored2DShader::new(&target);

    let mut rng = thread_rng();

    loop {
        while let Some(event) = engine.poll() {
            ws.event(&event);
            is.event(&event);

            if is.should_close() {
                return;
            }

            if let Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Space),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } = event
            {
                batcher.push_with(QuadMesh {
                    pos: Vec2::new(rng.gen_range(-0.9..=0.9), rng.gen_range(-0.9..=0.9)),
                    size: Vec2::new(rng.gen_range(0.01..=0.1), rng.gen_range(0.01..=0.1)),
                    col: Vec4::new(1.0, 1.0, 1.0, 1.0),
                });
            }
        }

        // ------
        //  draw
        // ------

        let timer = frame_reporter.begin();
        let mut frame = target.get_frame();

        let aspect = ws.aspect;
        if aspect != old_aspect {
            old_aspect = aspect;
            ubo.upload(&mut target, &mut frame, &[gen_mvp(aspect)]);
        }

        let (vbo, ibo) = batcher.generate(&mut target, &mut frame);

        frame
            .main_render_pass()
            .bind_vbo(vbo, 0)
            .bind_ibo(ibo)
            .bind_group(&shader.bind_group(&ubo))
            .bind_shader(&shader)
            .draw_indexed(0..ibo.capacity() as u32, 0, 0..1);

        target.finish_frame(frame);
        if frame_reporter.end(timer) {
            log::debug!(
                "\n{}",
                Reporter::report_all("3.0s", &[("FRAME", &frame_reporter)])
            )
        }
    }
}

fn gen_mvp(aspect: f32) -> Mat4 {
    Mat4::orthographic_lh(-aspect, aspect, -1.0, 1.0, 0.0, 100.0)
}

fn main() {
    env_logger::init();
    Engine::new().run(run);
}
