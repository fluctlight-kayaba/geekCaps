use tuirealm::command::{Cmd, CmdResult};
use tuirealm::event::{Key, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextModifiers};
use tuirealm::ratatui::layout::{Constraint, Direction, Layout, Rect};
use tuirealm::{
	AttrValue, Attribute, Component, Event, Frame, MockComponent, NoUserEvent, Props, State,
};

use crate::Msg;

use super::keycap::Keycap;

pub struct Keyboard {
	props: Props,
	keycaps: Vec<Vec<Keycap>>,
}

impl Default for Keyboard {
	fn default() -> Self {
		Self {
			props: Props::default(),
			keycaps: vec![vec![Keycap::new("?")]],
		}
	}
}

impl Keyboard {
	pub fn new() -> Self {
		let qwerty_layout = vec![
			vec![
				"Esc", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "\\", "`",
			],
			vec![
				"Tab:7", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "Bs:8",
			],
			vec![
				"Caps:9", "A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'", "Enter:11",
			],
			vec![
				"Shift:11", "Z", "X", "C", "V", "B", "N", "M", ",", ".", "/", "Shift:9", "Fn",
			],
			vec![":8", "Alt", "Cmd:9", "Space:27", "Cmd:9", "Alt"],
		];

		let mut keycaps = Vec::new();
		for row in qwerty_layout {
			let mut keycap_row = Vec::new();
			for key in row {
				keycap_row.push(Keycap::new(key));
			}
			keycaps.push(keycap_row);
		}

		Self {
			props: Props::default(),
			keycaps,
		}
	}

	pub fn with_custom_layout(layout: Vec<Vec<&str>>) -> Self {
		let mut keycaps = Vec::new();
		for row in layout {
			let mut keycap_row = Vec::new();
			for key in row {
				keycap_row.push(Keycap::new(key));
			}
			keycaps.push(keycap_row);
		}

		Self {
			props: Props::default(),
			keycaps,
		}
	}

	pub fn borders(mut self, b: Borders) -> Self {
		self.attr(Attribute::Borders, AttrValue::Borders(b));
		self
	}

	pub fn foreground(mut self, color: Color) -> Self {
		self.attr(Attribute::Foreground, AttrValue::Color(color));
		self
	}

	pub fn background(mut self, color: Color) -> Self {
		self.attr(Attribute::Background, AttrValue::Color(color));
		self
	}

	fn highlight_keycap(&mut self, label: &str) -> bool {
		for row in self.keycaps.iter_mut() {
			for keycap in row.iter_mut() {
				let mut borders = keycap
					.query(Attribute::Borders)
					.unwrap_or(AttrValue::Borders(
						Borders::default().modifiers(BorderType::Rounded),
					))
					.unwrap_borders();

				let label = keycap.get_label();
				if label.starts_with(':') && label[1..].chars().all(|c| c.is_digit(10)) {
					let transparent_borders = Borders::default().color(Color::Black); // Use black color for invisible borders
					keycap.attr(Attribute::Borders, AttrValue::Borders(transparent_borders));
				} else {
					borders = borders.color(Color::DarkGray);
					keycap.attr(Attribute::Borders, AttrValue::Borders(borders));
				}
			}
		}

		if label.is_empty() {
			return false;
		}

		for row in self.keycaps.iter_mut() {
			for keycap in row.iter_mut() {
				if keycap.get_display_label().to_lowercase() == label.to_lowercase() {
					let mut borders = keycap
						.query(Attribute::Borders)
						.unwrap_or(AttrValue::Borders(
							Borders::default().modifiers(BorderType::Rounded),
						))
						.unwrap_borders();

					borders = borders.color(Color::Yellow);
					keycap.attr(Attribute::Borders, AttrValue::Borders(borders));

					return true;
				}
			}
		}

		false
	}
}

impl MockComponent for Keyboard {
	fn view(&mut self, frame: &mut Frame, area: Rect) {
		let alignment = self
			.props
			.get_or(
				Attribute::TextAlign,
				AttrValue::Alignment(Alignment::Center),
			)
			.unwrap_alignment();
		let foreground = self
			.props
			.get_or(Attribute::Foreground, AttrValue::Color(Color::White)) // Default to white for better visibility
			.unwrap_color();
		let background = self
			.props
			.get_or(Attribute::Background, AttrValue::Color(Color::Reset))
			.unwrap_color();
		let modifiers = self
			.props
			.get_or(
				Attribute::TextProps,
				AttrValue::TextModifiers(TextModifiers::empty()),
			)
			.unwrap_text_modifiers();
		let default_borders = self
			.props
			.get_or(
				Attribute::Borders,
				AttrValue::Borders(
					Borders::default()
						.modifiers(BorderType::Rounded)
						.color(Color::DarkGray),
				),
			)
			.unwrap_borders();
		let _title = self
			.props
			.get_or(
				Attribute::Title,
				AttrValue::Title((String::default(), Alignment::Center)),
			)
			.unwrap_title();
		let _focus = self
			.props
			.get_or(Attribute::Focus, AttrValue::Flag(false))
			.unwrap_flag();

		let row_count = self.keycaps.len();
		// Use original size
		let row_constraints = vec![Constraint::Length(3); row_count];
		let row_chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints(row_constraints)
			.split(area);

		for (row_idx, row) in self.keycaps.iter_mut().enumerate() {
			if let Some(row_chunk) = row_chunks.get(row_idx) {
				let keycap_count = row.len();
				// Create constraints based on key labels with widths
				let mut keycap_constraints = Vec::with_capacity(keycap_count);
				for keycap in row.iter() {
					let label = keycap.get_label();

					// Parse width from labels like "key:width"
					if let Some(pos) = label.find(':') {
						if let Ok(width) = label[pos + 1..].parse::<u16>() {
							keycap_constraints.push(Constraint::Length(width));
						} else {
							keycap_constraints.push(Constraint::Length(5)); // Default if parsing fails
						}
					} else {
						keycap_constraints.push(Constraint::Length(5)); // Default width
					}
				}

				let keycap_chunks = Layout::default()
					.direction(Direction::Horizontal)
					.constraints(keycap_constraints)
					.split(*row_chunk);

				for (col_idx, keycap) in row.iter_mut().enumerate() {
					if let Some(keycap_chunk) = keycap_chunks.get(col_idx) {
						// Important: Set Text attribute from the label for display
						// Use display label (without width suffix) for rendering
						let display_label = keycap.get_display_label();
						keycap.attr(Attribute::Text, AttrValue::String(display_label));

						// Set text-related properties
						keycap.attr(Attribute::TextAlign, AttrValue::Alignment(alignment));
						keycap.attr(Attribute::TextProps, AttrValue::TextModifiers(modifiers));

						// Set colors - keeping foreground white for better visibility
						keycap.attr(Attribute::Foreground, AttrValue::Color(foreground));
						keycap.attr(Attribute::Background, AttrValue::Color(background));

						// Ensure keycaps have rounded borders (if not already set)
						if keycap.query(Attribute::Borders).is_none() {
							// Check if this is a spacer keycap (starting with ":") or the space key
							let label = keycap.get_label();
							// Check for pure spacer keycap or space key
							if label.starts_with(':') && label[1..].chars().all(|c| c.is_digit(10)) {
								// For spacer keycaps and space key, use transparent borders (no visible borders)
								let transparent_borders = Borders::default().color(Color::Black); // Use black color for invisible borders
								keycap.attr(Attribute::Borders, AttrValue::Borders(transparent_borders));
							} else {
								keycap.attr(
									Attribute::Borders,
									AttrValue::Borders(default_borders.clone()),
								);
							}
						}

						keycap.view(frame, *keycap_chunk);
					}
				}
			}
		}
	}

	fn query(&self, attr: Attribute) -> Option<AttrValue> {
		self.props.get(attr)
	}

	fn attr(&mut self, attr: Attribute, value: AttrValue) {
		self.props.set(attr, value);
	}

	fn state(&self) -> State {
		State::None
	}

	fn perform(&mut self, _: Cmd) -> CmdResult {
		CmdResult::None
	}
}

impl Component<Msg, NoUserEvent> for Keyboard {
	fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
		match ev {
			Event::Keyboard(key_event) => {
				if key_event.code == Key::Esc && key_event.modifiers == KeyModifiers::NONE {
					return Some(Msg::AppClose);
				}

				let key_str = match key_event.code {
					Key::Char(' ') => "space".to_string(),
					Key::Char(ch) => ch.to_string(),
					Key::Backspace => "Bs".to_string(),
					Key::Enter => "Enter".to_string(),
					Key::Left => "←".to_string(),
					Key::Right => "→".to_string(),
					Key::Up => "↑".to_string(),
					Key::Down => "↓".to_string(),
					Key::Tab => "tab".to_string(),
					_ => String::new(), // Empty for unknown keys
				};

				let mut modified = false;

				// First clear all highlights
				self.highlight_keycap(""); // Will reset all keycaps but not find any to highlight

				// Then apply new highlights
				if key_event.modifiers.contains(KeyModifiers::ALT) {
					self.highlight_keycap("Alt");
					modified = true;
				}
				if key_event.modifiers.contains(KeyModifiers::CONTROL) {
					self.highlight_keycap("Caps");
					modified = true;
				}
				if key_event.modifiers.contains(KeyModifiers::SHIFT) {
					self.highlight_keycap("Shift");
					modified = true;
				}

				if !key_str.is_empty() {
					self.highlight_keycap(&key_str);
					return Some(Msg::KeyPressed(key_str));
				} else if modified {
					return Some(Msg::KeyPressed("modifier".to_string()));
				}

				// Always return a message when keys are pressed to ensure redraw happens
				Some(Msg::KeyPressed("".to_string()))
			}
			_ => None,
		}
	}
}
