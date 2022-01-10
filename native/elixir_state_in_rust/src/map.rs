use crate::{SimpleValue, StringOrAtom};
use rustler::{Env, Error, ListIterator, NifStruct, ResourceArc};
use std::collections::HashMap;
use std::sync::RwLock;

type Value = SimpleValue;

#[derive(NifStruct)]
#[module = "ElixirStateInRust.Map"]
pub struct Map {
    reference: ResourceArc<HashMapResource>,
}

pub struct HashMapResource {
    data: RwLock<HashMap<String, Value>>,
}

#[rustler::nif]
fn new() -> Map {
    Map {
        reference: ResourceArc::new(HashMapResource {
            data: RwLock::new(HashMap::new()),
        }),
    }
}

#[rustler::nif]
fn clone(map: Map) -> Map {
    let new_map = map.reference.data.read().unwrap();

    Map {
        reference: ResourceArc::new(HashMapResource {
            data: RwLock::new(new_map.clone()),
        }),
    }
}

#[rustler::nif(name = "new")]
fn new_with_list<'a>(list: ListIterator<'a>) -> Result<Map, Error> {
    let mut hashmap = HashMap::new();
    for item in list {
        if let Ok((key, value)) = item.decode::<(StringOrAtom, Value)>() {
            hashmap.insert(key.to_string(), value);
        } else {
            return Err(Error::BadArg);
        }
    }

    Ok(Map {
        reference: ResourceArc::new(HashMapResource {
            data: RwLock::new(hashmap),
        }),
    })
}

#[rustler::nif]
fn put(map: Map, key: StringOrAtom, value: Value) -> Map {
    {
        let mut data = map.reference.data.write().unwrap();
        data.insert(key.to_string(), value);
    }
    map
}

#[rustler::nif(name = "_empty")]
fn is_empty(map: Map) -> bool {
    map.reference.data.read().unwrap().is_empty()
}

#[rustler::nif]
fn len(map: Map) -> usize {
    map.reference.data.read().unwrap().len()
}

#[rustler::nif]
fn contains(map: Map, key: String) -> bool {
    map.reference.data.read().unwrap().contains_key(&key)
}

#[rustler::nif(name = "pop")]
fn pop(map: Map) -> Option<(String, Value)> {
    let mut data = map.reference.data.write().unwrap();
    let entry = data.keys().next().cloned()?;
    data.remove_entry(&entry)
}

#[rustler::nif]
fn get(map: Map, key: StringOrAtom) -> Option<Value> {
    let data = map.reference.data.read().unwrap();
    data.get(&key.to_string()).cloned()
}

pub fn load(env: rustler::Env, _: rustler::Term) -> bool {
    on_load(env);
    true
}

pub fn on_load(env: Env) -> bool {
    rustler::resource!(HashMapResource, env);
    true
}
