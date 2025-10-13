use iced::Alignment;
use iced::advanced::Clipboard;
use iced::advanced::Layout;
use iced::advanced::Shell;
use iced::advanced::Widget;
use iced::advanced::layout;
use iced::advanced::layout::Limits;
use iced::advanced::renderer;
use iced::advanced::text;
use iced::advanced::widget::tree::{self, Tree};
use iced::alignment;
use iced::animation::Easing;
use iced::mouse;
use iced::time::Instant;
use iced::touch;
use iced::widget;
use iced::window;
use iced::{Border, Color, Element, Event, Length, Pixels, Rectangle, Size, Theme};
use lilt::Animated;

use crate::theme::theme_impl::OXITHEME;
use crate::utils::color::darken_color;

fn mix(a: Color, b: Color, factor: f32) -> Color {
    let b_amount = factor.clamp(0.0, 1.0);
    let a_amount = 1.0 - b_amount;

    let a_linear = a.into_linear().map(|c| c * a_amount);
    let b_linear = b.into_linear().map(|c| c * b_amount);

    Color::from_linear_rgba(
        a_linear[0] + b_linear[0],
        a_linear[1] + b_linear[1],
        a_linear[2] + b_linear[2],
        a_linear[3] + b_linear[3],
    )
}

#[allow(missing_debug_implementations)]
pub struct OxiRadio<'a, V, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    V: PartialEq + Clone,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    current: Option<V>,
    value: V,
    on_select: Option<Box<dyn Fn(V) -> Message + 'a>>,
    size: f32,
    spacing: f32,
    font: Option<Renderer::Font>,
    class: Theme::Class<'a>,
    last_status: Option<Status>,

    label: Option<text::Fragment<'a>>,
    text_size: Option<Pixels>,
    text_line_height: text::LineHeight,
    text_alignment: text::Alignment,
    text_shaping: text::Shaping,
    text_wrapping: text::Wrapping,
}

/// The state of the [`OxiRadio`]
#[derive(Debug)]
pub struct State<Paragraph>
where
    Paragraph: text::Paragraph,
{
    now: Instant,
    transition_selected: Animated<bool, Instant>,
    transition_hovered: Animated<bool, Instant>,
    is_hovered: bool,
    text_state: widget::text::State<Paragraph>,
}

impl<Paragraph> State<Paragraph>
where
    Paragraph: text::Paragraph,
{
    /// This check is meant to fix cases when we get a tainted state from another
    /// ['OxiRadio'] widget by finding impossible cases.
    fn is_animation_state_tainted(&self, is_toggled: bool) -> bool {
        is_toggled != self.transition_selected.value
    }
}

impl<'a, V, Message, Theme, Renderer> OxiRadio<'a, V, Message, Theme, Renderer>
where
    V: PartialEq + Clone,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    pub const DEFAULT_SIZE: f32 = 20.0;

    pub fn new(
        label: Option<impl Into<text::Fragment<'a>>>,
        current: Option<V>,
        value: V,
        on_select: Option<impl Fn(V) -> Message + 'a>,
    ) -> Self {
        OxiRadio {
            current,
            value,
            on_select: on_select.map(|val| Box::new(val) as Box<dyn Fn(V) -> Message + 'a>),
            label: label.map(|val| val.into()),
            size: Self::DEFAULT_SIZE,
            text_size: Some(Pixels::from(OXITHEME.font_lg)),
            text_line_height: text::LineHeight::default(),
            text_alignment: text::Alignment::Default,
            text_shaping: text::Shaping::default(),
            text_wrapping: text::Wrapping::Glyph,
            spacing: OXITHEME.padding_xxl,
            font: None,
            class: Theme::default(),
            last_status: None,
        }
    }

    pub fn label(mut self, label: impl text::IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    pub fn on_select(mut self, on_select: impl Fn(V) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    pub fn on_select_maybe(mut self, on_select: Option<impl Fn(V) -> Message + 'a>) -> Self {
        self.on_select = on_select.map(|on_select| Box::new(on_select) as _);
        self
    }

    /// Sets the size of the [`OxiRadio`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().0;
        self
    }

    /// Sets the text size o the [`OxiRadio`].
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into());
        self
    }

    /// Sets the text [`text::LineHeight`] of the [`OxiRadio`].
    pub fn text_line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.text_line_height = line_height.into();
        self
    }

    /// Sets the horizontal alignment of the text of the [`OxiRadio`]
    pub fn text_alignment(mut self, alignment: impl Into<text::Alignment>) -> Self {
        self.text_alignment = alignment.into();
        self
    }

    /// Sets the [`text::Shaping`] strategy of the [`OxiRadio`].
    pub fn text_shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }

    /// Sets the [`text::Wrapping`] strategy of the [`OxiRadio`].
    pub fn text_wrapping(mut self, wrapping: text::Wrapping) -> Self {
        self.text_wrapping = wrapping;
        self
    }

    /// Sets the spacing between the [`OxiRadio`] and the text.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Sets the [`Renderer::Font`] of the text of the [`OxiRadio`]
    ///
    /// [`Renderer::Font`]: crate::core::text::Renderer
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the style of the [`OxiRadio`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`OxiRadio`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<V, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for OxiRadio<'_, V, Message, Theme, Renderer>
where
    V: PartialEq + Clone,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    // what? is this necessary?
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<widget::text::State<Renderer::Paragraph>>()
    }

    // state, self explanatory
    fn state(&self) -> tree::State {
        tree::State::new(State {
            now: Instant::now(),
            transition_selected: Animated::new(self.current == Some(self.value.clone()))
                .easing(Easing::EaseOut),
            transition_hovered: Animated::new(false)
                .easing(Easing::EaseInOut)
                .duration(400.0),
            text_state: widget::text::State::<Renderer::Paragraph>::default(),
            is_hovered: false,
        })
    }

    // no words
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fixed(self.size),
            height: Length::Fixed(self.size),
        }
    }

    // html?
    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::next_to_each_other(
            limits,
            self.spacing,
            |limits| {
                if let Some(label) = self.label.as_deref() {
                    let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();
                    let max = limits.max();
                    let text_limits = &Limits::new(
                        limits.min(),
                        Size::new(max.width - self.spacing, max.height),
                    );
                    iced::advanced::widget::text::layout(
                        &mut state.text_state,
                        renderer,
                        text_limits,
                        label,
                        widget::text::Format {
                            width: Length::Fill,
                            height: Length::Shrink,
                            line_height: self.text_line_height,
                            size: self.text_size,
                            font: self.font,
                            align_x: self.text_alignment,
                            align_y: alignment::Vertical::Top,
                            shaping: self.text_shaping,
                            wrapping: self.text_wrapping,
                        },
                    )
                } else {
                    layout::Node::new(Size::ZERO)
                }
            },
            // TODO beforepr
            |limits| {
                let max = limits.max();
                let radio_limits = &Limits::new(
                    limits.min(),
                    Size::new(max.width + self.spacing, max.height),
                );
                layout::Node::new(Size::new(radio_limits.max().width, self.size)).align(
                    Alignment::End,
                    Alignment::Center,
                    radio_limits.max(),
                )
            },
        )
    }

    fn update(
        // all of self
        &mut self,
        // tree of self, not the entire app
        tree: &mut Tree,
        // predefined events by iced
        event: &Event,
        // from above
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        // the what?
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let Some(on_toggle) = &self.on_select else {
            return;
        };

        let is_mouse_over = cursor.is_over(layout.bounds());
        let is_selected = self.current == Some(self.value.clone());
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if is_mouse_over {
                    state
                        .transition_selected
                        .transition(!is_selected, Instant::now());
                    shell.request_redraw();
                    shell.publish(on_toggle(self.value.clone()));
                    shell.capture_event();
                }
            }
            _ => {}
        }
        state
            .transition_hovered
            .transition(is_mouse_over, Instant::now());
        shell.request_redraw();

        // why twice?
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

        let animation_progress = state
            .transition_selected
            .animate_bool(0.0, 1.0, Instant::now());
        let hover_animation_progress =
            state
                .transition_hovered
                .animate_bool(0.0, 1.0, Instant::now());
        state.is_hovered = is_mouse_over;
        let current_status = if self.on_select.is_none() {
            Status::Disabled
        } else if is_mouse_over {
            Status::Hovered {
                hovered: is_mouse_over,
                is_toggled: is_selected,
                translate_animation_progress: animation_progress,
                hover_animation_progress,
            }
        } else if is_selected {
            Status::Active {
                is_toggled: is_selected,
                animation_progress,
            }
        } else {
            Status::Hovered {
                hovered: is_mouse_over,
                is_toggled: is_selected,
                translate_animation_progress: animation_progress,
                hover_animation_progress,
            }
        };

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.now = *now;

            // Reset animation on tainted state
            if state.is_animation_state_tainted(is_selected) {
                state
                    .transition_selected
                    .transition_instantaneous(is_selected, Instant::now());
            }

            if state.transition_selected.in_progress(*now) {
                shell.request_redraw();
            }
            self.last_status = Some(current_status);
        } else if self
            .last_status
            .is_some_and(|status| status != current_status)
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            if self.on_select.is_some() {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::NotAllowed
            }
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        /// Makes sure that the border radius of the toggler looks good at every size.
        const BORDER_RADIUS_RATIO: f32 = 32.0 / 13.0;

        /// The space ratio between the background Quad and the OxiRadio bounds, and
        /// between the background Quad and foreground Quad.
        const SPACE_RATIO: f32 = 0.05;

        let mut children = layout.children();

        if self.label.is_some() {
            let label_layout = children.next().unwrap();
            let state: &widget::text::State<Renderer::Paragraph> = &tree
                .state
                .downcast_ref::<State<Renderer::Paragraph>>()
                .text_state;

            iced::widget::text::draw(
                renderer,
                &renderer::Style {
                    text_color: OXITHEME.text,
                },
                label_layout.bounds(),
                state.raw(),
                iced::widget::text::Style {
                    color: Some(OXITHEME.text),
                },
                viewport,
            );
        }
        let toggler_layout = children.next().unwrap();

        let bounds = toggler_layout.bounds();
        let style = theme.style(&self.class, self.last_status.unwrap_or(Status::Disabled));

        let border_radius = bounds.height / BORDER_RADIUS_RATIO;
        let space = SPACE_RATIO * bounds.height;

        let toggler_background_bounds = Rectangle {
            x: bounds.x + space + bounds.width - self.spacing * 2.0,
            y: bounds.y + space,
            width: self.size - (2.0 * space),
            height: bounds.height - (2.0 * space),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: toggler_background_bounds,
                border: Border {
                    radius: border_radius.into(),
                    width: style.background_border_width,
                    color: style.background_border_color,
                },
                ..renderer::Quad::default()
            },
            style.background,
        );

        let toggler_foreground_bounds = Rectangle {
            x: bounds.x + (2.0 * space) + bounds.width - self.spacing * 2.0,
            y: bounds.y + (2.0 * space),
            width: self.size - (4.0 * space),
            height: bounds.height - (4.0 * space),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: toggler_foreground_bounds,
                border: Border {
                    radius: border_radius.into(),
                    width: style.foreground_border_width,
                    color: style.foreground_border_color,
                },
                ..renderer::Quad::default()
            },
            style.foreground,
        );
    }
}

impl<'a, V, Message, Theme, Renderer> From<OxiRadio<'a, V, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    V: PartialEq + Clone + 'a,
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(
        toggler: OxiRadio<'a, V, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(toggler)
    }
}

/// The possible status of a [`OxiRadio`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    /// The [`OxiRadio`] can be interacted with.
    Active {
        /// Indicates whether the [`OxiRadio`] is toggled.
        is_toggled: bool,
        /// Current progress of the transition animation
        animation_progress: f32,
    },
    /// The [`OxiRadio`] is being hovered.
    Hovered {
        hovered: bool,
        /// Indicates whether the [`OxiRadio`] is toggled.
        is_toggled: bool,
        /// Current progress of the transition animation
        translate_animation_progress: f32,
        hover_animation_progress: f32,
    },
    /// The [`OxiRadio`] is disabled.
    Disabled,
}

/// The appearance of a toggler.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The background [`Color`] of the toggler.
    pub background: Color,
    /// The width of the background border of the toggler.
    pub background_border_width: f32,
    /// The [`Color`] of the background border of the toggler.
    pub background_border_color: Color,
    /// The foreground [`Color`] of the toggler.
    pub foreground: Color,
    /// The width of the foreground border of the toggler.
    pub foreground_border_width: f32,
    /// The [`Color`] of the foreground border of the toggler.
    pub foreground_border_color: Color,
    /// The horizontal progress ratio of the foreground bounds of the toggler.
    pub foreground_bounds_horizontal_progress: f32,
}

/// The theme catalog of a [`OxiRadio`].
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`OxiRadio`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of a [`OxiRadio`].
pub fn default(_: &Theme, status: Status) -> Style {
    let background = match status {
        Status::Active {
            is_toggled,
            animation_progress,
        }
        | Status::Hovered {
            is_toggled,
            translate_animation_progress: animation_progress,
            hover_animation_progress: _,
            hovered: _,
        } if is_toggled => mix(
            OXITHEME.primary,
            OXITHEME.secondary_bg,
            1.0 - animation_progress,
        ),
        Status::Active {
            is_toggled: _,
            animation_progress,
        } => mix(OXITHEME.primary_bg, OXITHEME.primary, animation_progress),
        Status::Hovered {
            is_toggled: _,
            translate_animation_progress: _,
            hover_animation_progress,
            hovered: _,
        } => mix(
            OXITHEME.secondary_bg,
            OXITHEME.primary,
            hover_animation_progress,
        ),
        Status::Disabled => OXITHEME.mantle,
    };

    let foreground = match status {
        Status::Hovered {
            is_toggled,
            translate_animation_progress: animation_progress,
            hover_animation_progress: _,
            hovered: _,
        }
        | Status::Active {
            is_toggled,
            animation_progress,
        } if is_toggled => mix(
            OXITHEME.primary,
            OXITHEME.secondary_bg,
            1.0 - animation_progress,
        ),
        Status::Active {
            is_toggled: _,
            animation_progress,
        } => mix(OXITHEME.secondary_bg, OXITHEME.primary, animation_progress),
        Status::Hovered {
            is_toggled: _,
            translate_animation_progress: _,
            hover_animation_progress,
            hovered: _,
        } => mix(
            OXITHEME.secondary_bg,
            darken_color(&OXITHEME.primary, 0.25),
            hover_animation_progress,
        ),
        Status::Disabled => OXITHEME.mantle,
    };

    let foreground_bounds_horizontal_progress = match status {
        Status::Active {
            is_toggled: _,
            animation_progress,
        } => animation_progress,
        Status::Hovered {
            is_toggled: _,
            translate_animation_progress: animation_progress,
            hover_animation_progress: _,
            hovered: _,
        } => animation_progress,
        Status::Disabled => 0.0,
    };

    Style {
        background,
        foreground,
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        background_border_width: 1.0,
        background_border_color: OXITHEME.primary,
        foreground_bounds_horizontal_progress,
    }
}
