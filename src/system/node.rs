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
        }
    }

    pub fn make_continuation(&mut self, track: &EdgeEnd) {
        assert!(self.get_node_type() == NodeType::Terminator,
                "Attempt to makeContinuation, but node is not a terminator");
        self.slots[SLOT_2] = EdgeEnd {
            ee_edge: track.ee_edge.clone(),
            ee_end:  track.ee_end,
        }
    }

    pub fn make_junction(&mut self, track: &EdgeEnd, slot: Slot) {
        assert!(self.get_node_type() == NodeType::Continuation,
                "Attempt to makeJunction, but node is not a continuation");

        // Swap slot 1 and 2 if the common edge is on slot 2.
        // NOTE: The NodeSlot for the Edge, and the EdgeEnd for the Node
        //       must both be updated, or the network will become corrupted.
        if (slot == SLOT_2) {
            let edge;
            let ns;
            let e1 = self.get_edge_end(SLOT_1);
            let e2 = self.get_edge_end(SLOT_2);

            eptr = e1.eeEdge.lock();
            if (!eptr) { throw std::runtime_error("Slot1 edge is null"); }
            ns = eptr->getNode(e1.eeEnd);
            if (ns.nsSlot != eSlot1) { throw std::runtime_error("Assert slot1"); }
            ns.nsSlot = eSlot2;
            eptr->assignNodeSlot(ns, e1.eeEnd);
            setEdgeEnd(e1, eSlot2);
    
            eptr = e2.eeEdge.lock();
            if (!eptr) { throw std::runtime_error("Slot2 edge is null"); }
            ns = eptr->getNode(e2.eeEnd);
            if (ns.nsSlot != eSlot2) { throw std::runtime_error("Assert slot2"); }
            ns.nsSlot = eSlot1;
            eptr->assignNodeSlot(ns, e2.eeEnd);
            setEdgeEnd(e2, eSlot1);
        }
        setEdgeEnd(track, eSlot3);
        m_switchState = eSwitchLeft;
        }

    pub fn get_node_type(&self) -> NodeType {
        if ! self.slots[SLOT_3].ee_edge.is_empty() { return NodeType::Junction; }
        if ! self.slots[SLOT_2].ee_edge.is_empty() { return NodeType::Continuation; }
        if ! self.slots[SLOT_1].ee_edge.is_empty() { return NodeType::Terminator; }
        return NodeType::Empty;
    }

    fn get_edge_end(&self, slot: Slot) -> EdgeEnd {
        EdgeEnd {
            ee_edge: self.slots[slot].ee_edge.clone(),
            ee_end:  self.slots[slot].ee_end,
        }
    }
}