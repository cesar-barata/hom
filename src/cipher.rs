use crate::message::Message;

#[derive(Debug, PartialEq)]
pub struct Cipher(pub String);

impl Cipher {
    fn compute_split_index(cipher: &str) -> usize {
        if cipher.len() % 2 == 0 {
            cipher.len() / 2
        } else {
            (cipher.len() as f64 / 2.0).ceil() as usize
        }
    }

    fn split_cipher(cipher: &str) -> (&str, &str) {
        let split_index = Self::compute_split_index(cipher);
        (&cipher[0..split_index], &cipher[split_index..])
    }

    fn compose_concat_cipher((a1, a2): (&str, &str), (b1, b2): (&str, &str)) -> String {
        if (a1.len() + a2.len()) % 2 == 0 {
            format!("{}{}{}{}", a1, b1, a2, b2)
        } else {
            format!("{}{}{}{}", a1, b2, a2, b1)
        }
    }

    pub fn concat(&self, other: &Cipher) -> Self {
        let cipher1 = Self::split_cipher(&self.0);
        let cipher2 = Self::split_cipher(&other.0);
        Cipher(Self::compose_concat_cipher(cipher1, cipher2))
    }

    pub fn decrypt(&self) -> Message {
        let cipher = &self.0;
        let (start, end) = Self::split_cipher(cipher);
        let mut message_text = String::new();
        for i in 0..end.len() {
            message_text.push_str(&start[i..i+1]);
            message_text.push_str(&end[i..i+1]);
        }
        if start.len() > end.len() {
            message_text.push_str(&start[start.len() - 1..]);
        }
        Message(message_text)
    }
}


impl From<&str> for Cipher {
    fn from(s: &str) -> Self {
        Self(s.clone().into())
    }
}