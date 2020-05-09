#[macro_export]
macro_rules! impl_attrs {
    () => {
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for attribute in attrs {
                self.attrs
                    .insert(attribute.0.to_owned(), attribute.1.to_owned());
            }
            self
        }

        pub fn get_attr(&self, attr_name: &str) -> Option<&str> {
            self.attrs.get(attr_name).map(|value| value.as_str())
        }
    };
}

pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                ..Default::default()
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn get_node(&self, node_name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == node_name)
        }

        impl_attrs!();
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Default)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_owned(),
                        to: to.to_owned(),
                        ..Default::default()
                    }
                }

                impl_attrs!();
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Default)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: String::from(name),
                        ..Default::default()
                    }
                }

                impl_attrs!();
            }
        }
    }
}
