#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathComponent<'a> {
    Name(&'a str),
    Any,
    AnyRecursive,
}

pub struct PathIter<'a> {
    it: Box<dyn Iterator<Item = PathComponent<'a>> + 'a>,
    last: Option<PathComponent<'a>>,
}

impl<'a> PathIter<'a> {
    pub fn new<S: AsRef<str> + ?Sized>(s: &'a S) -> PathIter<'a> {
        let it = s.as_ref().split(".").filter(|s| s.trim() != "").map(|s| match s.trim() {
            "*" => PathComponent::Any,
            "**" => PathComponent::AnyRecursive,
            name => PathComponent::Name(name),
        });
        PathIter {
            it: Box::new(it),
            last: None,
        }
    }

    pub fn wrap<I: Iterator<Item = PathComponent<'a>> + 'a>(it: I) -> PathIter<'a> {
        PathIter {
            it: Box::new(it),
            last: None,
        }
    }
}

impl<'a> Iterator for PathIter<'a> {
    type Item = PathComponent<'a>;

    fn next(&mut self) -> Option<PathComponent<'a>> {
        loop {
            match self.it.next() {
                None => {
                    self.last = None;
                    return None;
                }
                Some(c) => match c {
                    PathComponent::Any => match self.last {
                        None => {
                            self.last = Some(PathComponent::Any);
                            return self.last;
                        }
                        Some(pc) => match pc {
                            PathComponent::Name(_) => {
                                self.last = Some(PathComponent::Any);
                                return self.last;
                            }
                            PathComponent::Any | PathComponent::AnyRecursive => continue,
                        },
                    },
                    PathComponent::AnyRecursive => match self.last {
                        None => {
                            self.last = Some(PathComponent::AnyRecursive);
                            return self.last;
                        }
                        Some(pc) => match pc {
                            PathComponent::Name(_) | PathComponent::Any => {
                                self.last = Some(PathComponent::AnyRecursive);
                                return self.last;
                            }
                            PathComponent::AnyRecursive => continue,
                        },
                    },
                    PathComponent::Name(_) => {
                        self.last = Some(c);
                        return self.last;
                    }
                }
            }
        }
    }
}

impl<'a> From<&'a str> for PathIter<'a> {
    fn from(s: &'a str) -> PathIter<'a> {
        PathIter::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let test_cases = vec![
            ("", vec![]),
            (".", vec![]),
            ("...", vec![]),
            ("foo.", vec![PathComponent::Name("foo")]),
            (".foo.", vec![PathComponent::Name("foo")]),
            (
                "foo.bar.baz",
                vec![
                    PathComponent::Name("foo"),
                    PathComponent::Name("bar"),
                    PathComponent::Name("baz"),
                ],
            ),
            (
                "foo.*.bar.**.baz",
                vec![
                    PathComponent::Name("foo"),
                    PathComponent::Any,
                    PathComponent::Name("bar"),
                    PathComponent::AnyRecursive,
                    PathComponent::Name("baz"),
                ],
            ),
            (
                "foo.*.bar.**.baz.*.*.**.*",
                vec![
                    PathComponent::Name("foo"),
                    PathComponent::Any,
                    PathComponent::Name("bar"),
                    PathComponent::AnyRecursive,
                    PathComponent::Name("baz"),
                    PathComponent::Any,
                    PathComponent::AnyRecursive,
                ],
            ),
            (
                "  foo.   bar .baz ",
                vec![
                    PathComponent::Name("foo"),
                    PathComponent::Name("bar"),
                    PathComponent::Name("baz"),
                ],
            ),
            (
                " 1*2.***",
                vec![PathComponent::Name("1*2"), PathComponent::Name("***")],
            ),
        ];
        for (input_str, expected_output_vec) in test_cases {
            let path_iter: PathIter = input_str.into();
            let output_vec: Vec<PathComponent> = path_iter.collect();
            let matching = output_vec
                .iter()
                .zip(expected_output_vec.iter())
                .filter(|&(a, b)| a == b)
                .count();
            assert_eq!(matching, expected_output_vec.len());
            assert_eq!(matching, output_vec.len());
        }
    }
}
