pub mod handmade_render_tree {
    use crate::html_parser::handmade_html_parser;
    use crate::css_parser::handmade_css_parser;

    #[derive(Default, Clone)]
    pub struct RenderTreeNode {
        dom_node: handmade_html_parser::DOMNode,
        style_value: String, // for temporary
        child: Vec<usize>,
    }

    pub fn create_render_tree(dom_tree: Vec<handmade_html_parser::DOMNode>, cssom_tree:Vec<handmade_css_parser::Rule>) -> Vec<RenderTreeNode> {
        let result = Vec::new();

        // dfs dom_tree and check matched cssom_tree element.
        let mut current_dom_node = &dom_tree[0];
        let mut current_cssom_node = &cssom_tree[0];
        loop {
            if current_dom_node.node_content.node_name == current_cssom_node.selectors[0].items[0].item_string {
                let mut current_render_node = RenderTreeNode {
                    dom_node: current_dom_node.clone(),
                    style_value: current_cssom_node.selectors[0].items[0].item_string.clone(), // for temporary
                    child: current_dom_node.child_nodes_idx.clone(),
                };
            }
            break;
        }
        result
    }
}