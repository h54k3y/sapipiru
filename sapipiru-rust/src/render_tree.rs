pub mod handmade_render_tree {
    use crate::html_parser::handmade_html_parser;
    use crate::css_parser::handmade_css_parser;

    #[derive(Default, Clone)]
    pub struct RenderTreeNode {
    }

    pub fn create_render_tree(dom_tree: Vec<handmade_html_parser::DOMNode>, cssom_tree:Vec<handmade_css_parser::Rule>) -> Vec<RenderTreeNode> {
        let result = Vec::new();
        result
    }
}