

pub trait Map<K, V> {
    fn _get(&self, key:&K) -> Option<&V>;

    fn _size(&self) -> usize;

    fn _keys_vec(&self) -> Vec<K>;
}

impl<K, V> Map<K, V> for phf::Map<K, V> 
where  K : phf_shared::PhfBorrow<K>, K: Eq, K: phf::PhfHash, K: Clone
{
    fn _get(&self, key:&K) -> Option<&V> {
        self.get(key)
    }

    fn _size(&self) -> usize {
        self.len()
    }

    fn _keys_vec(&self) -> Vec<K> {
        self.keys().cloned().collect()
    }
}

impl<K, V> Map<K, V> for std::collections::HashMap<K, V> 
where K: Eq, K: std::hash::Hash, K: Clone
{
    fn _get(&self, key:&K) -> Option<&V> {
        self.get(key)
    }

    fn _size(&self) -> usize {
        self.len()
    }

    fn _keys_vec(&self) -> Vec<K> {
        self.keys().cloned().collect()
    }
}