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
}