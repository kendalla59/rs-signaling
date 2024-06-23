// common.rs

pub mod system::node;
use node::Node;

// Each node has three slots to which an edge can be
// attached. For a junction, slot 0 is the common edge,
// slot 1 is the left edge, and slot 2 is the right edge.
// This value also acts as the index into the array of
// Edge/End structures kept by the Node.
//
pub type Slot = u8;
pub const COMM_SLOT: Slot = 0;
pub const LEFT_SLOT: Slot = 1;
pub const RGHT_SLOT: Slot = 2;
pub const NUM_SLOTS: Slot = 3;

pub struct NodeSlot {
    ns_node: &mut Node,
    ns_slot: Slot,
}

pub struct EdgeEnd {
    ee_edge: &mut Edge,
    ee_end:  End,
}