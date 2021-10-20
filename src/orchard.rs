//! Signature types for the Orchard protocol.

use group::GroupEncoding;
use pasta_curves::pallas;

use crate::{private, SigType};

/// The byte-encoding of the basepoint for `OrchardSpendAuthSig`.
const ORCHARD_SPENDAUTHSIG_BASEPOINT_BYTES: [u8; 32] = [
    99, 201, 117, 184, 132, 114, 26, 141, 12, 161, 112, 123, 227, 12, 127, 12, 95, 68, 95, 62, 124,
    24, 141, 59, 6, 214, 241, 40, 179, 35, 85, 183,
];

/// The byte-encoding of the basepoint for `OrchardBindingSig`.
const ORCHARD_BINDINGSIG_BASEPOINT_BYTES: [u8; 32] = [
    145, 90, 60, 136, 104, 198, 195, 14, 47, 128, 144, 238, 69, 215, 110, 64, 72, 32, 141, 234, 91,
    35, 102, 79, 187, 9, 164, 15, 85, 68, 244, 7,
];

/// A type variable corresponding to Zcash's `OrchardSpendAuthSig`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SpendAuth {}
// This should not exist, but is necessary to use zeroize::DefaultIsZeroes.
impl Default for SpendAuth {
    fn default() -> Self {
        unimplemented!()
    }
}
impl SigType for SpendAuth {}
impl super::SpendAuth for SpendAuth {}

/// A type variable corresponding to Zcash's `OrchardBindingSig`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Binding {}
// This should not exist, but is necessary to use zeroize::DefaultIsZeroes.
impl Default for Binding {
    fn default() -> Self {
        unimplemented!()
    }
}
impl SigType for Binding {}
impl super::Binding for Binding {}

impl private::SealedScalar for pallas::Scalar {
    fn from_bytes_wide(bytes: &[u8; 64]) -> Self {
        <pallas::Scalar as pasta_curves::arithmetic::FieldExt>::from_bytes_wide(bytes)
    }
    fn from_raw(val: [u64; 4]) -> Self {
        pallas::Scalar::from_raw(val)
    }
}
impl private::Sealed<SpendAuth> for SpendAuth {
    const H_STAR_PERSONALIZATION: &'static [u8; 16] = b"Zcash_RedPallasH";
    type Point = pallas::Point;
    type Scalar = pallas::Scalar;

    fn basepoint() -> pallas::Point {
        pallas::Point::from_bytes(&ORCHARD_SPENDAUTHSIG_BASEPOINT_BYTES).unwrap()
    }
}
impl private::Sealed<Binding> for Binding {
    const H_STAR_PERSONALIZATION: &'static [u8; 16] = b"Zcash_RedPallasH";
    type Point = pallas::Point;
    type Scalar = pallas::Scalar;

    fn basepoint() -> pallas::Point {
        pallas::Point::from_bytes(&ORCHARD_BINDINGSIG_BASEPOINT_BYTES).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn orchard_spendauth_basepoint() {
        use super::ORCHARD_SPENDAUTHSIG_BASEPOINT_BYTES;
        use group::GroupEncoding;
        use pasta_curves::{arithmetic::CurveExt, pallas};

        assert_eq!(
            pallas::Point::hash_to_curve("z.cash:Orchard")(b"G").to_bytes(),
            ORCHARD_SPENDAUTHSIG_BASEPOINT_BYTES
        );
    }

    #[test]
    fn orchard_binding_basepoint() {
        use super::ORCHARD_BINDINGSIG_BASEPOINT_BYTES;
        use group::GroupEncoding;
        use pasta_curves::{arithmetic::CurveExt, pallas};

        assert_eq!(
            pallas::Point::hash_to_curve("z.cash:Orchard-cv")(b"r").to_bytes(),
            ORCHARD_BINDINGSIG_BASEPOINT_BYTES
        );
    }
}
