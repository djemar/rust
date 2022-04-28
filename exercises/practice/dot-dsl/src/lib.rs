pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;

    macro_rules! impl_attrs {
        () => {
           pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs.extend(attrs.iter()
                .map(|(attr1, attr2)| (attr1.to_string(), attr2.to_string())));
            self
        }

            pub fn get_attr(&self, name: &str) -> Option<&str> {
                self.attrs.get(name).map(|attr| attr.as_str())
            }
        }
    }

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_owned();
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_owned();
            self
        }
        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }
        impl_attrs!();
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(PartialEq, Eq, Default, Clone, Debug)]
            pub struct Edge {
                pub left_edge: String,
                pub right_edge: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(left_edge: &str, right_edge: &str) -> Self {
                    Edge {
                        left_edge: String::from(left_edge),
                        right_edge: String::from(right_edge),
                        attrs: HashMap::new(),
                    }
                }
                impl_attrs!();
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(PartialEq, Eq, Default, Clone, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: String::from(name),
                        attrs: HashMap::new(),
                    }
                }
                impl_attrs!();
            }
        }
    }
}

