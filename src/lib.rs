mod backend;

use backend::TypstBackend;
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

initiate_protocol!();

#[wasm_func]
pub fn run() -> Vec<u8> {
    let mut terminal = ratatui::Terminal::new(TypstBackend::new(20, 10)).unwrap();

    terminal
        .draw(|frame| frame.render_widget("hello", frame.area()))
        .unwrap();

    terminal.backend().to_vec()
}
