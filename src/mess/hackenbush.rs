
use ndarray::array;
use ndarray::Array;

use super::super::graphs::adjacency_matrix::*;
use super::super::graphs::graph::*;
use super::base::*;

struct HackenbushState<G: Graph<usize, bool> + Clone> {
    g:G,
    ground:Vec<bool>
}

pub fn random_hackenbush(size:usize) {
    let mut starting_graph = AdjacencyMatrixGraph {
        m:Array::from_elem((size, size), None)
    };

    let mut ground = vec![false; size];

    let mut starting_state = HackenbushState {
        g: starting_graph,
        ground: ground
    };
}