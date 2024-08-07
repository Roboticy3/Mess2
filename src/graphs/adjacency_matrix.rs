
use ndarray::{s, Array2, Axis};

use super::graph::*;
pub struct AdjacencyMatrixGraph<E> {
    pub m:Array2<Option<E>>
}

impl<E> AdjacencyMatrixGraph<E>
where E : Clone
{
    fn vertex_count(&self) -> usize {
        return self.m.len_of(Axis(1));
    }

    fn vertex_in_range(&self, vertex:usize) -> bool {
       vertex < self.vertex_count()
    }

    fn vertex_mask_to_vertex_list(&self, mask:Vec<bool>) ->  Vec<usize> {
        let size = mask.len();
        let mut list = Vec::with_capacity(size);
        for i in 0..size {
            if mask[i] { list.push(i) }
        }

        list
    }

    fn edge_in_range(&self, from:usize, to:usize) -> bool {
        (self.vertex_in_range(from)) && (self.vertex_in_range(to))
    }

    /**
     * assert that the graph matrix is square 
     * O(n)
     */
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

        Some(result)
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

    /**
     * Use Dijkstra's algorithm to compute the component of a vertex
     */
    fn get_connected(&self, vertex:usize) -> Option<Vec<usize>>  {
        if !(self.vertex_in_range(vertex)) {
           return None;
        }

        let size = self.vertex_count();
        let mut result:Vec<bool> = vec![false; size];
        let mut stack = Vec::with_capacity(size);
        stack.push(vertex);

        loop {
            let u = match stack.pop() {
                Some(vertex) => vertex,
                None => {break;}
            };

            let neighbors = self.get_neighbors(u)?;
            for i in 0..neighbors.len() {
                let w = neighbors[i];

                if !result[w] {stack.push(w);}
                result[w] = true;
            }
        }

        Some(self.vertex_mask_to_vertex_list(result))
    }

    fn is_connected(&self, from:usize, to:usize) -> bool {
        panic!("Not Implemented");
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
    fn add_vertex(&mut self, _vertex:usize) -> bool {
        panic!("Not Implemented");
    }

    fn add_edge(&mut self, edge:E, from:usize, to:usize) -> bool {
        if !self.edge_in_range(from, to) {return false;}

        let result = self.has_edge(&edge, from, to);

        self.m.slice_mut(s![from, to]).fill(Some(edge));

        result
    }

    fn remove_vertex(&mut self, _vertex:usize) -> bool {
        panic!("Not Implemented");
    }

    fn remove_edge(&mut self, edge:E, from:usize, to:usize) -> bool {
        if !self.edge_in_range(from, to) {return false;}

        let result = self.has_edge(&edge, from, to);

        self.m.slice_mut(s![from, to]).fill(None);

        result
    }
}
