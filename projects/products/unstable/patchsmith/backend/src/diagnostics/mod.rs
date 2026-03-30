pub mod error;

use serde::Deserializer;

pub fn deserialize_usize_from_number<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<usize, D::Error> {
    struct UsizeVisitor;
    impl<'de> serde::de::Visitor<'de> for UsizeVisitor {
        type Value = usize;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a non-negative integer")
        }
        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<usize, E> {
            usize::try_from(v).map_err(|_| E::custom("value too large for usize"))
        }
        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<usize, E> {
            usize::try_from(v).map_err(|_| E::custom("negative integer"))
        }
        fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<usize, E> {
            if v.fract() == 0.0 && v >= 0.0 && v <= usize::MAX as f64 {
                Ok(v as usize)
            } else {
                Err(E::custom(format!("cannot convert {v} to usize")))
            }
        }
    }
    deserializer.deserialize_any(UsizeVisitor)
}
