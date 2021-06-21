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
    pub struct Rule {
        selectors: Vec<Selector>,
        declarations: Vec<Declaration>,
        comment: String
    }

    #[derive(Default, Clone)]
    pub struct RuleAsString {
        selector: String,
        declaration: String
    }

    pub struct Stylesheet {
        rules: Vec<Rule>,
    }

    #[derive(PartialEq, Eq, Debug, Clone)]
    pub enum SelectorType {
        Element,
        Id, // #
        Class, //.
        Universal, // *
        Attribute, // []
        Colon, // :
        Desendants, // " "
        Child, // >
        NextSibiling, // +
        SubsequentSibling, // ~
        Pseudo, // ::
        Column // ||
    }

    // https://developer.mozilla.org/ja/docs/Web/CSS/CSS_Selectors
    #[derive(Default, Clone)]
    pub struct Selector {
        tag_name: String,
        element: String,
        base: String,
        class: Vec<String>,
        id: String,
        attribute: Vec<String>,
        join_node: Vec<(SelectorType, String)>,
        after_colon: Vec<String>,
        is_universal: bool
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
        fn parse_selector_and_declarations(& mut self, idx: usize) -> Vec<RuleAsString>;
        fn parse_css(&mut self, idx: usize) -> Vec<Rule>;
    }

    #[derive(Default)]
    pub struct CSSData {
        current_dir: String,
        links: Vec<String>,
        css_strs: Vec<String>
    }

    pub fn handle_selector(selector_str: String) ->Selector {
        let mut selector: Selector = Default::default();
        let mut selector_type: SelectorType = SelectorType::Element;
        let mut tmp_class: String = String::new();
        let mut tmp_string: String = String::new();
        for i in selector_str.chars() {
            match i {
                '#' => {
                    if !tmp_string.is_empty() {
                        if selector_type == SelectorType::Id {
                            selector.id = tmp_string.clone();
                        } else if selector_type == SelectorType::Class {
                            selector.class.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Attribute {
                            selector.attribute.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Colon {
                            selector.after_colon.push(tmp_string.clone());
                        } else if (selector_type == SelectorType::Desendants) || (selector_type == SelectorType::Child) 
                        || (selector_type == SelectorType::SubsequentSibling) || (selector_type == SelectorType::NextSibiling) {
                            selector.join_node.push((selector_type.clone(), tmp_string.clone()));
                        } else if selector_type == SelectorType::Element {
                            selector.base = tmp_string.clone();
                        }
                        tmp_string = String::new();
                    }
                    selector_type = SelectorType::Id;
                    continue;
                }
                '.' => {
                    if !tmp_string.is_empty() {
                        if selector_type == SelectorType::Id {
                            selector.id = tmp_string.clone();
                        } else if selector_type == SelectorType::Class {
                            selector.class.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Attribute {
                            selector.attribute.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Colon {
                            selector.after_colon.push(tmp_string.clone());
                        } else if (selector_type == SelectorType::Desendants) || (selector_type == SelectorType::Child) 
                        || (selector_type == SelectorType::SubsequentSibling) || (selector_type == SelectorType::NextSibiling) {
                            selector.join_node.push((selector_type.clone(), tmp_string.clone()));
                        } else if selector_type == SelectorType::Element {
                            selector.base = tmp_string.clone();
                        }
                        tmp_string = String::new();
                    }
                    selector_type = SelectorType::Class;
                    continue;
                }
                '*' => {
                    selector.is_universal = true;
                    if !tmp_string.is_empty() {
                        if selector_type == SelectorType::Id {
                            selector.id = tmp_string.clone();
                        } else if selector_type == SelectorType::Class {
                            selector.class.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Attribute {
                            selector.attribute.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Colon {
                            selector.after_colon.push(tmp_string.clone());
                        } else if (selector_type == SelectorType::Desendants) || (selector_type == SelectorType::Child) 
                        || (selector_type == SelectorType::SubsequentSibling) || (selector_type == SelectorType::NextSibiling) {
                            selector.join_node.push((selector_type.clone(), tmp_string.clone()));
                        } else if selector_type == SelectorType::Element {
                            selector.base = tmp_string.clone();
                        }
                        tmp_string = String::new();
                    }
                    selector_type = SelectorType::Element;
                    continue;
                }
                '[' => {
                    if !tmp_string.is_empty() {
                        if selector_type == SelectorType::Id {
                            selector.id = tmp_string.clone();
                        } else if selector_type == SelectorType::Class {
                            selector.class.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Attribute {
                            selector.attribute.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Colon {
                            selector.after_colon.push(tmp_string.clone());
                        } else if (selector_type == SelectorType::Desendants) || (selector_type == SelectorType::Child) 
                        || (selector_type == SelectorType::SubsequentSibling) || (selector_type == SelectorType::NextSibiling) {
                            selector.join_node.push((selector_type.clone(), tmp_string.clone()));
                        } else if selector_type == SelectorType::Element {
                            selector.base = tmp_string.clone();
                        }
                        tmp_string = String::new();
                    }
                    selector_type = SelectorType::Attribute;
                }
                ']' => {
                    // do nothing
                    selector_type = SelectorType::Element;
                }
                ':' => {
                    if !tmp_string.is_empty() {
                        if selector_type == SelectorType::Id {
                            selector.id = tmp_string.clone();
                        } else if selector_type == SelectorType::Class {
                            selector.class.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Attribute {
                            selector.attribute.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Colon {
                            selector.after_colon.push(tmp_string.clone());
                        } else if (selector_type == SelectorType::Desendants) || (selector_type == SelectorType::Child) 
                        || (selector_type == SelectorType::SubsequentSibling) || (selector_type == SelectorType::NextSibiling) {
                            selector.join_node.push((selector_type.clone(), tmp_string.clone()));
                        } else if selector_type == SelectorType::Element {
                            selector.base = tmp_string.clone();
                        }
                        tmp_string = String::new();
                    }
                    selector_type = SelectorType::Colon;
                }
                ' ' => {
                    if !tmp_string.is_empty() {
                        if selector_type == SelectorType::Id {
                            selector.id = tmp_string.clone();
                        } else if selector_type == SelectorType::Class {
                            selector.class.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Attribute {
                            selector.attribute.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Colon {
                            selector.after_colon.push(tmp_string.clone());
                        } else if (selector_type == SelectorType::Desendants) || (selector_type == SelectorType::Child) 
                        || (selector_type == SelectorType::SubsequentSibling) || (selector_type == SelectorType::NextSibiling) {
                            selector.join_node.push((selector_type.clone(), tmp_string.clone()));
                        } else if selector_type == SelectorType::Element {
                            selector.base = tmp_string.clone();
                        }
                        tmp_string = String::new();
                    }
                    selector_type = SelectorType::Desendants;
                }
                '>' => {
                    if !tmp_string.is_empty() {
                        if selector_type == SelectorType::Id {
                            selector.id = tmp_string.clone();
                        } else if selector_type == SelectorType::Class {
                            selector.class.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Attribute {
                            selector.attribute.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Colon {
                            selector.after_colon.push(tmp_string.clone());
                        } else if (selector_type == SelectorType::Desendants) || (selector_type == SelectorType::Child) 
                        || (selector_type == SelectorType::SubsequentSibling) || (selector_type == SelectorType::NextSibiling) {
                            selector.join_node.push((selector_type.clone(), tmp_string.clone()));
                        } else if selector_type == SelectorType::Element {
                            selector.base = tmp_string.clone();
                        }
                        tmp_string = String::new();
                    }
                    selector_type = SelectorType::Child;
                }
                '+' => {
                    if !tmp_string.is_empty() {
                        if selector_type == SelectorType::Id {
                            selector.id = tmp_string.clone();
                        } else if selector_type == SelectorType::Class {
                            selector.class.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Attribute {
                            selector.attribute.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Colon {
                            selector.after_colon.push(tmp_string.clone());
                        } else if (selector_type == SelectorType::Desendants) || (selector_type == SelectorType::Child) 
                        || (selector_type == SelectorType::SubsequentSibling) || (selector_type == SelectorType::NextSibiling) {
                            selector.join_node.push((selector_type.clone(), tmp_string.clone()));
                        } else if selector_type == SelectorType::Element {
                            selector.base = tmp_string.clone();
                        }
                        tmp_string = String::new();
                    }
                    selector_type = SelectorType::SubsequentSibling;
                }
                '~' => {
                    if !tmp_string.is_empty() {
                        if selector_type == SelectorType::Id {
                            selector.id = tmp_string.clone();
                        } else if selector_type == SelectorType::Class {
                            selector.class.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Attribute {
                            selector.attribute.push(tmp_string.clone());
                        } else if selector_type == SelectorType::Colon {
                            selector.after_colon.push(tmp_string.clone());
                        } else if (selector_type == SelectorType::Desendants) || (selector_type == SelectorType::Child) 
                        || (selector_type == SelectorType::SubsequentSibling) || (selector_type == SelectorType::NextSibiling) {
                            selector.join_node.push((selector_type.clone(), tmp_string.clone()));
                        } else if selector_type == SelectorType::Element {
                            selector.base = tmp_string.clone();
                        }
                        tmp_string = String::new();
                    }
                    selector_type = SelectorType::NextSibiling;
                }
                _ => {
                    tmp_string.push(i.clone());
                }
            }
        }
        
        if !tmp_string.is_empty() {
            if selector_type == SelectorType::Id {
                selector.id = tmp_string.clone();
            } else if selector_type == SelectorType::Class {
                selector.class.push(tmp_string.clone());
            } else if selector_type == SelectorType::Attribute {
                selector.attribute.push(tmp_string.clone());
            } else if selector_type == SelectorType::Colon {
                selector.after_colon.push(tmp_string.clone());
            } else if (selector_type == SelectorType::Desendants) || (selector_type == SelectorType::Child) 
            || (selector_type == SelectorType::SubsequentSibling) || (selector_type == SelectorType::NextSibiling) {
                selector.join_node.push((selector_type.clone(), tmp_string.clone()));
            } else if selector_type == SelectorType::Element {
                selector.base = tmp_string.clone();
            }
        }

        selector.element = selector_str.clone();
        selector
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
                println!("{}\n\n", &format_link);
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

        fn parse_selector_and_declarations(& mut self, idx: usize) -> Vec<RuleAsString> {
            let mut result = Vec::new();
            let mut tmp_str = String::new();
            let mut comment_str = String::new();
            let mut current_selectors: Vec<String> = Vec::new();
            let mut declaration_vec: Vec<String> = Vec::new();
            let mut stack_for_nest: Vec<Vec<String>> = Vec::new();
            let mut is_previous_slash: bool = false;
            let mut is_previous_asterisk: bool = false;
            let mut is_previous_space: bool = false;
            let mut is_comment: bool = false;
            for i in self.css_strs[idx].chars() {
                
            }
            result
        }

        fn parse_css(& mut self, idx: usize) -> Vec<Rule> {
            let mut result = Vec::new();
            let mut tmp_str = String::new();
            let mut comment_str = String::new();
            let mut current_selectors: Vec<String> = Vec::new();
            let mut declaration_vec = Vec::new();
            let mut stack_for_nest: Vec<Vec<String>> = Vec::new();
            let mut is_previous_slash: bool = false;
            let mut is_previous_asterisk: bool = false;
            let mut is_previous_space: bool = false;
            let mut is_comment: bool = false;
            for i in self.css_strs[idx].chars() {
                if !is_comment && ((is_previous_space && (i == ' ')) || (i == '\n')) {
                    // do nothing
                } else {
                    if is_comment == true {
                        if (i == '/') && (is_previous_asterisk == true) {
                            is_comment = false;
                            comment_str.pop();
                            let mut new_node: Rule = Default::default();
                            new_node.comment = comment_str.clone();
                            result.push(new_node);
                            continue;
                        } else {
                            comment_str.push(i);
                        }
                    } else if (i == '*') && (is_previous_slash == true) {
                        is_comment = true;
                        comment_str = String::new();
                        tmp_str.pop();
                        continue;
                    } else if i == '{' {
                        let mut selector_str = String::new();
                        let mut count_in_selector = 0;
                        let mut is_previous_space_j = false;
                        // handling selector
                        for j in tmp_str.chars() {
                            if j == ',' {
                                count_in_selector = 0;
                                if !selector_str.is_empty() && is_previous_space_j {
                                    selector_str.pop();
                                }
                                current_selectors.push(selector_str);
                                selector_str = String::new();
                            } else {
                                if (count_in_selector != 0) || (j != ' ') {
                                    selector_str.push(j);
                                }
                                is_previous_space_j = j == ' ';
                                count_in_selector += 1;
                            }
                        }

                        if !selector_str.is_empty() {
                            if is_previous_space_j {
                                selector_str.pop();
                            }
                            current_selectors.push(selector_str);
                        }
    
                        if !current_selectors.is_empty() {
                            stack_for_nest.push(current_selectors);
                        }
                        tmp_str = String::new();
                        current_selectors = Vec::new();
                    } else if (i == ';') || (i == '}') {
                        let mut current_declaration: Declaration = Default::default();
                        if !tmp_str.is_empty() {
                            let mut after_colon: bool = false;
                            let mut count_in_declaration = 0;
                            let mut is_previous_space_j = false;
                            for j in tmp_str.chars() {
                                if after_colon {
                                    if (count_in_declaration != 0) || (j != ' ') {
                                        current_declaration.value.push(j);
                                    }
                                    count_in_declaration += 1;
                                } else {
                                    if j == ':' {
                                        after_colon = true;
                                        count_in_declaration = 0;
                                        if !current_declaration.propery.is_empty() && is_previous_space_j {
                                            current_declaration.propery.pop();
                                        }
                                    } else {
                                        if (count_in_declaration != 0) || (j != ' ') {
                                            current_declaration.propery.push(j);
                                        }
                                        count_in_declaration += 1;
                                    }
                                }
                                is_previous_space_j = j ==' ';
                            }

                            if !current_declaration.value.is_empty() && is_previous_space_j {
                                current_declaration.value.pop();
                            }

                            if !current_declaration.propery.is_empty() && !current_declaration.value.is_empty() {
                                declaration_vec.push(current_declaration.clone());
                            }
                        }
                        tmp_str = String::new();

                        if i == '}' {
                            let mut new_node: Rule = Default::default();
                            if !stack_for_nest.is_empty() {
                                for j in stack_for_nest[stack_for_nest.len()-1].clone() {
                                    new_node.selectors.push(handle_selector(j.clone()));
                                }
                                stack_for_nest.pop();
                            } else {
                                println!("NO SELECTOR");
                            }
                            new_node.declarations = declaration_vec;
                            if !new_node.selectors.is_empty() || !new_node.declarations.is_empty() {
                                result.push(new_node);
                            }
                            declaration_vec = Vec::new();
                        }
                    } else {
                        tmp_str.push(i.clone());
                    }
    
                    is_previous_asterisk = i == '*';
                    is_previous_slash = i == '/';
                    is_previous_space = i == ' ';
                } 
            }
            
            // for debug
            for i in &result {
                println!("SELECTORS:");
                for j in &i.selectors {
                    println!("element:{}", &j.element);
                    //println!("base:{}, ", &j.base);
                    print!("class:");
                    for k in &j.class {
                        print!("{}, ", &k);
                    }
                    println!("\n");
                    println!("id:{}", &j.id);
                    print!("attribute:");
                    for k in &j.attribute {
                        print!("{}, ", &k);
                    }
                    println!("\n");
                    print!("join_node:");
                    for k in &j.join_node {
                        print!("{}, ", &k.1);
                    }
                    println!("\n");
                    print!("after_colon:");
                    for k in &j.after_colon {
                        print!("{}, ", &k);
                    }
                    println!("\n");
                    println!("is_universal:{}", &j.is_universal);
                }
                println!("\n\nDECLARATIONS:");
                for j in &i.declarations {
                    println!("property:{}", &j.propery);
                    println!("value:{}", &j.value);
                    println!("\n");
                }
                println!("COMMENT:");
                println!("{}", &i.comment);
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