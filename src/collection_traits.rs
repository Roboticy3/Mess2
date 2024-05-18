
pub trait Map<K, V> {
    fn _get(&self, key:&K) -> Option<&V>;
}

impl<K, V> Map<K, V> for phf::Map<K, V> 
where  K : phf_shared::PhfBorrow<K>, K: Eq, K: phf::PhfHash
{
    fn _get(&self, key:&K) -> Option<&V> {
        self.get(key)
    }
}

impl <K, V> Map<K, V> for std::collections::HashMap<K, V> 
where K: Eq, K: std::hash::Hash
{
    fn _get(&self, key:&K) -> Option<&V> {
        self.get(key)
    }
}