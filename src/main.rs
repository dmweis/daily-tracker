mod dot;

use dot::Dot;
use iced::{
    button, scrollable, Align, Background, Color, Column, Container, Element, Length, Row, Sandbox,
    Scrollable, Settings, Space, Text,
};

const DAYS_IN_MONTHS: [u32; 12] = [31, 28, 31, 30, 31, 31, 30, 31, 30, 31, 30, 31];
const MONTHS: [&str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];

#[derive(Debug, Default)]
struct DailyTracker {
    scroller_state: scrollable::State,
    button_state: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DayPressed(i32, i32),
}

fn main() -> iced::Result {
    DailyTracker::run(Settings::default())
}

impl Sandbox for DailyTracker {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Daily tracker")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&mut self) -> Element<'_, Self::Message> {
        let mut row = Row::new().width(Length::Shrink);

        for (month, day_count) in DAYS_IN_MONTHS.iter().enumerate() {
            let month_name = MONTHS[month];
            let mut month_column = Column::new()
                .align_items(Align::Center)
                .push(Text::new(month_name.to_owned()).size(30));
            for day in 1..*day_count {
                let day = Dot::new(
                    20.0,
                    Color::from_rgb(1.0, 0.0, 1.0),
                    Message::DayPressed(month as i32, day as i32),
                );

                month_column = month_column
                    .push(day)
                    .push(Space::new(Length::Shrink, Length::Units(10)));
            }
            row = row
                .push(month_column)
                .push(Space::new(Length::Units(20), Length::Shrink));
        }

        let scrollable = Scrollable::new(&mut self.scroller_state)
            .align_items(Align::Center)
            .width(Length::Fill)
            .height(Length::Shrink)
            .push(row)
            .style(DarkStyle);

        Container::new(scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(DarkStyle)
            .into()
    }
}

struct DarkStyle;

impl iced::container::StyleSheet for DarkStyle {
    fn style(&self) -> iced::container::Style {
        iced::container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x36, 0x39, 0x3F))),
            text_color: Some(Color::WHITE),
            ..iced::container::Style::default()
        }
    }
}

impl scrollable::StyleSheet for DarkStyle {
    fn active(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(Background::Color(SURFACE)),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: ACTIVE,
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> scrollable::Scrollbar {
        let active = self.active();

        scrollable::Scrollbar {
            background: Some(Background::Color(Color { a: 0.5, ..SURFACE })),
            scroller: scrollable::Scroller {
                color: HOVERED,
                ..active.scroller
            },
            ..active
        }
    }

    fn dragging(&self) -> scrollable::Scrollbar {
        let hovered = self.hovered();

        scrollable::Scrollbar {
            scroller: scrollable::Scroller {
                color: Color::from_rgb(0.85, 0.85, 0.85),
                ..hovered.scroller
            },
            ..hovered
        }
    }
}

const SURFACE: Color = Color::from_rgb(
    0x40 as f32 / 255.0,
    0x44 as f32 / 255.0,
    0x4B as f32 / 255.0,
);

const ACTIVE: Color = Color::from_rgb(
    0x72 as f32 / 255.0,
    0x89 as f32 / 255.0,
    0xDA as f32 / 255.0,
);

const HOVERED: Color = Color::from_rgb(
    0x67 as f32 / 255.0,
    0x7B as f32 / 255.0,
    0xC4 as f32 / 255.0,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn months_days_same_length() {
        assert_eq!(DAYS_IN_MONTHS.len(), MONTHS.len());
    }
}
