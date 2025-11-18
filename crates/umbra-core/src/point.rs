use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PointWrapper(pub EdwardsPoint);

// ---------- Serialize ----------
impl Serialize for PointWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.0.compress().to_bytes();
        serializer.serialize_bytes(&bytes)
    }
}

// ---------- Deserialize ----------
impl<'de> Deserialize<'de> for PointWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: &[u8] = Deserialize::deserialize(deserializer)?;
        if bytes.len() != 32 {
            return Err(D::Error::custom("invalid point length"));
        }

        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);

        let compressed = CompressedEdwardsY(arr);
        match compressed.decompress() {
            Some(point) => Ok(PointWrapper(point)),
            None => Err(D::Error::custom("invalid edwards point")),
        }
    }
}

// ---------- Utility ----------
impl PointWrapper {
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.compress().to_bytes()
    }
}
