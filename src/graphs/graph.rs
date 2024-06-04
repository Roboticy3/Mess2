
pub trait Graph<V, E> {
    fn get_neighbors(&self, vertex:V) -> Option<Vec<V>>;

    fn has_vertex(&self, vertex:V) -> bool;
    fn has_edge(&self, edge:E, from:V, to:V) -> bool;
}

pub trait MutableGraph<V, E> : Graph<V, E> {
    fn add_vertex(&self, vertex:V) -> bool;
    fn add_edge(&self, edge:E, from:V, to:V) -> bool;
    fn remove_vertex(&self, vertex:V) -> bool;
    fn remove_edge(&self, edge:E, from:V, to:V) -> bool;
}