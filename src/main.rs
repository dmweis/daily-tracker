mod dot;

use std::collections::HashMap;

use dot::Dot;
use iced::{
    scrollable, Align, Background, Color, Column, Container, Element, Length, Row, Sandbox,
    Scrollable, Settings, Space, Text,
};

const DAYS_IN_MONTHS: [u32; 12] = [31, 28, 31, 30, 31, 31, 30, 31, 30, 31, 30, 31];
const MONTHS: [&str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];
const DOT_SIZE: f32 = 20.0;

const POSITIVE_COLOR: Color = Color::from_rgb(0.0, 1.0, 0.0);
const NEGATIVE_COLOR: Color = Color::from_rgb(1.0, 0.0, 0.0);
const MISSING: Color = Color::WHITE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DayState {
    None,
    Positive,
    Negative,
}

impl DayState {
    fn next(&self) -> DayState {
        match *self {
            DayState::None => DayState::Positive,
            DayState::Positive => DayState::Negative,
            DayState::Negative => DayState::Positive,
        }
    }
}

impl From<DayState> for Color {
    fn from(day_state: DayState) -> Self {
        match day_state {
            DayState::None => MISSING,
            DayState::Positive => POSITIVE_COLOR,
            DayState::Negative => NEGATIVE_COLOR,
        }
    }
}

#[derive(Default)]
struct CalendarTracker {
    day_states: HashMap<(u32, u32), DayState>,
}

impl CalendarTracker {
    fn get_day(&self, day: (u32, u32)) -> DayState {
        *self.day_states.get(&day).unwrap_or(&DayState::None)
    }

    fn set_day(&mut self, day: (u32, u32), day_state: DayState) {
        self.day_states.insert(day, day_state);
    }
}

#[derive(Default)]
struct DailyTracker {
    scroller_state: scrollable::State,
    calendar: CalendarTracker,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DayPressed(u32, u32),
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

    fn update(&mut self, message: Self::Message) {
        let Message::DayPressed(month, day) = message;
        let current = self.calendar.get_day((month, day));
        let next = current.next();
        self.calendar.set_day((month, day), next);
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let mut row = Row::new().width(Length::Shrink);

        for (month, day_count) in DAYS_IN_MONTHS.iter().enumerate() {
            let month_name = MONTHS[month];
            let mut month_column = Column::new()
                .align_items(Align::Center)
                .push(Text::new(month_name.to_owned()).size(30));
            for day in 1..*day_count {
                let date = (month as u32, day);
                let day_state = self.calendar.get_day(date);
                let day = Dot::new(
                    DOT_SIZE,
                    day_state.into(),
                    Message::DayPressed(month as u32, day),
                );

                month_column = month_column
                    .push(day)
                    .push(Space::new(Length::Shrink, Length::Units(8)));
            }
            row = row
                .push(month_column)
                .push(Space::new(Length::Units(10), Length::Shrink));
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

    #[test]
    fn test_next_day_states() {
        let start = DayState::None;
        let first = start.next();
        assert_eq!(first, DayState::Positive);
        let second = first.next();
        assert_eq!(second, DayState::Negative);
    }

    #[test]
    fn calendar_starts_none() {
        let calendar = CalendarTracker::default();
        assert_eq!(calendar.get_day((0, 0)), DayState::None);
        assert_eq!(calendar.get_day((10, 10)), DayState::None);
    }

    #[test]
    fn calendar_starts_correct() {
        let mut calendar = CalendarTracker::default();
        calendar.set_day((0, 0), DayState::Positive);
        assert_eq!(calendar.get_day((0, 0)), DayState::Positive);
    }
}
