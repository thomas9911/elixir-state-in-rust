use im::HashMap;
use rustler::{Env, Error, ListIterator, NifStruct, ResourceArc};
use shared::{SimpleValue, StringOrAtom};
use std::sync::RwLock;

type Value = SimpleValue;

#[derive(NifStruct)]
#[module = "ElixirStateInRust.ImMap"]
pub struct Map {
    reference: ResourceArc<HashMapResource>,
}
impl Map {
    pub fn new(map: HashMap<String, Value>) -> Self {
        Map {
            reference: ResourceArc::new(HashMapResource {
                data: RwLock::new(map),
            }),
        }
    }
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
    Map::new(new_map.clone())
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

    Ok(Map::new(hashmap))
}

#[rustler::nif]
fn put(map: Map, key: StringOrAtom, value: Value) -> Map {
    let data = map
        .reference
        .data
        .read()
        .unwrap()
        .update(key.to_string(), value);

    Map::new(data)
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

#[rustler::nif]
fn pop(map: Map) -> (Option<(String, Value)>, Map) {
    let might_entry = {
        let data = map.reference.data.read().unwrap();
        data.keys().next().cloned()
    };
    if let Some(entry) = might_entry {
        let mut data = map.reference.data.read().unwrap().clone();
        let key_value = {
            data.remove_with_key(&entry)
        };
        (key_value, Map::new(data))
    } else {
        (None, map)
    }
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
