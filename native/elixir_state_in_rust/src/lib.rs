use rustler::{Env, Error, ListIterator, NifStruct, ResourceArc, Term, Atom, NifResult};
use std::collections::HashMap;
use std::sync::RwLock;


#[derive(Debug)]
enum StringOrAtom {
    Atom(String),
    String(String)
}

impl StringOrAtom {
    fn to_string(self) -> String {
        match self {
            StringOrAtom::Atom(atom) => atom,
            StringOrAtom::String(string) => string
        }
    }
}

impl<'a> rustler::Decoder<'a> for StringOrAtom {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        if let Ok(string) = term.decode::<String>() {
            Ok(StringOrAtom::String(string))
        } else if let Ok(atom) = term.atom_to_string() {
            Ok(StringOrAtom::Atom(atom))
        } else {
            Err(Error::BadArg)
        }
    }
}

impl rustler::Encoder for StringOrAtom {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            StringOrAtom::Atom(atom) => Atom::from_str(env, atom).unwrap().encode(env),
            StringOrAtom::String(string) => string.encode(env)
        }
    }
}

#[derive(NifStruct)]
#[module = "ElixirStateInRust"]
pub struct Map {
    reference: ResourceArc<TestResource>,
}

pub struct TestResource {
    data: RwLock<HashMap<String, usize>>,
}

#[rustler::nif]
fn new() -> Map {
    Map {
        reference: ResourceArc::new(TestResource {
            data: RwLock::new(HashMap::new()),
        }),
    }
}

#[rustler::nif(name = "new")]
fn new_with_list<'a>(list: ListIterator<'a>) -> Result<Map, Error> {
    let mut hashmap = HashMap::new();
    for item in list {
        if let Ok((key, value)) = item.decode::<(StringOrAtom, usize)>() {
            hashmap.insert(key.to_string(), value);
        } else {
            return Err(Error::BadArg);
        }
    }

    Ok(Map {
        reference: ResourceArc::new(TestResource {
            data: RwLock::new(hashmap),
        }),
    })
}

#[rustler::nif]
fn put(map: Map, key: StringOrAtom, value: usize) -> Map {
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
fn pop(map: Map) -> Option<(String, usize)> {
    let mut data = map.reference.data.write().unwrap();
    let entry = data.keys().next().cloned()?;
    data.remove_entry(&entry)
}

#[rustler::nif]
fn get(map: Map, key: &str) -> Option<usize> {
    let data = map.reference.data.read().unwrap();
    data.get(key).copied()
}

rustler::init!(
    "Elixir.ElixirStateInRust",
    [new, new_with_list, put, get, is_empty, pop, len, contains],
    load = load
);

fn load(env: rustler::Env, _: rustler::Term) -> bool {
    on_load(env);
    true
}

pub fn on_load(env: Env) -> bool {
    rustler::resource!(TestResource, env);
    true
}
