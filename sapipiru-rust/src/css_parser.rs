pub mod handmade_css_parser {
    use iced::{
        button, text_input, scrollable, Button, Text, TextInput, Column, Scrollable,
        Container, Element, Length, Row, Sandbox,
        Settings, 
    };
    use std::io::Read;

    lazy_static! {
        pub static ref CSSTEXT: String = String::new();
    }

    use crate::ui::styling_window;

    pub fn get_css(path: String) {
        let mut css_text = String::new();
        reqwest::blocking::get(&path).unwrap().read_to_string(&mut css_text);
        *CSSTEXT = css_text;
    }
}