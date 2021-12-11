use std::collections::{HashMap, VecDeque};

use serde_json;
use serde_yaml;

use super::path::{PathComponent, PathIter};

pub fn first_value<'a, 'b, I>(path_it: I, values: &'a [Value]) -> Option<&'a Value>
    where I: Into<PathIter<'b>>
{
    let path: Vec<PathComponent<'b>> = path_it.into().collect();
    for value in values {
        if let Some(value) = value.value(PathIter::wrap(path.clone().into_iter())) {
            return Some(value);
        }
    }
    None
}

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
    pub fn value<'a, 'b, T>(&'a self, t: T) -> Option<&'a Value>
    where
        T: Into<PathIter<'b>>,
    {
        self.value_iter(t).next()
    }

    pub fn value_iter<'a, 'b, T>(&'a self, t: T) -> ValueIter<'a, 'b>
    where
        T: Into<PathIter<'b>>,
    {
        ValueIter::new(self, t)
    }

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
    stack: VecDeque<ValueIterInner<'a, 'b>>,
}

struct ValueIterInner<'a, 'b> {
    root: &'a Value,
    path: Vec<PathComponent<'b>>,
    path_index: usize,
    recursive: bool,
}

impl<'a, 'b> ValueIter<'a, 'b> {
    pub fn new<T>(value: &'a Value, t: T) -> ValueIter<'a, 'b>
    where
        T: Into<PathIter<'b>>,
    {
        let root_value_iter = ValueIterInner::new(value, t);
        let mut stack = VecDeque::with_capacity(1);
        stack.push_front(root_value_iter);
        ValueIter { stack }
    }
}

impl<'a, 'b> Iterator for ValueIter<'a, 'b> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<&'a Value> {
        let mut inner_stack = VecDeque::new();
        loop {
            if self.stack.is_empty() {
                return None;
            }
            let result = self.stack[0].next_value(&mut inner_stack);
            if inner_stack.len() > 0 {
                self.stack.append(&mut inner_stack);
            }
            match result {
                None => {
                    self.stack.pop_front();
                    continue;
                }
                Some(value) => return Some(value),
            }
        }
    }
}

impl<'a, 'b> ValueIterInner<'a, 'b> {
    pub fn new<T>(value: &'a Value, t: T) -> ValueIterInner<'a, 'b>
    where
        T: Into<PathIter<'b>>,
    {
        ValueIterInner {
            root: value,
            path: t.into().collect(),
            path_index: 0,
            recursive: false,
        }
    }

    fn next_value(&mut self, stack: &mut VecDeque<ValueIterInner<'a, 'b>>) -> Option<&'a Value> {
        while self.path_index < self.path.len() {
            match self.path[self.path_index] {
                PathComponent::Name(name) => {
                    let opt_value = match self.root {
                        Value::Null | Value::String(_) | Value::Boolean(_) | Value::Number(_) => {
                            None
                        }
                        Value::Sequence(seq) => name
                            .parse::<usize>()
                            .ok()
                            .and_then(|index| {
                                if index >= seq.len() {
                                    return None;
                                }
                                if self.recursive {
                                    for (value_index, value) in seq.iter().enumerate() {
                                        if index == value_index {
                                            continue;
                                        }
                                        stack.push_back(ValueIterInner {
                                            root: value,
                                            path: self.path[self.path_index..].to_vec(),
                                            path_index: 0,
                                            recursive: true,
                                        });
                                    }
                                }
                                Some(&seq[index])
                            })
                            .or_else(|| {
                                for value in seq {
                                    stack.push_back(ValueIterInner {
                                        root: value,
                                        path: self.path[self.path_index..].to_vec(),
                                        path_index: 0,
                                        recursive: true,
                                    });
                                }
                                None
                            }),
                        Value::Mapping(map) => {
                            let name = name.to_lowercase();
                            let result = map.get(&name);
                            if self.recursive {
                                for (key, value) in map {
                                    if result.is_some() && key == &name {
                                        continue;
                                    }
                                    stack.push_back(ValueIterInner {
                                        root: value,
                                        path: self.path[self.path_index..].to_vec(),
                                        path_index: 0,
                                        recursive: true,
                                    });
                                }
                            }
                            result
                        }
                    };
                    match opt_value {
                        Some(value) => {
                            self.path_index += 1;
                            self.root = value;
                            self.recursive = false;
                            continue;
                        }
                        None => return None,
                    }
                }
                // no need to take into account recursive-ness when at an "any" path,
                // as this is not possible due to the normalization process applied on a map prior to using it in ValueIter
                PathComponent::Any => match self.root {
                    Value::Null | Value::String(_) | Value::Boolean(_) | Value::Number(_) => {
                        // return value if last element, otherwise will end up being None
                        self.path_index += 1;
                        continue;
                    }
                    Value::Sequence(seq) => {
                        for value in seq {
                            stack.push_back(ValueIterInner {
                                root: value,
                                path: self.path[self.path_index + 1..].to_vec(),
                                path_index: 0,
                                recursive: false,
                            });
                        }
                        self.path_index = self.path.len();
                        return None;
                    }
                    Value::Mapping(map) => {
                        for value in map.values() {
                            stack.push_back(ValueIterInner {
                                root: value,
                                path: self.path[self.path_index + 1..].to_vec(),
                                path_index: 0,
                                recursive: false,
                            });
                        }
                        self.path_index = self.path.len();
                        return None;
                    }
                },
                // no need to take into account recursive-ness when at an "anyRecursive" path,
                // as this is not possible due to the normalization process applied on a map prior to using it in ValueIter
                PathComponent::AnyRecursive => match self.root {
                    Value::Null | Value::String(_) | Value::Boolean(_) | Value::Number(_) => {
                        // return value if last element, otherwise will end up being None
                        self.path_index += 1;
                        self.recursive = true;
                        continue;
                    }
                    Value::Sequence(seq) => {
                        for value in seq {
                            stack.push_back(ValueIterInner {
                                root: value,
                                path: self.path[self.path_index + 1..].to_vec(),
                                path_index: 0,
                                recursive: true,
                            });
                        }
                        self.path_index = self.path.len();
                        return None;
                    }
                    Value::Mapping(map) => {
                        for value in map.values() {
                            stack.push_back(ValueIterInner {
                                root: value,
                                path: self.path[self.path_index + 1..].to_vec(),
                                path_index: 0,
                                recursive: true,
                            });
                        }
                        self.path_index = self.path.len();
                        return None;
                    }
                },
            }
        }

        None
    }
}
