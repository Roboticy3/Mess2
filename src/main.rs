use std::collections::HashMap;
use std::io;
use std::io::ErrorKind;

use phf_macros;

use mess::base::*;
use mess::twgintw::*;
use terminal_tools::looped_key_menu;

pub mod mess;
pub mod terminal_tools;
pub mod collection_traits;


fn main() -> io::Result<()> {

    let game:Mess<i32> = select_game();

    game.play(twgintw_stdio_play, twgintw_stdio_display_state);

    Ok(())
}

const SELECT_GAME_MESSAGE:&str = 
"
Select Game:
\t(T)he Worst Game In The World


";

fn select_game() -> Mess<i32> {
    let result = looped_key_menu(SELECT_GAME_MESSAGE.to_string(), &GAME_SELECTION_MAP);
    match result {
        Ok(game) => game,
        Err(_) => panic!("io error!")
    }
}

const GAME_SELECTION_MAP:phf::Map<char, Mess<i32>> = phf_macros::phf_map! {
    'T' => TWGINTW
};