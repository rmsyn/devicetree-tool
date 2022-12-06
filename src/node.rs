// Copyright (c) 2022, Michael Zhao
// SPDX-License-Identifier: MIT

use crate::attribute::InternalAttribute;
use std::sync::{Arc, Mutex};

pub struct Node {
    name: String,
    attributes: Vec<Arc<Mutex<dyn InternalAttribute>>>,
    sub_nodes: Vec<Arc<Mutex<Node>>>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: String::from(name),
            attributes: Vec::new(),
            sub_nodes: Vec::new(),
        }
    }

    pub fn add_attr(&mut self, attr: Arc<Mutex<dyn InternalAttribute>>) {
        self.attributes.push(attr);
    }

    pub fn add_sub_node(&mut self, sub_node: Node) {
        self.sub_nodes.push(Arc::new(Mutex::new(sub_node)));
    }

    pub fn to_dts(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Node: {}", self.name));
        for attr in self.attributes.iter() {
            s.push_str(&attr.lock().unwrap().to_dts());
        }

        for sub_node in self.sub_nodes.iter() {
            s.push_str(&sub_node.lock().unwrap().to_dts());
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attribute::Attribute;

    #[test]
    fn test_simple_node() {
        let attr = Arc::new(Mutex::new(Attribute::new("attr1", 42u32)));
        let mut node = Node::new("node");
        node.add_attr(attr);
        node.add_attr(Arc::new(Mutex::new(Attribute::new("attr2", 12.3456f32))));
        println!("Node: {}", node.to_dts());
    }

    #[test]
    fn test_sub_node() {
        let attr = Arc::new(Mutex::new(Attribute::new("attr1", 42u32)));
        let mut node = Node::new("node1");
        node.add_attr(attr);
        node.add_attr(Arc::new(Mutex::new(Attribute::new("attr12", 12.3456f32))));

        let mut sub_node = Node::new("node2");
        sub_node.add_attr(Arc::new(Mutex::new(Attribute::new("attr3", 56.78f32))));
        sub_node.add_attr(Arc::new(Mutex::new(Attribute::new("attr4", 99u32))));

        node.add_sub_node(sub_node);
        println!("Node: {}", node.to_dts());
    }
}