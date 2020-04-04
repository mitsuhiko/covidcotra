pub mod base64 {
    use serde::{Deserialize, de::Error, ser::Serializer, de::Deserializer};
    use serde::private::de::{ContentDeserializer, Content};

    pub fn serialize<T, S>(buffer: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: AsRef<[u8]>,
        S: Serializer,
    {
        serializer.serialize_str(&base64::encode(buffer.as_ref()))
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        let bytes = String::deserialize(deserializer)
            .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))?;
        T::deserialize(ContentDeserializer::new(Content::ByteBuf(bytes)))
    }
}