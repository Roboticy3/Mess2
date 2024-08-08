
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

type HackenbushGraph = AdjacencyMatrixGraph<Color>;
type Hackenbush = Mess<HackenbushState>;

const NODE_COUNT:usize = 15;
const GROUND_COUNT:usize = 3;

pub fn hackenbush_stdio_round() -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let size = NODE_COUNT;
    let starting_state_option = random_hackenbush(NODE_COUNT, GROUND_COUNT, rng.next_u64());

    let mut starting_state = match starting_state_option {
        None => {return Err(io::Error::new(ErrorKind::Other, "failed to produce starting state"));},
        Some(s) => s
    };
    
    {
        println!("untrimmed state:");
        println!("{:?}", starting_state.graph.expose());
    }
    
    {
        trim_hackenbush(&mut starting_state);
        println!("trimmed state:");
        println!("{:?}", starting_state.graph.expose());
    }

    Ok(())
}

pub struct HackenbushState {
    graph:HackenbushGraph,
    ground:Vec<bool>
}

pub fn trim_hackenbush(s:&mut HackenbushState) -> Vec<HackenbushState> {
    
    let mut graph = &mut s.graph;
    let size = graph.vertex_count();
    let ground = &s.ground;
    let mut grounded_component = vec![false; size];

    //get all vertices connected to the ground
    for i in 0..size {
        let g = ground[i];
        if g {
            let mask = graph.get_connected_mask(i);
            join_masks(&mut grounded_component, &mask);
        }
    }
    
    println!("reachable vertices: {:?}", grounded_component);

    //remove all edges of vertices not connected to the ground
    for i in 0..size {
        if grounded_component[i] { continue; }
    
        let neighbors = graph.get_neighbors(i);
        for j in neighbors {
            println!("removing edge {} -> {}", i, j);
            graph.disconnect(i, j);
        }
    }

    Vec::new()

    
}

fn join_masks(a:&mut Vec<bool>, b:&Vec<bool>) {
    for i in 0..a.len() {
        a[i] = a[i] || b[i];
    }
}

pub fn random_hackenbush(size:usize, on_ground:usize, seed:u64) -> Option<HackenbushState> {
    
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
        if on_ground_count >= on_ground {break;}
    
        let i = rng.gen_range(0..size);
        if ground[i] {continue;}

        ground[i] = true;
        on_ground_count += 1;
    }

    println!("generated ground: {:?}", ground);

    Some(ground)
}

const GROUND_START_DEGREE:usize = 2;
const NGROUND_START_DEGREE:usize = 3;
fn random_starting_graph(size:usize, ground:&Vec<bool>, seed:u64) -> Option<HackenbushGraph> {
    let mut m = AdjacencyMatrixGraph {
        m:Array::from_elem((size, size), None)
    };
    
    let mut rng = Pcg32::seed_from_u64(seed);

    for i in 0..size {
        let is_ground = ground[i];

        let cap = match is_ground {
            true => rng.gen_range(1..GROUND_START_DEGREE),
            false => rng.gen_range(1..NGROUND_START_DEGREE)
        };

        for _ in 0..1 {
            let mut neighbor = rng.gen_range(0..size);
            if neighbor == i { loop {
                neighbor = rng.gen_range(0..size);
                if is_ground && ground[neighbor] {continue;}
                if neighbor != i {break;}
            }}

            let color = match rng.gen_range(0..2) {
                0 => Color::Blue,
                _ => Color::Red
            };

            m.add_edge(color.clone(), i, neighbor);
            m.add_edge(color, neighbor, i);
        }
    }

    Some(m)
    
}
