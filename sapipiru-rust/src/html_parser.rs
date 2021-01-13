pub mod handmade_html_parser {
    use std::ascii::AsciiExt;
    //use std::rc::Rc;
    //use std::cell::RefCell;

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
    #[derive(PartialEq, Eq, Debug)]
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

    #[derive(Default, Clone)]
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
    }

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

    struct NodeContent {
        node_type: NodeType,
        node_name: String,
        base_uri: String,
        is_connected: bool,
        owner_document: DocumentNode,
        parent_element: ElementNode,
        node_value: String,
        text_content: String,
    }

    struct DOMNode {
        node_content: NodeContent,
        this_node_idx: i32,
        parent_node_idx: i32,
        child_nodes_idx: Vec<i32>,
        first_child_idx: i32,
        last_child_idx: i32,
        previous_sibiling_idx: i32,
        next_sibiling_idx: i32,
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
        create_DOM_tree(tokens);
        String::from("")
        // print_token(tokens)
    }

    fn tokenize(original_html : &String) -> Vec<Token> {
        let mut current_state = TokenizeState::Data;
        let mut current_token: Token = Default::default();
        let mut tokens: Vec<Token> = Vec::new();
        let mut token_end_flag = false;
        let mut count_comment_end_char: i32 = 0;
        let mut tmp_string: String = Default::default();
        let mut tmp_attribute_strings: (String, String) = Default::default();
        for i in original_html.as_str().chars() {
            match current_state {
                TokenizeState::Data => {
                    if i == '<' {
                        token_end_flag = true;
                        current_state = TokenizeState::TagOpen;
                    } else {
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
        tokens
    }

    // will follow https://html.spec.whatwg.org/multipage/parsing.html#data-state
    // 13.2.6.4 The rules for parsing tokens in HTML content
    fn create_DOM_tree(tokens: Vec<Token>) -> Vec<DOMNode> {
        let mut mode = Mode::Initial;
        let mut dom_tree: Vec<DOMNode> = Vec::new();
        let mut current_node: DOMNode;
        let mut doc_node: DOMNode;
        let mut node_stack: Vec<DOMNode> = Vec::new();
        let mut count = 0;
        for i in &tokens {
            match mode {
                Mode::Initial => {
                    if (i.token_type == TokenType::Doctype)
                    {
                        let content = NodeContent {
                            node_type: NodeType::DocumentType,
                            node_name: i.token_data,
                            base_uri: Default::default(),
                            is_connected: false,
                            owner_document: Default::default(),
                            parent_element: Default::default(),
                            node_value: Default::default(),
                            text_content: Default::default(),
                        };
                        doc_node = DOMNode {
                            node_content: content,
                            this_node_idx: count,
                            parent_node_idx: -1,
                            child_nodes_idx: Vec::new(),
                            first_child_idx: -1,
                            last_child_idx: -1,
                            previous_sibiling_idx: -1,
                            next_sibiling_idx: -1,
                        };
                        dom_tree.push(doc_node);
                        current_node = doc_node;
                        mode = Mode::BeforeHTML;
                    }
                },
                Mode::BeforeHTML => {
                    if (i.token_type == TokenType::StratTag) && (i.tag_name.to_uppercase() == "HTML") {
                        // TODO: will add attribute
                        let mut element_node = ElementNode {
                            namespace_uri: Default::default(),
                            prefix: Default::default(),
                            local_name: Default::default(),
                            tag_name: i.tag_name.to_uppercase(),
                            id: Default::default(),
                            class_name: Default::default(),
                            class_list: Default::default(),
                            slot: Default::default(),
                            attributes: Default::default(),
                            shadow_root: Default::default(),
                        };
                        let content = NodeContent {
                            node_type: NodeType::Element,
                            node_name: i.tag_name.to_uppercase(),
                            base_uri: Default::default(),
                            is_connected: false,
                            owner_document: Default::default(),
                            parent_element: Default::default(),
                            node_value: Default::default(),
                            text_content: Default::default(),
                        };
                        let node = DOMNode {
                            node_content: content,
                            this_node_idx: count,
                            parent_node_idx: current_node.this_node_idx,
                            child_nodes_idx: Vec::new(),
                            first_child_idx: -1,
                            last_child_idx: -1,
                            previous_sibiling_idx: -1,
                            next_sibiling_idx: -1,
                        };
                        current_node.child_nodes_idx.push(node.this_node_idx);
                        current_node.first_child_idx = node.this_node_idx;
                        current_node.last_child_idx = node.this_node_idx;
                        dom_tree.push(node);
                        current_node = node;
                        node_stack.push(node);
                        mode = Mode::BeforeHead;
                    }
                },
                Mode::BeforeHead => {
                    if (i.token_type == TokenType::StratTag) && (i.tag_name.to_uppercase() == "HEAD") {
                        // TODO: will add attribute
                        let mut element_node = ElementNode {
                            namespace_uri: Default::default(),
                            prefix: Default::default(),
                            local_name: Default::default(),
                            tag_name: i.tag_name.to_uppercase(),
                            id: Default::default(),
                            class_name: Default::default(),
                            class_list: Default::default(),
                            slot: Default::default(),
                            attributes: Default::default(),
                            shadow_root: Default::default(),
                        };

                        let content = NodeContent {
                            node_type: NodeType::Element,
                            node_name: i.tag_name.to_uppercase(),
                            base_uri: Default::default(),
                            is_connected: false,
                            owner_document: Default::default(),
                            parent_element: Default::default(),
                            node_value: Default::default(),
                            text_content: Default::default(),
                        };
                        let node = DOMNode {
                            node_content: content,
                            this_node_idx: count,
                            parent_node_idx: current_node.this_node_idx,
                            child_nodes_idx: Vec::new(),
                            first_child_idx: -1,
                            last_child_idx: -1,
                            previous_sibiling_idx: -1,
                            next_sibiling_idx: -1,
                        };
                        current_node.child_nodes_idx.push(node.this_node_idx);
                        current_node.first_child_idx = node.this_node_idx;
                        current_node.last_child_idx = node.this_node_idx;
                        dom_tree.push(node);
                        current_node = node;
                        mode = Mode::InHead;
                        node_stack = Default::default();
                    }
                },
                Mode::InHead => {
                    if (i.token_type == TokenType::EndTag) && (i.tag_name.to_uppercase() == "HEAD") {
                            mode = Mode::AfterHead;
                    } else {
                        if i.token_type == TokenType::StratTag {
                            let mut element_node = ElementNode {
                                namespace_uri: Default::default(),
                                prefix: Default::default(),
                                local_name: Default::default(),
                                tag_name: i.tag_name.to_uppercase(),
                                id: Default::default(),
                                class_name: Default::default(),
                                class_list: Default::default(),
                                slot: Default::default(),
                                attributes: Default::default(),
                                shadow_root: Default::default(),
                            };
                            let content = NodeContent {
                                node_type: NodeType::Element,
                                node_name: i.tag_name.to_uppercase(),
                                base_uri: Default::default(),
                                is_connected: false,
                                owner_document: Default::default(),
                                parent_element: Default::default(),
                                node_value: Default::default(),
                                text_content: Default::default(),
                            };
                            let node = DOMNode {
                                node_content: content,
                                this_node_idx: count,
                                parent_node_idx: -1,
                                child_nodes_idx: Vec::new(),
                                first_child_idx: -1,
                                last_child_idx: -1,
                                previous_sibiling_idx: -1,
                                next_sibiling_idx: -1,
                            };

                            node.parent_node_idx = 
                                if node_stack.is_empty() {
                                    current_node.this_node_idx
                                } else {
                                    if node_stack[node_stack.len() -1].node_content.node_type == NodeType::Element {
                                        //node.node_content.parent_element = 
                                    }
                                    node_stack[node_stack.len() -1].this_node_idx
                                };
                            current_node.child_nodes_idx.push(node.this_node_idx);
                            current_node.first_child_idx = 
                                if current_node.first_child_idx == -1 {
                                    node.this_node_idx
                                } else {
                                    current_node.first_child_idx
                                };
                            current_node.last_child_idx = node.this_node_idx;
                            if current_node.parent_node_idx == node.parent_node_idx {
                                current_node.next_sibiling_idx = node.this_node_idx;
                                node.previous_sibiling_idx = current_node.this_node_idx;
                            }
                            dom_tree.push(node);
                            current_node = node;
                            node_stack.push(node);
                        } else if i.token_type == TokenType::EndTag {
                            if node_stack[node_stack.len() -1].node_content.node_name == i.tag_name {
                                node_stack.pop();
                            }
                        } else if i.token_type == TokenType::Text {
                            let content = NodeContent {
                                node_type: NodeType::Element,
                                node_name: i.tag_name.to_uppercase(),
                                base_uri: Default::default(),
                                is_connected: false,
                                owner_document: Default::default(),
                                parent_element: Default::default(),
                                node_value: i.token_data,
                                text_content: Default::default(),
                            };
                            let node = DOMNode {
                                node_content: content,
                                this_node_idx: count,
                                parent_node_idx: -1,
                                child_nodes_idx: Vec::new(),
                                first_child_idx: -1,
                                last_child_idx: -1,
                                previous_sibiling_idx: -1,
                                next_sibiling_idx: -1,
                            };
                            node.parent_node_idx = 
                                if node_stack.is_empty() {
                                    // should not come here
                                    -1
                                } else {
                                    if node_stack[node_stack.len() -1].node_content.node_type == NodeType::Element {
                                        //node.node_content.parent_element = 
                                    }
                                    node_stack[node_stack.len() -1].this_node_idx
                                };
                        }
                    }
                },
                Mode::AfterHead => {
                    if (i.token_type == TokenType::StratTag) && (i.tag_name.to_uppercase() == "BODY") {
                        // TODO: will add attribute
                        let mut element_node = ElementNode {
                            namespace_uri: Default::default(),
                            prefix: Default::default(),
                            local_name: Default::default(),
                            tag_name: i.tag_name.to_uppercase(),
                            id: Default::default(),
                            class_name: Default::default(),
                            class_list: Default::default(),
                            slot: Default::default(),
                            attributes: Default::default(),
                            shadow_root: Default::default(),
                        };

                        let content = NodeContent {
                            node_type: NodeType::Element,
                            node_name: i.tag_name.to_uppercase(),
                            base_uri: Default::default(),
                            is_connected: false,
                            owner_document: Default::default(),
                            parent_element: Default::default(),
                            node_value: Default::default(),
                            text_content: Default::default(),
                        };
                        let node = DOMNode {
                            node_content: content,
                            this_node_idx: count,
                            parent_node_idx: current_node.this_node_idx,
                            child_nodes_idx: Vec::new(),
                            first_child_idx: -1,
                            last_child_idx: -1,
                            previous_sibiling_idx: -1,
                            next_sibiling_idx: -1,
                        };
                        current_node.child_nodes_idx.push(node.this_node_idx);
                        current_node.first_child_idx = node.this_node_idx;
                        current_node.last_child_idx = node.this_node_idx;
                        dom_tree.push(node);
                        current_node = node;
                        mode = Mode::InBody;
                        node_stack = Default::default();
                    }
                },
                Mode::InBody => {
                    if (i.token_type == TokenType::EndTag) && (i.tag_name.to_uppercase() == "BODY") {
                        mode = Mode::AfterBody;
                    } else {
                        if i.token_type == TokenType::StratTag {
                            let mut element_node = ElementNode {
                                namespace_uri: Default::default(),
                                prefix: Default::default(),
                                local_name: Default::default(),
                                tag_name: i.tag_name.to_uppercase(),
                                id: Default::default(),
                                class_name: Default::default(),
                                class_list: Default::default(),
                                slot: Default::default(),
                                attributes: Default::default(),
                                shadow_root: Default::default(),
                            };
                            let content = NodeContent {
                                node_type: NodeType::Element,
                                node_name: i.tag_name.to_uppercase(),
                                base_uri: Default::default(),
                                is_connected: false,
                                owner_document: Default::default(),
                                parent_element: Default::default(),
                                node_value: Default::default(),
                                text_content: Default::default(),
                            };
                            let node = DOMNode {
                                node_content: content,
                                this_node_idx: count,
                                parent_node_idx: -1,
                                child_nodes_idx: Vec::new(),
                                first_child_idx: -1,
                                last_child_idx: -1,
                                previous_sibiling_idx: -1,
                                next_sibiling_idx: -1,
                            };

                            node.parent_node_idx = 
                                if node_stack.is_empty() {
                                    current_node.this_node_idx
                                } else {
                                    if node_stack[node_stack.len() -1].node_content.node_type == NodeType::Element {
                                        //node.node_content.parent_element = 
                                    }
                                    node_stack[node_stack.len() -1].this_node_idx
                                };
                            current_node.child_nodes_idx.push(node.this_node_idx);
                            current_node.first_child_idx = 
                                if current_node.first_child_idx == -1 {
                                    node.this_node_idx
                                } else {
                                    current_node.first_child_idx
                                };
                            current_node.last_child_idx = node.this_node_idx;
                            if current_node.parent_node_idx == node.parent_node_idx {
                                current_node.next_sibiling_idx = node.this_node_idx;
                                node.previous_sibiling_idx = current_node.this_node_idx;
                            }
                            dom_tree.push(node);
                            current_node = node;
                            node_stack.push(node);
                        } else if i.token_type == TokenType::EndTag {
                            if node_stack[node_stack.len() -1].node_content.node_name == i.tag_name {
                                node_stack.pop();
                            }
                        } else if i.token_type == TokenType::Text {
                            let content = NodeContent {
                                node_type: NodeType::Element,
                                node_name: i.tag_name.to_uppercase(),
                                base_uri: Default::default(),
                                is_connected: false,
                                owner_document: Default::default(),
                                parent_element: Default::default(),
                                node_value: i.token_data,
                                text_content: Default::default(),
                            };
                            let node = DOMNode {
                                node_content: content,
                                this_node_idx: count,
                                parent_node_idx: -1,
                                child_nodes_idx: Vec::new(),
                                first_child_idx: -1,
                                last_child_idx: -1,
                                previous_sibiling_idx: -1,
                                next_sibiling_idx: -1,
                            };
                            node.parent_node_idx = 
                                if node_stack.is_empty() {
                                    // should not come here
                                    -1
                                } else {
                                    if node_stack[node_stack.len() -1].node_content.node_type == NodeType::Element {
                                        //node.node_content.parent_element = 
                                    }
                                    node_stack[node_stack.len() -1].this_node_idx
                                };
                            }
                        }
                    }
                },
                Mode::AfterBody => {
                    if (i.token_type == TokenType::EndTag) && (i.tag_name.to_uppercase() == "HTML") {
                        mode = Mode::AfterAfterBody;
                    }
                },
                Mode::AfterAfterBody => {
                    if (i.token_type == TokenType::EndOfFile) {
                        break;
                    }
                },
            }
            count += 1;
        }
        dom_tree
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
}