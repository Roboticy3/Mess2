
use std::io;
use std::io::ErrorKind;

use ndarray::Array2;
use ndarray::ArrayBase;
use rand::RngCore;
use rand_pcg::Pcg32;
use rand::{Rng, SeedableRng, rngs::StdRng};

use colored::Color;

use ndarray::array;
use ndarray::Array;

use super::super::graphs::adjacency_matrix::*;
use super::super::graphs::graph::*;
use super::base::*;

pub fn hackenbush_stdio_round() -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let starting_state_option = random_hackenbush(10, 4, rng.next_u64());

    let starting_state = match starting_state_option {
        None => {return Err(io::Error::new(ErrorKind::Other, "failed to produce starting state"));},
        Some(s) => s
    };

    Ok(())
}

pub struct HackenbushState<G: Graph<usize, Color> + Clone> {
    graph:G,
    ground:Vec<bool>
}

pub fn random_hackenbush(size:usize, on_ground:usize, seed:u64) -> Option<HackenbushState<AdjacencyMatrixGraph<Color>>> {
    
    let ground = match random_ground(size, on_ground, seed) {
        Some(g) => g,
        None => {return None;}
    };

    let graph = match random_starting_graph(size, &ground, seed) {
        Some(s) => s,
        None => {return None;}
    };

    Some(HackenbushState {
        graph:graph, ground:ground
    })
}

fn random_ground(size:usize, on_ground:usize, seed:u64) -> Option<Vec<bool>> {

    if on_ground > size / 2 { return None; }

    let mut ground = vec![false; size];
    let mut rng = Pcg32::seed_from_u64(seed);
    let mut on_ground_count:usize = 0;

    loop {
        let i = rng.gen_range(0..size);
        if ground[i] {continue;}

        ground[i] = true;
        on_ground_count += 1;

        if on_ground_count >= on_ground {break;}
    }

    println!("generated ground: {:?}", ground);

    Some(ground)
}

fn random_starting_graph(size:usize, ground:&Vec<bool>, seed:u64) -> Option<AdjacencyMatrixGraph<Color>> {
    let mut m = AdjacencyMatrixGraph {
        m:Array::from_elem((size, size), None)
    };
    
    let mut rng = Pcg32::seed_from_u64(seed);

    for i in 0..size {
        let is_ground = ground[i];

        let cap = match is_ground {
            true => rng.gen_range(0..7),
            false => rng.gen_range(0..4)
        };

        for _ in 0..cap {
            let mut neighbor = rng.gen_range(0..size);
            if is_ground || neighbor == i { loop {
                if !ground[neighbor] && neighbor != i {break;}
                neighbor = rng.gen_range(0..size);
            }}

            let color = match rng.gen_range(0..2) {
                0 => Color::Blue,
                _ => Color::Red
            };

            m.add_edge(color.clone(), i, neighbor);
            m.add_edge(color, neighbor, i);
        }
    }

    println!("generated graph:");
    for i in 0..size {
        println!("{}: {:?}", i, m.get_neighbors(i));
    }

    Some(m)
    
}
