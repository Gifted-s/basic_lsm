use crate::bst::BST;
use std::{error::Error, fmt::Debug};
const MEMTABLE_SIZE: usize = 1024;
use rbtree::RBTree;

/*
 * In-Memory table that is just modified
 * NOTE: only one MemTable can be active at a time
 * The MemTable is searched first before proceeding to the SSTables
 */

#[derive(Debug)]
struct MemTable<K: Ord + ExactSizeIterator, V: ExactSizeIterator> {
    entries: RBTree<K, (Option<V>, u128, bool)>,
    size: usize,
}

struct MemTableEntry<K: Debug, V: Debug> {
    key: K,
    value: Option<V>,
    timestamp: u128,
    deleted: bool,
}

impl<K: Ord + ExactSizeIterator + Clone + Debug, V: ExactSizeIterator + Clone + Debug>
    MemTable<K, V>
{
    // Create a new empty MemTable
    pub fn new(key: K, val: V) -> MemTable<K, V> {
        MemTable {
            entries: RBTree::new(),
            size: 0,
        }
    }
    fn insert(&mut self, k: &K, v: &V, timestamp: u128) {
        match self
            .entries
            .insert(k.to_owned(), (Some(v.to_owned()), timestamp, false))
        {
            Ok(true) => {
                self.size += k.len() + v.len() + 16 + 1; // Increase the size of the MemTable by the Key size, Value size, Timestamp size (16 bytes), Tombstone size (1 byte).
            }
            Err(err_msg) => {
                println!("Error inserting to memtable, {}", err_msg);
            }
            Ok(false) => {
                println!("Encounterd Error while inserting this data but it's unknown");
            }
        }
    }

    fn find(&mut self, k: K) -> Option<MemTableEntry<K, V>> {
        if let Some(e) = self.records.find(k) {
            return Some(MemTableEntry {
                key: e.0,
                value: e.1,
            });
        }
        None
    }
 
    fn delete(&mut self, k: &K, timestamp: &u128) {
        match self.records.delete(k.to_owned(), timestamp) {
            Ok(true) => {
                self.size += k.len() + 16 + 1; // Increase the size of the MemTable by the Key size, Value size, Timestamp size (16 bytes), Tombstone size (1 byte).
            }
            Err(err_msg) => {
                println!("Error inserting to memtable, {}", err_msg);
            }
            Ok(false) => {
                println!("Encounterd Error while inserting this data but it's unknown");
            }
        }
    }
    /// Gets the number of records in the MemTable.
    pub fn len(&self) -> usize {
        self.size
    }
    /// Gets all of the records from the MemTable.
    pub fn entries(&self) -> &BST<K, V> {
        &self.records
    }
 
    // Returns all the elements within a particular range
    pub fn range_scan(&self, start:V, end: V) -> Option<Vec<MemTableEntry>>{
        if self.size==0{
            return None;
        }
        // get first element from start, iterate until key is greater than end;
        loop {
          let element = &self.records.find(start);
          
        }
    }
}
