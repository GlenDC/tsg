use std::collections::{HashMap, VecDeque};

use serde_json;
use serde_yaml;

use super::path::{PathIter, PathComponent};

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    String(String),
    Boolean(bool),
    Number(f64),
    Sequence(Vec<Value>),
    Mapping(HashMap<String, Value>),
}

impl Value {
    pub fn glob_first<'a, 'b, T> (&'a self, t: T) -> Option<&'a Value>
        where T: Into<PathIter<'b>>
    {
        let v = self.glob(t);
        v.into_iter().next()
    }

    // TOOD: implement as an iterator, so we do not go deeper than required

    pub fn glob<'a, 'b, T> (&'a self, t: T) -> Vec<&'a Value>
        where T: Into<PathIter<'b>>
    {
        let mut it = t.into();
        let mut v = Vec::new();
        self.glob_inner(&mut it, &mut v);
        v
    }

    fn glob_inner<'a, 'b, T> (&'a self, it: &mut T, output: &mut Vec<&'a Value>) -> Option<()>
        where T: Iterator<Item = PathComponent<'b>> + ?Sized
    {
        let mut value = self;
        loop {
            match it.next() {
                None => {
                    output.push(value);
                    return Some(());
                },
                Some(component) => match component {
                    PathComponent::Empty => (), // skip and continue
                    PathComponent::Any => match value {
                        Value::Mapping(mapping) => {
                            for value in mapping.values() {
                                value.glob_inner(it, output);
                            }
                        }
                        Value::Sequence(sequence) => {
                            for value in sequence {
                                value.glob_inner(it, output);
                            }
                        }
                        // skip any for these values, such that its value will be returned in case iterator is exhausted
                        Value::Null | Value::String(_) | Value::Boolean(_) | Value::Number(_) => continue,
                    },
                    PathComponent::AnyRecursive => return self.glob_inner_recursive(it, output),
                    PathComponent::Name(name) => match value {
                        Value::Mapping(mapping) => match mapping.get(name) {
                            Some(found_value) => value = found_value,
                            None => return None,
                        },
                        Value::Sequence(sequence) => {
                            let index: usize = match name.parse() {
                                Ok(i) => i,
                                Err(_) => return None,
                            };
                            if index >= sequence.len() {
                                return None;
                            }
                            value = &sequence[index];
                        }
                        _ => return None,
                    },
                },
            }
        }
    }

    // TODO: figure out on paper how we'll do this recursive business,
    // code should become obvious after that is figured out

    fn glob_inner_recursive<'a, 'b, T> (&'a self, it: &mut T, output: &mut Vec<&'a Value>) -> Option<()>
        where T: Iterator<Item = PathComponent<'b>> + ?Sized
    {
        let path: Vec<PathComponent<'b>> = it.filter(|pc| match pc {
            PathComponent::Empty => false,
            _ => true,
        }).skip_while(|pc| match pc {
            PathComponent::Any | PathComponent::AnyRecursive => true,
            _ => false,
        }).collect();

        match self {
            Value::Null | Value::String(_) | Value::Boolean(_) | Value::Number(_) => if path.is_empty() {
                output.push(self);
                Some(())
            } else {
                None
            },
            Value::Sequence(seq) => {
                let n = output.len();
                for (index, value) in seq.iter().enumerate() {
                    match value {
                        Value::Null | Value::String(_) | Value::Boolean(_) | Value::Number(_) => if path.is_empty() {
                            output.push(self);
                        } else if let PathComponent::Name(name) = path[0] {
                            if let Ok(name_as_index) = name.parse::<usize>() {
                                if name_as_index == index {
                                    let mut path = path[1..].to_vec().into_iter();
                                    self.glob_inner(&mut path, output);
                                }
                            }
                        },
                        Value::Sequence(_seq) => {
                            // TODO
                        }
                        Value::Mapping(_map) => (), // TODO
                    }
                }
                if n < output.len() {
                    Some(())
                } else {
                    None
                }
            },
            Value::Mapping(map) => {
                let n = output.len();
                for (_key, _value) in map {
                }
                if n < output.len() {
                    Some(())
                } else {
                    None
                }
            }
        }
    }


    // TODO: implement set_path(&mut self, ...) and set_path_iter(&mut self, ...)

    // TODO: support glob?!?

    pub fn as_none(&self) -> Option<()> {
        match self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn to_string(self) -> Option<String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(x) => Some(*x),
            _ => None,
        }
    }

    pub fn as_sequence(&self) -> Option<&[Value]> {
        match self {
            Value::Sequence(v) => Some(&v[..]),
            _ => None,
        }
    }

    pub fn as_mapping(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Mapping(m) => Some(&m),
            _ => None,
        }
    }

    pub fn to_mapping(self) -> Option<HashMap<String, Value>> {
        match self {
            Value::Mapping(m) => Some(m),
            _ => None,
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Value {
        Value::Null
    }
}

impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Value {
        Value::String(String::from(s))
    }
}

impl From<bool> for Value {
    fn from(x: bool) -> Value {
        Value::Boolean(x)
    }
}

impl From<f32> for Value {
    fn from(x: f32) -> Value {
        Value::Number(x as f64)
    }
}

impl From<f64> for Value {
    fn from(x: f64) -> Value {
        Value::Number(x)
    }
}

impl From<i8> for Value {
    fn from(x: i8) -> Value {
        Value::Number(x as f64)
    }
}

impl From<i16> for Value {
    fn from(x: i16) -> Value {
        Value::Number(x as f64)
    }
}

impl From<i32> for Value {
    fn from(x: i32) -> Value {
        Value::Number(x as f64)
    }
}

impl From<i64> for Value {
    fn from(x: i64) -> Value {
        Value::Number(x as f64)
    }
}

impl From<i128> for Value {
    fn from(x: i128) -> Value {
        Value::Number(x as f64)
    }
}

impl From<usize> for Value {
    fn from(x: usize) -> Value {
        Value::Number(x as f64)
    }
}

impl From<u8> for Value {
    fn from(x: u8) -> Value {
        Value::Number(x as f64)
    }
}

impl From<u16> for Value {
    fn from(x: u16) -> Value {
        Value::Number(x as f64)
    }
}

impl From<u32> for Value {
    fn from(x: u32) -> Value {
        Value::Number(x as f64)
    }
}

impl From<u64> for Value {
    fn from(x: u64) -> Value {
        Value::Number(x as f64)
    }
}

impl From<u128> for Value {
    fn from(x: u128) -> Value {
        Value::Number(x as f64)
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(s: Vec<T>) -> Value {
        let vec = s.into_iter().map(|v| v.into()).collect();
        Value::Sequence(vec)
    }
}

impl<T> From<&[T]> for Value
where
    T: Into<Value> + Clone,
{
    fn from(s: &[T]) -> Value {
        let vec = s.iter().cloned().map(|v| v.into()).collect();
        Value::Sequence(vec)
    }
}

impl<T> From<HashMap<String, T>> for Value
where
    T: Into<Value>,
{
    fn from(m: HashMap<String, T>) -> Value {
        let m: HashMap<String, Value> = m.into_iter().map(|(k, v)| (k, v.into())).collect();
        Value::Mapping(m)
    }
}

impl From<serde_yaml::Value> for Value {
    fn from(v: serde_yaml::Value) -> Value {
        match v {
            serde_yaml::Value::Null => Value::Null,
            serde_yaml::Value::String(s) => Value::String(s),
            serde_yaml::Value::Bool(b) => Value::Boolean(b),
            serde_yaml::Value::Number(n) => Value::Number(n.as_f64().unwrap_or(std::f64::NAN)),
            serde_yaml::Value::Sequence(s) => s.into(),
            serde_yaml::Value::Mapping(m) => {
                let m: HashMap<String, Value> = m.into_iter()
                    .filter(|(k, _)| match k {
                        // filter out complex keys as these can anyhow not be indexed nicely by TSG user
                        serde_yaml::Value::Sequence(_) | serde_yaml::Value::Mapping(_) => false,
                        _ => true,
                    })
                    .map(|(k, v)| (match k {
                        serde_yaml::Value::String(s) => s,
                        serde_yaml::Value::Null => "".to_owned(),
                        serde_yaml::Value::Number(n) => n.to_string(),
                        serde_yaml::Value::Bool(b) => b.to_string(),
                        _ => panic!("report a bug if this line is ever reached, should be impossible due to filter"),
                    }, v.into())).collect();
                Value::Mapping(m)
            }
        }
    }
}

impl From<serde_json::Value> for Value {
    fn from(v: serde_json::Value) -> Value {
        match v {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Bool(b) => Value::Boolean(b),
            serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap_or(std::f64::NAN)),
            serde_json::Value::Array(arr) => arr.into(),
            serde_json::Value::Object(o) => {
                let m: HashMap<String, Value> = o.into_iter().map(|(k, v)| (k, v.into())).collect();
                Value::Mapping(m)
            }
        }
    }
}

pub struct ValueIter<'a, 'b> {
    root: Option<&'a Value>,
    path: Vec<PathComponent<'b>>,
    path_index: usize,
    children: Vec<ValueIter<'a, 'b>>,
    recursive: bool,
}

impl <'a, 'b> ValueIter<'a, 'b> {
    pub fn new<T>(value: &'a Value, path_iter: T) -> ValueIter<'a, 'b>
        where T: Into<PathIter<'b>>
    {
        ValueIter{
            root: Some(value),
            path: normalize_path_iter_to_vec(path_iter),
            path_index: 0,
            children: Vec::new(),
            recursive: false,
        }
    }
}

impl<'a, 'b> Iterator for ValueIter<'a, 'b> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<&'a Value> {
        let mut value_stack = VecDeque::with_capacity(2);
        value_stack.push_back(self);
        loop {
            if value_stack.is_empty() {
                break
            }

            if value_stack[0].path.is_empty() {
                return match value_stack.pop_front() {
                    Some(v) => v.root,
                    None => None,
                };
            }

            if value_stack[0].path_index >= value_stack[0].path.len() {
                value_stack.pop_front();
                continue;
            }
            

            // TODO:
            // - implement match
            // - somehow be able to move path...
            match value_stack[0].path[value_stack[0].path_index] {
                PathComponent::Empty => (),
                _ => (),
            }
        }
        None
    }
}

fn normalize_path_iter_to_vec<'a, T>(path_iter: T) -> Vec<PathComponent<'a>>
    where T: Into<PathIter<'a>>
{
    let (mut out, p) = path_iter.into().fold((Vec::new(), PathComponent::Empty), |(mut v, p), c| match c {
        PathComponent::Empty => (v, p),
        PathComponent::Name(_) => {
            match p {
                PathComponent::Any | PathComponent::AnyRecursive => v.push(p),
                _ => (),
            }
            v.push(c);
            (v, c)
        },
        PathComponent::Any => {
            if matches!(c, PathComponent::AnyRecursive) {
                (v, p)
            } else {
                (v, c)
            }
        },
        PathComponent::AnyRecursive => (v, c),
    });
    match p {
        PathComponent::Any | PathComponent::AnyRecursive => out.push(p),
        _ => (),
    }
    out
}
