mod backend;

use backend::TypstBackend;
use ratatui::widgets::Block;
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

initiate_protocol!();

#[wasm_func]
pub fn run(width: &[u8], height: &[u8]) -> Vec<u8> {
    let width = u16::from_le_bytes(width.try_into().unwrap());
    let height = u16::from_le_bytes(height.try_into().unwrap());

    let mut terminal = ratatui::Terminal::new(TypstBackend::new(width, height)).unwrap();

    terminal
        .draw(|frame| frame.render_widget(Block::bordered().title("fuck"), frame.area()))
        .unwrap();

    terminal.backend().to_vec()
}
