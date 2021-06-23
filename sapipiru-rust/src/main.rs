mod ui;
mod html_parser;
mod css_parser;
mod render_tree;
use crate::ui::styling_window;

#[macro_use]
extern crate lazy_static;

fn main() {
    styling_window::initialize_window();
}