
use ndarray::{s, Array2, Axis};

use super::graph::*;
pub struct AdjacencyMatrixGraph<E> {
    pub m:Array2<Option<E>>
}

impl<E> AdjacencyMatrixGraph<E>
where E : Clone
{
    fn vertex_in_range(&self, vertex:usize) -> bool {
        vertex < self.m.len_of(Axis(1))
    }

    fn edge_in_range(&self, from:usize, to:usize) -> bool {
        (self.vertex_in_range(from)) && (self.vertex_in_range(to))
    }

    fn is_valid(&self) -> bool {
        let s = self.m.shape();
        for i in 0..(s.len() - 1) {
            if s[i] != s[i + 1] {
                return false;
            }
        }
        return true;
    }
}

impl<E> Graph<usize, E> for AdjacencyMatrixGraph<E> 
where E : PartialEq + Clone
{
    fn get_neighbors(&self, vertex:usize) -> Option<Vec<usize>> {

        if !(self.vertex_in_range(vertex)) {
            return None;
        }

        let mut result:Vec<usize> = vec![];

        let row = self.m.slice(s![vertex, ..]);
        for i in 0..row.len() {
            match self.m.get((vertex, i)) {
                Some(maybe_edge) => {
                    match maybe_edge {
                        Some(_edge) => { result.push(i); },
                        None => { continue; }
                    }
                },
                None => { continue; }
            }
        }

        return Some(result);
    }

    fn has_vertex(&self, vertex:usize) -> bool {
        self.vertex_in_range(vertex)
    }

    fn has_edge(&self, edge:&E, from:usize, to:usize) -> bool {
        if !(self.edge_in_range(from, to)) {return false;} 

        let optional_edge = self.m.get((from, to));
        
        match optional_edge {
            Some(e) => {
                match e {
                    Some(e2) => {
                        return e2 == edge
                    },
                    None => false
                }
            }
            None => false
        }
    }
}

impl<E> Clone for AdjacencyMatrixGraph<E>
where E : Clone {
    fn clone(&self) -> Self {
        Self { m: self.m.clone() }
    }
}

impl<E> MutableGraph<usize, E> for AdjacencyMatrixGraph<E> 
where E : PartialEq + Clone
{
    fn add_vertex(&mut self, vertex:usize) -> bool {
        false
    }

    fn add_edge(&mut self, edge:E, from:usize, to:usize) -> bool {
        if !self.edge_in_range(from, to) {return false;}

        let result = self.has_edge(&edge, from, to);

        self.m.slice_mut(s![from, to]).fill(Some(edge));

        result
    }

    fn remove_vertex(&mut self, vertex:usize) -> bool {
        false
    }

    fn remove_edge(&mut self, edge:E, from:usize, to:usize) -> bool {
        if !self.edge_in_range(from, to) {return false;}

        let result = self.has_edge(&edge, from, to);

        self.m.slice_mut(s![from, to]).fill(None);

        result
    }
}