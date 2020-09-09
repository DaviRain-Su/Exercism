
// pub mod graph {
//     use graph_items::edge::Edge;
//     use graph_items::node::Node;
//     use std::collections::HashMap;
    
//     pub struct Graph<'a> {
//         pub nodes: Vec<Node<'a>>,
//         pub edges: Vec<Edge<'a>>,
//         pub attrs: HashMap<String, String>,
//     }

//     impl<'a> Default for Graph<'a>  {
//         fn default() -> Graph<'a> {
//             Graph{
//                 nodes: Vec::default(),
//                 edges: Vec::default(),
//                 attrs: HashMap::default(),
//             }
//         }
//     }

//     impl<'a> Graph<'a> {
//         pub fn new() -> Self {
//             Graph::default()
//         }
//         pub fn with_nodes(self, nodes: &Vec<Node<'a>>) -> Self {
//             let mut result = Vec::new();
//             for item in nodes.iter() {
//                 result.push(item.clone());
//             }

//             Self{
//                 nodes: result,
//                 ..self
//             }
//         }

//         pub fn with_edges(self, _edges: &Vec<Edge<'a>>) -> Self {
//             let mut result = Vec::new();
//             for item in _edges.iter(){
//                 result.push(item.clone());
//             }
//             Self{
//                 edges: result,
//                 ..self
//             }
//         }

//         pub fn with_attrs(self, _attrs: &[(&str, &str)]) -> Self {
//             let mut temp_attrs = HashMap::new();
//             for item in _attrs {
//                 temp_attrs.insert(item.0.to_string(), item.1.to_string());
//             }
//             Self{
//                 attrs: temp_attrs,
//                 ..self
//             }
//         }

//         pub fn get_node(&self, _node: &str) -> Option<&Node<'a>> {
//             let mut result : Option<&Node<'a>> = None;
//             for item in self.nodes.iter() {
//                 if item.get_node_str() == _node{
//                     result = Some(item);
//                 }
//             }
//             result
//         }
//     }



//     pub mod graph_items {
//         pub mod node {
//             use std::collections::HashMap;
//             #[derive(Debug, Clone, PartialEq)]
//             pub struct Node<'a>{
//                 item: &'a str,
//                 attrs: HashMap<String, String>
//             }

//             impl<'a>  Node<'a> {
//                 pub fn new(item: &'a str) -> Self {
//                     Self{ 
//                         item,
//                         attrs: HashMap::new(),
//                     }
//                 } 
                
//                 pub fn with_attrs(self, _attrs: &[(&str, &str)]) -> Self {
//                     let mut temp_attrs = HashMap::new();
//                     for item in _attrs {
//                         temp_attrs.insert(item.0.to_string(), item.1.to_string());
//                     }
//                     Self{
//                         attrs: temp_attrs,
//                         ..self
//                     }
//                 }
//                 pub fn get_attr(&self, _attr: &str) -> Option<&str> {
//                     if self.attrs.contains_key(_attr) {
//                         let result = self.attrs.get_key_value(_attr).unwrap();
//                         Some(result.1)
//                     }else {
//                         None
//                     }
//                 }
                
//                 pub fn get_node_str(&self) -> &str {
//                     self.item
//                 }
//             }
//         }

//         pub mod edge {
//             use std::collections::HashMap;
//             #[derive(Debug, PartialEq, Clone)]
//             pub struct Edge<'a> {
//                 item1: &'a str,
//                 item2: &'a str,
//                 attrs: HashMap<String, String>
//             }
//             impl<'a> Edge<'a> {
//                 pub fn new(item1: &'a str, item2: &'a str) -> Self {
//                     Self {
//                         item1,
//                         item2,
//                         attrs: HashMap::new(),
//                     }
//                 }

//                 pub fn with_attrs(self, _attrs: &[(&str, &str)]) -> Self {
//                     let mut temp_attrs = HashMap::new();
//                     for item in _attrs {
//                         temp_attrs.insert(item.0.to_string(), item.1.to_string());
//                     }
//                     Self{
//                         attrs: temp_attrs,
//                         ..self
//                     }
//                 }
//             }
//         }
//     }
// }



use std::collections::HashMap;


macro_rules! impl_attrs {
    () => {
        pub fn get_attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|s| s.as_str() )
        }
        
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self{
            self.attrs = attrs
                .iter()
                .map(|(a, b)|{
                    (a.to_string(), b.to_string())
                })
                .collect();
            self 
        }
    }
}
#[derive(PartialEq, Eq, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self{
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.to_vec();
        self
    }

    pub fn get_node(&self, name: &str) -> Option<&Node>{
        self.nodes.iter().find(|node| node.name == name)
    }

    impl_attrs!();
}
#[derive(PartialEq, Eq, Default, Clone, Debug)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self{
            name: name.to_string(),
            ..Self::default()
        }
    }

    impl_attrs!();
}

#[derive(PartialEq, Eq, Default, Clone, Debug)]
pub struct Edge {
    pub from: String, 
    pub to : String,
    pub attrs: HashMap<String, String>,
}
impl  Edge {
    pub fn new(from : &str, to : &str) -> Self{
        Self{
            from: from.to_string(),
            to: to.to_string(),
            ..Self::default()
        }
    }
    impl_attrs!();
}

pub mod graph {
    pub use super::Graph;
    pub mod graph_items {
        pub mod edge{
            pub use super::super::super::Edge;
        }
        pub mod node {
            pub use super::super::super::Node; 
        }
    }
}