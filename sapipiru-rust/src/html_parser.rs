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
    enum NodeType {
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
    struct NodeContent {
        node_type: NodeType,
        node_name: String,
        node_value: String,
    }

    #[derive(Default, Clone)]
    struct DOMNode {
        node_content: NodeContent,
        this_node_idx: usize,
        parent_node_idx: usize,
        child_nodes_idx: Vec<usize>,
        first_child_idx: usize,
        last_child_idx: usize,
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


    pub fn parse_html(original_html : &String) -> String {
        let tokens: Vec<Token> = tokenize(&original_html);
        let dom_nodes = create_DOM_tree(tokens);
        let mut result = String::new();
        print_dom_node(0, &dom_nodes, &mut result);
        result
        //String::from("")
        // print_token(tokens)
    }

    fn tokenize(original_html : &String) -> Vec<Token> {
        // println!("start tokenize"); 
        let mut current_state = TokenizeState::Data;
        let mut current_token: Token = Default::default();
        let mut tokens: Vec<Token> = Vec::new();
        let mut token_end_flag = false;
        let mut is_after_open_tag = false;
        let mut count_comment_end_char: i32 = 0;
        let mut tmp_string: String = Default::default();
        let mut tmp_attribute_strings: (String, String) = Default::default();
        for i in original_html.as_str().chars() {
            match current_state {
                TokenizeState::Data => {
                    if i == '<' {
                        token_end_flag = true;
                        current_state = TokenizeState::TagOpen;
                        is_after_open_tag = false;
                    } else if is_after_open_tag {
                        // This is handling the text, push current char to the token data.
                        if current_token.token_type != TokenType::Text {
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
                            current_token.token_type = 
                                if current_token.tag_name.to_uppercase().starts_with("!DOCTYPE") {
                                    current_token.tag_name = String::from("doctype");
                                    TokenType::Doctype
                                } else {
                                    current_token.token_type
                                };
                            current_state = TokenizeState::TagAttribute;
                        } else {
                            current_token.tag_name.push(i);
                        }
                    } else {
                        // push to tag_name
                        current_token.tag_name.push(i);
                    }

                    // count continuous comment end tag
                    if i == '-' {
                        count_comment_end_char += 1;
                    } else {
                        count_comment_end_char = 0;
                    }
                }
                TokenizeState::TagAttribute => {
                    // duplicated code, need to refactor.
                    if i == '>' {
                        current_state = TokenizeState::Data;
                        token_end_flag = true;
                        tmp_string = Default::default();
                        tmp_attribute_strings = Default::default();
                    } else if i == '=' {
                        tmp_attribute_strings.0 = tmp_string;
                        tmp_string = Default::default();
                    } else if i =='"' {
                        if !tmp_string.is_empty() {
                            tmp_attribute_strings.1 = tmp_string;
                            current_token.tag_attribute.push(tmp_attribute_strings);
                            tmp_attribute_strings = Default::default();
                            tmp_string = Default::default();
                        }
                    } else if i !=' ' {
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

    // will follow https://html.spec.whatwg.org/multipage/parsing.html#data-state
    // 13.2.6.4 The rules for parsing tokens in HTML content
    fn create_DOM_tree(tokens: Vec<Token>) -> Vec<DOMNode> {
        // println!("start create DOM tree"); 
        let mut mode = Mode::Initial;
        let mut dom_tree: Vec<DOMNode> = Vec::new();
        let mut idx_of_node_stack: Vec<usize> = Vec::new();
        let mut count = 0;
        for i in &tokens {
            match i.token_type {
                TokenType::Doctype => {
                    if mode == Mode::Initial {
                        let content = NodeContent {
                            node_type: NodeType::DocumentType,
                            node_name: i.token_data.clone(),
                            node_value: Default::default(),
                        };
        
                        let node = DOMNode {
                            node_content: content,
                            this_node_idx: count,
                            parent_node_idx: UMAX,
                            child_nodes_idx: Vec::new(),
                            first_child_idx: UMAX,
                            last_child_idx: UMAX,
                            previous_sibiling_idx: UMAX,
                            next_sibiling_idx: UMAX,
                        };
                        // println!("{}", node.node_content.node_name);
                        idx_of_node_stack.push(node.this_node_idx);
                        dom_tree.push(node);
                        count += 1;
                        mode = Mode::BeforeHTML;
                    }
                },
                TokenType::StratTag | TokenType::Text | TokenType::Comment=> {
                    if (mode == Mode::InHead) && (i.tag_name.to_lowercase() == "link") {
                        find_css_path(i.clone());
                    }

                    let current_node_type = match i.token_type {
                        TokenType::StratTag => NodeType::Element,
                        TokenType::Text => NodeType::Text,
                        TokenType::Comment => NodeType::Comment,
                        _ => NodeType::default(),
                    };
                    let content = NodeContent {
                        node_type: current_node_type,
                        node_name: i.tag_name.to_uppercase(),
                        node_value: i.token_data.clone(),
                    };
    
                    let mut node = DOMNode {
                        node_content: content,
                        this_node_idx: count,
                        parent_node_idx: UMAX,
                        child_nodes_idx: Vec::new(),
                        first_child_idx: UMAX,
                        last_child_idx: UMAX,
                        previous_sibiling_idx: UMAX,
                        next_sibiling_idx: UMAX,
                    };
    
                    if !idx_of_node_stack.is_empty() {
                        let last_stack_idx_in_dom = idx_of_node_stack[idx_of_node_stack.len() -1];
                        node.parent_node_idx = dom_tree[last_stack_idx_in_dom].this_node_idx;
                        dom_tree[last_stack_idx_in_dom].child_nodes_idx.push(node.this_node_idx);
                        dom_tree[last_stack_idx_in_dom].first_child_idx = 
                            if dom_tree[last_stack_idx_in_dom].first_child_idx == UMAX {
                                node.this_node_idx
                            } else {
                                dom_tree[last_stack_idx_in_dom].first_child_idx
                            };
                        if dom_tree[last_stack_idx_in_dom].last_child_idx != UMAX {
                            let last_child_idx = dom_tree[last_stack_idx_in_dom].last_child_idx;
                            dom_tree[last_child_idx].next_sibiling_idx = node.this_node_idx;
                            node.previous_sibiling_idx = dom_tree[last_child_idx].this_node_idx;
                        }
                        dom_tree[last_stack_idx_in_dom].last_child_idx = node.this_node_idx;
                    }
    
                    if i.token_type == TokenType::StratTag {
                        idx_of_node_stack.push(node.this_node_idx);
                    }
                    // println!("{}", node.node_content.node_name);
                    dom_tree.push(node);
                    count += 1;
                    let parent_element_idx = dom_tree.len() - 1;
    
                    // Handle attribute
                    for j in &i.tag_attribute {
                        let attribute_content = NodeContent {
                            node_type: NodeType::Attribute,
                            node_name: j.0.clone(),
                            node_value: j.1.clone(),
                        };
                        let mut attribute_node = DOMNode {
                            node_content: attribute_content,
                            this_node_idx: count,
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
    
                        // println!("attr_name");
                        // println!("{}", attribute_node.node_content.node_name);
                        dom_tree.push(attribute_node);
                        count += 1;
                    }
    
                    if (mode == Mode::BeforeHTML) && (i.tag_name.to_uppercase() == "HTML") {
                        mode = Mode::BeforeHead;
                    } else if (mode == Mode::BeforeHead) && (i.tag_name.to_uppercase() == "HEAD") {
                        mode = Mode::InHead;
                    } else if (mode == Mode::AfterHead) && (i.tag_name.to_uppercase() == "BODY") {
                        mode = Mode::InBody;
                    }
                },
                TokenType::EndTag => {
                    let last_stack_idx = idx_of_node_stack.len() -1;
                    let last_stack_idx_in_dom = idx_of_node_stack[last_stack_idx];
                    if dom_tree[last_stack_idx_in_dom].node_content.node_name == i.tag_name {
                        idx_of_node_stack.pop();
                    }
    
                    if (mode == Mode::InHead) && (i.tag_name.to_uppercase() == "HEAD") {
                        mode = Mode::AfterHead;
                    } else if (mode == Mode::InBody) && (i.tag_name.to_uppercase() == "BODY") {
                        mode = Mode::AfterBody;
                    } else if (mode == Mode::AfterBody) && (i.tag_name.to_uppercase() == "HTML") {
                        mode = Mode::AfterAfterBody;
                    }
                },
                TokenType::EndOfFile => {
                    if mode == Mode::AfterAfterBody {
                        break;
                    }
                },
                _ => {
                },
            };
        }
        // println!("end create DOM tree"); 
        dom_tree
    }

    fn find_css_path(token: Token) {
        let mut is_stylesheet: bool = false;
        let mut herf_link: String =Default::default();
        for i in token.tag_attribute {
            if (i.0.to_lowercase() == "rel") && (i.1.to_lowercase() == "stylesheet") {
                is_stylesheet = true;
            } else if i.0.to_lowercase() == "href" {
                herf_link = i.1;
            }
        }

        if is_stylesheet {
            handmade_css_parser::get_css(herf_link);
        }
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
    
    fn print_dom_node(current_node_idx: usize, tree: &Vec<DOMNode>, result: &mut String) {
        //print_dom_vec(&tree);
        let idx: i32 = current_node_idx.try_into().unwrap();
        let last_child_idx: i128 = tree[current_node_idx].last_child_idx.try_into().unwrap();
        /*println!("{}", idx.to_string()); 
        println!("{}", last_child_idx.to_string()); 
        println!("\n"); 
        println!("\n"); */
        if tree[current_node_idx].child_nodes_idx.is_empty() {
            result.push_str("[");
            result.push_str(&idx.to_string());
            result.push_str(": ");
            result.push_str(&tree[current_node_idx].node_content.node_name);
            result.push_str(", ");
            result.push_str(&tree[current_node_idx].node_content.node_value);
            result.push_str("]");
            result.push_str("   ");
        } else {
            result.push_str("\n");
            result.push_str("\n");
            result.push_str("*PARENT ");
            result.push_str("[");
            result.push_str(&idx.to_string());
            result.push_str(": ");
            result.push_str(&tree[current_node_idx].node_content.node_name);
            result.push_str(", ");
            result.push_str(&tree[current_node_idx].node_content.node_value);
            result.push_str("]");
            result.push_str("\n");
            result.push_str("*CHILDS");
            for i in &tree[current_node_idx].child_nodes_idx {
                print_dom_node(*i, tree, result);
            }
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