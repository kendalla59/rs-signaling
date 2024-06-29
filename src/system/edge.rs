// edge.rs

use super::common::*;

pub struct Edge
{
    pub name: String,
    pub ends: [NodeSlot; NUM_ENDS],
}
