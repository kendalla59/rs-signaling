// train.rs

use super::common;
use common::EdgeEnd;
use common::END_A;
use common::END_B;

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

    pub fn place_on_track(&mut self, start: &String, end: &String) {
        self.edge.ee_edge = start.clone();
        self.edge.ee_end = END_B; // get_optimal_route determines the final value.
        self.destination = end.clone();
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