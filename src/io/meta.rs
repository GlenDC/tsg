use std::collections::HashMap;

use anyhow::Result;
use regex::bytes::Regex;
use serde_yaml;

use super::data::{Value, ValueIter};
use super::path::PathIter;

pub struct Meta {
    content: Value,
}

use super::file::FileFormat;

impl Meta {
    pub fn extract(format: FileFormat, content: &mut Vec<u8>) -> Result<Option<Meta>> {
        match format {
            // extract as header data
            FileFormat::Markdown => Meta::extract_markdown(content),
            FileFormat::Html => Meta::extract_html(content),
            // extract from entire file data
            FileFormat::Yaml => Meta::extract_yaml(content),
            FileFormat::Json => Meta::extract_json(content),
            // other file formats do not support Meta data, and thus we can immediately return None
            FileFormat::Rhai | FileFormat::Bash => Ok(None),
        }
    }

    pub fn value<'a, 'b, T>(&'a self, t: T) -> Option<&'a Value>
    where
        T: Into<PathIter<'b>>,
    {
        self.content.value(t)
    }

    pub fn value_iter<'a, 'b, T>(&'a self, t: T) -> ValueIter<'a, 'b>
    where
        T: Into<PathIter<'b>>,
    {
        self.content.value_iter(t)
    }

    pub fn value_mut<'a, 'b, T>(&'a mut self, t: T) -> Option<&'a mut Value>
    where
        T: Into<PathIter<'b>>,
    {
        self.content.value_mut(t)
    }

    fn extract_html(content: &mut Vec<u8>) -> Result<Option<Meta>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?m)^\s*<!--\s*[\n\r]+\s*(?P<meta>.+?)\s*[\n\r]+\s*-->\s*\n*(?P<next>.)?"
            )
            .unwrap();
        }
        Meta::extract_header(&RE, content)
    }

    fn extract_markdown(content: &mut Vec<u8>) -> Result<Option<Meta>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?m)^\s*---\s*[\n\r]+\s*(?P<meta>.+?)\s*[\n\r]+\s*---\s*\n*(?P<next>.)?"
            )
            .unwrap();
        }
        Meta::extract_header(&RE, content)
    }

    fn extract_yaml(content: &mut Vec<u8>) -> Result<Option<Meta>> {
        let m: HashMap<String, serde_yaml::Value> = serde_yaml::from_slice(&content)?;
        let map: HashMap<String, Value> = m.into_iter().map(|(k, v)| (k, v.into())).collect();
        drop_first_n_bytes(content, content.len());
        Ok(Some(Meta {
            content: Value::Mapping(map),
        }))
    }

    fn extract_json(content: &mut Vec<u8>) -> Result<Option<Meta>> {
        let m: HashMap<String, serde_json::Value> = serde_json::from_slice(&content)?;
        let map: HashMap<String, Value> = m.into_iter().map(|(k, v)| (k, v.into())).collect();
        drop_first_n_bytes(content, content.len());
        Ok(Some(Meta {
            content: Value::Mapping(map),
        }))
    }

    fn extract_header(re: &Regex, content: &mut Vec<u8>) -> Result<Option<Meta>> {
        let result = match re
            .captures(content)
            .and_then(|m| m.name("meta").and_then(|meta| Some((m, meta))))
        {
            None => None,
            Some((m, meta)) => Some((
                meta.as_bytes().to_vec(),
                m.name("next").and_then(|n| Some(n.start())),
            )),
        };
        match result {
            None => Ok(None),
            Some((raw_content, n_opt)) => {
                if let Some(n) = n_opt {
                    drop_first_n_bytes(content, n);
                }
                let m: HashMap<String, serde_yaml::Value> = serde_yaml::from_slice(&raw_content)?;
                let map: HashMap<String, Value> =
                    m.into_iter().map(|(k, v)| (k, v.into())).collect();
                Ok(Some(Meta {
                    content: Value::Mapping(map),
                }))
            }
        }
    }
}

fn drop_first_n_bytes(vec: &mut Vec<u8>, n: usize) {
    // We have to make sure vec has enough elements.
    // You could make the function unsafe and ask the caller to ensure
    // this condition.
    assert!(vec.len() >= n);

    unsafe {
        // We need to update the vector. The values from index `i` to its end
        // need to be moved at the beginning of the vector.
        // SAFETY:
        //  * We have an exclusive reference to the vector. It is both valid for reads and writes.
        std::ptr::copy(vec.as_ptr().add(n), vec.as_mut_ptr(), n);

        // And update the length of `vec`.
        // SAFETY: This subtraction is safe because we previously checked that `vec.len() >= n`.
        vec.set_len(vec.len() - n);
    }
}
