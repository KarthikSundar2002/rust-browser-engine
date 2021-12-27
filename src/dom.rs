use std::collections::{HashMap, HashSet};
use std::fmt;

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(Element_data),
    Comment(String),
}

struct Element_data {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

impl Node {
    fn new(children: Vec<Node>, node_type: NodeType) -> Node {
        Node {
            children,
            node_type,
        }
    }
}

impl Element_data {
    fn new(tag_name: String, attributes: AttrMap) -> Element_data {
        Element_data {
            tag_name,
            attributes,
        }
    }

    fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    fn get_classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(s) => s.split(" ").collect(),
            None => HashSet::new(),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeType::Text(ref t) | NodeType::Comment(ref t) => write!(f, "{}", t),
            NodeType::Element(ref e) => write!(f, "{:?}", e),
        }
    }
}

impl fmt::Debug for Element_data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut attributes_string = String::new();

        for (attr, value) in self.attributes.iter() {
            attributes_string.push_str(&format!("  {}=\"{}\"", attr, value));
        }
        write!(f, "<{},{}>", self.tag_name, attributes_string)
    }
}

fn pretty_print(n: &Node, indent_size: usize) {
    let indent = (0..indent_size).map(|_| " ").collect::<String>();

    match n.node_type {
        NodeType::Element(ref e) => println!("{}{:?}", indent, e),
        NodeType::Text(ref t) => println!("{}{}", indent, t),
        NodeType::Comment(ref c) => println!("{}<---{}--->", indent, c),
    }

    for child in n.children.iter() {
        pretty_print(&child, indent_size + 2);
    }

    match n.node_type {
        NodeType::Element(ref e) => println!("{}<{}/>", indent, e.tag_name),
        _ => {}
    }
}
