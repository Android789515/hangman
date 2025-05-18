mod util;
mod app;
mod figure;

use std::error::Error;

use app::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();

    let mut app = App::init();
    let run_result = app.run(&mut terminal);

    ratatui::restore();

    run_result
}
