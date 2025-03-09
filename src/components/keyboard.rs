use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextModifiers};
use tuirealm::ratatui::layout::{Constraint, Direction, Layout, Rect};
use tuirealm::{
	AttrValue, Attribute, Component, Event, Frame, MockComponent, NoUserEvent, Props, State,
};

use crate::Msg;

use super::keycap::Keycap;

pub struct Keyboard {
	props: Props,
	label: String,
}

impl Default for Keyboard {
	fn default() -> Self {
		Self {
			props: Props::default(),
			label: "?".to_string(),
		}
	}
}

impl Keyboard {
	pub fn label<S>(mut self, label: S) -> Self
	where
		S: AsRef<str>,
	{
		self.attr(
			Attribute::Text,
			AttrValue::String(label.as_ref().to_string()),
		);
		self
	}

	pub fn new(label: &str) -> Self {
		Self {
			props: Props::default(),
			label: label.to_string(),
		}
	}

	pub fn borders(mut self, b: Borders) -> Self {
		self.attr(Attribute::Borders, AttrValue::Borders(b));
		self
	}
}

impl MockComponent for Keyboard {
	fn view(&mut self, frame: &mut Frame, area: Rect) {
		let text = self.label.to_string();
		let alignment = self
			.props
			.get_or(
				Attribute::TextAlign,
				AttrValue::Alignment(Alignment::Center),
			)
			.unwrap_alignment();
		let foreground = self
			.props
			.get_or(Attribute::Foreground, AttrValue::Color(Color::Red))
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
		let borders = self
			.props
			.get_or(
				Attribute::Borders,
				AttrValue::Borders(Borders::default().modifiers(BorderType::Rounded)),
			)
			.unwrap_borders();
		let title = self
			.props
			.get_or(
				Attribute::Title,
				AttrValue::Title((String::default(), Alignment::Center)),
			)
			.unwrap_title();
		let focus = self
			.props
			.get_or(Attribute::Focus, AttrValue::Flag(false))
			.unwrap_flag();

		// Create horizontal layout for keycaps
		let constraints = vec![Constraint::Length(3); self.label.chars().count()];
		let keycap_area = Layout::default()
			.direction(Direction::Horizontal)
			.constraints(constraints)
			.split(area);

		// Render each character as a Keycap
		for (i, c) in self.label.chars().enumerate() {
			if let Some(chunk) = keycap_area.get(i) {
				let mut keycap = Keycap::new(&c.to_string()).borders(borders.clone());

				keycap.attr(Attribute::Foreground, AttrValue::Color(foreground));
				keycap.attr(Attribute::Background, AttrValue::Color(background));
				keycap.attr(Attribute::TextAlign, AttrValue::Alignment(alignment));
				keycap.attr(Attribute::TextProps, AttrValue::TextModifiers(modifiers));

				keycap.view(frame, *chunk);
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
	fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
		None
	}
}
