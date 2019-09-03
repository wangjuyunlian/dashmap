use super::iter::{Iter, IterMut};
use super::mapref::one::{DashMapRef, DashMapRefMut};
use super::DashMap;
use std::borrow::Borrow;
use std::hash::Hash;
use super::util;

pub trait ExecutableQuery {
    type Output;

    fn exec(self) -> Self::Output;
}

// -- Query

pub struct Query<'a, K: Eq + Hash, V> {
    map: &'a DashMap<K, V>,
}

impl<'a, K: Eq + Hash, V> Query<'a, K, V> {
    pub fn new(map: &'a DashMap<K, V>) -> Self {
        Self { map }
    }

    pub fn insert(self, key: K, value: V) -> QueryInsert<'a, K, V> {
        QueryInsert::new(self, key, value)
    }

    pub fn get<'k, Q: Eq + Hash>(self, key: &'k Q) -> QueryGet<'a, 'k, Q, K, V>
    where
        K: Borrow<Q>,
    {
        QueryGet::new(self, key)
    }

    pub fn remove<'k, Q: Eq + Hash>(self, key: &'k Q) -> QueryRemove<'a, 'k, Q, K, V>
    where
        K: Borrow<Q>,
    {
        QueryRemove::new(self, key)
    }

    pub fn len(self) -> QueryLength<'a, K, V> {
        QueryLength::new(self)
    }

    pub fn clear(self) -> QueryClear<'a, K, V> {
        QueryClear::new(self)
    }

    pub fn is_empty(self) -> QueryIsEmpty<'a, K, V> {
        QueryIsEmpty::new(self)
    }

    pub fn iter(self) -> QueryIter<'a, K, V> {
        QueryIter::new(self)
    }

    pub fn iter_mut(self) -> QueryIterMut<'a, K, V> {
        QueryIterMut::new(self)
    }

    pub fn alter_all<F: FnMut(&K, V) -> V>(self, f: F) -> QueryAlterAll<'a, K, V, F> {
        QueryAlterAll::new(self, f)
    }
}

// --

// -- QueryAlterAll

pub struct QueryAlterAll<'a, K: Eq + Hash, V, F: FnMut(&K, V) -> V> {
    inner: Query<'a, K, V>,
    f: F,
}

impl<'a, K: Eq + Hash, V, F: FnMut(&K, V) -> V> QueryAlterAll<'a, K, V, F> {
    pub fn new(inner: Query<'a, K, V>, f: F) -> Self {
        Self { inner, f }
    }

    pub fn sync(self) -> QueryAlterAllSync<'a, K, V, F> {
        QueryAlterAllSync::new(self)
    }
}

// --

// -- QueryAlterAllSync

pub struct QueryAlterAllSync<'a, K: Eq + Hash, V, F: FnMut(&K, V) -> V> {
    inner: QueryAlterAll<'a, K, V, F>,
}

impl<'a, K: Eq + Hash, V, F: FnMut(&K, V) -> V> QueryAlterAllSync<'a, K, V, F> {
    pub fn new(inner: QueryAlterAll<'a, K, V, F>) -> Self {
        Self { inner }
    }
}

impl<'a, K: Eq + Hash, V, F: FnMut(&K, V) -> V> ExecutableQuery for QueryAlterAllSync<'a, K, V, F> {
    type Output = ();

    fn exec(mut self) -> Self::Output {
        self.inner.inner.map.query().iter_mut().exec().for_each(|mut r| util::map_in_place_2(r.pair_mut(), &mut self.inner.f));
    }
}

// --

// -- QueryClear

pub struct QueryClear<'a, K: Eq + Hash, V> {
    inner: Query<'a, K, V>,
}

impl<'a, K: Eq + Hash, V> QueryClear<'a, K, V> {
    pub fn new(inner: Query<'a, K, V>) -> Self {
        Self { inner }
    }

    pub fn sync(self) -> QueryClearSync<'a, K, V> {
        QueryClearSync::new(self)
    }
}

// --

// -- QueryIter

pub struct QueryIter<'a, K: Eq + Hash, V> {
    inner: Query<'a, K, V>,
}

impl<'a, K: Eq + Hash, V> QueryIter<'a, K, V> {
    pub fn new(inner: Query<'a, K, V>) -> Self {
        Self { inner }
    }
}

impl<'a, K: Eq + Hash, V> ExecutableQuery for QueryIter<'a, K, V> {
    type Output = Iter<'a, K, V>;

    fn exec(self) -> Self::Output {
        Iter::new(self.inner.map)
    }
}

// --

// -- QueryIterMut

pub struct QueryIterMut<'a, K: Eq + Hash, V> {
    inner: Query<'a, K, V>,
}

impl<'a, K: Eq + Hash, V> QueryIterMut<'a, K, V> {
    pub fn new(inner: Query<'a, K, V>) -> Self {
        Self { inner }
    }
}

impl<'a, K: Eq + Hash, V> ExecutableQuery for QueryIterMut<'a, K, V> {
    type Output = IterMut<'a, K, V>;

    fn exec(self) -> Self::Output {
        IterMut::new(self.inner.map)
    }
}

// --

// -- QueryClearSync

pub struct QueryClearSync<'a, K: Eq + Hash, V> {
    inner: QueryClear<'a, K, V>,
}

impl<'a, K: Eq + Hash, V> QueryClearSync<'a, K, V> {
    pub fn new(inner: QueryClear<'a, K, V>) -> Self {
        Self { inner }
    }
}

impl<'a, K: Eq + Hash, V> ExecutableQuery for QueryClearSync<'a, K, V> {
    type Output = ();

    fn exec(self) -> Self::Output {
        let shards = self.inner.inner.map.shards();
        for shard in &**shards {
            shard.write().clear();
        }
    }
}

// --

// -- QueryIsEmpty

pub struct QueryIsEmpty<'a, K: Eq + Hash, V> {
    inner: Query<'a, K, V>,
}

impl<'a, K: Eq + Hash, V> QueryIsEmpty<'a, K, V> {
    pub fn new(inner: Query<'a, K, V>) -> Self {
        Self { inner }
    }

    pub fn sync(self) -> QueryIsEmptySync<'a, K, V> {
        QueryIsEmptySync::new(self)
    }
}

// --

// -- QueryIsEmptySync

pub struct QueryIsEmptySync<'a, K: Eq + Hash, V> {
    inner: QueryIsEmpty<'a, K, V>,
}

impl<'a, K: Eq + Hash, V> QueryIsEmptySync<'a, K, V> {
    pub fn new(inner: QueryIsEmpty<'a, K, V>) -> Self {
        Self { inner }
    }
}

impl<'a, K: Eq + Hash, V> ExecutableQuery for QueryIsEmptySync<'a, K, V> {
    type Output = bool;

    fn exec(self) -> Self::Output {
        self.inner.inner.map.query().len().sync().exec() == 0
    }
}

// --

// -- QueryLength

pub struct QueryLength<'a, K: Eq + Hash, V> {
    inner: Query<'a, K, V>,
}

impl<'a, K: Eq + Hash, V> QueryLength<'a, K, V> {
    pub fn new(inner: Query<'a, K, V>) -> Self {
        Self { inner }
    }

    pub fn sync(self) -> QueryLengthSync<'a, K, V> {
        QueryLengthSync::new(self)
    }
}

// --

// -- QueryLengthSync

pub struct QueryLengthSync<'a, K: Eq + Hash, V> {
    inner: QueryLength<'a, K, V>,
}

impl<'a, K: Eq + Hash, V> QueryLengthSync<'a, K, V> {
    pub fn new(inner: QueryLength<'a, K, V>) -> Self {
        Self { inner }
    }
}

impl<'a, K: Eq + Hash, V> ExecutableQuery for QueryLengthSync<'a, K, V> {
    type Output = usize;

    fn exec(self) -> Self::Output {
        let shards = self.inner.inner.map.shards();
        let mut total = 0;
        for shard in &**shards {
            total += shard.read().len();
        }
        total
    }
}

// --

// -- QueryRemove

pub struct QueryRemove<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> {
    inner: Query<'a, K, V>,
    key: &'k Q,
}

impl<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> QueryRemove<'a, 'k, Q, K, V> {
    pub fn new(inner: Query<'a, K, V>, key: &'k Q) -> Self {
        Self { inner, key }
    }

    pub fn sync(self) -> QueryRemoveSync<'a, 'k, Q, K, V> {
        QueryRemoveSync::new(self)
    }
}

// --

// -- QueryRemoveSync

pub struct QueryRemoveSync<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> {
    inner: QueryRemove<'a, 'k, Q, K, V>,
}

impl<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> QueryRemoveSync<'a, 'k, Q, K, V> {
    pub fn new(inner: QueryRemove<'a, 'k, Q, K, V>) -> Self {
        Self { inner }
    }
}

impl<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> ExecutableQuery
    for QueryRemoveSync<'a, 'k, Q, K, V>
{
    type Output = Option<(K, V)>;

    fn exec(self) -> Self::Output {
        let shard_id = self.inner.inner.map.determine_map(&self.inner.key);
        let shards = self.inner.inner.map.shards();
        let mut shard = shards[shard_id].write();
        shard.remove_entry(&self.inner.key)
    }
}

// --

// -- QueryInsert

pub struct QueryInsert<'a, K: Eq + Hash, V> {
    inner: Query<'a, K, V>,
    key: K,
    value: V,
}

impl<'a, K: Eq + Hash, V> QueryInsert<'a, K, V> {
    pub fn new(inner: Query<'a, K, V>, key: K, value: V) -> Self {
        Self { inner, key, value }
    }

    pub fn sync(self) -> QueryInsertSync<'a, K, V> {
        QueryInsertSync::new(self)
    }
}

// --

// -- QueryInsertSync

pub struct QueryInsertSync<'a, K: Eq + Hash, V> {
    inner: QueryInsert<'a, K, V>,
}

impl<'a, K: Eq + Hash, V> QueryInsertSync<'a, K, V> {
    pub fn new(inner: QueryInsert<'a, K, V>) -> Self {
        Self { inner }
    }
}

impl<'a, K: Eq + Hash, V> ExecutableQuery for QueryInsertSync<'a, K, V> {
    type Output = Option<V>;

    fn exec(self) -> Self::Output {
        let shard_id = self.inner.inner.map.determine_map(&self.inner.key);
        let shards = self.inner.inner.map.shards();
        let mut shard = shards[shard_id].write();
        shard.insert(self.inner.key, self.inner.value)
    }
}

// --

// -- QueryGet

pub struct QueryGet<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> {
    inner: Query<'a, K, V>,
    key: &'k Q,
}

impl<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> QueryGet<'a, 'k, Q, K, V> {
    pub fn new(inner: Query<'a, K, V>, key: &'k Q) -> Self {
        Self { inner, key }
    }

    pub fn sync(self) -> QueryGetSync<'a, 'k, Q, K, V> {
        QueryGetSync::new(self)
    }

    pub fn mutable(self) -> QueryGetMut<'a, 'k, Q, K, V> {
        QueryGetMut::new(self)
    }
}

// --

// -- QueryGetMut

pub struct QueryGetMut<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> {
    inner: QueryGet<'a, 'k, Q, K, V>,
}

impl<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> QueryGetMut<'a, 'k, Q, K, V> {
    pub fn new(inner: QueryGet<'a, 'k, Q, K, V>) -> Self {
        Self { inner }
    }

    pub fn sync(self) -> QueryGetMutSync<'a, 'k, Q, K, V> {
        QueryGetMutSync::new(self)
    }
}

// --

// -- QueryGetSync

pub struct QueryGetSync<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> {
    inner: QueryGet<'a, 'k, Q, K, V>,
}

impl<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> QueryGetSync<'a, 'k, Q, K, V> {
    pub fn new(inner: QueryGet<'a, 'k, Q, K, V>) -> Self {
        Self { inner }
    }
}

impl<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> ExecutableQuery
    for QueryGetSync<'a, 'k, Q, K, V>
{
    type Output = Option<DashMapRef<'a, K, V>>;

    fn exec(self) -> Self::Output {
        let shard_id = self.inner.inner.map.determine_map(&self.inner.key);
        let shards = self.inner.inner.map.shards();
        let shard = shards[shard_id].read();
        if let Some((k, v)) = shard.get_key_value(&self.inner.key) {
            unsafe {
                let k = util::change_lifetime_const(k);
                let v = util::change_lifetime_const(v);
                return Some(DashMapRef::new(shard, k, v));
            }
        }

        None
    }
}

// --

// -- QueryGetMutSync

pub struct QueryGetMutSync<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> {
    inner: QueryGetMut<'a, 'k, Q, K, V>,
}

impl<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> QueryGetMutSync<'a, 'k, Q, K, V> {
    pub fn new(inner: QueryGetMut<'a, 'k, Q, K, V>) -> Self {
        Self { inner }
    }
}

impl<'a, 'k, Q: Eq + Hash, K: Eq + Hash + Borrow<Q>, V> ExecutableQuery
    for QueryGetMutSync<'a, 'k, Q, K, V>
{
    type Output = Option<DashMapRefMut<'a, K, V>>;

    fn exec(self) -> Self::Output {
        let shard_id = self
            .inner
            .inner
            .inner
            .map
            .determine_map(&self.inner.inner.key);
        let shards = self.inner.inner.inner.map.shards();
        let shard = shards[shard_id].write();

        if let Some((k, v)) = shard.get_key_value(&self.inner.inner.key) {
            unsafe {
                let k = util::change_lifetime_const(k);
                let v = util::change_lifetime_mut(util::to_mut(v));
                return Some(DashMapRefMut::new(shard, k, v));
            }
        }

        None
    }
}

// --
