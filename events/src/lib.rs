use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
	Top,
	Bottom,
	Center,
}

#[derive(Debug, Eq, PartialEq)]
 pub struct Notification {
	pub size: u32,
	pub color: (u8, u8, u8),
	pub position: Position,
	pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
	Remainder(&'a str),
	Registration(Duration),
	Appointment(&'a str),
	Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        let col_content = self.content.truecolor(self.color.0, self.color.1, self.color.2);
        write!(f, "({:?}, {}, {})", self.position, self.size, col_content)
    }
}

use Event::*;

impl Event <'_>{
	pub fn notify(&self) -> Notification {
        match self {
            Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            Registration(duration) => {
                let total_seconds = duration.num_seconds();
                let hours = total_seconds / 3600;
                let remaining_seconds_after_hours = total_seconds % 3600;
                let minutes = remaining_seconds_after_hours / 60;
                let seconds = remaining_seconds_after_hours % 60;

                let content = format!("You have {:02}H:{:02}M:{:02}S left before the registration ends", hours, minutes, seconds);
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content,
                }
            }
            Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),
            },
            Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
	}
}