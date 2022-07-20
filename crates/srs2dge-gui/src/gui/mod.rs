use self::{generated::GeneratedGui, graphics::GuiGraphics};
use crate::prelude::{GuiEvent, GuiEventTy, PointerState, Root, Widget, WidgetBase, WidgetLayout};
use srs2dge_core::{
    main_game_loop::{event::Event, prelude::WindowState},
    prelude::Frame,
    target::Target,
    texture::Texture,
    wgpu::TextureView,
    winit::event::{DeviceEvent, DeviceId, ElementState, WindowEvent},
};
use srs2dge_text::glyphs::Glyphs;
use std::collections::HashMap;

//

pub mod generated;
pub mod geom;
pub mod graphics;
pub mod prelude;
pub mod renderer;

//

#[derive(Debug)]
pub struct Gui<T = ()> {
    ws: WindowState,
    pointers: HashMap<DeviceId, PointerState>,

    graphics: GuiGraphics,

    root: WidgetBase<T>,
    state: T,
}

//

impl<T> Gui<T> {
    pub fn new(target: &Target, state: T) -> Self {
        Self {
            ws: WindowState::new(&target.get_window().unwrap()), // TODO: allow headless
            pointers: Default::default(),

            graphics: GuiGraphics::new(target),

            root: Root.into_widget(),
            state,
        }
    }

    /// default texture if custom one is not provided
    pub fn texture(&mut self, target: &Target) -> &Texture {
        self.graphics.get_texture(target)
    }

    /// SDF glyph texture
    pub fn glyphs(&mut self) -> &mut Glyphs {
        &mut self.graphics.glyphs
    }

    /// handle gui events
    pub fn event(&mut self, event: Event<'static>) {
        self.ws.event(&event);

        let event = match event {
            Event::DeviceEvent {
                event: DeviceEvent::Text { codepoint },
                ..
            } => Some(GuiEvent::new(GuiEventTy::Text(codepoint))),

            Event::DeviceEvent {
                event: DeviceEvent::Key(key),
                ..
            } => Some(GuiEvent::new(GuiEventTy::Key(key))),

            Event::WindowEvent { window_id, event } if Some(window_id) == self.ws.id => match event
            {
                WindowEvent::CursorMoved {
                    device_id,
                    position,
                    ..
                } => {
                    let pointer = self.pointers.entry(device_id).or_default();
                    pointer.moved_physical(position, &self.ws);
                    Some(GuiEvent::new(GuiEventTy::Pointer(*pointer)))
                }
                WindowEvent::MouseInput {
                    device_id,
                    state: ElementState::Pressed,
                    button,
                    ..
                } => {
                    let pointer = self.pointers.entry(device_id).or_default();
                    pointer.pressed(button);
                    Some(GuiEvent::new(GuiEventTy::Pointer(*pointer)))
                }
                WindowEvent::MouseInput {
                    device_id,
                    state: ElementState::Released,
                    button,
                    ..
                } => {
                    let pointer = self.pointers.entry(device_id).or_default();
                    pointer.released(button);
                    Some(GuiEvent::new(GuiEventTy::Pointer(*pointer)))
                }
                _ => None,
            },
            _ => None,
        };

        if let Some(event) = event {
            self.root.event(
                &mut self.state,
                WidgetLayout::from_window_state(&self.ws),
                event,
            );
        }

        self.update_pointers();
    }

    /// the root widget
    ///
    /// push (or expand) subwidgets here
    pub fn root(&mut self) -> &mut WidgetBase<T> {
        &mut self.root
    }

    /// generate drawable geometry
    pub fn draw(&mut self, target: &mut Target, frame: &mut Frame) -> GeneratedGui {
        self.draw_inner(target, frame, None)
    }

    /// generate drawable geometry with custom texture
    pub fn draw_with(
        &mut self,
        target: &mut Target,
        frame: &mut Frame,
        texture: &TextureView,
    ) -> GeneratedGui {
        self.draw_inner(target, frame, Some(texture))
    }

    /// gui internal `WindowState`
    pub fn window_state(&self) -> &WindowState {
        &self.ws
    }

    fn draw_inner(
        &mut self,
        target: &mut Target,
        frame: &mut Frame,
        texture: Option<&TextureView>,
    ) -> GeneratedGui {
        let layout = WidgetLayout::from_window_state(&self.ws);
        self.root
            .event(&mut self.state, layout, GuiEvent::default());
        self.root.draw(&mut self.graphics, target, layout);

        self.graphics.draw(target, frame, &self.ws, texture)
    }

    /// clear pointer `released` states
    fn update_pointers(&mut self) {
        self.pointers
            .values_mut()
            .for_each(|pointer| pointer.update());
    }
}
