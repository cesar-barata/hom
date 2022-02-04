mod message;
mod cipher;

#[cfg(test)]
mod tests {
    use crate::message::Message;

    #[test]
    fn homomorphic_property_even_sized_message1_even_sized_message2() {
        let message1 = Message("abcd".into());
        let message2 = Message("uvwx".into());
        let concat_messages = message1.concat(&message2);

        let cipher1 = message1.encrypt();
        let cipher2 = message2.encrypt();
        let concat_ciphers = cipher1.concat(&cipher2);

        assert_eq!(concat_messages.encrypt(), concat_ciphers);
        assert_eq!(concat_messages, concat_ciphers.decrypt());
    }

    #[test]
    fn homomorphic_property_even_sized_message1_odd_sized_message2() {
        let message1 = Message("abcd".into());
        let message2 = Message("uvw".into());
        let concat_messages = message1.concat(&message2);

        let cipher1 = message1.encrypt();
        let cipher2 = message2.encrypt();
        let concat_ciphers = cipher1.concat(&cipher2);

        assert_eq!(concat_messages.encrypt(), concat_ciphers);
        assert_eq!(concat_messages, concat_ciphers.decrypt());
    }

    #[test]
    fn homomorphic_property_odd_sized_message1_even_sized_message2() {
        let message1 = Message("abcd".into());
        let message2 = Message("uvw".into());
        let concat_messages = message1.concat(&message2);

        let cipher1 = message1.encrypt();
        let cipher2 = message2.encrypt();
        let concat_ciphers = cipher1.concat(&cipher2);

        assert_eq!(concat_messages.encrypt(), concat_ciphers);
        assert_eq!(concat_messages, concat_ciphers.decrypt());
    }

    #[test]
    fn homomorphic_property_odd_sized_message1_odd_sized_message2() {
        let message1 = Message("abc".into());
        let message2 = Message("uvw".into());
        let concat_messages = message1.concat(&message2);

        let cipher1 = message1.encrypt();
        let cipher2 = message2.encrypt();
        let concat_ciphers = cipher1.concat(&cipher2);

        assert_eq!(concat_messages.encrypt(), concat_ciphers);
        assert_eq!(concat_messages, concat_ciphers.decrypt());
    }

    #[test]
    fn decrypt_after_encrypt_is_identity() {
        let message = Message("message".into());
        let cipher = message.encrypt();
        assert_eq!(message, cipher.decrypt());

        let message = Message("abcduvwx".into());
        let cipher = message.encrypt();
        assert_eq!(message, cipher.decrypt());

        let message = Message("abcduvwx1".into());
        let cipher = message.encrypt();
        assert_eq!(message, cipher.decrypt());
    }
}