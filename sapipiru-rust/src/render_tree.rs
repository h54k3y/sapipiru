pub mod handmade_render_tree {
    use crate::html_parser::handmade_html_parser;
    use crate::css_parser::handmade_css_parser;

    #[derive(Default, Clone)]
    pub struct RenderTreeNode {
        dom_node: handmade_html_parser::DOMNode,
        style_value: String, // for temporary
        child: Vec<usize>,
    }

    pub trait HandleRederTree {
        fn create_render_tree(&mut self, dom_tree: Vec<handmade_html_parser::DOMNode>, cssom_tree:Vec<handmade_css_parser::Rule>) -> Vec<RenderTreeNode>;
        fn dfs(&mut self, current_node_idx: usize, parent_node_idx: usize);
    }

    #[derive(Default, Clone)]
    pub struct TreesData {
        dom_tree: Vec<handmade_html_parser::DOMNode>,
        cssom_tree: Vec<handmade_css_parser::Rule>,
        render_tree: Vec<RenderTreeNode>
    }

    impl HandleRederTree for TreesData {
        fn create_render_tree(&mut self, dom_tree: Vec<handmade_html_parser::DOMNode>, cssom_tree:Vec<handmade_css_parser::Rule>) -> Vec<RenderTreeNode> {
            let result = Vec::new();
            self.dom_tree = dom_tree;
            self.cssom_tree = cssom_tree;
            result
        }

        fn dfs(&mut self, current_node_idx: usize, parent_node_idx: usize) {
            let parent_node = &self.dom_tree[parent_node_idx];
            if current_node_idx == parent_node.last_child_idx {
                return;
            }
    
            let current_dom_node = self.dom_tree[current_node_idx].clone();
            let mut current_render_node: RenderTreeNode = Default::default();
            for current_cssom_node in &self.cssom_tree {
                if current_dom_node.node_content.node_name == current_cssom_node.selectors[0].items[0].item_string {
                    current_render_node = RenderTreeNode {
                        dom_node: current_dom_node.clone(),
                        style_value: current_cssom_node.selectors[0].items[0].item_string.clone(), // for temporary
                        child: current_dom_node.child_nodes_idx.clone(),
                    };
                }
            }
            
            // push only last one.
            self.render_tree.push(current_render_node);
    
            for child_idx in &current_dom_node.child_nodes_idx {
                self.dfs(child_idx.clone(), current_dom_node.this_node_idx.clone());
            }
        }
    }
}