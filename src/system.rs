// system.rs
//
// Author: Kendall Auel
//
// The system module represents the entire railroad system
// including the track network and all trains running on the
// tracks.
//

pub mod edge;
pub mod node;
pub mod train;

use std::collections::HashMap;

struct System
{
    edge_map:    HashMap<String, Edge&>,
    node_map:    NodeMap<String, Node&>,
    train_map:   TrainMap<String, Train&>,
}

pub const EMPTY_STR: String = String::new();

pub fn edge_count() -> i32 {
    return 0;
}