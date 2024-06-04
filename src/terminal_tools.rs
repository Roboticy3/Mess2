use std::io;
use std::io::Error;
use std::io::ErrorKind;

use phf;

use super::collection_traits;

pub fn key_menu<M: collection_traits::Map<char, V>, V:Clone>(key_map:&M) -> Result<V, Error> {
    let mut buf = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    let _result = stdin.read_line(&mut buf)?;

    let first = buf.chars().next().unwrap().to_ascii_uppercase();

    let final_result = key_map._get(&first);

    match final_result {
        Some(result) => Ok(result.to_owned()),
        None => Err(Error::other("no selection"))
    }
}

pub fn looped_key_menu<M: collection_traits::Map<char, V>, V:Clone>(message:String, key_map:&M) -> std::io::Result<V> {
    let mut result;

    loop {
        println!("{}",message);
        result = key_menu(key_map);

        match result {
            Ok(game) => {return Ok(game);}
            Err(e) => {
                match e.kind() {
                    ErrorKind::Other => {continue;}
                    error => {panic!("io error! {:?}", error)}
                }
            }
        }
    }
}

pub const alphabeticOptions:[char; 26] = 
    ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];