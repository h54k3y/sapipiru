pub mod handmade_html_parser {
    use std::ascii::AsciiExt;
    //use std::rc::Rc;
    //use std::cell::RefCell;
    use std::convert::TryInto;
    use crate::css_parser::handmade_css_parser;

    const UMAX: usize = usize::MAX;

    #[derive(PartialEq, Eq, Debug)]
    enum TokenType {
        Unknown,
        Doctype,
        StratTag,
        EndTag,
        Comment,
        Text,
        EndOfFile,   //Not used now...
    }

    impl Default for TokenType {
        fn default() -> Self { TokenType::Unknown }
    }

    impl Copy for TokenType {
    }

    impl Clone for TokenType {
        fn clone(&self) -> TokenType { *self }
    }

    /*#[derive(Default, Clone)]
    struct Attributes {
        name: String,
        value: String,
    }*/

    #[derive(Default, Clone)]
    /*struct Token {
        token_type: TokenType,
        tag_name: String,                    // Doctype, StratTag, EndTag
        public_identifier: String,             // Doctype
        system_identifier: String,             // Doctype
        force_quirks_flag: bool,               // Doctype, StratTag, EndTag
        self_closing_flag: bool,               // StratTag, EndTag
        list_of_attributes: Vec<Attributes>,   // StratTag, EndTag
        token_data: String,                    // Comment, Character
    }*/
    struct Token {
        token_type: TokenType,
        tag_name: String,
        token_data: String,
        tag_attribute: Vec<(String, String)>,
    }

    enum TokenizeState {  // TODO: Add state according to https://html.spec.whatwg.org/multipage/parsing.html#data-state
        Data,
        TagOpen,
        TagName,
        TagAttribute,
        EndTagOpen,
    }

    // from https://dom.spec.whatwg.org/#interface-node
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub enum NodeType {
        Element,
        Attribute,
        Text,
        CDataSection,
        EntityReference,
        Entity,
        ProcessingInstruction,
        Comment,
        Document,
        DocumentType,
        DocumentFragment,
        Notation,
    }

    impl Default for NodeType {
        fn default() -> Self { NodeType::Document }
    }

    #[derive(Default, Clone)]
    struct DocumentType {
        name: String,
        public_id: String,
        system_id: String,
    }

    #[derive(Default, Clone)]
    struct DocumentNode {
        url: String,
        document: String,
        compat_mode: String,
        character_set: String,
        charset: String,
        input_encoding: String,
        content_type: String,
        doctype: DocumentType,
    }

    #[derive(Default, Clone)]
    struct DOMTokenList {
        length: i128,
        value: String,
    }

    #[derive(Default, Clone)]
    struct NamedNodeMap {
        length: i128,
    }

    #[derive(Clone)]
    enum ShadowRootMode {
        Open,
        Closed,
    }

    impl Default for ShadowRootMode {
        fn default() -> Self { ShadowRootMode::Open }
    }

    #[derive(Default, Clone)]
    struct ShadowRoot {
        mode: ShadowRootMode,
        //host: Element,
    }

    /*#[derive(Default, Clone)]
    struct ElementNode {
        namespace_uri: String,
        prefix: String,
        local_name: String,
        tag_name: String,
        id: String,
        class_name: String,
        class_list: DOMTokenList,
        slot: String,
        attributes: NamedNodeMap,
        shadow_root: ShadowRoot,
    }*/

    /*enum Node {
        Nil,
        NodeContent {
            node_type: NodeType,
            node_name: String,
            base_uri: String,
            is_connected: bool,
            owner_document: DocumentNode,
            parent_node: Box<Node>,
            parent_element: ElementNode,
            child_nodes: Vec<Box<Node>>,
            first_child: Box<Node>,
            last_child: Box<Node>,
            previous_sibiling: Box<Node>,
            next_sibiling: Box<Node>,
            node_value: String,
            text_content: String,
        }
    }*/

    #[derive(Default, Clone)]
    /*
    struct NodeContent {
        node_type: NodeType,
        node_name: String,
        base_uri: String,
        is_connected: bool,
        owner_document: DocumentNode,
        parent_element: ElementNode,
        node_value: String,
        text_content: String,
    }*/
    pub struct NodeContent {
        node_type: NodeType,
        pub node_name: String,
        node_value: String,
    }

    #[derive(Default, Clone)]
    pub struct DOMNode {
        pub node_content: NodeContent,
        pub this_node_idx: usize,
        parent_node_idx: usize,
        pub child_nodes_idx: Vec<usize>,
        first_child_idx: usize,
        pub last_child_idx: usize,
        previous_sibiling_idx: usize,
        next_sibiling_idx: usize,
    }

    // TODO: will add other modes
    #[derive(PartialEq, Eq, Debug)]
    enum Mode {
        Initial,
        BeforeHTML,
        BeforeHead,
        InHead,
        AfterHead,
        InBody,
        AfterBody,
        AfterAfterBody,
    }


    pub fn parse_html(original_html : &String) -> (Vec<DOMNode>, Vec<String>) {
        //let mut links_vec: Vec<String> = Vec::new();
        let tokens: Vec<Token> = tokenize(&original_html);
        create_DOM_tree(tokens)
        //let mut result = String::new();
        //print_dom_node(&dom_links.0, &mut result);
        //String::from("")
        //print_token(tokens)
    }

    fn tokenize(original_html : &String) -> Vec<Token> {
        // println!("start tokenize"); 
        let mut current_state = TokenizeState::Data;
        let mut current_token: Token = Default::default();
        let mut tokens: Vec<Token> = Vec::new();
        let mut token_end_flag = false;
        let mut is_after_open_tag = false;
        let mut is_attribute_value = false;
        let mut count_comment_end_char: i32 = 0;
        let mut tmp_string: String = Default::default();
        let mut tmp_attribute_strings: (String, String) = Default::default();
        let mut is_previous_space: bool = false;
        for mut i in original_html.as_str().chars() {
            if i == '\n' {
                i = ' ';
            }

            if i == ' ' {
                if is_previous_space == true {
                    continue;
                }
                is_previous_space = true;
            } else {
                is_previous_space = false;
            }

            match current_state {
                TokenizeState::Data => {
                    if i == '<' {
                        token_end_flag = true;
                        current_state = TokenizeState::TagOpen;
                        is_after_open_tag = false;
                    } else if is_after_open_tag {
                        // This is handling the text, push current char to the token data.
                        if current_token.token_type != TokenType::Text {
                            current_token.tag_name = String::from("text");
                            current_token.token_type = TokenType::Text;
                        }
                        current_token.token_data.push(i);
                    }
                }
                TokenizeState::TagOpen => {
                    if i == '/' {
                        current_state = TokenizeState::EndTagOpen;
                    } else {
                        if current_token.token_type != TokenType::StratTag {
                            current_token.token_type = TokenType::StratTag;
                        }
                        is_after_open_tag = true;
                        current_state = TokenizeState::TagName;
                        current_token.tag_name.push(i);
                    }
                }
                TokenizeState::TagName => {
                    if i == '>' {
                        current_state = TokenizeState::Data;
                        token_end_flag = true;
                        current_token.token_type = {
                            if (current_token.token_type == TokenType::StratTag) && current_token.tag_name.to_uppercase().starts_with("!DOCTYPE") {
                                current_token.tag_name = String::from("doctype");
                                TokenType::Doctype
                            } else if ((current_token.token_type == TokenType::StratTag) && /*(current_token.tag_name.to_uppercase().starts_with("COMMENT") ||*/ current_token.tag_name.starts_with("!--")) || (current_token.token_type == TokenType::Comment) {
                                if count_comment_end_char == 2 {
                                    //current_token.token_data = &current_token.tag_name.slice_chars(3,current_token.tag_name.len() - 3);
                                    current_token.token_data = current_token.tag_name;
                                    current_token.tag_name = String::from("comment");
                                    current_token.token_data.remove(0);
                                    current_token.token_data.remove(0);
                                    current_token.token_data.remove(0);
                                    assert_eq!(Some('-'), current_token.token_data.pop());
                                    assert_eq!(Some('-'), current_token.token_data.pop());
                                } else {
                                    // This ">" is part of the comment.
                                    current_state = TokenizeState::TagName;
                                    token_end_flag = false;
                                    current_token.tag_name.push(i);
                                }
                                TokenType::Comment
                            } else {
                                current_token.token_type
                            }
                        };
                    } else if i ==' ' {
                        if !current_token.tag_name.starts_with("!--") && (current_token.token_type != TokenType::Comment) {
                            if current_token.tag_name.to_uppercase().starts_with("!DOCTYPE") {
                                current_token.tag_name = String::from("doctype");
                                current_token.token_type = TokenType::Doctype;
                            } else {
                                current_state = TokenizeState::TagAttribute;
                                is_attribute_value = false;
                                tmp_string = Default::default();
                                tmp_attribute_strings = Default::default();

                            }
                        } else {
                            current_token.tag_name.push(i);
                        }
                    } else {
                        if current_token.token_type == TokenType::Doctype {
                            current_token.token_data.to_lowercase().push(i);
                        } else {
                            // push to tag_name
                            current_token.tag_name.push(i);
                        }
                    }

                    // count continuous comment end tag
                    if i == '-' {
                        count_comment_end_char += 1;
                    } else {
                        count_comment_end_char = 0;
                    }
                }
                TokenizeState::TagAttribute => {
                    if i == '>' {
                        current_state = TokenizeState::Data;
                        token_end_flag = true;
                        is_attribute_value = false;
                        tmp_string = Default::default();
                        tmp_attribute_strings = Default::default();
                    }  else if i =='"' {
                        if is_attribute_value == true {
                            tmp_attribute_strings.1 = tmp_string;
                            current_token.tag_attribute.push(tmp_attribute_strings);
                            tmp_attribute_strings = Default::default();
                            tmp_string = Default::default();
                            is_attribute_value = false;
                        } else {
                            is_attribute_value = true;
                        }
                    } else if (i == '=') && (is_attribute_value == false) {
                        // end attribute name
                        tmp_attribute_strings.0 = tmp_string;
                        tmp_string = Default::default();
                    } else if (i == ' ') && (is_attribute_value == false) {
                        if !tmp_string.is_empty() {
                            tmp_attribute_strings.0 = tmp_string;
                            current_token.tag_attribute.push(tmp_attribute_strings);
                            tmp_attribute_strings = Default::default();
                            tmp_string = Default::default();
                        }
                    } else {
                        tmp_string.push(i);
                    }
                }
                TokenizeState::EndTagOpen => {
                    if current_token.token_type != TokenType::EndTag {
                        current_token.token_type = TokenType::EndTag;
                    }
                    current_state = TokenizeState::TagName;
                    current_token.tag_name.push(i);
                }
            }

            if token_end_flag == true {
                if !current_token.tag_name.is_empty() || !current_token.token_data.is_empty() {
                    // TODO: remove clone if it's possible...
                    tokens.push(current_token.clone());
                    current_token = Default::default();
                }
                token_end_flag = false;
            }
        }

        // add EOF token
        let eof_token: Token = Token {
            token_type: TokenType::EndOfFile,
            tag_name: Default::default(),
            token_data: Default::default(),
            tag_attribute: Vec::new(),
        };
        tokens.push(eof_token);
        // println!("end tokenize"); 
        tokens
    }

    const VOIDTAG: [&str; 23] = ["area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr", "menuitem", 
                                // for compatibility
                                 "basefont", "bgsound", "frame", "keygen",
                                // other
                                 "image", "isindex", "comment", "text"
                                ];

    fn add_new_node(token: &Token, count: &mut usize, node_stack_idx: &mut Vec<usize>, dom_tree: &mut Vec<DOMNode>) {
        let current_node_type = match token.token_type {
            TokenType::Doctype => NodeType::Document,
            TokenType::StratTag => NodeType::Element,
            TokenType::Text => NodeType::Text,
            TokenType::Comment => NodeType::Comment,
            _ => NodeType::default(),
        };

        let content = NodeContent {
            node_type: current_node_type,
            node_name: token.tag_name.to_lowercase(),
            node_value: token.token_data.clone(),
        };

        let mut node = DOMNode {
            node_content: content,
            this_node_idx: *count,
            parent_node_idx: UMAX,
            child_nodes_idx: Vec::new(),
            first_child_idx: UMAX,
            last_child_idx: UMAX,
            previous_sibiling_idx: UMAX,
            next_sibiling_idx: UMAX,
        };

        if !node_stack_idx.is_empty() {
            let last_idx = node_stack_idx[node_stack_idx.len() -1];
            node.parent_node_idx = dom_tree[last_idx].this_node_idx;
            dom_tree[last_idx].child_nodes_idx.push(node.this_node_idx);
            dom_tree[last_idx].first_child_idx = 
                if dom_tree[last_idx].first_child_idx == UMAX {
                    node.this_node_idx
                } else {
                    dom_tree[last_idx].first_child_idx
                };
            if dom_tree[last_idx].last_child_idx != UMAX {
                let last_child_idx = dom_tree[last_idx].last_child_idx;
                dom_tree[last_child_idx].next_sibiling_idx = node.this_node_idx;
                node.previous_sibiling_idx = dom_tree[last_child_idx].this_node_idx;
            }
            dom_tree[last_idx].last_child_idx = node.this_node_idx;
        }

        let mut should_push_stack = true;
        for i in &VOIDTAG {
            if token.tag_name.to_lowercase() == i.to_string() {
                should_push_stack = false;
            }
        }

        if should_push_stack {
            node_stack_idx.push(node.this_node_idx);
        }

        dom_tree.push(node);
        *count += 1;

        if *count != 0 {
            let parent_element_idx = dom_tree.len() - 1;
            // Handle attribute
            for j in &token.tag_attribute {
                let attribute_content = NodeContent {
                    node_type: NodeType::Attribute,
                    node_name: j.0.clone(),
                    node_value: j.1.clone(),
                };
                let mut attribute_node = DOMNode {
                    node_content: attribute_content,
                    this_node_idx: *count,
                    parent_node_idx: UMAX,
                    child_nodes_idx: Vec::new(),
                    first_child_idx: UMAX,
                    last_child_idx: UMAX,
                    previous_sibiling_idx: UMAX,
                    next_sibiling_idx: UMAX,
                };

                attribute_node.parent_node_idx = dom_tree[parent_element_idx].this_node_idx;
                dom_tree[parent_element_idx].child_nodes_idx.push(attribute_node.this_node_idx);
                dom_tree[parent_element_idx].first_child_idx = 
                    if dom_tree[parent_element_idx].first_child_idx == UMAX {
                        attribute_node.this_node_idx
                    } else {
                        dom_tree[parent_element_idx].first_child_idx
                    };
                if dom_tree[parent_element_idx].last_child_idx != UMAX {
                    let last_child_idx = dom_tree[parent_element_idx].last_child_idx;
                    dom_tree[last_child_idx].next_sibiling_idx = attribute_node.this_node_idx;
                    attribute_node.previous_sibiling_idx = dom_tree[last_child_idx].this_node_idx;
                }
                dom_tree[parent_element_idx].last_child_idx = attribute_node.this_node_idx;
                dom_tree.push(attribute_node);
                *count += 1;
            }
        }
    }

    // will follow https://html.spec.whatwg.org/multipage/parsing.html#data-state
    // 13.2.6.4 The rules for parsing tokens in HTML content
    fn create_DOM_tree(tokens: Vec<Token>) -> (Vec<DOMNode>, Vec<String>) {
        // println!("start create DOM tree"); 
        let mut mode = Mode::Initial;
        let mut dom_tree: Vec<DOMNode> = Vec::new();
        let mut node_stack_idx: Vec<usize> = Vec::new();
        let mut links_vec: Vec<String> = Vec::new();
        let mut count = 0;
        for i in &tokens {
            match mode{
                Mode::Initial => {
                    /*println!("Initial: tag name");
                    println!("{}", i.tag_name);*/
                    if i.token_type == TokenType::Doctype {
                        mode = Mode::BeforeHTML;
                        add_new_node(&i, &mut count, &mut node_stack_idx, &mut dom_tree);
                    }
                },
                Mode::BeforeHTML => {
                    if i.token_type == TokenType::StratTag {
                        if i.tag_name.to_lowercase() == "html" {
                            mode = Mode::BeforeHead;
                            add_new_node(&i, &mut count, &mut node_stack_idx, &mut dom_tree);
                        }
                    }
                },
                Mode::BeforeHead => {
                    if i.token_type == TokenType::StratTag {
                        if i.tag_name.to_lowercase() == "head" {
                            mode = Mode::InHead;
                        }
                        add_new_node(&i, &mut count, &mut node_stack_idx, &mut dom_tree);
                    }
                },
                Mode::InHead => {
                    if (i.token_type == TokenType::StratTag) || (i.token_type == TokenType::Text) || (i.token_type == TokenType::Comment) {
                        if i.tag_name.to_lowercase() == "link" {
                            let link = find_css_path(i.clone());
                            if !link.is_empty() {
                                links_vec.push(link);
                            }
                        }
                        add_new_node(&i, &mut count, &mut node_stack_idx, &mut dom_tree);
                    } else if i.token_type == TokenType::EndTag {
                        let last_idx = node_stack_idx[node_stack_idx.len() -1];
                        if dom_tree[last_idx].node_content.node_name == i.tag_name {
                            node_stack_idx.pop();
                        }

                        if i.tag_name.to_lowercase() == "head" {
                            mode = Mode::AfterHead;
                        }
                    } 
                },
                Mode::AfterHead => {
                    if i.token_type == TokenType::StratTag {
                        if i.tag_name.to_lowercase() == "body" {
                            mode = Mode::InBody;
                        }
                        add_new_node(&i, &mut count, &mut node_stack_idx, &mut dom_tree);
                    }
                },
                Mode::InBody => {
                    if (i.token_type == TokenType::StratTag) || (i.token_type == TokenType::Text) || (i.token_type == TokenType::Comment) {
                        add_new_node(&i, &mut count, &mut node_stack_idx, &mut dom_tree);
                    } else if i.token_type == TokenType::EndTag {
                        let last_idx = node_stack_idx[node_stack_idx.len() -1];
                        if dom_tree[last_idx].node_content.node_name == i.tag_name {
                            node_stack_idx.pop();
                        }

                        if i.tag_name.to_lowercase() == "body" {
                            mode = Mode::AfterBody;
                        }
                    }
                },
                Mode::AfterBody => {
                    if i.token_type == TokenType::EndTag {
                        let last_idx = node_stack_idx[node_stack_idx.len() -1];
                        if dom_tree[last_idx].node_content.node_name == i.tag_name {
                            node_stack_idx.pop();
                        }

                        if i.tag_name.to_lowercase() == "html" {
                            mode = Mode::AfterAfterBody;
                        }
                    }
                },
                Mode::AfterAfterBody => {
                    if i.token_type == TokenType::EndOfFile {
                    }
                }
            };
        }
        (dom_tree, links_vec)
    }

    fn find_css_path(token: Token) -> String {
        let mut is_stylesheet: bool = false;
        let mut herf_link: String = Default::default();
        for i in token.tag_attribute {
            if (i.0.to_lowercase() == "rel") && (i.1.to_lowercase() == "stylesheet") {
                is_stylesheet = true;
            } else if i.0.to_lowercase() == "href" {
                herf_link = i.1;
            }
        }

        if is_stylesheet == false {
            herf_link = Default::default();
        }
        herf_link
    }

    fn convert_tokentype_to_string(token_type: TokenType) -> String {
        let result: String = match token_type {
            TokenType::Unknown => String::from("Unknown"),
            TokenType::Doctype => String::from("Doctype"),
            TokenType::StratTag => String::from("StratTag"),
            TokenType::EndTag => String::from("EndTag"),
            TokenType::Comment => String::from("Comment"),
            TokenType::Text => String::from("Text"),
            TokenType::EndOfFile => String::from("EndOfFile")
        };
        result
    }

    fn print_token(tokens: Vec<Token>) -> String {
        let mut result = String::new();
        let mut count: i32 = 0;
        for i in &tokens {
            result.push_str(&count.to_string());
            result.push_str(":   ");
            result.push_str("TokenType-> ");
            result.push_str(&convert_tokentype_to_string(i.token_type));
            result.push_str("   TagName-> ");
            result.push_str(&i.tag_name);
            result.push_str("   TagAttributes-> ");
                for j in &i.tag_attribute {
                    result.push_str(" [");
                    result.push_str("type: ");
                    result.push_str(&j.0);
                    result.push_str(",  data: ");
                    result.push_str(&j.1);
                    result.push_str("] ");
                };
            result.push_str("   TokenData-> ");
            result.push_str(&i.token_data);
            result.push_str("\n");
            count += 1;
        };
        //println!("{}", result);   //for debug
        result
    }
    
    fn print_dom_node(tree: &Vec<DOMNode>, result: &mut String) {
        //print_dom_vec(&tree);
        for i in tree {
            let idx: i32 = i.this_node_idx.try_into().unwrap();
            result.push_str("\n");
            result.push_str("\n");
            result.push_str("PARENT ");
            result.push_str("[ ");
            result.push_str("index: ");
            result.push_str(&idx.to_string());
            result.push_str(", ");
            result.push_str("name: ");
            result.push_str(&i.node_content.node_name);
            result.push_str(", ");
            result.push_str("value: ");
            result.push_str(&i.node_content.node_value);
            result.push_str(" ]");
            result.push_str("\n");
            let child_count: i128 = i.child_nodes_idx.len().try_into().unwrap();
            result.push_str("CHILD_COUNT: ");
            result.push_str(&child_count.to_string());
            result.push_str("\n");
            result.push_str("CHILDS  ");
            result.push_str("[ ");
            for j in &i.child_nodes_idx {
                result.push_str(&j.to_string());
                //result.push_str(": ");
                //result.push_str(&tree[j.clone()].node_content.node_name);
                result.push_str(", ");
                //result.push_str(&tree[j.clone()].node_content.node_value);
            }
            result.push_str(" ]");
        }
    }

    fn print_dom_vec(tree: &Vec<DOMNode>) {
        for i in tree {
            let idx: i32 = i.this_node_idx.try_into().unwrap();
            println!("{}", idx.to_string()); 
            println!(": \n"); 
            if !i.child_nodes_idx.is_empty() {
                println!("childs\n");
                for j in i.child_nodes_idx.clone() {
                    let j_idx: i32 = j.try_into().unwrap();
                    println!("{}", j_idx.to_string()); 
                }
            } else {
                println!("NO child\n");
            }
        }
    }
}