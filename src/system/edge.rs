// edge.rs

use super::common;
use common::NodeSlot;
use common::End;

pub struct Edge
{
    pub name: String,
    pub ends: [NodeSlot; common::NUM_ENDS],
}

impl Edge {

    pub fn get_node(&self, end: End) -> NodeSlot
    {
        assert!(end == common::END_A || end == common::END_B);
        NodeSlot {
            ns_node: self.ends[end].ns_node.clone(),
            ns_slot: self.ends[end].ns_slot
        }
    }

    pub fn assign_node_slot(&mut self, node: &NodeSlot, node_end: End) {
        assert!(node_end == common::END_A || node_end == common::END_B);
        self.ends[node_end].ns_node = node.ns_node.clone();
        self.ends[node_end].ns_slot = node.ns_slot;
    }
}
