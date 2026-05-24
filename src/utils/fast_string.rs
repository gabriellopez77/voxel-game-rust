use std::hash::{Hash, Hasher};


#[derive(Eq)]
pub enum FastString<'a> {
    AllocatedString(String),
    StaticRawString(&'static str),
    RawString(&'a str),
    None,
}

impl<'a> FastString<'a> {
    pub fn get(&self) -> &str {
        match self {
            FastString::AllocatedString(s) => s.as_str(),
            FastString::StaticRawString(s) => s,
            FastString::RawString(s) => s,
            FastString::None => panic!("fast string is `None` value")
        }
    }
}

impl<'a> Hash for FastString<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let value = self.get();

        value.hash(state);
    }
}

impl<'a> PartialEq for FastString<'a> {
    fn eq(&self, other: &FastString) -> bool {
        return self.get() == other.get();
    }
}

impl<'a> PartialEq<&str> for FastString<'a> {
    fn eq(&self, other: &&str) -> bool {
        return self.get() == *other;
    }
}

impl<'a> PartialEq<String> for FastString<'a> {
    fn eq(&self, other: &String) -> bool {
        return self.get() == *other;
    }
}
