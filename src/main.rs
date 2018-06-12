extern crate cursive;

mod model;
mod ui;

fn main() {
    let mut ui = ui::create_ui();
    ui.run();
}