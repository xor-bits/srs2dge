/* use main_game_loop::{
    io::input_state::{Input, InputAxis, InputState, Triggered},
    AnyEngine, Event, GameLoop, Runnable,
};
use glam::{Mat4, Vec2, Vec4};
use glium::{texture::RawImage2d, uniform, DrawParameters, Frame, Program, Surface, Texture2d};
use srs2dge::{
    batch::{quad::QuadMesh, BatchRenderer, Idx},
    program::{default_program, DefaultVertex},
    BuildEngine, Engine,
};
use winit::window::WindowBuilder;

//

struct App {
    left: Idx,
    right: Idx,
    left_a: Idx,
    left_b: Idx,
    left_c: Idx,
    left_d: Idx,
    right_a: Idx,
    right_b: Idx,
    right_c: Idx,
    right_d: Idx,
    batcher: BatchRenderer<DefaultVertex, QuadMesh>,

    program: Program,

    texture: Texture2d,

    input: InputState,
}

//

impl Runnable<Engine> for App {
    fn update(&mut self, _: &mut GameLoop<Engine>) {}

    fn event(&mut self, gl: &mut GameLoop<Engine>, event: &Event) {
        self.input.event(event);

        if self.input.should_close() {
            gl.stop();
        }

        let color = |active: bool| -> Vec4 {
            if active {
                Vec4::new(1.0, 0.0, 0.0, 1.0)
            } else {
                Vec4::new(0.0, 0.0, 1.0, 1.0)
            }
        };

        let move_axis = self.input.get_axis(InputAxis::Move, 0);
        let left = self.batcher.get_mut(self.left);
        left.pos = move_axis / 4.0 + Vec2::new(-0.5, -0.5);
        left.col = color(move_axis.length_squared() <= 0.1_f32.powi(2));

        let look_axis = self.input.get_axis(InputAxis::Look, 0);
        let right = self.batcher.get_mut(self.right);
        right.pos = look_axis / 4.0 + Vec2::new(0.5, -0.5);
        right.col = color(look_axis.length_squared() <= 0.1_f32.powi(2));

        self.batcher.get_mut(self.left_a).col =
            color(self.input.get_input(Input::RollDown, 0).triggered());
        self.batcher.get_mut(self.left_b).col =
            color(self.input.get_input(Input::RollUp, 0).triggered());
        self.batcher.get_mut(self.left_c).col =
            color(self.input.get_input(Input::RollRight, 0).triggered());
        self.batcher.get_mut(self.left_d).col =
            color(self.input.get_input(Input::RollLeft, 0).triggered());

        self.batcher.get_mut(self.right_a).col =
            color(self.input.get_input(Input::Jump, 0).triggered());
        self.batcher.get_mut(self.right_b).col =
            color(self.input.get_input(Input::Inventory, 0).triggered());
        self.batcher.get_mut(self.right_c).col =
            color(self.input.get_input(Input::Reload, 0).triggered());
        self.batcher.get_mut(self.right_d).col =
            color(self.input.get_input(Input::Crouch, 0).triggered());
    }

    fn draw(&mut self, gl: &mut GameLoop<Engine>, frame: &mut Frame, _: f32) {
        frame.clear_all_srgb((0.2, 0.2, 0.2, 1.0), 1.0, 0);

        let (vbo, ibo) = self.batcher.draw(gl);

        let ubo = uniform! {
            mat: Mat4::orthographic_rh_gl(-gl.aspect, gl.aspect, 1.0, -1.0, -1.0, 1.0).to_cols_array_2d(),
            sprite: self.texture.sampled()
        };

        frame
            .draw(
                vbo,
                ibo,
                &self.program,
                &ubo,
                &DrawParameters {
                    primitive_restart_index: true,
                    ..Default::default()
                },
            )
            .unwrap();
    }
}

//

fn main() {
    env_logger::init();

    let engine = WindowBuilder::new().with_title("GamePad").build_engine();

    let mut batcher = BatchRenderer::<DefaultVertex, QuadMesh>::new(&engine);
    let left = batcher.push_with(QuadMesh {
        pos: Vec2::new(-0.5, 0.0),
        size: Vec2::new(0.1, 0.1),
        col: Vec4::new(1.0, 1.0, 1.0, 1.0),
    });
    let right = batcher.push_with(QuadMesh {
        pos: Vec2::new(0.5, 0.0),
        size: Vec2::new(0.1, 0.1),
        col: Vec4::new(1.0, 1.0, 1.0, 1.0),
    });

    let left_a = batcher.push_with(QuadMesh {
        pos: Vec2::new(-0.75, 0.0),
        size: Vec2::new(0.05, 0.05),
        col: Vec4::new(1.0, 1.0, 1.0, 1.0),
    });
    let left_b = batcher.push_with(QuadMesh {
        pos: Vec2::new(-0.75, 0.1),
        size: Vec2::new(0.05, 0.05),
        col: Vec4::new(1.0, 1.0, 1.0, 1.0),
    });
    let left_c = batcher.push_with(QuadMesh {
        pos: Vec2::new(-0.7, 0.05),
        size: Vec2::new(0.05, 0.05),
        col: Vec4::new(1.0, 1.0, 1.0, 1.0),
    });
    let left_d = batcher.push_with(QuadMesh {
        pos: Vec2::new(-0.8, 0.05),
        size: Vec2::new(0.05, 0.05),
        col: Vec4::new(1.0, 1.0, 1.0, 1.0),
    });

    let right_a = batcher.push_with(QuadMesh {
        pos: Vec2::new(0.75, 0.0),
        size: Vec2::new(0.05, 0.05),
        col: Vec4::new(1.0, 1.0, 1.0, 1.0),
    });
    let right_b = batcher.push_with(QuadMesh {
        pos: Vec2::new(0.75, 0.1),
        size: Vec2::new(0.05, 0.05),
        col: Vec4::new(1.0, 1.0, 1.0, 1.0),
    });
    let right_c = batcher.push_with(QuadMesh {
        pos: Vec2::new(0.7, 0.05),
        size: Vec2::new(0.05, 0.05),
        col: Vec4::new(1.0, 1.0, 1.0, 1.0),
    });
    let right_d = batcher.push_with(QuadMesh {
        pos: Vec2::new(0.8, 0.05),
        size: Vec2::new(0.05, 0.05),
        col: Vec4::new(1.0, 1.0, 1.0, 1.0),
    });

    let program = default_program(&engine);
    let texture = RawImage2d::from_raw_rgba_reversed(&[1.0, 1.0, 1.0, 1.0], (1, 1));
    let texture = Texture2d::new(&engine, texture).unwrap();

    let input = InputState::new();

    engine.build_main_game_loop().run(App {
        left,
        right,
        left_a,
        left_b,
        left_c,
        left_d,
        right_a,
        right_b,
        right_c,
        right_d,
        batcher,

        program,
        texture,

        input,
    });
}
 */
