pub mod handmade_css_parser {
    use iced::{
        button, text_input, scrollable, Button, Text, TextInput, Column, Scrollable,
        Container, Element, Length, Row, Sandbox,
        Settings, 
    };
    use std::io::Read;
    use std::sync::Mutex;
    use std::clone::Clone;

    lazy_static! {
        pub static ref CSSTEXT: Mutex<String> = Mutex::new("".to_string());
    }

    use crate::ui::styling_window;

    pub fn get_css(path: String) {
        println!("start get css"); 
        let mut css_text = String::new();
        reqwest::blocking::get(&path).unwrap().read_to_string(&mut css_text);
        *CSSTEXT.lock().unwrap() = css_text;
        println!("end get css"); 
    }

    pub fn return_css_text() -> String {
        println!("start return css"); 
        CSSTEXT.lock().unwrap().clone()
    }
}