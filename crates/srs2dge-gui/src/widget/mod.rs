use self::base::WidgetBase;

//

pub mod base;
pub mod button;
pub mod empty;
pub mod fill;
pub mod grid;
pub mod prelude;
pub mod root;
// pub mod text;

//

pub trait Widget {
    fn base(&self) -> WidgetBase;
}
