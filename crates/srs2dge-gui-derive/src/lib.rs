use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error, Generics, Ident, Index, Type};

//

#[derive(FromDeriveInput)]
#[darling(attributes(gui), supports(struct_any))]
struct WidgetTraitOpts {
    ident: Ident,
    generics: Generics,

    data: Data<(), WidgetField>,

    #[darling(default)]
    event_handler: bool,
    #[darling(default)]
    draw_handler: bool,
    #[darling(default)]
    builder: bool,
}

#[derive(FromField)]
#[darling(attributes(gui))]
struct WidgetField {
    ident: Option<Ident>,

    ty: Type,

    #[darling(default)]
    core: bool,
    #[darling(default)]
    skip: bool,
}

//

fn opt_ident_to_ident(i: usize, ident: Option<&Ident>) -> TokenStream {
    ident
        .map(|i| {
            quote! {#i}
        })
        .unwrap_or_else(|| {
            let i = Index::from(i);
            quote! {#i}
        })
}

#[proc_macro_derive(Widget, attributes(gui))]
pub fn derive_widget(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(tokens as DeriveInput);
    let gui_input: WidgetTraitOpts =
        WidgetTraitOpts::from_derive_input(&parsed).unwrap_or_else(|err| panic!("{err}"));

    let WidgetTraitOpts {
        ident,
        generics,
        data,
        // size,
        // offset,
        event_handler,
        draw_handler,
        builder,
        ..
    } = gui_input;
    let (imp, ty, wher) = generics.split_for_impl();

    let fields: Vec<(TokenStream, WidgetField)> = data
        .take_struct()
        .unwrap()
        .fields
        .into_iter()
        .enumerate()
        .map(|(i, field)| (opt_ident_to_ident(i, field.ident.as_ref()), field))
        .collect();
    let field_names_forward = &fields
        .iter()
        .filter(|(_, field)| !(field.core || field.skip))
        .map(|(id, _)| id.clone())
        .collect::<Vec<_>>()[..];
    let field_names_reverse = &fields
        .iter()
        .filter(|(_, field)| !(field.core || field.skip))
        .rev()
        .map(|(id, _)| id.clone())
        .collect::<Vec<_>>()[..];

    let self_event = if event_handler {
        quote! { self.event_handler(layout, gui_layout, event)?; }
    } else {
        Default::default()
    };
    let self_draw = if draw_handler {
        quote! { self.draw_handler(layout, gui_layout, draw)?; }
    } else {
        Default::default()
    };

    let core = fields
        .iter()
        .filter(|(_, field)| field.core)
        .map(|(i, _)| i.clone())
        .next()
        .unwrap_or_else(|| {
            panic!(
                "{}",
                Error::new(
                    Span::call_site(),
                    "widget core is missing, mark field with type `WidgetCore` with `#[gui(core)]`"
                )
            )
        });

    let builder = if builder {
        let field_builders = fields
            .iter()
            .filter(|(_, field)| !(field.core || field.skip))
            .map(|(id, field)| {
                let ty = &field.ty;
                quote! {
                    let #id: #ty = WidgetBuilder::build(gui, Default::default())?;
                }
            });

        quote! {
            impl #imp WidgetBuilder for #ident #ty #wher {
                fn build(gui: &mut Gui, style: taffy::style::Style) -> Result<Self, taffy::Error> {
                    #(#field_builders)*

                    let core = WidgetCore::new(gui, style, &[
                        #(#field_names_forward.node(),)*
                    ])?;

                    Ok(Self {
                        #(#field_names_forward,)*

                        core
                    })
                }
            }
        }
    } else {
        Default::default()
    };

    (quote! {
        #builder

        impl #imp Widget for #ident #ty #wher {
            fn event(&mut self, parent_layout: WidgetLayout, gui_layout: &mut GuiLayout, event: &mut GuiEvent) -> Result<(), taffy::Error> {
                let layout = gui_layout.get(self)?.to_absolute(parent_layout);

                #(Widget::event(&mut self.#field_names_reverse, layout, gui_layout, event)?;)*

                #self_event

                Ok(())
            }

            fn draw(&mut self, parent_layout: WidgetLayout, gui_layout: &mut GuiLayout, draw: &mut GuiDraw) -> Result<(), taffy::Error> {
                let layout = gui_layout.get(self)?.to_absolute(parent_layout);

                #self_draw

                #(Widget::draw(&mut self.#field_names_forward, layout, gui_layout, draw)?;)*

                Ok(())
            }

            fn core(&self) -> &WidgetCore {
                &self.#core
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    })
    .into()
}
