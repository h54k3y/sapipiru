pub mod handmade_css_parser {
    use iced::{
        button, text_input, scrollable, Button, Text, TextInput, Column, Scrollable,
        Container, Element, Length, Row, Sandbox,
        Settings, 
    };
    use std::io::Read;
    use std::sync::Mutex;
    use std::clone::Clone;

    /*lazy_static! {
        pub static ref CSSTEXT: Mutex<String> = Mutex::new("".to_string());
    }*/

    pub trait HanleCSSData {
        fn push_link(&mut self, link: String);
        fn push_css_from_link(&mut self, link: String);
        fn get_css_text(&mut self, idx: usize) -> String;
    }

    #[derive(Default)]
    struct CSSData {
        links: Vec<String>,
        css_strs: Vec<String>
    }

    impl HanleCSSData for CSSData {
        fn push_link(&mut self, link: String) {
            self.links.push(link.clone());
            self.push_css_from_link(link);
        }

        fn push_css_from_link(&mut self, link: String) {
            let mut css_str = String::new();
            reqwest::blocking::get(&link).unwrap().read_to_string(&mut css_str);
            self.css_strs.push(css_str);
        }

        fn get_css_text(&mut self, idx: usize) -> String {
            self.css_strs[idx].clone()
        }
    }

    use crate::ui::styling_window;

    /*pub fn get_css(path: String) {
        // println!("start get css"); 
        let mut css_text = String::new();
        reqwest::blocking::get(&path).unwrap().read_to_string(&mut css_text);
        *CSSTEXT.lock().unwrap() = css_text;
        // println!("end get css"); 
    }

    pub fn return_css_text() -> String {
        // println!("start return css"); 
        CSSTEXT.lock().unwrap().clone()
    }*/
}