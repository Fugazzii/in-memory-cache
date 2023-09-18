/**
 * Database implementation is just using hashmap for storing values
 * Current version only supports get and set methods
 * TODO: Remove method
 */

use bytes::Bytes;
use std::collections::HashMap;

pub struct Db {
    entries: HashMap<String, Bytes>
}

impl Db {
    pub fn new() -> Self {
        Db { entries: HashMap::new() }
    }

    pub fn write(&mut self, arr: &[String]) -> Result<&str, &'static str> {
        let key = &arr[1];
        let value = &arr[2];

        let val = value.clone();
        let res: &Option<Bytes> = &self.entries.insert(String::from(key), Bytes::from(val));

        match res {
            Some(_res) => Ok("r Ok"),
            None => Ok("Ok"),
        }
    }

    pub fn read(&mut self, arr: &[String]) -> Result<&Bytes, &'static str> {
        let key = &arr[1];
        let query_result = self.entries.get(key);
    
        if let Some(value) = query_result {
            return Ok(value);
        } else {
            return Err("no such key found");
        }
    }
}