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

    #[derive(Default, Clone)]
    pub struct CSSOMNode {
        selector: Vec<String>,
        declarations: Vec<Declaration>
    }

    struct Selector {

    }

    #[derive(Default, Clone)]
    pub struct Declaration {
        propery: String,
        value: String
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum Mode {
        Selector,
        DeclarationProperty,
        DeclarationValue
    }

    pub trait HandleCSSData {
        fn push_original_url(&mut self, url: String);
        fn push_links(&mut self, links_vec: Vec<String>);
        fn push_css_from_link(&mut self, link: String);
        fn get_css_text(&mut self, idx: usize) -> String;
        fn handle_link_format(&mut self, link: String) -> String;
        fn parse_css(& mut self, idx: usize) -> Vec<CSSOMNode>;
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
                if c == '/' {
                    count = 0;
                } else {
                    count += 1;
                }
            }

            let mut cp_url = url.clone();
            for i in 0..(count + 1) {
                cp_url.pop();
            }

            self.current_dir = cp_url;
        }

        fn push_links(&mut self, links_vec: Vec<String>) {
            for i in links_vec {
                println!("Before");
                println!("{}\n", &i);
                let format_link = self.handle_link_format(i);
                println!("After");
                println!("{}", &format_link);
                self.links.push(format_link.clone());
                self.push_css_from_link(format_link);
            }
        }

        fn push_css_from_link(&mut self, link: String) {
            let mut css_str = String::new();
            reqwest::blocking::get(&link).unwrap().read_to_string(&mut css_str);
            self.css_strs.push(css_str);
        }

        fn get_css_text(&mut self, idx: usize) -> String {
            if self.css_strs.is_empty() {
                println!("Empty");
                let empty_str = Default::default();
                return empty_str
            }
            self.parse_css(idx);
            self.css_strs[idx].clone()
        }

        fn handle_link_format(&mut self, link: String) -> String {
            let mut result = String::new();
            if link.starts_with("http://") || link.starts_with("https://") {
                result = link.clone();
            } else {
                if link.starts_with("/") {
                    let cur_vec: Vec<&str> = self.current_dir.split('/').collect();
                    let mut idx = 0;
                    for i in cur_vec {
                        if idx == 3 {
                            break;
                        }
                        result.push_str(i);

                        if idx != 2 {
                            result.push('/');
                        }
                        idx += 1;
                    }
                    result.push_str(&link.clone());
                } else {
                    result.push_str(&self.current_dir.clone());
                    result.push('/');
                    result.push_str(&link.clone());
                }
            }
            result
        }

        fn parse_css(& mut self, idx: usize) -> Vec<CSSOMNode> {
            let mut result = Vec::new();
            let mut cur_mode = Mode::Selector;
            let mut cur_node: CSSOMNode = Default::default();
            let mut tmp_str = String::new();
            let mut dec_property_str = String::new();
            let mut dec_value_str = String::new();
            let mut cur_declaration: Declaration = Default::default();
            let mut declaration_vec = Vec::new();
            for i in self.css_strs[idx].chars() {
                match cur_mode {
                    Mode::Selector => {
                        if i == '{' {
                            cur_node.selector.push(tmp_str);
                            tmp_str = String::new();
                            cur_mode = Mode::DeclarationProperty;
                            cur_declaration = Default::default();
                            declaration_vec = Vec::new();
                            dec_property_str = String::new();
                            dec_value_str = String::new();
                        } else if i ==',' {
                            cur_node.selector.push(tmp_str);
                            tmp_str = String::new();
                        }else {
                            tmp_str.push(i);
                        }
                    },
                    Mode::DeclarationProperty => {
                        if i == ':' {
                            cur_declaration.propery = dec_property_str;
                            dec_property_str = String::new();
                            cur_mode = Mode::DeclarationValue;
                        } else if i == '}' {
                            cur_node.declarations = declaration_vec.clone();
                            result.push(cur_node);
                            declaration_vec = Vec::new();
                            dec_property_str = String::new();
                            cur_node = Default::default();
                            cur_mode = Mode::Selector;
                        } else {
                            dec_property_str.push(i.clone());
                        }
                    }
                    Mode::DeclarationValue => {
                        if (i == ';') /*|| (i == '}')*/ {
                            cur_declaration.value = dec_value_str;
                            declaration_vec.push(cur_declaration.clone());
                            /*if i == '}' {
                                cur_node.declarations = declaration_vec.clone();
                                result.push(cur_node);
                                declaration_vec = Vec::new();
                                dec_property_str = String::new();
                                cur_node = Default::default();
                                cur_mode = Mode::Selector;
                            } else {
                                cur_mode = Mode::DeclarationProperty;
                            }*/
                            cur_mode = Mode::DeclarationProperty;
                            cur_declaration = Default::default();
                            dec_value_str = String::new();
                        } else {
                            dec_value_str.push(i.clone());
                        }
                    }
                }
            }

            // for debug
            for i in &result {
                print!("SELECTOR: ");
                for j in &i.selector {
                    print!("{},  ", &j);
                }
                println!("\n\nDECLARATIONS: ");
                for j in &i.declarations {
                    print!("property: {}", &j.propery);
                    println!(",    value: {}", &j.value);
                }
                println!("\n");
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