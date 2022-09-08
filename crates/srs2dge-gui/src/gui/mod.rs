use self::{
    generated::GeneratedGui,
    graphics::GuiGraphics,
    layout::{GuiLayout, WidgetLayout},
};
use crate::prelude::{GuiDraw, GuiEvent, GuiEventTy, PointerState, Widget};
use srs2dge_core::{
    main_game_loop::{event::Event, prelude::WindowState},
    prelude::Frame,
    target::Target,
    texture::Texture,
    wgpu::TextureView,
    winit::event::{DeviceEvent, DeviceId, ElementState, WindowEvent},
};
use srs2dge_text::glyphs::Glyphs;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
use taffy::prelude::{Number, Size};

//

pub mod generated;
pub mod geom;
pub mod graphics;
pub mod layout;
pub mod prelude;
pub mod renderer;

//

#[derive(Debug)]
pub struct Gui {
    ws: WindowState,
    pointers: HashMap<DeviceId, PointerState>,

    graphics: GuiGraphics,

    layout: GuiLayout,

    state: HashMap<TypeId, Box<dyn Any>>,
}

//

impl Gui {
    pub fn new(target: &Target) -> Self {
        let ws = WindowState::new(&target.get_window().unwrap()); // TODO: allow headless
        let layout = GuiLayout {
            height: ws.size.height as f32,
            ..Default::default()
        };

        Self {
            ws,
            pointers: Default::default(),

            graphics: GuiGraphics::new(target),

            layout,

            state: Default::default(),
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
    pub fn event<T: Widget>(
        &mut self,
        root: &mut T,
        event: Event<'static>,
    ) -> Result<(), taffy::Error> {
        if let Some(mut event) = self.event_inner(event) {
            self.layout.compute_layout(
                root.node(),
                Size {
                    width: Number::Defined(self.ws.size.width as f32),
                    height: Number::Defined(self.ws.size.height as f32),
                },
            )?;
            root.event(WidgetLayout::default(), &mut self.layout, &mut event)?;
        }

        self.update_pointers();

        Ok(())
    }

    /// generate drawable geometry
    pub fn draw<T: Widget>(
        &mut self,
        root: &mut T,
        target: &mut Target,
        frame: &mut Frame,
    ) -> Result<GeneratedGui, taffy::Error> {
        self.draw_inner(root, target, frame, None)
    }

    /// generate drawable geometry with custom texture
    pub fn draw_with<T: Widget>(
        &mut self,
        root: &mut T,
        target: &mut Target,
        frame: &mut Frame,
        texture: &TextureView,
    ) -> Result<GeneratedGui, taffy::Error> {
        self.draw_inner(root, target, frame, Some(texture))
    }

    /// gui internal `WindowState`
    pub fn window_state(&self) -> &WindowState {
        &self.ws
    }

    pub fn state<T: Any + Default>(&mut self) -> &mut T {
        self.state
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(T::default()) as Box<dyn Any>)
            .downcast_mut()
            .unwrap()
    }

    pub fn layout(&self) -> &GuiLayout {
        &self.layout
    }

    pub fn layout_mut(&mut self) -> &mut GuiLayout {
        &mut self.layout
    }

    fn event_inner(&mut self, event: Event<'static>) -> Option<GuiEvent> {
        self.ws.event(&event);
        self.layout.height = self.ws.size.height as f32;

        match event {
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
        }
    }

    fn draw_inner<T: Widget>(
        &mut self,
        root: &mut T,
        target: &mut Target,
        frame: &mut Frame,
        texture: Option<&TextureView>,
    ) -> Result<GeneratedGui, taffy::Error> {
        self.layout.compute_layout(
            root.node(),
            Size {
                width: Number::Defined(self.ws.size.width as f32),
                height: Number::Defined(self.ws.size.height as f32),
            },
        )?;

        root.event(
            WidgetLayout::default(),
            &mut self.layout,
            &mut GuiEvent::default(),
        )?;
        root.draw(
            WidgetLayout::default(),
            &mut self.layout,
            &mut GuiDraw {
                graphics: &mut self.graphics,
                target,
            },
        )?;

        Ok(self.graphics.draw(target, frame, &self.ws, texture))
    }

    /// clear pointer `released` states
    fn update_pointers(&mut self) {
        self.pointers
            .values_mut()
            .for_each(|pointer| pointer.update());
    }
}
