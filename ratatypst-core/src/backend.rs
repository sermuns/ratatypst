use core::{
    cmp::max,
    fmt::{self, Write},
    iter,
};
use ratatui::{
    backend::{ClearType, WindowSize},
    buffer::Cell,
    prelude::*,
};
use unicode_width::UnicodeWidthStr;

pub struct TypstBackend {
    buffer: Buffer,
    scrollback: Buffer,
    pos: Position,
}

impl fmt::Display for TypstBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("set par(leading: 0pt);")?;

        for cells in self.buffer.content.chunks(self.buffer.area.width as usize) {
            let mut overwritten = vec![];
            let mut skip: usize = 0;
            for (x, c) in cells.iter().enumerate() {
                if skip == 0 {
                    write!(f, r#"text(fill: {}, raw("{}"));"#, c.fg, c.symbol())?;
                } else {
                    overwritten.push((x, c.symbol()));
                }
                skip = max(skip, c.symbol().width()).saturating_sub(1);
            }
            if !overwritten.is_empty() {
                f.write_str(" Hidden by multi-width symbols: {overwritten:?}")?;
            }

            f.write_str("linebreak();")?;
        }

        Ok(())
    }
}

impl TypstBackend {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            buffer: Buffer::empty(Rect::new(0, 0, width, height)),
            scrollback: Buffer::empty(Rect::new(0, 0, width, 0)),
            pos: Position::new(0, 0),
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl ratatui::backend::Backend for TypstBackend {
    type Error = core::convert::Infallible;

    fn draw<'a, I>(&mut self, content: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>,
    {
        for (x, y, c) in content {
            self.buffer[(x, y)] = c.clone();
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn get_cursor_position(&mut self) -> Result<Position, Self::Error> {
        Ok(self.pos)
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> Result<(), Self::Error> {
        self.pos = position.into();
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        self.buffer.reset();
        Ok(())
    }

    fn clear_region(&mut self, clear_type: ClearType) -> Result<(), Self::Error> {
        let region = match clear_type {
            ClearType::All => return self.clear(),
            ClearType::AfterCursor => {
                let index = self.buffer.index_of(self.pos.x, self.pos.y) + 1;
                &mut self.buffer.content[index..]
            }
            ClearType::BeforeCursor => {
                let index = self.buffer.index_of(self.pos.x, self.pos.y);
                &mut self.buffer.content[..index]
            }
            ClearType::CurrentLine => {
                let line_start_index = self.buffer.index_of(0, self.pos.y);
                let line_end_index = self.buffer.index_of(self.buffer.area.width - 1, self.pos.y);
                &mut self.buffer.content[line_start_index..=line_end_index]
            }
            ClearType::UntilNewLine => {
                let index = self.buffer.index_of(self.pos.x, self.pos.y);
                let line_end_index = self.buffer.index_of(self.buffer.area.width - 1, self.pos.y);
                &mut self.buffer.content[index..=line_end_index]
            }
        };
        for cell in region {
            cell.reset();
        }
        Ok(())
    }

    /// Inserts n line breaks at the current cursor position.
    ///
    /// After the insertion, the cursor x position will be incremented by 1 (unless it's already
    /// at the end of line). This is a common behaviour of terminals in raw mode.
    ///
    /// If the number of lines to append is fewer than the number of lines in the buffer after the
    /// cursor y position then the cursor is moved down by n rows.
    ///
    /// If the number of lines to append is greater than the number of lines in the buffer after
    /// the cursor y position then that number of empty lines (at most the buffer's height in this
    /// case but this limit is instead replaced with scrolling in most backend implementations) will
    /// be added after the current position and the cursor will be moved to the last row.
    fn append_lines(&mut self, line_count: u16) -> Result<(), Self::Error> {
        let Position { x: cur_x, y: cur_y } = self.get_cursor_position()?;
        let Rect { width, height, .. } = self.buffer.area;

        // the next column ensuring that we don't go past the last column
        let new_cursor_x = cur_x.saturating_add(1).min(width.saturating_sub(1));

        let max_y = height.saturating_sub(1);
        let lines_after_cursor = max_y.saturating_sub(cur_y);

        if line_count > lines_after_cursor {
            // We need to insert blank lines at the bottom and scroll the lines from the top into
            // scrollback.
            let scroll_by: usize = (line_count - lines_after_cursor).into();
            let width: usize = self.buffer.area.width.into();
            let cells_to_scrollback = self.buffer.content.len().min(width * scroll_by);

            append_to_scrollback(
                &mut self.scrollback,
                self.buffer.content.splice(
                    0..cells_to_scrollback,
                    iter::repeat_with(Default::default).take(cells_to_scrollback),
                ),
            );
            self.buffer.content.rotate_left(cells_to_scrollback);
            append_to_scrollback(
                &mut self.scrollback,
                iter::repeat_with(Default::default).take(width * scroll_by - cells_to_scrollback),
            );
        }

        let new_cursor_y = cur_y.saturating_add(line_count).min(max_y);
        self.set_cursor_position(Position::new(new_cursor_x, new_cursor_y))?;

        Ok(())
    }

    fn size(&self) -> Result<Size, Self::Error> {
        Ok(self.buffer.area.as_size())
    }

    fn window_size(&mut self) -> Result<WindowSize, Self::Error> {
        Ok(WindowSize {
            columns_rows: self.buffer.area.as_size(),
            pixels: Size::ZERO, // does it even matter?
        })
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Append the provided cells to the bottom of a scrollback buffer. The number of cells must be a
/// multiple of the buffer's width. If the scrollback buffer ends up larger than 65535 lines tall,
/// then lines will be removed from the top to get it down to size.
fn append_to_scrollback(scrollback: &mut Buffer, cells: impl IntoIterator<Item = Cell>) {
    scrollback.content.extend(cells);
    let width = scrollback.area.width as usize;
    let new_height = (scrollback.content.len() / width).min(u16::MAX as usize);
    let keep_from = scrollback
        .content
        .len()
        .saturating_sub(width * u16::MAX as usize);
    scrollback.content.drain(0..keep_from);
    scrollback.area.height = new_height as u16;
}
