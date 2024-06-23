// system.rs
//
// Author: Kendall Auel
//
// The system module represents the entire railroad system
// including the track network and all trains running on the
// tracks.
//

use super::common;

pub mod edge;
use edge::Edge;

pub mod node;
use node::Node;

pub mod train;
use train::Train;

use std::collections::HashMap;

pub struct System
{
    edge_map:   HashMap<String, Edge>,
    node_map:   HashMap<String, Node>,
    train_map:  HashMap<String, Train>,
}

impl System {
    pub fn create_edge(&mut self, name: &str) -> Option<&Edge> {
        let edge_name;
        if name.is_empty() {
            edge_name = self.get_unique_edge_name();
        }
        else {
            edge_name = String::from(name);
            // Verify the given edge name is unique.
            match self.edge_map.get(&edge_name) {
                Some(_) => return None,
                _ => (),
            }
        }
        let edge = Edge { name: edge_name.clone() };
        self.edge_map.insert(edge_name.clone(), edge);

        // Place terminator nodes at each end of the edge.
        if let mut nodeA = createNode("") {
            nodeA.make_terminator()
        }
        self.edge_map.get(&edge_name)
    }

    pub fn create_node(&mut self, name: &str) -> Option<&Node> {
        let node_name;
        if name.is_empty() {
            node_name = self.get_unique_node_name();
        }
        else {
            node_name = String::from(name);
            // Verify the given node name is unique.
            match self.node_map.get(&node_name) {
                Some(_) => return None,
                _ => (),
            }
        }
        let node = Node { name: node_name.clone() };
        self.node_map.insert(node_name.clone(), node);
        self.node_map.get(&node_name)
    }

    pub fn create_train(&mut self, name: &str) -> Option<&Train> {
        // Verify the name is not already used.
        match self.train_map.get(name) {
            Some(_) => return None,
            _ => (),
        }
        let train_name;
        if name.is_empty() {
            train_name = self.get_unique_train_name();
        }
        else {
            train_name = String::from(name);
        }
        let train = Train { name: train_name.clone() };
        self.train_map.insert(train_name.clone(), train);
        self.train_map.get(&train_name)
    }

    fn get_unique_edge_name(&self) -> String {
        let mut ix = 1;
        let mut name = String::from("tseg001");
        while self.edge_map.get(&name).is_some() {
            ix += 1;
            name = format!("tseg{:03}", ix);
        }
        name
    }

    fn get_unique_node_name(&self) -> String {
        let mut ix = 1;
        let mut name = String::from("node001");
        while self.node_map.get(&name).is_some() {
            ix += 1;
            name = format!("node{:03}", ix);
        }
        name
    }

    fn get_unique_train_name(&self) -> String {
        let mut ix = 1;
        let mut name = String::from("train1");
        while self.train_map.get(&name).is_some() {
            ix += 1;
            name = format!("train{ix}");
        }
        name
    }
}
/*

NodePtr nptrA = createNode();
nptrA->makeTerminator(EdgeEnd(rval, eEndA));
rval->assignNodeSlot(NodeSlot(nptrA, eSlot1), eEndA);

NodePtr nptrB = createNode();
nptrB->makeTerminator(EdgeEnd(rval, eEndB));
rval->assignNodeSlot(NodeSlot(nptrB, eSlot1), eEndB);

return rval;
*/
pub fn create_system() -> System {
    System {
        edge_map:   HashMap::new(),
        node_map:   HashMap::new(),
        train_map:  HashMap::new(),
    }
}

pub const EMPTY_STR: String = String::new();

pub fn edge_count() -> i32 {
    return 0;
}
