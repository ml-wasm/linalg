use super::FloatsVector;
use ndarray::Array1;
use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

impl Serialize for FloatsVector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.data.iter().map(|x| *x))
    }
}

struct Floats1dVisitor;

impl<'de> Visitor<'de> for Floats1dVisitor {
    type Value = FloatsVector;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an array of numbers")
    }

    fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let mut vec: Vec<f64> = Vec::with_capacity(access.size_hint().unwrap_or(0));

        while let Some(val) = access.next_element()? {
            vec.push(val);
        }

        Ok(FloatsVector {
            data: Array1::from_vec(vec),
        })
    }
}

impl<'de> Deserialize<'de> for FloatsVector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(Floats1dVisitor)
    }
}
