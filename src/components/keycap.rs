use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{Alignment, BorderType, Borders, Color, Style, TextModifiers};
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::{
	AttrValue, Attribute, Component, Event, Frame, MockComponent, NoUserEvent, Props, State,
};

use crate::Msg;

use super::helper;

pub struct Keycap {
	props: Props,
	label: String,
}

impl Default for Keycap {
	fn default() -> Self {
		Self {
			props: Props::default(),
			label: "?".to_string(),
		}
	}
}

impl Keycap {
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

	pub fn foreground(mut self, color: Color) -> Self {
		self.attr(Attribute::Foreground, AttrValue::Color(color));
		self
	}

	pub fn background(mut self, color: Color) -> Self {
		self.attr(Attribute::Background, AttrValue::Color(color));
		self
	}

	pub fn alignment(mut self, alignment: Alignment) -> Self {
		self.attr(Attribute::TextAlign, AttrValue::Alignment(alignment));
		self
	}

	pub fn modifiers(mut self, modifiers: TextModifiers) -> Self {
		self.attr(Attribute::TextProps, AttrValue::TextModifiers(modifiers));
		self
	}
}

impl MockComponent for Keycap {
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

		frame.render_widget(
			Paragraph::new(text)
				.block(helper::get_block(borders, title, focus))
				.style(
					Style::default()
						.fg(foreground)
						.bg(background)
						.add_modifier(modifiers),
				)
				.alignment(alignment),
			area,
		);
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

impl Component<Msg, NoUserEvent> for Keycap {
	fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
		None
	}
}
