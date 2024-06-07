
use std::io;

use phf_macros;

use mess::{hackenbush::hackenbush_stdio_round, twgintw::*};
use terminal_tools::looped_key_menu;

pub mod mess;
pub mod graphs;
pub mod terminal_tools;
pub mod collection_traits;


fn main() -> io::Result<()> {

    let game = select_game();

    return game();
}

const SELECT_GAME_MESSAGE:&str = 
"
Select Game:
\t(T)he Worst Game In The World
\t(H)ackenbush
";

fn select_game() -> fn() -> io::Result<()> {
    let result = looped_key_menu(SELECT_GAME_MESSAGE.to_string(), &GAME_SELECTION_MAP);
    match result {
        Ok(game) => game,
        Err(_) => panic!("io error!")
    }
}

const GAME_SELECTION_MAP:phf::Map<char, fn() -> io::Result<()>> = phf_macros::phf_map! {
    'T' => twgintw_stdio_round,
    'H' => hackenbush_stdio_round
};
