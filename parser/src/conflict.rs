pub trait ConflictResolver<K> {
    fn resolve_conflict(&self, id: &K) -> K;
}
