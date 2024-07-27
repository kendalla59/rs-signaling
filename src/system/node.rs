// node.rs

use super::common;
use common::*;
use common::JSwitch::JSwitchNone;
use common::JSwitch::JSwitchLeft;
use common::JSwitch::JSwitchRight;

use crate::system::System;

pub struct Node
{
    pub name: String,
    pub slots: [EdgeEnd; NUM_SLOTS],
    pub switch_state: JSwitch,
}

impl Node {
    pub fn make_terminator(&mut self, track: &EdgeEnd) {
        assert!(self.get_node_type() == NodeType::Empty,
                "Attempt to makeTerminator, but node is not empty");
        self.slots[SLOT_1] = EdgeEnd {
            ee_edge: track.ee_edge.clone(),
            ee_end:  track.ee_end
        };
    }

    pub fn make_continuation(&mut self, track: &EdgeEnd) {
        assert!(self.get_node_type() == NodeType::Terminator,
                "Attempt to makeContinuation, but node is not a terminator");
        self.slots[SLOT_2] = EdgeEnd {
            ee_edge: track.ee_edge.clone(),
            ee_end:  track.ee_end,
        };
    }

    pub fn make_junction(&mut self, track: &EdgeEnd) {
        assert!(self.get_node_type() == NodeType::Continuation,
                "Attempt to makeJunction, but node is not a continuation");

        self.slots[SLOT_3] = EdgeEnd {
            ee_edge: track.ee_edge.clone(),
            ee_end:  track.ee_end,
        };
        self.switch_state = JSwitch::JSwitchLeft;
    }

    pub fn get_node_type(&self) -> NodeType {
        if ! self.slots[SLOT_3].ee_edge.is_empty() { return NodeType::Junction; }
        if ! self.slots[SLOT_2].ee_edge.is_empty() { return NodeType::Continuation; }
        if ! self.slots[SLOT_1].ee_edge.is_empty() { return NodeType::Terminator; }
        return NodeType::Empty;
    }

    pub fn get_edge_end(&self, slot: Slot) -> EdgeEnd {
        EdgeEnd {
            ee_edge: self.slots[slot].ee_edge.clone(),
            ee_end:  self.slots[slot].ee_end,
        }
    }

    pub fn set_edge_end(&mut self, edge: &EdgeEnd, slot: Slot) {
        assert!(slot == SLOT_1 || slot == SLOT_2 || slot == SLOT_3,
                "Invalid slot for setEdgeEnd");
        self.slots[slot] = EdgeEnd {
            ee_edge: edge.ee_edge.clone(),
            ee_end:  edge.ee_end,
        };
    }

    pub fn get_next(&self, slot: Slot) -> EdgeEnd {
        let rval;
        match self.get_node_type() {
            NodeType::Empty => {
                panic!("Unexpected result in Node::get_next");
            }
            NodeType::Terminator => {
                // Nowhere to go from here (return empty EdgeEnd).
                rval = EdgeEnd { ee_edge: String::new(), ee_end: NUM_ENDS }
            }
            NodeType::Continuation => {
                if slot == SLOT_1 {
                    rval = EdgeEnd { ee_edge: self.slots[SLOT_2].ee_edge.clone(),
                                     ee_end:  self.slots[SLOT_2].ee_end }
                }
                else if slot == SLOT_2 {
                    rval = EdgeEnd { ee_edge: self.slots[SLOT_1].ee_edge.clone(),
                                     ee_end:  self.slots[SLOT_1].ee_end }
                }
                else {
                    // Should never happen (panic?).
                    rval = EdgeEnd { ee_edge: String::new(), ee_end: NUM_ENDS }
                }
            }
            NodeType::Junction => {
                match self.switch_state {
                    JSwitch::JSwitchLeft => {
                        if slot == SLOT_1 {
                            rval = EdgeEnd { ee_edge: self.slots[SLOT_2].ee_edge.clone(),
                                             ee_end:  self.slots[SLOT_2].ee_end }
                        }
                        else if slot == SLOT_2 {
                            rval = EdgeEnd { ee_edge: self.slots[SLOT_1].ee_edge.clone(),
                                             ee_end:  self.slots[SLOT_1].ee_end }
                        }
                        else {
                            // Otherwise blocked on the right fork (return empty).
                            rval = EdgeEnd { ee_edge: String::new(), ee_end: NUM_ENDS }
                        }
                    }
                    JSwitch::JSwitchRight => {
                        if slot == SLOT_1 {
                            rval = EdgeEnd { ee_edge: self.slots[SLOT_3].ee_edge.clone(),
                                             ee_end:  self.slots[SLOT_3].ee_end }
                        }
                        else if slot == SLOT_3 {
                            rval = EdgeEnd { ee_edge: self.slots[SLOT_1].ee_edge.clone(),
                                             ee_end:  self.slots[SLOT_1].ee_end }
                        }
                        else {
                            // Otherwise blocked on the left fork (return empty).
                            rval = EdgeEnd { ee_edge: String::new(), ee_end: NUM_ENDS }
                        }
                    }
                    JSwitch::JSwitchNone => {
                        // Should never happen (panic?)
                        rval = EdgeEnd { ee_edge: String::new(), ee_end: NUM_ENDS }
                    }
                }
            }
        }
        rval
    }

    pub fn get_switch_pos(&self) -> JSwitch {
        let rval = self.switch_state;
        rval
    }
    pub fn set_switch_pos(&mut self, jsw: JSwitch) {
        self.switch_state = jsw;
    }
    pub fn toggle_switch_pos(&mut self) {
        self.switch_state =
            if self.switch_state == JSwitchLeft { JSwitchRight }
            else                                { JSwitchLeft  };
    }

    pub fn show(&self, sys: &System) {
        let mut nstr = String::new();
        let mut edge = &self.slots[0].ee_edge;
        nstr += format!("{:>12}:", self.name).as_str();

        for ix in 0..NUM_SLOTS {
            edge = &self.slots[ix].ee_edge;
            if !edge.is_empty() {
                if let Some(eref) = sys.get_edge(edge) {
                    let next = eref.get_adjacent(self.slots[ix].ee_end).ns_node;
                    if !next.is_empty() {
                        if ix > 0 { nstr += ","; }
                        nstr += format!("{:>10}", next).as_str();
                    }
                }
            }
            else { break }
        }

        // This is true if and only if the node is a junction.
        if !edge.is_empty() {
            nstr += format!("{:>9}", "(switch").as_str();
            match &self.switch_state {
                JSwitchNone  => nstr += ": none)",
                JSwitchLeft  => nstr += ": left)",
                JSwitchRight => nstr += ": right)",
            }
        }
        println!("{nstr}");
        }
}
