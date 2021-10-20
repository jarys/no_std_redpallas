#![no_std]

mod error;
mod hash;
pub mod orchard;
mod signature;
mod signing_key;
mod verification_key;

/// An element of the protocol's scalar field used for randomization of public and secret keys.
pub type Randomizer<S> = <S as private::Sealed<S>>::Scalar;

pub use error::Error;
use hash::HStar;
pub use signature::Signature;
pub use signing_key::SigningKey;
pub use verification_key::{VerificationKey, VerificationKeyBytes};

/// Abstracts over different RedJubJub parameter choices, [`Binding`]
/// and [`SpendAuth`].
///
/// As described [at the end of §5.4.6][concretereddsa] of the Zcash
/// protocol specification, the generator used in RedJubjub is left as
/// an unspecified parameter, chosen differently for each of
/// `BindingSig` and `SpendAuthSig`.
///
/// To handle this, we encode the parameter choice as a genuine type
/// parameter.
///
/// [concretereddsa]: https://zips.z.cash/protocol/protocol.pdf#concretereddsa
pub trait SigType: private::Sealed<Self> {}

/// A trait corresponding to `BindingSig` in Zcash protocols.
pub trait Binding: SigType {}

/// A trait corresponding to `SpendAuthSig` in Zcash protocols.
pub trait SpendAuth: SigType {}

pub(crate) mod private {
    use super::*;

    pub trait SealedScalar {
        fn from_bytes_wide(bytes: &[u8; 64]) -> Self;
        fn from_raw(val: [u64; 4]) -> Self;
    }

    pub trait Sealed<T: SigType>:
        Copy + Clone + Default + Eq + PartialEq + core::fmt::Debug
    {
        const H_STAR_PERSONALIZATION: &'static [u8; 16];
        type Scalar: group::ff::PrimeField + SealedScalar;
        type Point: group::cofactor::CofactorCurve<Scalar = Self::Scalar>;

        fn basepoint() -> T::Point;
    }
}
