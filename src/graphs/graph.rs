
pub trait Graph<V, E> {

    fn has_vertex(&self, vertex:V) -> bool;
    fn has_edge(&self, edge:&E, from:V, to:V) -> bool;

    fn get_neighbors(&self, vertex:V) -> Vec<V>;
    fn get_connected(&self, vertex:V) -> Vec<V>;

    fn get_direct_edges(&self, vertex:V) -> Vec<&E>;

    fn get_path(&self, from:V, to:V) -> Option<Vec<&E>>;
}

pub trait MutableGraph<V, E> : Graph<V, E> {
    fn add_vertex(&mut self, vertex:V) -> bool;
    fn add_edge(&mut self, edge:&E, from:V, to:V) -> bool;
    fn remove_vertex(&mut self, vertex:V) -> bool;
    fn remove_edge(&mut self, edge:&E, from:V, to:V) -> bool;
}

pub trait MaskGraph<E> : Graph<usize, E> {
    fn get_neighbors_mask(&self, vertex:usize) -> Vec<bool>;
    fn get_connected_mask(&self, vertex:usize) -> Vec<bool>;
}

pub trait SearchableGraph<V, E> : Graph<V, E> {
    fn get_edges<F: Fn(&E, V, V) -> bool>(&self, filter:F) -> Vec<(&E, V, V)>;
    fn get_vertices<F: Fn(V) -> bool>(&self, filter:F) -> Vec<V>;
}
