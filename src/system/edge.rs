// edge.rs

use crate::system;
use super::common;
use common::*;
use common::JSwitch::JSwitchNone;
use common::JSwitch::JSwitchLeft;
use common::JSwitch::JSwitchRight;

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

    pub fn show(&self, sys: &system::System, show_end: End) {
        let edge_name = &self.name;
        let mut msg = String::new();
        let show_edge;
        match sys.edge_map.get(edge_name) {
            None => return,
            Some(e) => show_edge = e,
        }

        if (show_end == END_A) || (show_end == NUM_ENDS) {
            let node = &show_edge.ends[END_A];
            match sys.node_map.get(&node.ns_node) {
                None => { println!("ERROR: Edge has null end node"); return },
                Some(n) => {
                    match n.get_node_type() {
                        NodeType::Empty => {
                            // TODO: Report error?
                        }
                        NodeType::Terminator => {
                            msg += "<term-> ||== ";
                        }
                        NodeType::Continuation => {
                            let edge = n.get_next(node.ns_slot);
                            msg += &edge.ee_edge;
                            msg += " <==> ";
                        }
                        NodeType::Junction => {
                            let sw = n.get_switch_pos();
                            let slot =
                                if sw == JSwitchRight { SLOT_3 } else { SLOT_2 };
                            if node.ns_slot == SLOT_1 {
                                let edge = n.get_edge_end(slot);
                                if edge.ee_edge.is_empty()  { msg += "<empty>"; }
                                else                        { msg += &edge.ee_edge; }

                                if      sw == JSwitchNone   { msg += " XX"; }
                                else if sw == JSwitchLeft   { msg += " //"; }
                                else                        { msg += " \\\\"; }
                                msg += "=> ";
                            }
                            else {
                                let edge = n.get_edge_end(SLOT_1);
                                if edge.ee_edge.is_empty()  { msg += "<empty>"; }
                                else                        { msg += &edge.ee_edge; }

                                if slot == node.ns_slot     { msg += " <="; }
                                else                        { msg += " X="; }

                                if      sw == JSwitchNone   { msg += "XX "; }
                                else if sw == JSwitchLeft   { msg += "// "; }
                                else                        { msg += "\\\\ "; }
                            }
                        }
                    }
                }
            }
            // TODO: Add signals (R/G).
            msg += "_ ";
        }

        msg += edge_name;

        if (show_end == END_B) || (show_end == NUM_ENDS) {
            // TODO: Add signals (R/G).
            msg += " _";

            let node = &show_edge.ends[END_B];
            match sys.node_map.get(&node.ns_node) {
                None => { println!("ERROR: Edge has null end node"); return },
                Some(n) => {
                    match n.get_node_type() {
                        NodeType::Empty => {
                            // TODO: Report error?
                        }
                        NodeType::Terminator => {
                            msg += " ==|| <-term>";
                        }
                        NodeType::Continuation => {
                            let edge = n.get_next(node.ns_slot);
                            msg += " <==> ";
                            msg += &edge.ee_edge;
                        }
                        NodeType::Junction => {
                            let sw = n.get_switch_pos();
                            let slot =
                                if sw == JSwitchRight { SLOT_3 } else { SLOT_2 };
                            if node.ns_slot == SLOT_1 {
                                msg += " <=";
                                let edge = n.get_edge_end(slot);
                                if      sw == JSwitchNone   { msg += "XX "; }
                                else if sw == JSwitchLeft   { msg += "// "; }
                                else                        { msg += "\\\\ "; }

                                if edge.ee_edge.is_empty()  { msg += "<empty>"; }
                                else                        { msg += &edge.ee_edge; }
                            }
                            else {
                                if      sw == JSwitchNone   { msg += " XX"; }
                                else if sw == JSwitchLeft   { msg += " //"; }
                                else                        { msg += " \\\\"; }

                                if slot == node.ns_slot     { msg += "=> "; }
                                else                        { msg += "=X "; }

                                let edge = n.get_edge_end(SLOT_1);
                                if edge.ee_edge.is_empty()  { msg += "<empty>"; }
                                else                        { msg += &edge.ee_edge; }
                            }
                        }
                    }
                }
            }
        }
        println!("{msg}");
        /*
        if (m_train) {
            if (m_train->getPosition().eeEnd == eEndA) {
                msg += "  /[o==o]-[o==o]  ";
            }
            else {
                msg += "   [o==o]-[o==o]\\ ";
            }
            msg += m_train->name();
        }
        std::cout << msg << std::endl;
        */
    }
}
