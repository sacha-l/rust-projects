use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

#[derive(Debug)]
pub enum Value<'buf> {
    // single value
    Single(&'buf str),
    // multiple chars
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// use the From trait to convert string slice into hashmap
// TryFrom does not support lifetimes so we use From
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        // split a string on the & pattern
        // gives an iterator over the sub-strings
        for sub_str in s.split('&'){
            let mut key = sub_str;
            let mut val = "";
            // look for the equal sign
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            // Hashmap provides interface to swap data easily
            data.entry(key)
            // add ability to modify an element in the map
            // accepts a closure
            .and_modify(|existing: &mut Value | match existing {
                Value::Single(pre_val) => {
                    // the `*`: follow the pointer and write the new value to replace the previous value 
                    *existing = Value::Multiple(vec![pre_val, val]);
                }
                Value::Multiple(vec) => {}
            })
            // check if key exists
            .or_insert(Value::Single(val)); 

        }

        QueryString { data }
    }
}