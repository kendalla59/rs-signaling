// node.rs

use super::common::*;

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

    pub fn set_edge_end(&mut self, _edge: &EdgeEnd, _slot: Slot) {
        
    }
}
