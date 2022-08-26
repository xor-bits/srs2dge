use srs2dge::prelude::*;

//

struct App {
    target: Target,
    reporter: Reporter,
    texture: TextureAtlasMap<u8>,

    gui: Gui,
    root: Root,
}

#[derive(Debug, Clone, Copy, PartialEq, Widget)]
#[gui(builder)]
struct Root {
    #[gui(style = "flex_item")]
    left_panel: LeftPanel,
    #[gui(style = "flex_item")]
    right_panel: RightPanel,
    #[gui(style = "float_item")]
    float_panel: FloatPanel,

    #[gui(core)]
    core: WidgetCore,
}

#[derive(Debug, Clone, Copy, PartialEq, Widget)]
#[gui(builder)]
pub struct LeftPanel {
    #[gui(style = "max_size fill_0")]
    fill: Fill,

    #[gui(core)]
    core: WidgetCore,
}

#[derive(Debug, Clone, Copy, PartialEq, Widget)]
#[gui(builder)]
pub struct RightPanel {
    #[gui(style = "max_size fill_1")]
    fill: Fill,

    #[gui(core)]
    core: WidgetCore,
}

#[derive(Debug, Clone, Copy, PartialEq, Widget)]
#[gui(builder)]
pub struct FloatPanel {
    #[gui(style = "max_size fill_2")]
    fill: Fill,

    #[gui(core)]
    core: WidgetCore,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();

        let mut gui = Gui::new(&target);

        let texture = TextureAtlasMap::builder()
            .with_bytes(0, res::texture::EMPTY)
            .unwrap()
            .with_bytes(1, res::texture::SPRITE)
            .unwrap()
            .with_label(Some("Atlas".to_string()))
            .build(&target);

        let tex0 = texture.get(&0).unwrap_or_default();
        let tex1 = texture.get(&1).unwrap_or_default();

        let mut stylesheet = StyleSheet::default();
        stylesheet.insert(
            "fill_0".to_string(),
            WidgetStyle {
                color: Some(Color::CHARTREUSE),
                texture: Some(tex0),
                ..Default::default()
            }
            .into(),
        );
        stylesheet.insert(
            "fill_1".to_string(),
            WidgetStyle {
                color: Some(Color::AZURE),
                texture: Some(tex0),
                ..Default::default()
            }
            .into(),
        );
        stylesheet.insert(
            "fill_2".to_string(),
            WidgetStyle {
                color: Some(Color::WHITE),
                texture: Some(tex1),
                ..Default::default()
            }
            .into(),
        );
        stylesheet.insert(
            "flex_item".to_owned(),
            LayoutStyle {
                flex_shrink: Some(1.0),
                flex_grow: Some(1.0),
                flex_basis: Some(Dimension::Points(5.0)),
                ..Default::default()
            }
            .into(),
        );
        stylesheet.insert(
            "float_item".to_owned(),
            LayoutStyle {
                position_type: Some(PositionType::Absolute),
                position: Some(LayoutRect {
                    start: Dimension::Points(0.0),
                    end: Dimension::Auto,
                    top: Dimension::Points(0.0),
                    bottom: Dimension::Auto,
                }),
                size: Some(Size {
                    width: Dimension::Points(350.0),
                    height: Dimension::Points(350.0),
                }),
                ..Default::default()
            }
            .into(),
        );
        stylesheet.insert(
            "max_size".to_owned(),
            LayoutStyle {
                size: Some(Size {
                    width: Dimension::Percent(1.0),
                    height: Dimension::Percent(1.0),
                }),
                ..Default::default()
            }
            .into(),
        );

        // let panel = float_panel(tex0);
        // let left = left_panel(tex0, tex1, col.clone());
        // let right = right_panel(tex0, col);
        // let grid = Grid::new_row([left, right])
        //     .with_size(border_size(BORDER))
        //     .with_offset(border_offset(BORDER));
        // gui.root().extend([grid, panel]);

        // panic!("{:#?}", gui.root());

        let root: Root = Root::build(&mut gui, WidgetCore::root_style(), &stylesheet).unwrap();

        Self {
            target,
            reporter: Reporter::new(),
            texture,

            gui,

            root,
        }
    }
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        let event = match event.to_static() {
            Some(some) => some,
            None => return,
        };

        self.gui.event(&mut self.root, event).unwrap();

        if self.gui.window_state().should_close {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
        let timer = self.reporter.begin();

        let mut frame = self.target.get_frame();

        // drawing
        let gui = self
            .gui
            .draw_with(&mut self.root, &mut self.target, &mut frame, &self.texture)
            .unwrap();
        frame.primary_render_pass().draw_gui(&gui);

        self.target.finish_frame(frame);

        self.reporter.end(timer);
        if self.reporter.should_report() {
            self.reporter.reset();
        }
    }
}

// fn left_panel(tex0: TexturePosition, tex1: TexturePosition, col: Arc<RwLock<Color>>) -> WidgetBase {
//     // top and bottom quad base
//     let quad_base_size = (BaseSize * Const(Vec2::splat(0.5))) // half the size of parent
//         .min(Const(Vec2::splat(200.0))) // 200x200 px at max
//         .force_ratio_with_x(1.0) // modify x to force square
//         .min(BaseSize) // size of parent at max
//         .force_ratio_with_y(1.0); // modify y to force square
//     let top_quad = Fill::new()
//         .with_color(Color::WHITE)
//         .with_texture(tex1)
//         .into_widget()
//         .with_size(quad_base_size)
//         .with_offset(align(Vec2::new(0.5, 1.0))); // x center, y top
//     let bottom_quad = Fill::new()
//         .with_color(Color::CHARTREUSE)
//         .with_texture(tex0)
//         .into_widget()
//         .with_size(quad_base_size)
//         .with_offset(align(Vec2::new(0.5, 0.0))); // x center, y bottom;

//     // background
//     Fill::new()
//         .with_color(col)
//         .with_texture(tex0)
//         .into_widget_with([top_quad, bottom_quad])
//         .with_size(border_size(BORDER))
//         .with_offset(border_offset(BORDER))
// }

// fn right_panel(tex0: TexturePosition, col: Arc<RwLock<Color>>) -> WidgetBase {
//     let grid = [[0, 1, 2], [3, 4, 5], [6, 7, 8]].map(|row| {
//         row.map(|i| {
//             let button_col = Color::new_mono((i as f32 / 8.0).powf(2.2));
//             let text_col = button_col.foreground();
//             let col = col.clone();

//             // buttons
//             let animator = Animator::new();
//             let button = Trigger::new()
//                 .with_handler(TriggerFilter::OnEnter, {
//                     let animator = animator.clone();
//                     move |_| animator.trigger()
//                 })
//                 .with_handler(TriggerFilter::OnExit, {
//                     let animator = animator.clone();
//                     move |_| animator.trigger_rev()
//                 })
//                 .with_handler(TriggerFilter::OnPressButton(MouseButton::Left), {
//                     let animator = animator.clone();
//                     move |_| animator.trigger_rev()
//                 })
//                 .with_handler(TriggerFilter::OnReleaseButton(MouseButton::Left), {
//                     let animator = animator.clone();
//                     move |_| animator.trigger()
//                 })
//                 .with_handler(TriggerFilter::OnClickButton(MouseButton::Left), move |_| {
//                     // set left side bg color to match
//                     // the color of the clicked button
//                     *col.write().unwrap() = button_col;
//                 })
//                 .into_widget();

//             // button texts
//             let label = Text::new()
//                 .with_text(
//                     FormatString::builder()
//                         .with(text_col)
//                         .with(18.0)
//                         .with(format!("{button_col:#}")),
//                 )
//                 .with_config(
//                     Animated::new(Text::default_config(), animator.clone()).then(
//                         Duration::from_millis(100),
//                         AnimationCurve::Poly,
//                         TextConfig {
//                             scale: 1.05,
//                             ..Text::default_config()
//                         },
//                     ),
//                 )
//                 .into_widget();

//             // button backgrounds
//             Fill::new()
//                 .with_color(button_col)
//                 .with_texture(tex0)
//                 .into_widget_with([button, label])
//                 .with_size(
//                     Animated::new(border_size(6.0).into(), animator.clone()).then(
//                         Duration::from_millis(100),
//                         AnimationCurve::Poly,
//                         border_size(1.0).into(),
//                     ),
//                 )
//                 .with_offset(Animated::new(border_offset(6.0).into(), animator).then(
//                     Duration::from_millis(100),
//                     AnimationCurve::Poly,
//                     border_offset(1.0).into(),
//                 ))
//         })
//     });

//     let grid = Grid::new_grid(grid);

//     // background
//     Fill::new()
//         .with_color(Color::ROSE)
//         .with_texture(tex0)
//         .into_widget_with([grid])
//         .with_size(border_size(BORDER))
//         .with_offset(border_offset(BORDER))
// }

// fn float_panel(tex0: TexturePosition) -> WidgetBase {
//     // perf text
//     let text = Text::new()
//         .with_text(FormatString::from_iter([
//             Color::MINT.foreground().into(),
//             20.0.into(),
//             format!("FPS: {fps}\n", fps = 0.0).into(),
//             16.0.into(),
//             format!("Frametime: {ft}", ft = 0.0).into(),
//         ]))
//         .into_widget();

//     let offset = Arc::new(RwLock::new(GuiCalcOffset::Const(Const(
//         Vec2::splat(10.0), /* + drag */
//     ))));
//     let position = Arc::new(RwLock::new(Vec2::splat(10.0)));

//     // top and bottom draggers
//     let drag_fill = Fill::new()
//         .with_color(Color::new_mono_a(0.0, 0.2))
//         .with_texture(tex0);
//     let drag_0 = DragZone::new()
//         .with_handler(
//             DragZoneFilter::OnDrag,
//             captures! {[offset, position] move |_, d| {
//                 let mut lock = offset.write().unwrap();
//                 match &mut *lock {
//                     GuiCalcOffset::Const(c) => c.0 = d + *position.read().unwrap(),
//                     _ => unreachable!()
//                 };
//             }},
//         )
//         .with_handler(
//             DragZoneFilter::OnRelease,
//             captures! {[position] move |_, d| {
//                 let mut lock = position.write().unwrap();
//                 *lock += d;
//             }},
//         )
//         .into_widget_with([drag_fill.clone().into_widget()])
//         .with_size(BaseSize * Const(Vec2::splat(0.25)));
//     let drag_1 = DragZone::new()
//         .into_widget_with([drag_fill.into_widget()])
//         .with_size(BaseSize * Const(Vec2::splat(0.25)))
//         .with_offset(align(Vec2::ONE));

//     // draggable tile/window/frame/whatever
//     Fill::new()
//         .with_color(Color::MINT)
//         .with_texture(tex0)
//         .into_widget_with([text, drag_0, drag_1])
//         .with_size(Const(Vec2::new(200.0, 300.0)))
//         .with_offset(offset)
// }

//

main_app!(async App);
