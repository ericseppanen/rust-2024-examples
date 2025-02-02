pub struct Name {
    value: Option<String>,
}

impl Name {
    pub fn is_short(&self) -> bool {
        match &self.value {
            Some(ref val) => val.len() <= 4,
            None => true,
        }
    }
}

/// (Name, Age)
pub type PersonRef<'a> = (&'a str, &'a u32);

pub fn find_oldest(people: Vec<PersonRef<'_>>) -> PersonRef<'_> {
    *people.iter().max_by_key(|(_, &age)| age).unwrap()
}
