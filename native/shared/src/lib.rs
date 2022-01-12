use rustler::{Atom, Env, Error, NifResult, Term};

#[derive(Debug, Clone)]
pub enum SimpleValue {
    String(String),
    Integer(i64),
    Bool(bool),
    None,
}

impl<'a> rustler::Decoder<'a> for SimpleValue {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        if let Ok(integer) = term.decode::<i64>() {
            Ok(SimpleValue::Integer(integer))
        } else if let Ok(boolean) = term.decode::<bool>() {
            Ok(SimpleValue::Bool(boolean))
        } else if let Ok(_) = term.decode::<()>() {
            Ok(SimpleValue::None)
        } else if let Ok(string) = term.decode::<String>() {
            Ok(SimpleValue::String(string))
        } else {
            Err(Error::BadArg)
        }
    }
}

impl rustler::Encoder for SimpleValue {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            SimpleValue::String(string) => string.encode(env),
            SimpleValue::Integer(integer) => integer.encode(env),
            SimpleValue::Bool(boolean) => boolean.encode(env),
            SimpleValue::None => ().encode(env),
        }
    }
}

#[derive(Debug)]
pub enum StringOrAtom {
    Atom(String),
    String(String),
}

impl StringOrAtom {
    pub fn to_string(self) -> String {
        match self {
            StringOrAtom::Atom(atom) => atom,
            StringOrAtom::String(string) => string,
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
            StringOrAtom::String(string) => string.encode(env),
        }
    }
}
