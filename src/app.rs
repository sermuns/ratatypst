use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{Block, Paragraph, Widget},
};

pub struct App {
    focus: Focus,
    num_steps: usize,
}

#[derive(PartialEq, Debug)]
enum Focus {
    TopRight,
    TopLeft,
    Bottom,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl App {
    pub fn from_input_str(input: &str) -> Self {
        let mut s = Self {
            focus: Focus::Bottom,
            num_steps: input.chars().count(),
        };
        for c in input.chars() {
            match c {
                'j' | 'w' => s.change_focus(Direction::Down),
                'h' | 'a' => s.change_focus(Direction::Left),
                'k' | 's' => s.change_focus(Direction::Up),
                'l' | 'd' => s.change_focus(Direction::Right),
                _ => {}
            }
        }
        s
    }

    fn change_focus(&mut self, direction: Direction) {
        match self.focus {
            Focus::TopRight => match direction {
                Direction::Down => self.focus = Focus::Bottom,
                Direction::Left => self.focus = Focus::TopLeft,
                _ => {}
            },
            Focus::TopLeft => match direction {
                Direction::Down => self.focus = Focus::Bottom,
                Direction::Right => self.focus = Focus::TopRight,
                _ => {}
            },
            Focus::Bottom => match direction {
                Direction::Up => self.focus = Focus::TopLeft,
                Direction::Right => self.focus = Focus::TopRight,
                _ => {}
            },
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(" ratatypst ")
            .title_bottom(format!(" Steps processed: {} ", self.num_steps))
            .title_alignment(HorizontalAlignment::Center);

        (&block).render(area, buf);

        let [top, bottom] = block
            .inner(area)
            .layout(&Layout::vertical([Constraint::Fill(1); 2]));

        let [top_left, top_right] = top.layout(&Layout::horizontal([Constraint::Fill(1); 2]));

        for (area, focus) in [
            (top_left, Focus::TopLeft),
            (top_right, Focus::TopRight),
            (bottom, Focus::Bottom),
        ] {
            let block = if focus == self.focus {
                Block::bordered().border_set(border::THICK)
            } else {
                Block::bordered()
            };
            Paragraph::new(format!("{:?}", focus))
                .block(block)
                .render(area, buf);
        }
    }
}
