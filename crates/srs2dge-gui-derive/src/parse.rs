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
    pub builder: bool,

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
    Core,
    Inherit { style: Option<String> },
    SubWidget { style: Option<String> },
    Skip,
}

//

impl DeriveParsed {
    pub fn new(input: DeriveInput) -> Result<Self, Error> {
        let input: DerivePreParsed = DerivePreParsed::from_derive_input(&input)?;

        let DerivePreParsed {
            ident,
            generics,
            data,
            event_handler,
            draw_handler,
            builder,
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

        let mut main_fields = fields.iter().cloned().filter(|field| {
            matches!(
                field.inner,
                FieldParsedType::Core | FieldParsedType::Inherit { .. }
            )
        });
        let main_field_err = || {
            Error::new(
                Span::call_site(),
                format!(
                    "Exactly one field marked with `#[gui(core)]` OR `#[gui(inherit)]` is allowed"
                ),
            )
        };
        let main_field = main_fields.next().ok_or_else(main_field_err)?;
        if main_fields.next().is_some() {
            return Err(main_field_err());
        }

        Ok(Self {
            ident,
            imp,
            ty,
            wher,
            event_handler,
            draw_handler,
            builder,
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

        let inner = match (core, inherit, skip, style) {
            (true, true, _, _) => {
                return Err(Error::new(
                    span,
                    format!(
                    "Exactly one field marked with `#[gui(core)]` OR `#[gui(inherit)]` is allowed"
                ),
                ));
            }
            (true, false, true, _) | (false, true, true, _) => {
                return Err(Error::new(
                    span,format!("Fields marked with `#[gui(core)]` OR `#[gui(inherit)]` should not be skipped as a widget.\n`#[gui(skip)]` is only for non-widget and non-core fields")));
            }
            (true, false, _, Some(_)) => {
                return Err(Error::new(
                    span,format!("Fields marked with `#[gui(core)]` CANNOT have a style. However fields marked with `#[gui(inherit)]` CAN have a style.")));
            }
            (false, false, true, Some(_)) => {
                return Err(Error::new(
                    span,
                    format!("Fields marked with `#[gui(skip)]` should not have a style."),
                ))
            }

            (true, false, false, None) => FieldParsedType::Core,
            (false, true, false, style) => FieldParsedType::Inherit { style },
            (false, false, true, None) => FieldParsedType::Skip,
            (false, false, false, style) => FieldParsedType::SubWidget { style },
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
    builder: bool,
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
