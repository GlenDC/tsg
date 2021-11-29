use std::collections::HashMap;

use anyhow::{anyhow, Result};
use regex::bytes::Regex;
use serde_yaml;

use super::data::Value;

pub struct Meta {
    content: HashMap<String, Value>,
}

use super::file::FileFormat;

impl Meta {
    pub fn extract(format: FileFormat, content: &mut Vec<u8>) -> Result<Option<Meta>> {
        match format {
            FileFormat::Markdown => Meta::extract_markdown(content),
            FileFormat::Html => Meta::extract_html(content),
            // other file formats do not support Meta data, and thus we can immediately return None
            _ => Ok(None),
        }
    }

    // TODO: implement get/set :)

    pub fn get_path(&self, _path: &str) -> Option<Value> {
        None // TODO: implement
    }

    pub fn get_path_iter(&self, _it: &mut dyn Iterator<Item = &str>) -> Option<Value> {
        None // TODO: implement
    }

    pub fn set_path(&mut self, _path: &str, _value: Value) {
        // TODO: implement
    }

    // TODO: support glob?!?

    fn extract_html(content: &mut Vec<u8>) -> Result<Option<Meta>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?m)^\s*<!--\s*[\n\r]+\s*(?P<meta>.+?)\s*[\n\r]+\s*-->\s*").unwrap();
        }
        Meta::extract_header(&RE, content)
    }

    fn extract_markdown(content: &mut Vec<u8>) -> Result<Option<Meta>> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?m)^\s*---\s*[\n\r]+\s*(?P<meta>.+?)\s*[\n\r]+\s*---\s*(?P<next>.)?")
                    .unwrap();
        }
        Meta::extract_header(&RE, content)
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
                let content: HashMap<String, Value> =
                    m.into_iter().map(|(k, v)| (k, v.into())).collect();
                Ok(Some(Meta { content }))
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
