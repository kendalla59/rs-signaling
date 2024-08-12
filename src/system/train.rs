// train.rs

use super::common;
use common::EdgeEnd;
use common::END_A;
use common::END_B;
use common::NUM_ENDS;

use crate::system;
use system::System;

use super::edge;
use edge::Edge;

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

    pub fn place_on_track(&mut self, sys: &mut System, start: Option<&mut Edge>, end: Option<&mut Edge>) -> i32 {
        if let Some(gref) = sys.get_edge_mut(&self.edge.ee_edge) {
            gref.set_train("");
            self.edge.ee_edge = String::new();
            self.edge.ee_end = NUM_ENDS;
            self.destination = String::new();
        }
        match start {
            None => { return 0 }            // Not starting anywhere
            Some(eref1) => {
                if let Some(_) = sys.get_train(eref1.get_train()) {
                    println!("ERROR: A train is already on segment: {}", eref1.name);
                }
                match end {
                    None => { return 0; }   // Not ending anywhere
                    Some(_eref2) => {
                        eref1.set_train(&self.name);
                        self.edge.ee_edge = eref1.name.clone();
                        self.edge.ee_end = END_B; // get_optimal_route determines the final value.

                        self.get_optimal_route(sys);
                    }
                }
            }
        }
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