use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::{layout::Position, widgets::Widget};

pub struct App {
    pos: Position,
}

impl App {
    pub fn from_input_str(input: &str) -> Self {
        let mut pos = Position::default();
        for c in input.chars() {
            match c {
                'j' | 'w' => pos.y = pos.y.saturating_sub(1),
                'h' | 'a' => pos.x = pos.x.saturating_sub(1),
                'k' | 's' => pos.y = pos.y.saturating_add(1),
                'l' | 'd' => pos.x = pos.x.saturating_add(1),
                _ => {}
            }
        }
        Self { pos }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered().title("ratatypst").render(area, buf);

        let Position { x, y } = self.pos;
        buf.set_string(x, y, "x", Style::default());
    }
}
