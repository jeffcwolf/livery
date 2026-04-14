use crate::styles;
use crate::typography::{scale, weights};
use crate::Theme;
use iced::widget::{button, row, text};
use iced::{Element, Font};

pub fn tab_bar<'a, Tab, Message>(
    tabs: &[(&'a str, Tab)],
    active: Tab,
    on_select: impl Fn(Tab) -> Message + 'static,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    Tab: Clone + PartialEq + 'static,
    Message: Clone + 'static,
{
    let mut tab_row = row![].spacing(2);
    for (label, tab) in tabs {
        let msg = on_select(tab.clone());
        let tab_label = text(*label).size(scale::size::BODY_SM).font(Font {
            weight: weights::MEDIUM,
            ..Default::default()
        });
        let btn = button(tab_label).on_press(msg).padding([4, 10]);
        tab_row = tab_row.push(if *tab == active {
            btn.style(styles::btn::primary(theme))
        } else {
            btn.style(styles::btn::secondary(theme))
        });
    }
    tab_row.into()
}
