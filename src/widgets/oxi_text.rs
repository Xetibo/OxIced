use iced_widget::Text;

pub fn text<'a, Theme, Renderer>(
    text: impl iced_widget::text::IntoFragment<'a>,
) -> Text<'a, Theme, Renderer>
where
    Theme: iced_widget::text::Catalog + 'a,
    Renderer: iced_core::text::Renderer,
{
    Text::new(text)
}
