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
        let (core, core_mut) = match &main_field.inner {
            FieldParsedType::Inherit { .. } => (
                quote! { Widget::core(&self.#main_field_ident) },
                quote! { Widget::core_mut(&mut self.#main_field_ident) },
            ),
            FieldParsedType::Core { .. } => (
                quote! { &self.#main_field_ident },
                quote! { &mut self.#main_field_ident },
            ),
            _ => unreachable!(),
        };

        // event/draw handlers
        let self_event = if *event_handler {
            quote! { self.event_handler(layout, event); }
        } else {
            Default::default()
        };
        let self_draw = if *draw_handler {
            quote! { self.draw_handler(layout, draw); }
        } else {
            Default::default()
        };
        let (inherit_event, inherit_draw, inherit_layout) =
            if let FieldParsedType::Inherit { .. } = self.main_field.inner {
                (
                    quote! { Widget::event(&mut self.#main_field_ident, event); },
                    quote! { Widget::draw(&mut self.#main_field_ident, draw); },
                    quote! { Widget::layout(&mut self.#main_field_ident, parent_layout); },
                )
            } else {
                Default::default()
            };

        // subwidget fields
        let fields = fields
            .iter()
            .filter(|field| matches!(field.inner, FieldParsedType::SubWidget { .. }))
            .map(|field| &field.ident);
        let fields_2 = fields.clone();
        let fields_3 = fields.clone();
        let fields_rev = fields.clone().rev();

        // actual impl
        let new_tokens = quote! {
            impl #imp Widget for #ident #ty #wher {
                fn event(&mut self, event: &mut GuiEvent) {
                    let layout = self.core().layout;

                    #(Widget::event(&mut self.#fields_rev, event);)*

                    #self_event
                    #inherit_event
                }

                fn draw(&mut self, draw: &mut GuiDraw) {
                    let layout = self.core().layout;

                    #self_draw
                    #inherit_draw

                    #(Widget::draw(&mut self.#fields, draw);)*
                }

                fn layout(&mut self, parent_layout: WidgetLayout) {

                    // TODO: self_layout
                    self.gen_layout(parent_layout);
                    #inherit_layout

                    let layout = self.core().layout;

                    #(Widget::layout(&mut self.#fields_2, layout);)*
                }

                fn subwidgets(&self) -> std::borrow::Cow<'_, [&dyn Widget]> {
                    [#(&self.#fields_3 as &dyn Widget,)*].to_vec().into()
                }

                fn name(&self) -> &'static str {
                    std::any::type_name::<Self>()
                }

                fn core(&self) -> &WidgetCore {
                    #core
                }

                fn core_mut(&mut self) -> &mut WidgetCore {
                    #core_mut
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
        if self.no_builder {
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
            FieldParsedType::SubWidget { styles } => Some((field, styles)),
            _ => None,
        });
        let fields = subwidgets.clone().map(|(f, _)| &f.ident);
        let fields_2 = fields.clone();

        // all subwidget builders
        let field_builders = subwidgets.map(|(field, style)| {
            let id = &field.ident;
            let ty = &field.ty;

            quote! {
                let #id: #ty = WidgetBuilder::build(
                    StyleRef::from_styles(#style, __style_sheet),
                    __style_sheet
                );
            }
        });

        // convert core or inherit to builder
        //
        // inherit needs its style also
        let main_field_ident = &main_field.ident;
        let main_field_setup = match &main_field.inner {
            FieldParsedType::Core { styles } => {
                // `style` is the style given to the builder
                // `styles` are the styles marked with #[gui(style = "...")]
                quote! {
                    let #main_field_ident = WidgetCore::new()
                        .with_style(__style.merge(StyleRef::from_styles(#styles, __style_sheet)));
                }
            }
            FieldParsedType::Inherit { styles } => {
                // `style` is the style given to the builder
                // `styles` are the styles marked with #[gui(style = "...")]
                quote! {
                    let #main_field_ident = WidgetBuilder::build(
                        __style.merge(StyleRef::from_styles(#styles, __style_sheet)),
                        __style_sheet
                    );
                }
            }
            _ => unreachable!(),
        };

        let new_tokens = quote! {
            impl #imp WidgetBuilder for #ident #ty #wher {
                fn build(__style: StyleRef, __style_sheet: &StyleSheet) -> Self {
                    #(#field_builders)*

                    #main_field_setup

                    Self {
                        #(#fields_2,)*

                        #main_field_ident
                    }
                }
            }
        };
        tokens.extend(new_tokens);
    }
}
