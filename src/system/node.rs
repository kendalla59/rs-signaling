// node.rs

use super::common::*;

pub struct Node
{
    pub name: String,
    pub slots: [EdgeEnd; NUM_SLOTS],
    pub switch_state: JSwitch,
}

impl Node {
    pub fn make_terminator(&mut self, track: EdgeEnd) {
        assert!(self.get_node_type() == NodeType::Empty,
                "Attempt to makeTerminator, but node is not empty");
        self.slots[SLOT_1] = EdgeEnd {
            ee_edge: track.ee_edge.clone(),
            ee_end:  track.ee_end
        }
    }

    fn get_node_type(&self) -> NodeType {
        if ! self.slots[SLOT_3].ee_edge.is_empty() { return NodeType::Junction; }
        if ! self.slots[SLOT_2].ee_edge.is_empty() { return NodeType::Continuation; }
        if ! self.slots[SLOT_1].ee_edge.is_empty() { return NodeType::Terminator; }
        return NodeType::Empty;
    }
}