use std::time::Duration;

use tuirealm::event::NoUserEvent;
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge};
use tuirealm::{Application, AttrValue, Attribute, EventListenerCfg, Update};

use super::{Id, Msg};
use crate::components::Keyboard;

pub struct Model<T>
where
	T: TerminalAdapter,
{
	pub app: Application<Id, Msg, NoUserEvent>,
	pub quit: bool,
	pub redraw: bool,
	pub terminal: TerminalBridge<T>,
}

impl Default for Model<CrosstermTerminalAdapter> {
	fn default() -> Self {
		Self {
			app: Self::init_app(),
			quit: false,
			redraw: true,
			terminal: TerminalBridge::init_crossterm().expect("Cannot initialize terminal"),
		}
	}
}

impl<T> Model<T>
where
	T: TerminalAdapter,
{
	pub fn view(&mut self) {
		assert!(self
			.terminal
			.draw(|f| {
				let chunks = Layout::default()
					.direction(Direction::Vertical)
					.margin(1)
					.constraints([Constraint::Length(18)].as_ref())
					.split(f.area());

				self.app.view(&Id::Keyboard, f, chunks[0]);
			})
			.is_ok());
	}

	fn init_app() -> Application<Id, Msg, NoUserEvent> {
		let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
			EventListenerCfg::default()
				.crossterm_input_listener(Duration::from_millis(25), 3)
				.poll_timeout(Duration::from_millis(10))
				.tick_interval(Duration::from_secs(1)),
		);

		assert!(app
			.mount(Id::Keyboard, Box::new(Keyboard::new()), Vec::new(),)
			.is_ok());

		assert!(app.active(&Id::Keyboard).is_ok());
		app
	}
}

impl<T> Update<Msg> for Model<T>
where
	T: TerminalAdapter,
{
	fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
		if let Some(msg) = msg {
			// Always set redraw flag to true when a message is received
			self.redraw = true;

			match msg {
				Msg::AppClose => {
					self.quit = true;
					None
				}
				Msg::CounterChanged(v) => {
					assert!(self
						.app
						.attr(
							&Id::SimpleCounter,
							Attribute::Text,
							AttrValue::String(format!("Counter: {}", v))
						)
						.is_ok());
					None
				}
				Msg::KeyPressed(_key) => {
					// We already updated the keycap in the keyboard component
					// Force redraw when any key is pressed
					self.redraw = true;
					None
				}
			}
		} else {
			None
		}
	}
}
