#[derive(Debug, Clone, Copy)]
pub enum PathComponent<'a> {
    Empty,
    Name(&'a str),
    Any,
    AnyRecursive,
}

pub struct PathIter<'a> {
    it: Box<dyn Iterator<Item = &'a str> + 'a>,
}

impl<'a> PathIter<'a> {
    fn new<S: AsRef<str> + ?Sized>(s: &'a S) -> PathIter<'a> {
        let it = s.as_ref().split(".");
        PathIter { it: Box::new(it) }
    }
}

impl<'a> Iterator for PathIter<'a> {
    type Item = PathComponent<'a>;

    fn next(&mut self) -> Option<PathComponent<'a>> {
        match self.it.next() {
            None => None,
            Some(s) => Some(match s.trim() {
                "" => PathComponent::Empty,
                "*" => PathComponent::Any,
                "**" => PathComponent::AnyRecursive,
                _ => PathComponent::Name(s),
            }),
        }
    }
}

impl<'a> From<&'a str> for PathIter<'a> {
    fn from(s: &'a str) -> PathIter<'a> {
        PathIter::new(s)
    }
}

impl<'a> From<Box<dyn Iterator<Item = &'a str>>> for PathIter<'a> {
    fn from(it: Box<dyn Iterator<Item = &'a str>>) -> PathIter<'a> {
        PathIter{ it }
    }
}