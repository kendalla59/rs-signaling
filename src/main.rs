// main.rs
//
// Author: Kendall Auel
// Description:
//     Entry point for the railroad signaling case study implementation.
//
pub mod version;
use version::SG_VERSION_MAJOR;
use version::SG_VERSION_MINOR;

pub mod common;
use common::END_A;
use common::END_B;
use common::NUM_ENDS;
use common::JSwitch::JSwitchLeft;

pub mod system;
use system::System;
use system::edge_count;

use std::io;
use std::io::Write;

fn enter_name() -> String
{
    let mut resp = String::new();
    print!("Enter track segment name: ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut resp) {
        Ok(_)   => resp = resp.trim_end().to_string(),
        Err(_)  => resp.clear(),
    }
    if resp.is_empty() {
        println!("No name entered, quitting...");
    }
    resp
}

fn name_from_number(num: &String) -> String
{
    let unum : u32;
    match num.trim().parse() {
        Ok(n) => unum = n,
        Err(_) => return String::new(),
    }
    format!("tseg{:03}", unum)
}

fn enter_a_or_b() -> common::End
{
    let mut rval = NUM_ENDS;
    while rval == NUM_ENDS {
        let mut resp = String::new();
        print!("Enter track end (A or B): ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut resp) {
            Ok(_)   => resp = resp.trim_end().to_string(),
            Err(_)  => resp.clear(),
        }
        if      (resp == "a") || (resp == "A") { rval = END_A; }
        else if (resp == "b") || (resp == "B") { rval = END_B; }
        else if ! resp.is_empty() {
            println!("Expected one of [ABab], got {resp}");
        }
    }
    rval
}

fn cmd_add_segment(sys: &mut System) -> i32 {
    if let Some(edge) = sys.create_edge("") {

        println!("Added new track segment \"{}\"", edge.name);
        return 0;
    }
    else {
        println!("ERROR: Failed to add a new track segment");
        return 1;
    }
}
fn cmd_connect_segments(sys: &mut System) -> i32 {
    let resp1 = enter_name();
    if resp1.is_empty() { return 0; }
    let edge1_name;
    if sys.has_edge(&resp1) {
        edge1_name = resp1;
    }
    else {
        let rnum = name_from_number(&resp1);
        if sys.has_edge(&rnum) {
            edge1_name = rnum;
        }
        else {
            println!("No such segment \"{resp1}\"");
            return 1; // EINVAL
        }
    }
    let end1 = enter_a_or_b();

    let resp2 = enter_name();
    if resp2.is_empty() { return 0; }
    let edge2_name;
    if sys.has_edge(&resp2) {
        edge2_name = resp2;
    }
    else {
        let rnum = name_from_number(&resp2);
        if sys.has_edge(&rnum) {
            edge2_name = rnum;
        }
        else {
            println!("No such segment \"{resp2}\"");
            return 1; // EINVAL
        }
    }
    let end2 = enter_a_or_b();

    let seg1 = common::EdgeEnd { ee_edge: edge1_name.clone(), ee_end: end1 };
    let seg2 = common::EdgeEnd { ee_edge: edge2_name.clone(), ee_end: end2 };
    let rc = sys.connect_segments(&seg1, &seg2);
    if rc == 0 {
        if let Some(edge) = sys.get_edge(&edge1_name) {
            edge.show(sys, NUM_ENDS);
        }
    }
    rc
}

fn cmd_place_signal(sys: &mut System) -> i32 {
    let mut resp1 = enter_name();
    if resp1.is_empty() { return 0; }
    let mut edge = sys.get_edge_mut(&resp1);
    match edge {
        None => {
            let rnum = resp1;
            resp1 = name_from_number(&rnum);
            edge = sys.get_edge_mut(&resp1);
            match edge {
                None => {
                    println!("No such segment \"{rnum}\"");
                    return 1;
                }
                Some(_) => (),
            }
        }
        Some(_) => (),
    }
    let end1 = enter_a_or_b();

    if let Some(eref) = edge {
        let rc = eref.place_signal_light(end1);
        if rc == 0 {
            sys.update_all_signals();

            // Get the immutable edge to show the final result.
            let shed = sys.get_edge(&resp1);
            if let Some(sref) = shed {
                sref.show(sys, end1);
            }
        }
        return rc;
    }
    println!("ERROR: cmd_place_signal failed");
    return 1;
}
fn cmd_toggle_switch(sys: &mut System) -> i32 {
    let jctv = sys.get_all_junctions();
    if jctv.is_empty() {
        println!(">>> There are no junctions in the track network <<<");
        return 0;
    }
    let mut jnum = 0;
    for name in &jctv {
        jnum += 1;
        println!("{jnum}: {name}");
    }
    let mut numstr = String::new();
    print!("Enter junction (1..{jnum}): ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut numstr) {
        Ok(_)   => numstr = numstr.trim_end().to_string(),
        Err(_)  => numstr.clear(),
    }
    if numstr.is_empty() {
        println!("No entry, quitting function...");
    }
    let mut val : usize;
    match numstr.trim().parse() {
        Ok(n) => val = n,
        Err(_) => val = 0,
    }
    if (val < 1) || (val > jnum) {
        println!("No such junction");
        return 1;
    }
    val -= 1; // Make the index zero based.
    let mut jpos = common::JSwitch::JSwitchNone;
    if let Some(node) = sys.get_node(&jctv[val]) {
        node.toggle_switch_pos();
        jpos = node.get_switch_pos();
    }
    println!("{}: junction switch is {}",
        &jctv[val], if jpos == JSwitchLeft { "LEFT" } else { "RIGHT" });

    sys.update_all_signals();
    return 0;
}
fn cmd_list_segments(sys: &System) -> i32 {
    sys.show_edges();
    return 0;
}
fn cmd_show_connections(sys: &System) -> i32 {
    sys.show_nodes();
    return 0;
}
fn cmd_place_train(sys: &mut System) -> i32 {
    let mut tname = String::new();
    print!("Enter train name (RETURN to create new): ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut tname) {
        Ok(_)   => tname = tname.trim_end().to_string(),
        Err(_)  => tname.clear(),
    }
    if tname.is_empty() {
        let topt = sys.create_train("");
        match topt {
            Some(tref) => {
                tname = tref.name.clone();
                println!("Placing new train \"{}\":", &tname);
            }
            None => {
                panic!("Failed to create a new train");
            }
        }
    }
    else {
        let topt = sys.get_train(&tname);
        if let None = topt {
            println!("No such train: \"{}\"", &tname);
            return 1;
        }
    }
    let ename1;
    print!("Starting - ");
    let resp = enter_name();
    if resp.is_empty() { return 0; }
    let mut eopt = sys.get_edge(&resp);
    if let None = eopt {
        ename1 = name_from_number(&resp);
        eopt = sys.get_edge(&ename1);
        if let None = eopt {
            println!("No such segment \"{}\"", &resp);
            return 1;
        }
    }
    else {
        ename1 = resp;
    }
    if let Some(eref) = eopt {
        if !eref.train.is_empty() {
            println!("ERROR: A train is already on segment: {}", &ename1);
            return 1;
        }
    }
    print!("Ending - ");
    let ename2;
    let resp = enter_name();
    if resp.is_empty() { return 0; }
    let mut eopt = sys.get_edge(&resp);
    if let None = eopt {
        ename2 = name_from_number(&resp);
        eopt = sys.get_edge(&ename2);
        if let None = eopt {
            println!("No such segment \"{}\"", &resp);
            return 1;
        }
    }
    else {
        ename2 = resp;
    }

    let mut ename = "";
    if let Some(tref) = sys.get_train(&tname) {
        let edge = tref.get_position();
        ename = edge.ee_edge.as_str();
    }
    if let Some(gref) = sys.get_edge_mut(&String::from(ename)) {
        gref.set_train("");
    }
    if let Some(tref) = sys.get_train_mut(&tname) {
        tref.place_on_track(&ename1, &ename2);
    }
    if let Some(eref) = sys.get_edge_mut(&ename1) {
        eref.set_train(tname.as_str());
    }
    sys.update_all_signals();
    if let Some(tref) = sys.get_train(&tname) {
        tref.show(sys);
    }
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

fn run_command_build(sys: &mut System) -> i32 {
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
            rc = cmd_add_segment(sys);
            println!("----------------------------------------------------");
        }
        2 => {
            println!("-------------- Connect Track Segment ---------------");
            rc = cmd_connect_segments(sys);
            println!("----------------------------------------------------");
        }
        3 => {
            println!("---------------- Place Signal Light ----------------");
            rc = cmd_place_signal(sys);
            println!("----------------------------------------------------");
        }
        4 => {
            println!("-------------- Toggle Junction Switch --------------");
            rc = cmd_toggle_switch(sys);
            println!("----------------------------------------------------");
        }
        5 => {
            println!("--------------- List Track Segments ----------------");
            rc = cmd_list_segments(sys);
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

fn run_command(sys: &mut System) -> i32 {
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
            while run_command_build(sys) == 0 {}
            rc = 0;
            println!("----------------------------------------------------");
        }
        2 => {
            println!("--------------- List Track Segments ----------------");
            rc = cmd_list_segments(sys);
            println!("----------------------------------------------------");
        }
        3 => {
            println!("----------------- Show Connections -----------------");
            rc = cmd_show_connections(sys);
            println!("----------------------------------------------------");
        }
        4 => {
            println!("--------------- Place Train On Track ---------------");
            rc = cmd_place_train(sys);
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

    let mut sys = system::create_system();

    while run_command(&mut sys) == 0 {}
}

//    sys().resetTrackNetwork();
//    return 0;
//}
