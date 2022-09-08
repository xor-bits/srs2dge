//

#[macro_export]
macro_rules! stylesheet {
    ($($rule_name:expr => {
        $($($field_name:ident).* : $field_val:expr),* $(,)?
    })*) => {{
        let mut stylesheet = StyleSheet::new();
        $(
            let mut style = Style::default();
            $(style . $($field_name).* = Some($field_val);)*
            stylesheet.insert($rule_name, style);
        )*

        stylesheet
    }};
}

#[macro_export]
macro_rules! stylesheet_entry {
    ($style:tt, flex: $grow:tt, $shrink:tt, $basis:tt) => {
        stylesheet_entry!($style, flex_grow: $grow);
        stylesheet_entry!($style, flex_shrink: $shrink);
        stylesheet_entry!($style, flex_basis: $basis);
    };

    ($style:tt, flex_basis: auto) => {
        $style.layout.flex_basis = Some(Dimension::Auto);
    };
    ($style:tt, flex_basis: unset) => {
        $style.layout.flex_basis = None;
    };
    ($style:tt, flex_basis: %$val:expr) => {
        $style.layout.flex_basis = Some(Dimension::Percent($val));
    };
    ($style:tt, flex_basis: $val:expr) => {
        $style.layout.flex_basis = Some(Dimension::Points($val));
    };

    ($style:tt, flex_grow: $val:expr) => {
        $style.layout.flex_grow = Some($val);
    };
    ($style:tt, flex_grow: unset) => {
        $style.layout.flex_grow = None;
    };

    ($style:tt, flex_shrink: $val:expr) => {
        $style.layout.flex_shrink = Some($val);
    };
    ($style:tt, flex_shrink: unset) => {
        $style.layout.flex_shrink = None;
    };
}

//

#[test]
fn x() {
    use crate::prelude::Style;
    use taffy::prelude::Dimension;
    let mut style = Style::default();
    stylesheet_entry!( style, flex: 4.0, 2.0, 5.0 );
    panic!("{style:#?}");
}
