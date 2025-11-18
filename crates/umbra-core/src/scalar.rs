use curve25519_dalek::scalar::Scalar;
use rand_core::{CryptoRng, RngCore};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone, Debug, Zeroize, ZeroizeOnDrop)]
pub struct ScalarWrapper(pub Scalar);

// ---------- Serialize ----------
impl Serialize for ScalarWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.0.to_bytes();
        serializer.serialize_bytes(&bytes)
    }
}

// ---------- Deserialize ----------
impl<'de> Deserialize<'de> for ScalarWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: &[u8] = Deserialize::deserialize(deserializer)?;
        if bytes.len() != 32 {
            return Err(D::Error::custom("invalid scalar length"));
        }

        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);

        Ok(ScalarWrapper(Scalar::from_bytes_mod_order(arr)))
    }
}

impl ScalarWrapper {
    pub fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        ScalarWrapper(Scalar::random(rng))
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        ScalarWrapper(Scalar::from_bytes_mod_order(bytes))
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }
}
