use crate::parse::{DeriveParsed, FieldParsedType};
use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;

//

impl ToTokens for DeriveParsed {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.derive_widget(tokens);
        self.derive_widget_builder(tokens);
    }
}

impl DeriveParsed {
    fn derive_widget(&self, tokens: &mut TokenStream) {
        let DeriveParsed {
            ident,
            imp,
            ty,
            wher,
            event_handler,
            draw_handler,
            fields,
            main_field,
            ..
        } = self;

        // where to get the widget core?
        //
        // take from inherited if this
        // widget inherits
        //
        // or take from the marked core
        let main_field_ident = &main_field.ident;
        let core = match &main_field.inner {
            FieldParsedType::Inherit { .. } => quote! { Widget::core(&self.#main_field_ident) },
            FieldParsedType::Core => quote! { &self.#main_field_ident },
            _ => unreachable!(),
        };

        // event/draw handlers
        let self_event = if *event_handler {
            quote! { self.event_handler(layout, gui_layout, event)?; }
        } else {
            Default::default()
        };
        let self_draw = if *draw_handler {
            quote! { self.draw_handler(layout, gui_layout, draw)?; }
        } else {
            Default::default()
        };
        let inherit_event = if let FieldParsedType::Inherit { .. } = self.main_field.inner {
            quote! { Widget::event(&mut self.#main_field_ident, parent_layout, gui_layout, event)?; }
        } else {
            Default::default()
        };
        let inherit_draw = if let FieldParsedType::Inherit { .. } = self.main_field.inner {
            quote! { Widget::draw(&mut self.#main_field_ident, parent_layout, gui_layout, draw)?; }
        } else {
            Default::default()
        };

        // subwidget fields
        let fields = fields
            .iter()
            .filter(|field| matches!(field.inner, FieldParsedType::SubWidget { .. }))
            .map(|field| &field.ident);
        let fields_rev = fields.clone().rev();

        // actual impl
        let new_tokens = quote! {
            impl #imp Widget for #ident #ty #wher {
                fn event(&mut self, parent_layout: WidgetLayout, gui_layout: &mut GuiLayout, event: &mut GuiEvent) -> Result<(), taffy::Error> {
                    let layout = gui_layout.get(self)?.to_absolute(parent_layout);

                    #(Widget::event(&mut self.#fields_rev, layout, gui_layout, event)?;)*

                    #self_event
                    #inherit_event

                    Ok(())
                }

                fn draw(&mut self, parent_layout: WidgetLayout, gui_layout: &mut GuiLayout, draw: &mut GuiDraw) -> Result<(), taffy::Error> {
                    let layout = gui_layout.get(self)?.to_absolute(parent_layout);

                    #self_draw
                    #inherit_draw

                    #(Widget::draw(&mut self.#fields, layout, gui_layout, draw)?;)*

                    Ok(())
                }

                fn core(&self) -> &WidgetCore {
                    #core
                }

                fn as_any(&self) -> &dyn std::any::Any {
                    self
                }

                fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
                    self
                }
            }
        };
        tokens.extend(new_tokens);
    }

    fn derive_widget_builder(&self, tokens: &mut TokenStream) {
        if !self.builder {
            return;
        }

        let DeriveParsed {
            ident,
            imp,
            ty,
            wher,
            fields,
            main_field,
            ..
        } = self;

        let subwidgets = fields.iter().filter_map(|field| match &field.inner {
            FieldParsedType::SubWidget { style } => Some((field, style)),
            _ => None,
        });
        let fields = subwidgets.clone().map(|(f, _)| &f.ident);
        let fields_2 = fields.clone();

        // all subwidget builders
        let field_builders =
            subwidgets
            .map(|( field, style)| {
                let id = &field.ident;
                let ty = &field.ty;
                let part_merges = Self::style_str_to_part_merges(style);

                quote! {
                    let #id: #ty = WidgetBuilder::build(gui, Style::default() #(#part_merges)*, styles, &[])?;
                }
            });

        // convert core or inherit to builder
        //
        // inherit needs its style also
        let main_field_ident = &main_field.ident;
        let main_field_setup = match &main_field.inner {
            FieldParsedType::Core => {
                quote! {
                    let #main_field_ident = WidgetCore::new(gui, style.layout, &[
                        #(#fields.node(),)*
                    ])?;
                }
            }
            FieldParsedType::Inherit { style } => {
                let part_merges = Self::style_str_to_part_merges(style);
                quote! {
                    let #main_field_ident = WidgetBuilder::build(gui, style #(#part_merges)*, styles, &[
                        #(#fields.node(),)*
                    ])?;
                }
            }
            _ => unreachable!(),
        };

        let new_tokens = quote! {
            impl #imp WidgetBuilder for #ident #ty #wher {
                fn build(gui: &mut Gui, style: Style, styles: &StyleSheet, children: &[Node]) -> Result<Self, taffy::Error> {
                    #(#field_builders)*

                    #main_field_setup

                    Ok(Self {
                        #(#fields_2,)*

                        #main_field_ident
                    })
                }
            }
        };
        tokens.extend(new_tokens);
    }

    fn style_str_to_part_merges<'s>(
        style: &'s Option<String>,
    ) -> impl Iterator<Item = TokenStream> + 's {
        style
            .as_ref()
            .into_iter()
            .flat_map(|s| s.split_whitespace())
            .rev()
            .map(|style_part_name| quote! { .merge_from_styles(styles, #style_part_name) })
    }
}
