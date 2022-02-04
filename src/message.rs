use crate::cipher::Cipher;

#[derive(Debug, PartialEq)]
pub struct Message(pub String);

impl Message {
    pub fn concat(&self, other: &Message) -> Self {
        format!("{}{}", self.0, other.0).into()
    }

    pub fn encrypt(&self) -> Cipher {
        let (start, end) = &self.0.chars().enumerate()
            .fold(("".to_string(), "".to_string()), |mut acc, (i, e)| {
                let the_string = if i % 2 == 0 { &mut acc.0 } else { &mut acc.1 };
                the_string.push(e);
                acc
            });
        Cipher(format!("{}{}", start, end))
    }
}

impl From<String> for Message {
    fn from(s: String) -> Self {
        Self(s)
    }
}