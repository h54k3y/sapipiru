pub mod handmade_css_parser {
    use iced::{
        button, text_input, scrollable, Button, Text, TextInput, Column, Scrollable,
        Container, Element, Length, Row, Sandbox,
        Settings, 
    };
    use std::io::Read;
    use std::sync::Mutex;
    use std::clone::Clone;
    use std::convert::TryInto;

    /*lazy_static! {
        pub static ref CSSTEXT: Mutex<String> = Mutex::new("".to_string());
    }*/

    pub trait HandleCSSData {
        fn push_original_url(&mut self, url: String);
        fn push_link(&mut self, link: String);
        fn push_css_from_link(&mut self, link: String);
        fn get_css_text(&mut self, idx: usize) -> String;
        fn handle_link_format(&mut self, link: String) -> String;
    }

    #[derive(Default)]
    pub struct CSSData {
        current_dir: String,
        links: Vec<String>,
        css_strs: Vec<String>
    }

    impl HandleCSSData for CSSData {
        fn push_original_url(&mut self, url: String) {
            let mut count = 0;
            for c in url.chars() {
                if c == '\\' {
                    count = 0;
                } else {
                    count += 1;
                }
            }

            for i in 0..count {
                url.pop();
            }

            self.current_dir = url;
        }

        fn push_link(&mut self, link: String) {
            let format_link = self.handle_link_format(link);
            self.links.push(format_link.clone());
            self.push_css_from_link(format_link);
        }

        fn push_css_from_link(&mut self, link: String) {
            let mut css_str = String::new();
            reqwest::blocking::get(&link).unwrap().read_to_string(&mut css_str);
            self.css_strs.push(css_str);
        }

        fn get_css_text(&mut self, idx: usize) -> String {
            self.css_strs[idx].clone()
        }

        fn handle_link_format(&mut self, link: String) -> String {
            let mut result = String::new();
            if link.starts_with("http://") || link.starts_with("https://") {
                result = link.clone();
            } else {
                let mut str_vec: Vec<&str> = self.current_dir.split('/').collect();
                let mut tmp_cnt = 0;
                let mut up_cnt = 0;
                for c in link.chars() {
                    if c == '.' {
                        tmp_cnt += 1;
                    } else if  c == '/' {
                        if tmp_cnt >= 2 {
                            up_cnt += 1;
                            tmp_cnt = 0;
                        }
                    }
                }
                let last_count = str_vec.len() - up_cnt;
                let mut idx = 0;
                for i in str_vec {
                    if idx == last_count {
                        break;
                    }
                    result.push_str(i);
                    idx += 1;
                }
                result.push_str(&link.clone());
            }
            result
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