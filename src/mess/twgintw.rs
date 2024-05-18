
use super::base::*;

use super::super::terminal_tools::*;

pub const TWGINTW:Mess<i32> = Mess {
    starting_state:0,
    option:twginw_option
};

fn twginw_option(s:i32) -> Vec<i32> {
    match s {
        0 => return vec![0, 1],
        _ => return vec![]
    }
}

pub fn twgintw_stdio_play(options:Vec<i32>) -> Option<i32> {
    let mut key_map = std::collections::HashMap::new();
    let mut message = "Select Option:\n".to_string();
    let mut k = 0;
    for o in options.iter() {
        key_map.insert(alphabeticOptions[k].to_ascii_uppercase(), o.clone());
        message.push_str(format!("\t({}): {:?}\n", alphabeticOptions[k], o.clone()).as_str());
        k += 1;
        if k == 26 {break;}
    }

    let result = looped_key_menu(message, &key_map);
    match result {
        Ok(o) => Some(o),
        _ => None
    }
}

pub fn twgintw_stdio_display_state(s:i32) {
    println!("Current State: {}",s);
}