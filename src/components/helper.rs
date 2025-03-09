use tuirealm::props::{Alignment, Borders, Color, Style};
use tuirealm::ratatui::widgets::Block;

pub fn get_block<'a>(props: Borders, title: (String, Alignment), focus: bool) -> Block<'a> {
	Block::default()
		.borders(props.sides)
		.border_style(props.style()) // Always use the border style from props
		.border_type(props.modifiers)
		.title(title.0)
		.title_alignment(title.1)
}

