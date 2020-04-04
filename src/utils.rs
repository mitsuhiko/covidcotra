pub mod base64 {
    use serde::private::de::{Content, ContentDeserializer};
    use serde::{de::Deserializer, de::Error, ser::Serializer, Deserialize};

    pub fn serialize<T, S>(buffer: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: AsRef<[u8]> + ?Sized,
        S: Serializer,
    {
        serializer.serialize_str(&base64::encode(buffer.as_ref()))
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        let bytes = String::deserialize(deserializer).and_then(|string| {
            base64::decode(&string).map_err(|err| Error::custom(err.to_string()))
        })?;
        // this is stupid but i'm lazy
        T::deserialize(ContentDeserializer::new(Content::Seq(
            bytes.into_iter().map(Content::U8).collect(),
        )))
    }
}
