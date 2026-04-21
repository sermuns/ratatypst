mod app;
mod backend;

use backend::TypstBackend;
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

use crate::app::App;

initiate_protocol!();

#[wasm_func]
pub fn run(width: &[u8], height: &[u8], input: &[u8]) -> Vec<u8> {
    let width = u16::from_le_bytes(width.try_into().expect("width is not u16"));
    let height = u16::from_le_bytes(height.try_into().expect("height is not u16"));

    let mut terminal = ratatui::Terminal::new(TypstBackend::new(width, height)).unwrap();

    let app = App::from_input_str(str::from_utf8(input).expect("valid utf-8 input str"));

    terminal
        .draw(|frame| frame.render_widget(&app, frame.area()))
        .unwrap();

    terminal.backend().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let width = 10u16.to_le_bytes();
        let height = 5u16.to_le_bytes();
        let input = b"jjhhll";

        let output = run(&width, &height, input);
        assert!(!output.is_empty());
    }
}
