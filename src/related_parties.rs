use std::collections::HashMap;

use crate::indexed_vec::Idx;
use crate::{DbId, Error};

#[derive(Default)]
pub struct RelatedPartiesCache {
    descr_len: usize,
    // The keys are all descr_len, and if DbId is none then more than one matches
    parties: HashMap<Vec<u8>, DbId>,
}

impl RelatedPartiesCache {
    pub fn descr_len(&self) -> usize {
        self.descr_len
    }
    pub fn find_item(&self, descr: &str) -> Option<DbId> {
        let n = self.descr_len;
        let descr = descr.as_bytes();
        if descr.len() < n {
            None
        } else {
            self.parties.get(&descr[0..n]).copied()
        }
    }
    fn inserty_do(&mut self, db_id: DbId, descr: &str) {
        if descr.len() < self.descr_len {
            panic!(
                "item {db_id} has descriptor {descr} that is too short < {}",
                self.descr_len
            );
        }
        let key = &descr.as_bytes()[0..self.descr_len];
        if let Some(collision) = self.parties.get_mut(key) {
            *collision = DbId::none();
        } else {
            self.parties.insert(key.to_vec(), db_id);
        }
    }
    pub fn create<F, I>(descr_len: usize, iter: I, f: F) -> Result<Self, Error>
    where
        I: Iterator<Item = DbId>,
        F: Fn(DbId, &mut dyn for<'a> FnMut(DbId, &'a str)),
    {
        let parties = HashMap::default();
        let mut s = Self { descr_len, parties };
        for db_id in iter {
            f(db_id, &mut |a, b| s.inserty_do(a, b))
        }
        Ok(s)
    }
}

#[derive(Default)]
pub struct RelatedParties {
    min_len: usize,
    max_len: usize,
    step: usize,
    // caches in order of descr length - shortest first
    caches: Vec<RelatedPartiesCache>,
}

impl RelatedParties {
    pub fn new(min_len: usize, max_len: usize, step: usize) -> Self {
        let caches = vec![];
        Self {
            min_len,
            max_len,
            step,
            caches,
        }
    }
    pub fn is_none(&self) -> bool {
        self.max_len == 0
    }
    pub fn find_item_with_collisions(&self, descr: &str) -> Option<DbId> {
        if self.caches.is_empty() {
            return Some(DbId::none());
        }
        for c in &self.caches {
            if let Some(db_id) = c.find_item(descr) {
                if !db_id.is_none() {
                    return Some(db_id);
                }
            } else {
                return None;
            }
        }
        Some(DbId::none())
    }
    pub fn add_new_cache<F, I>(&mut self, iter: I, f: F) -> Result<(), Error>
    where
        I: Iterator<Item = DbId>,
        F: Fn(DbId, &mut dyn for<'a> FnMut(DbId, &'a str)),
    {
        let mut descr_len = self.min_len;
        if let Some(c) = self.caches.last() {
            descr_len = c.descr_len();
            if descr_len >= self.max_len {
                eprintln!(
                    "add_new_cache: descr_len >= self.max_len {} {}",
                    descr_len, self.max_len
                );
                return Err("Related parties cache size exceeded".to_string().into());
            }
            eprintln!(
                "add_new_cache: will add new cache level beyond last {} {}",
                descr_len, self.step
            );
            descr_len += self.step;
        }

        eprintln!(
            "add_new_cache: new cache level {} {}",
            descr_len, self.max_len
        );
        let cache = RelatedPartiesCache::create(descr_len, iter, f)?;
        self.caches.push(cache);
        eprintln!(
            "add_new_cache: added cache level {}",
            self.caches.last().unwrap().descr_len()
        );
        Ok(())
    }
}
