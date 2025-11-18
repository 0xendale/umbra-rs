use crate::{Identity, PointWrapper, ScalarWrapper};
use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;
use curve25519_dalek::edwards::EdwardsPoint;
use curve25519_dalek::scalar::Scalar;
use rand_core::{CryptoRng, RngCore};
use sha2::{Digest, Sha512};

fn hash_to_scalar(input: &[u8]) -> Scalar {
    let digest = Sha512::digest(input);
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&digest[..32]);
    Scalar::from_bytes_mod_order(bytes)
}

#[derive(Clone, Debug)]
pub struct InitiatorOutput {
    pub one_time_pubkey: PointWrapper,
    pub ephemeral_pubkey: PointWrapper,
}

#[derive(Clone, Debug)]
pub struct ClaimantRecovery {
    pub shared_secret_hash: ScalarWrapper,
    pub derived_spend_scalar: ScalarWrapper,
}

/// Initiator generates output (one-time address)
pub fn derive_for_initiator<R: RngCore + CryptoRng>(
    identity: &Identity,
    rng: &mut R,
) -> InitiatorOutput {
    // 1. ephemeral scalar
    let ephemeral_scalar = Scalar::random(rng);
    let ephemeral_pubkey = ED25519_BASEPOINT_POINT * ephemeral_scalar;

    // 2. shared secret point = r * B
    let shared_secret_point = identity.initiator_view_pk.0 * ephemeral_scalar;
    let shared_secret_bytes = shared_secret_point.compress().to_bytes();

    // 3. h = H(S)
    let shared_secret_hash = hash_to_scalar(&shared_secret_bytes);

    // 4. P = A + h·G
    let one_time_pubkey =
        identity.initiator_spend_pk.0 + (ED25519_BASEPOINT_POINT * shared_secret_hash);

    InitiatorOutput {
        one_time_pubkey: PointWrapper(one_time_pubkey),
        ephemeral_pubkey: PointWrapper(ephemeral_pubkey),
    }
}

/// Claimant scans & recovers spend authority
pub fn derive_for_claimant(
    identity: &Identity,
    one_time_pubkey: &PointWrapper,
    ephemeral_pubkey: &PointWrapper,
) -> Option<ClaimantRecovery> {
    // 1. recovered S = b * R
    let recovered_shared_point = ephemeral_pubkey.0 * identity.initiator_view_sk.0;
    let recovered_shared_bytes = recovered_shared_point.compress().to_bytes();

    // 2. h' = H(S')
    let recovered_shared_hash = hash_to_scalar(&recovered_shared_bytes);

    // 3. reconstruct P' = A + h'·G
    let reconstructed_pubkey =
        identity.initiator_spend_pk.0 + (ED25519_BASEPOINT_POINT * recovered_shared_hash);

    if reconstructed_pubkey == one_time_pubkey.0 {
        // 4. derive spend scalar: a + h'
        let derived_scalar = identity.initiator_spend_sk.0 + recovered_shared_hash;
        Some(ClaimantRecovery {
            shared_secret_hash: ScalarWrapper(recovered_shared_hash),
            derived_spend_scalar: ScalarWrapper(derived_scalar),
        })
    } else {
        None
    }
}
