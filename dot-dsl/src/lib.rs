pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub use super::super::node;
        pub use super::super::edge;
        pub use super::super::attr;
    }

    #[derive(Default, Clone, PartialEq, Eq, Debug)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl<'a> Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
            self.nodes = nodes.to_owned();
            self
        }

        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Self {
            self.edges = edges.to_owned();
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&graph_items::node::Node> {
            self.nodes.iter().find(|x| x.name == name)
        }
    }

    impl<'a> graph_items::attr::Attr<'a, Graph> for Graph {
        fn get_attrs(&self) -> &HashMap<String, String> {
            &self.attrs
        }

        fn get_attrs_mut(&mut self) -> &mut HashMap<String, String> {
            &mut self.attrs
        }
    }
}

pub mod node {
    use std::collections::HashMap;

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Node {
        pub name: String,
        pub attrs: HashMap<String, String>,
    }

    impl Node {
        pub fn new(name: &str) -> Self {
            Node {
                name: name.to_string(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            attrs.iter().for_each(|x| {
                self.attrs.insert(x.0.to_string(), x.1.to_string());
            });
            self
        }
    }

    impl<'a> super::attr::Attr<'a, Node> for Node {
        fn get_attrs(&self) -> &HashMap<String, String> {
            &self.attrs
        }

        fn get_attrs_mut(&mut self) -> &mut HashMap<String, String> {
            &mut self.attrs
        }
    }
}

pub mod edge {
    use std::collections::HashMap;

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Edge {
        pub a: String,
        pub b: String,
        pub attrs: HashMap<String, String>,
    }

    impl Edge {
        pub fn new(a: &str, b: &str) -> Self {
            Edge {
                a: a.to_string(),
                b: b.to_string(),
                attrs: HashMap::new(),
            }
        }
    }

    impl<'a> super::attr::Attr<'a, Edge> for Edge {
        fn get_attrs(&self) -> &HashMap<String, String> {
            &self.attrs
        }

        fn get_attrs_mut(&mut self) -> &mut HashMap<String, String> {
            &mut self.attrs
        }
    }
}

pub mod attr {
    use std::collections::HashMap;
    pub trait Attr<'a, T> {
        fn get_attrs(&self) -> &HashMap<String, String>;

        fn get_attrs_mut(&mut self) -> &mut HashMap<String, String>;

        fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self
            where
                Self: Sized,
        {
            attrs.iter().for_each(|x| {
                self.get_attrs_mut()
                    .insert(x.0.to_string(), x.1.to_string());
            });
            self
        }

        fn get_attr(&self, attr: &str) -> Option<&str> {
            match self.get_attrs().get(attr) {
                Some(x) => Some(x),
                None => None,
            }
        }
    }
}