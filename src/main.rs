// main.rs
//
// Author: Kendall Auel
// Description:
//     Entry point for the railroad signaling case study implementation.
//
pub mod version;
use version::SG_VERSION_MAJOR;
use version::SG_VERSION_MINOR;

pub mod system;
use system::edge_count;

use std::io;
use std::io::Write;

fn cmd_add_segment() -> i32 {
    let edge = system::Edge::new();
    println!("Added new track segment \"{}\"", edge.name);
    return 0;
}
fn cmd_connect_segments() -> i32 {
    println!("Here is where we connect track segments.");
    return 0;
}
fn cmd_place_signal() -> i32 {
    println!("Here is where we place a traffic signal.");
    return 0;
}
fn cmd_toggle_switch() -> i32 {
    println!("Here is where we toggle a track switch.");
    return 0;
}
fn cmd_list_segments() -> i32 {
    println!("Here is where we list the track segments.");
    return 0;
}
fn cmd_show_connections() -> i32 {
    println!("Here is where we show track connections.");
    return 0;
}
fn cmd_place_train() -> i32 {
    println!("Here is where we place a train on a track segment.");
    return 0;
}
fn cmd_step_simulation() -> i32 {
    println!("Here is where we step the train simulation.");
    return 0;
}
fn cmd_run_simulation() -> i32 {
    println!("Here is where we run the simulation until complete.");
    return 0;
}
fn cmd_save_network() -> i32 {
    println!("Here is where we save the current network.");
    return 0;
}
fn cmd_load_network() -> i32 {
    if edge_count() != 0 {
        println!("WARNING: This will delete the existing network");
        println!("         Press RETURN key at the prompt to quit");
    }
/*  std::string path;
    std::cout << "Enter file path: ";
    std::getline(std::cin, path);
    if (path.empty()) {
        std::cout << "No response, quitting..." << std::endl;
        return 0;
    }
    std::ifstream ifstr(path);
    if (!ifstr.good()) {
        std::cout << path << " not found, quitting..." << std::endl;
        return ENOENT;
    }

    int rc = sys().deserialize(ifstr);
    ifstr.close();
    return rc;

    println!("Here is where we load a saved track network."); */
    return 0;
}
fn cmd_signal_all_junctions() -> i32 {
    println!("Here is where we add a signal to all junctions.");
    return 0;
}

fn run_command_build() -> i32 {
    println!();
    println!("Build Track Network submenu");
    println!("1. Add a track segment");
    println!("2. Connect track segments");
    println!("3. Place a signal light");
    println!("4. Toggle junction switch");
    println!("5. List track segments");
    println!("6. Save track network");
    println!("7. Load track network");
    println!("8. Add Signals To All Junctions");
    println!("R/return");

    let mut resp = String::new();
    while resp.is_empty() {
        print!("=> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut resp) {
            Ok(_)   => resp = resp.trim_end().to_string(),
            Err(_)  => resp.clear(),
        }
    }
    if resp == "R" || resp == "r" || resp == "q" || resp == "return" {
        return 1;
    }

    let cmd;
    match resp.trim().parse() { Ok(n) => cmd = n, Err(_) => cmd = 0 }

    let rc;

    match cmd {
        1 => {
            println!("---------------- Add Track Segment -----------------");
            rc = cmd_add_segment();
            println!("----------------------------------------------------");
        }
        2 => {
            println!("-------------- Connect Track Segment ---------------");
            rc = cmd_connect_segments();
            println!("----------------------------------------------------");
        }
        3 => {
            println!("---------------- Place Signal Light ----------------");
            rc = cmd_place_signal();
            println!("----------------------------------------------------");
        }
        4 => {
            println!("-------------- Toggle Junction Switch --------------");
            rc = cmd_toggle_switch();
            println!("----------------------------------------------------");
        }
        5 => {
            println!("--------------- List Track Segments ----------------");
            rc = cmd_list_segments();
            println!("----------------------------------------------------");
        }
        6 => {
            println!("---------------- Save Track Network ----------------");
            rc = cmd_save_network();
            println!("----------------------------------------------------");
        }
        7 => {
            println!("---------------- Load Track Network ----------------");
            rc = cmd_load_network();
            println!("----------------------------------------------------");
        }
        8 => {
            println!("---------- Place Signals On All Junctions ----------");
            rc = cmd_signal_all_junctions();
            println!("----------------------------------------------------");
        }
        _ => {
            println!("Invalid entry: \"{resp}\"");
            rc = 21;
            println!("----------------------------------------------------");
        }
    }
    if rc != 0 { println!("(Error code {rc} was returned)"); }
    return 0;
}

fn run_command() -> i32 {
    println!();
    println!("Train Signaling System Simulator");
    println!("1. Build track network (submenu)");
    println!("2. List track segments");
    println!("3. Show track connections");
    println!("4. Place train on a track segment");
    println!("5. [S]tep the train simulation");
    println!("6. [R]un the train simulation");
    println!("Q/quit/exit");

    let mut resp = String::new();

    while resp.is_empty() {
        print!("=> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut resp) {
            Ok(_)   => resp = resp.trim_end().to_string(),
            Err(_)  => resp.clear(),
        }
    }
    if resp == "Q" || resp == "q" || resp == "quit" || resp == "exit" {
        return 1;
    }
    let cmd;
    if resp == "S" || resp == "s"       { cmd = 5; }    // Special case for "step".
    else if resp == "R" || resp == "r"  { cmd = 6; }    // Special case for "run".
    else { match resp.trim().parse() { Ok(n) => cmd = n, Err(_) => cmd = 0 } }

    let rc;

    match cmd {
        1 => {
            println!("--------------- Build Track Network ----------------");
            while run_command_build() == 0 {}
            rc = 0;
            println!("----------------------------------------------------");
        }
        2 => {
            println!("--------------- List Track Segments ----------------");
            rc = cmd_list_segments();
            println!("----------------------------------------------------");
        }
        3 => {
            println!("----------------- Show Connections -----------------");
            rc = cmd_show_connections();
            println!("----------------------------------------------------");
        }
        4 => {
            println!("--------------- Place Train On Track ---------------");
            rc = cmd_place_train();
            println!("----------------------------------------------------");
        }
        5 => {
            println!("----------------- Step Simulation ------------------");
            rc = cmd_step_simulation();
            println!("----------------------------------------------------");
        }
        6 => {
            println!("------------------ Run Simulation ------------------");
            rc = cmd_run_simulation();
            println!("----------------------------------------------------");
        }
        _ => {
            println!("Invalid entry: \"{resp}\"");
            rc = 21;
            println!("----------------------------------------------------");
        }
    }
    if rc != 0 { println!("(Error code {rc} was returned)"); }
    return 0;
}

// ------------------------------------------------------------------
// main -- Entry point
// ------------------------------------------------------------------

fn main() {
    println!("Case Study Implementation -- Railroad Signaling System");
    println!("Version {SG_VERSION_MAJOR}.{SG_VERSION_MINOR}");

    while run_command() == 0 {}
}

//    sys().resetTrackNetwork();
//    return 0;
//}
