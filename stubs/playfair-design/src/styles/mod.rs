pub mod button;
pub mod checkbox;
pub mod container;
pub mod pick_list;
pub mod progress_bar;
pub mod radio;
pub mod scrollable;
pub mod slider;
pub mod text;
pub mod text_input;
pub mod toggler;

pub use button as btn;
pub use checkbox as chk;
pub use container as cont;
pub use pick_list as pl;
pub use progress_bar as prog;
pub use radio as rad;
pub use scrollable as scroll;
pub use slider as sld;
pub use toggler as tog;

#[cfg(feature = "iced_aw")]
pub mod aw;
