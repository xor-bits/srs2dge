use darling::{ast::Data, FromDeriveInput, FromField, ToTokens};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Error, Generics, Index, Type};

//

pub struct DeriveParsed {
    pub ident: Ident,
    pub imp: TokenStream,
    pub ty: TokenStream,
    pub wher: TokenStream,

    pub event_handler: bool,
    pub draw_handler: bool,
    pub no_builder: bool,

    pub fields: Vec<FieldParsed>,
    pub main_field: FieldParsed,
}

#[derive(Clone)]
pub struct FieldParsed {
    pub ident: TokenStream,
    pub span: Span,

    pub ty: Type,

    pub inner: FieldParsedType,
}

#[derive(Clone)]
pub enum FieldParsedType {
    // Widget core
    Core { styles: String },

    // Widget core also
    Inherit { styles: String },

    // Subwidget
    SubWidget { styles: String },

    // Ignored field
    Skip,
}

//

impl DeriveParsed {
    pub fn new(input: &DeriveInput) -> Result<Self, Error> {
        let input: DerivePreParsed = DerivePreParsed::from_derive_input(input)?;

        let DerivePreParsed {
            ident,
            generics,
            data,
            event_handler,
            draw_handler,
            no_builder,
        } = input;
        let (imp, ty, wher) = generics.split_for_impl();
        let (imp, ty, wher) = (
            imp.to_token_stream(),
            ty.to_token_stream(),
            wher.to_token_stream(),
        );

        let fields = data
            .take_struct()
            .unwrap()
            .fields
            .into_iter()
            .enumerate()
            .map(FieldParsed::new)
            .collect::<Result<Vec<FieldParsed>, Error>>()?;

        // make sure there is exactly one 'core' or 'inherit'
        let main_fields: Vec<&FieldParsed> = fields
            .iter()
            .filter(|field| {
                matches!(
                    field.inner,
                    FieldParsedType::Core { .. } | FieldParsedType::Inherit { .. }
                )
            })
            .collect();

        let main_field = match main_fields[..] {
            [the_only_main_field] => the_only_main_field.clone(),
            _ => {
                return Err(Error::new(
                    Span::call_site(),
                    "Exactly one field marked with `#[gui(core)]` OR `#[gui(inherit)]` is allowed",
                ));
            }
        };

        Ok(Self {
            ident,
            imp,
            ty,
            wher,
            event_handler,
            draw_handler,
            no_builder,
            fields,
            main_field,
        })
    }
}

impl FieldParsed {
    fn new((i, field): (usize, FieldPreParsed)) -> Result<Self, Error> {
        let FieldPreParsed {
            ident,
            ty,
            core,
            inherit,
            skip,
            style,
        } = field;
        let span = ident.span();

        let ident = ident
            .map(|i| {
                quote! {#i}
            })
            .unwrap_or_else(|| {
                let i = Index::from(i);
                quote! {#i}
            });

        // process different types of fields
        let inner = match (core, inherit, skip, style) {
            (true, true, _, _) => {
                return Err(Error::new(
                    span,
                        "Exactly one field marked with `#[gui(core)]` OR `#[gui(inherit)]` (not both) is allowed".to_string()
                ));
            }
            (true, false, true, _) | (false, true, true, _) => {
                return Err(Error::new(
                    span,"Fields marked with `#[gui(core)]` OR `#[gui(inherit)]` should not be skipped as a widget.\n`#[gui(skip)]` is only for non-widget and non-core fields".to_string()));
            }
            (false, false, true, Some(_)) => {
                return Err(Error::new(
                    span,
                    "Fields marked with `#[gui(skip)]` should not have a style.".to_string(),
                ))
            }

            (true, false, false, style) => FieldParsedType::Core {
                styles: style.unwrap_or_default(),
            },
            (false, true, false, style) => FieldParsedType::Inherit {
                styles: style.unwrap_or_default(),
            },
            (false, false, false, style) => FieldParsedType::SubWidget {
                styles: style.unwrap_or_default(),
            },
            (false, false, true, None) => FieldParsedType::Skip,
        };

        Ok(Self {
            ident,
            span,
            ty,
            inner,
        })
    }
}

//

#[derive(FromDeriveInput)]
#[darling(attributes(gui), supports(struct_any))]
struct DerivePreParsed {
    ident: Ident,
    generics: Generics,

    data: Data<(), FieldPreParsed>,

    #[darling(default)]
    event_handler: bool,
    #[darling(default)]
    draw_handler: bool,
    #[darling(default)]
    no_builder: bool,
}

#[derive(FromField)]
#[darling(attributes(gui))]
struct FieldPreParsed {
    ident: Option<Ident>,

    ty: Type,

    #[darling(default)]
    core: bool,
    #[darling(default)]
    inherit: bool,
    #[darling(default)]
    skip: bool,
    #[darling(default)]
    style: Option<String>,
}
