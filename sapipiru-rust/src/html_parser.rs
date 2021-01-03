pub mod handmade_html_parser {
    enum TokenType {
        Doctype,
        StratTag,
        EndTag,
        Comment,
        Character,
        EndOfFile,
    }

    struct Attributes {
        name : String,
        value : String,
    }

    struct Token {
        token_type : TokenType,
        token_name : String,                    // Doctype, StratTag, EndTag
        public_identifier : String,             // Doctype
        system_identifier : String,             // Doctype
        force_quirks_flag : bool,               // Doctype, StratTag, EndTag
        self_closing_flag : bool,               // StratTag, EndTag
        list_of_attributes : Vec<Attributes>,   // StratTag, EndTag
        token_data : String,                    // Comment,
    }

    enum TokenTree<Token> {
        Nil,    // Last node, doesn't have any child
        Node {
            token : Token,
            next : Box<TokenTree<Token>>,
        },
    }

    enum TokenizeState {  // TODO: Add state according to https://html.spec.whatwg.org/multipage/parsing.html#data-state
        Data,
        TagOpen,
        TagName,
        EndTagOpen,
    }

    pub fn parse_html (original_html : &String) -> String {
        tokenize(&original_html);
        "hello".to_string() // Tmp return value
    }

    fn tokenize(original_html : &String) {
        let mut current_state = TokenizeState::Data;
        for i in original_html.as_str().chars() {
            if i == '<' {
                current_state = TokenizeState::TagOpen;
            }
        }
    }
    
}