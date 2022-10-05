#[macro_export]
macro_rules! stylesheet {
    (
        // stylesheet default
        $(=> { $($default_field_name:ident : $default_field_val:expr),* $(,)? })?

        // stylesheet entries
        $($rule_name:expr => {
            $($field_name:ident : $field_val:expr),* $(,)?
        })*
    ) => {{
        let mut stylesheet = StyleSheet::new();
        $(
            let mut style = Style::default();
            $(style . $default_field_name = Some($default_field_val);)*
            stylesheet.set_default(style);
        )?

        $(
            let mut style = Style::default();
            $(style . $field_name = Some($field_val);)*
            stylesheet.insert($rule_name, style);
        )*

        stylesheet
    }};
}
