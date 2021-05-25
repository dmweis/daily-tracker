use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{event, Clipboard, Event};
use iced_native::{
    layout, mouse, Background, Color, Element, Hasher, Layout, Length, Point, Rectangle, Size,
    Widget,
};

pub struct Dot<Message> {
    radius: f32,
    color: iced::Color,
    on_press: Message,
    hovered: bool,
}

impl<Message> Dot<Message> {
    pub fn new(radius: f32, color: iced::Color, on_press: Message) -> Self {
        Dot {
            radius,
            color,
            on_press,
            hovered: false,
        }
    }
}

impl<Message, B> Widget<Message, Renderer<B>> for Dot<Message>
where
    Message: Clone,
    B: Backend,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &Renderer<B>, _limits: &layout::Limits) -> layout::Node {
        layout::Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer<B>,
        _clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        let bounds = layout.bounds();
        // check if we need hovered decorations
        if bounds.contains(cursor_position) {
            self.hovered = true;
        } else {
            self.hovered = false;
        }
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if bounds.contains(cursor_position) {
                messages.push(self.on_press.clone());
                return event::Status::Captured;
            }
        }
        // hovering doesn't mean we capture the event
        event::Status::Ignored
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;
        self.radius.to_bits().hash(state);
        self.hovered.hash(state);
        Hash::hash_slice(
            &self
                .color
                .into_linear()
                .iter()
                .map(|num| *num as i32)
                .collect::<Vec<_>>(),
            state,
        );
    }

    fn draw(
        &self,
        _renderer: &mut Renderer<B>,
        _defaults: &Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) -> (Primitive, mouse::Interaction) {
        let border_radius = if self.hovered { self.radius * 0.2 } else { 0.0 };
        (
            Primitive::Quad {
                bounds: layout.bounds(),
                background: Background::Color(self.color),
                border_radius: self.radius,
                border_width: border_radius,
                border_color: Color::BLACK,
            },
            mouse::Interaction::default(),
        )
    }
}

impl<'a, Message, B> From<Dot<Message>> for Element<'a, Message, Renderer<B>>
where
    Message: 'a + Clone,
    B: Backend,
{
    fn from(element: Dot<Message>) -> Element<'a, Message, Renderer<B>> {
        Element::new(element)
    }
}
