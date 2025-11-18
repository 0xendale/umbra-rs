use crate::{PointWrapper, ScalarWrapper};
use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;
use rand_core::{CryptoRng, RngCore};

#[derive(Clone, Debug)]
pub struct Identity {
    pub initiator_spend_sk: ScalarWrapper,
    pub initiator_view_sk: ScalarWrapper,
    pub initiator_spend_pk: PointWrapper,
    pub initiator_view_pk: PointWrapper,
}

impl Identity {
    pub fn new_random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        let spend_scalar = ScalarWrapper::random(rng);
        let view_scalar = ScalarWrapper::random(rng);

        let spend_pubkey = PointWrapper(ED25519_BASEPOINT_POINT * spend_scalar.0);
        let view_pubkey = PointWrapper(ED25519_BASEPOINT_POINT * view_scalar.0);

        Self {
            initiator_spend_sk: spend_scalar,
            initiator_view_sk: view_scalar,
            initiator_spend_pk: spend_pubkey,
            initiator_view_pk: view_pubkey,
        }
    }
}
