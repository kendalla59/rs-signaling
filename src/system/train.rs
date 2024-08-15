// train.rs

use super::common;
use common::EdgeEnd;
use common::END_A;
//use common::END_B;
use common::NUM_ENDS;

use crate::system;
use system::System;

//use super::edge;
//use edge::Edge;

pub struct Train
{
    pub name: String,
    pub edge: EdgeEnd,
    pub destination: String,
}

impl Train {
    pub fn get_position(&self) -> &EdgeEnd {
        &self.edge
    }

    pub fn place_on_track(&mut self, sys: &mut System, start: &String, end: &String) -> i32 {
        if let Some(gref) = sys.get_edge_mut(&self.edge.ee_edge) {
            gref.set_train("");
            self.edge.ee_edge = String::new();
            self.edge.ee_end = NUM_ENDS;
            self.destination = String::new();
        }
        println!("Train {} starting at {} and going to {}.", self.name, start, end);
        return 0;
    }

    pub fn show(&self, sys: &System) {
        println!("Train: {}", &self.name);
        if let Some(eref) = sys.get_edge(&self.edge.ee_edge) {
            println!("  Location: track segment \"{}\"", eref.name);
            println!("  Direction: toward segment end {}",
                        if self.edge.ee_end == END_A { "A" } else { "B" });
        }
    }

    pub fn get_optimal_route(&mut self, _sys: &mut System) {
    }

}