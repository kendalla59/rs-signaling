// common.rs

// Nodes can be terminator, continuation, or junction types.
//
#[derive(PartialEq)]
pub enum NodeType {
    Empty,          // No edges have been connected.
    Terminator,     // Only one connected edge.
    Continuation,   // Two connected edges.
    Junction,       // Three connected edges.
}

// A junction switch can be in one of three possible states.
//
#[derive(PartialEq, Copy, Clone)]
pub enum JSwitch {
    JSwitchNone,    // Unknown or possibly in motion.
    JSwitchLeft,    // Switch from common to left track.
    JSwitchRight,   // Switch from common to right track.
}


// Each node has three slots to which an edge can be
// attached. For a junction, slot 1 is the common edge,
// slot 2 is the left edge, and slot 3 is the right edge.
// This value also acts as the index into the array of
// Edge/End structures kept by the Node.
//
pub type Slot = usize;
pub const SLOT_1: Slot = 0;
pub const SLOT_2: Slot = 1;
pub const SLOT_3: Slot = 2;
pub const NUM_SLOTS: Slot = 3;

// Each edge has two ends, each connected to a node.
// This enum also acts as the index into the array of
// Node/Slot structures kept by the Edge.
//
pub type End = usize;
pub const END_A: End = 0;
pub const END_B: End = 1;
pub const NUM_ENDS: End = 2;

pub struct NodeSlot {
    pub ns_node: String,
    pub ns_slot: Slot,
}

pub struct EdgeEnd {
    pub ee_edge: String,
    pub ee_end:  End,
}