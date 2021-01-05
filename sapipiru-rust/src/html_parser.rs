pub mod handmade_html_parser {
    use std::ascii::AsciiExt;

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
    }

    enum TokenizeState {  // TODO: Add state according to https://html.spec.whatwg.org/multipage/parsing.html#data-state
        Data,
        TagOpen,
        TagName,
        EndTagOpen,
    }

    pub fn parse_html(original_html : &String) -> String {
        let tokens: Vec<Token> = tokenize(&original_html);
        debug_print(tokens)
    }

    fn tokenize(original_html : &String) -> Vec<Token> {
        let mut current_state = TokenizeState::Data;
        let mut current_token: Token = Default::default();
        let mut tokens: Vec<Token> = Vec::new();
        let mut should_push_to_tag = false;
        let mut should_push_to_data = false;
        let mut token_end_flag = false;
        let mut is_tag_name_defined = false;   // TODO: need to change this value
        let mut count_comment_end_char: i32 = 0;
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
                        should_push_to_data = true;
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
                        should_push_to_tag = true;
                    }
                }
                TokenizeState::TagName => {
                    if i == '>' {
                        current_token.token_type = {
                            if (current_token.token_type == TokenType::StratTag) && current_token.tag_name.to_uppercase().starts_with("!DOCTYPE") {
                                current_token.tag_name = String::from("doctype");
                                TokenType::Doctype
                            } else if (current_token.token_type == TokenType::StratTag) && (current_token.tag_name.to_uppercase().starts_with("COMMENT") || current_token.tag_name.starts_with("!--")) {
                                current_token.tag_name = String::from("comment");
                                TokenType::Comment
                            } else {
                                current_token.token_type
                            }
                        };

                        if (current_token.token_type != TokenType::Comment) || (count_comment_end_char == 2) {
                            token_end_flag = true;
                            current_state = TokenizeState::Data;
                            if current_token.token_type == TokenType::Comment {
                                assert_eq!(Some('-'), current_token.token_data.pop());
                                assert_eq!(Some('-'), current_token.token_data.pop());
                            }
                        } else {
                            is_tag_name_defined = true;
                            should_push_to_data = true;
                        }
                    } else if i ==' ' {
                            is_tag_name_defined = true;
                    } else if is_tag_name_defined == false {
                        should_push_to_tag = true;
                    } else {
                        should_push_to_data = true;
                    }

                    if i == '-' {
                        count_comment_end_char += 1;
                    } else {
                        count_comment_end_char = 0;
                    }
                }
                TokenizeState::EndTagOpen => {
                    if current_token.token_type != TokenType::EndTag {
                        current_token.token_type = TokenType::EndTag;
                    }
                    current_state = TokenizeState::TagName;
                    should_push_to_tag = true;
                }
            }

            if should_push_to_tag == true {
                current_token.tag_name.push(i);
                should_push_to_tag = false;
            }

            if should_push_to_data == true {
                current_token.token_data.push(i);
                should_push_to_data = false;
            }

            if token_end_flag == true {
                if !current_token.tag_name.is_empty() || !current_token.token_data.is_empty() {
                    // TODO: remove clone if it's possible...
                    tokens.push(current_token.clone());
                    current_token = Default::default();
                }
                is_tag_name_defined = false;
                token_end_flag = false;
            }
        }
        
        // add EOF token
        let eof_token: Token = Token {
            token_type: TokenType::EndOfFile,
            tag_name: String::from(""),
            token_data: String::from(""),
        };
        tokens.push(eof_token);
        tokens
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

    fn debug_print(tokens: Vec<Token>) -> String {
        let mut result = String::new();
        let mut count: i32 = 0;
        for i in &tokens {
            result.push_str(&count.to_string());
            result.push_str(":   ");
            result.push_str("TokenType-> ");
            result.push_str(&convert_tokentype_to_string(i.token_type));
            result.push_str("   TagName-> ");
            result.push_str(&i.tag_name);
            result.push_str("   TokenData-> ");
            result.push_str(&i.token_data);
            result.push_str("\n");
            count += 1;
        };
        //println!("{}", result);   //for debug
        result
    }
}