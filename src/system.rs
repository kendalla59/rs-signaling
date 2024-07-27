// system.rs
//
// Author: Kendall Auel
//
// The system module represents the entire railroad system
// including the track network and all trains running on the
// tracks.
//

use super::common;
use common::*;

pub mod edge;
use edge::Edge;

pub mod node;
use node::Node;

pub mod train;
use train::Train;

type NodeVec = Vec<String>;

use std::collections::HashMap;
use itertools::Itertools;

pub struct System
{
    edge_map:   HashMap<String, Edge>,
    node_map:   HashMap<String, Node>,
    train_map:  HashMap<String, Train>,
}

impl System {
    // ==============================================================
    // create_edge
    // ==============================================================
    pub fn create_edge(&mut self, name: &str) -> Option<&mut Edge> {
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
        assert!(! edge_name.is_empty(), "The edge name is empty");

        let edge = Edge {
            name: edge_name.clone(),
            ends: [ NodeSlot { ns_node: String::new(), ns_slot: SLOT_1, },
                    NodeSlot { ns_node: String::new(), ns_slot: SLOT_1, } ],
        };
        match self.edge_map.insert(edge_name.clone(), edge) {
            None => (),
            Some(_) => panic!("Insert for {} failed", &edge_name),
        }

        // Place terminator nodes at each end of the edge.
        let node_a_name;
        let node_b_name;
        if let Some(node_a) = self.create_node("") {
            node_a_name = node_a.name.clone();
            node_a.make_terminator(
                &EdgeEnd { ee_edge: edge_name.clone(), ee_end: END_A, });
        } else { panic!("Failed to create node_a"); }
        if let Some(node_b) = self.create_node("") {
            node_b_name = node_b.name.clone();
            node_b.make_terminator(
                &EdgeEnd { ee_edge: edge_name.clone(), ee_end: END_B});
        } else { panic!("Failed to create node_b"); }

        match self.edge_map.get_mut(&edge_name) {
            None => (),
            Some(e) => {
                e.assign_node_slot(
                    &NodeSlot { ns_node: node_a_name, ns_slot: SLOT_1 }, END_A);
                e.assign_node_slot(
                    &NodeSlot { ns_node: node_b_name, ns_slot: SLOT_1 }, END_B);
            }
        }
        self.edge_map.get_mut(&edge_name)
    }

    pub fn get_edge(&self, name: &String) -> Option<&Edge> {
        self.edge_map.get(name)
    }
    pub fn has_edge(&self, name: &String) -> bool {
        let rval;
        match self.edge_map.get(name) {
            None    => rval = false,
            Some(_) => rval = true,
        }
        rval
    }

    // ==============================================================
    // create_node
    // ==============================================================
    pub fn create_node(&mut self, name: &str) -> Option<&mut Node> {
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
        assert!(! node_name.is_empty(), "The node name is empty");
        let node = Node {
            name: node_name.clone(),
            slots: [ EdgeEnd { ee_edge: String::new(), ee_end: END_A },
                     EdgeEnd { ee_edge: String::new(), ee_end: END_A },
                     EdgeEnd { ee_edge: String::new(), ee_end: END_A }, ],
            switch_state: JSwitch::JSwitchNone,
        };
        self.node_map.insert(node_name.clone(), node);
        self.node_map.get_mut(&node_name)
    }

    pub fn get_node(&mut self, name: &String) -> Option<&mut Node> {
        self.node_map.get_mut(name)
    }

    // ==============================================================
    // create_train
    // ==============================================================
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

    // ==============================================================

    pub fn connect_segments(&mut self, s1: &EdgeEnd, s2: &EdgeEnd) -> i32
    {
        // If either track is invalid, there is nothing more to do.
        let edge1;
        let edge2;

        match self.edge_map.get(&s1.ee_edge) {
            None => return 1, // EINVAL
            Some(e) => edge1 = e,
        }
        match self.edge_map.get(&s2.ee_edge) {
            None => return 1, // EINVAL
            Some(e) => edge2 = e,
        }

        let cnct_node = edge1.get_node(s1.ee_end);
        let rmov_node = edge2.get_node(s2.ee_end);
        let mut repl_node = NodeSlot {
            ns_node: String::new(),
            ns_slot: NUM_SLOTS,
        };

        // Return error if the end of the other track is
        // not a terminator -- i.e., it must be unconnected.
        match self.node_map.get(&rmov_node.ns_node) {
            None => return 1, // throw
            Some(n) => {
                if n.get_node_type() != NodeType::Terminator {
                    println!("ERROR: Cannot connect if end of other is occupied");
                    return 1; // EBUSY
                }
            }
        }

        let node1;
        match self.node_map.get_mut(&cnct_node.ns_node) {
            None => return 1, // throw
            Some(n) => node1 = n,
        }

        // Connect to the other track as implied by this track's connection.
        match node1.get_node_type() {
            NodeType::Terminator => {
                // Connect results in a continuation node.
                node1.make_continuation(s2);

                // Replace the other edge's node slot entry.
                repl_node.ns_node = node1.name.clone();
                repl_node.ns_slot = SLOT_2;

                match self.edge_map.get_mut(&s2.ee_edge) {
                    None => panic!("Where did edge {} go?", s1.ee_edge),
                    Some(e) => {
                        e.assign_node_slot(&repl_node, s2.ee_end);
                    }
                }
            }
            NodeType::Continuation => {
                // This connection results in a junction from
                // this track to the currently connected track
                // (left) or to the new track (right).

                if cnct_node.ns_slot == SLOT_2 {
                    // Swap slot 1 and 2 if the common edge is on slot 2.
                    // NOTE: The NodeSlot for the Edge, and the EdgeEnd
                    //       for the Node must both be updated, or the
                    //       network will become corrupted.
                    let e1 = node1.get_edge_end(SLOT_1);
                    let e2 = node1.get_edge_end(SLOT_2);

                    match self.edge_map.get_mut(&e1.ee_edge) {
                        None => panic!("Where did edge {} go?", e1.ee_edge),
                        Some(e) => {
                            let mut ns = e.get_node(e1.ee_end);
                            assert!(ns.ns_slot == SLOT_1);
                            ns.ns_slot = SLOT_2;
                            e.assign_node_slot(&ns, e1.ee_end);
                        }
                    }
                    match self.edge_map.get_mut(&e2.ee_edge) {
                        None => panic!("Where did edge {} go?", e2.ee_edge),
                        Some(e) => {
                            let mut ns = e.get_node(e2.ee_end);
                            assert!(ns.ns_slot == SLOT_2);
                            ns.ns_slot = SLOT_1;
                            e.assign_node_slot(&ns, e2.ee_end);
                        }
                    }
                    node1.set_edge_end(&e1, SLOT_2);
                    node1.set_edge_end(&e2, SLOT_1);
                }
                node1.make_junction(s2);

                // Replace the other edge's node slot entry.
                repl_node.ns_node = cnct_node.ns_node;
                repl_node.ns_slot = SLOT_3;
                match self.edge_map.get_mut(&s2.ee_edge) {
                    None => panic!("Where did edge {} go?", s1.ee_edge),
                    Some(e) => {
                        e.assign_node_slot(&repl_node, s2.ee_end);
                    }
                }
            }
            NodeType::Junction => {
                // We cannot connect any more tracks to this end.
                println!("ERROR: Attempt to connect to a junction");
                return 1; // throw
            }
            _ => {
                println!("ERROR: Unexpected node type in connectEdge");
                return 1; // throw
            }
        }
        return 0;
    }

    pub fn get_all_junctions(&self) -> NodeVec {
        let mut rval = vec![];
        for (name, node) in &self.node_map {
            if node.get_node_type() == NodeType::Junction {
                rval.push(name.clone());
            }
        }
        rval
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

    pub fn show_edges(&self) {
        for name in self.edge_map.keys().sorted() {
            self.edge_map[name].show(self, NUM_ENDS);
        }
        println!("TOTAL: {} track segments", self.edge_map.len());
    }

    pub fn show_nodes(&self) {
        for name in self.node_map.keys().sorted() {
            self.node_map[name].show(self);
        }
    }

    pub fn update_all_signals(&mut self) {

    }
}

pub fn create_system() -> System {
    System {
        edge_map:   HashMap::new(),
        node_map:   HashMap::new(),
        train_map:  HashMap::new(),
    }
}

pub fn edge_count() -> i32 {
    return 0;
}
