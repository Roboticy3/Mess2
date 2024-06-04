
use std::option;

use ndarray::{s, Array2};

use super::graph::*;
use super::super::collection_traits;

pub struct AdjacencyMatrixGraph<E> {
    m:Array2<Option<E>>
}

impl<E> Graph<usize, E> for AdjacencyMatrixGraph<E> 
where E : PartialEq + Clone
{
    fn get_neighbors(&self, vertex:usize) -> Option<Vec<usize>> {
        let size = self.m.shape();

        if !(self.has_vertex(vertex)) {
            return None;
        }

        let mut result:Vec<usize> = vec![];

        let row = self.m.slice(s![vertex, ..]);
        for i in 0..row.len() {
            match self.m.get((vertex, i)) {
                Some(_edge) => {
                    result.push(i);
                },
                None => {
                    continue;
                }
            }
        }

        return Some(result);
    }

    fn has_vertex(&self, vertex:usize) -> bool {
        let size = self.m.shape();
        
        vertex >= size[0]
    }

    fn has_edge(&self, edge:E, from:usize, to:usize) -> bool {
        if !(self.has_vertex(from)) || !(self.has_vertex(to)) {return false;} 

        let optional_edge = self.m.get((from, to));
        
        match optional_edge {
            Some(e) => {
                match e {
                    Some(e2) => {
                        return e2.clone() == edge
                    },
                    None => false
                }
            }
            None => false
        }
    }
}