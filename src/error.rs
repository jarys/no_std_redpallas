// -*- mode: rust; -*-
//
// This file is part of reddsa.
// Copyright (c) 2019-2021 Zcash Foundation
// See LICENSE for licensing information.
//
// Authors:
// - Deirdre Connolly <deirdre@zfnd.org>
// - Henry de Valence <hdevalence@hdevalence.ca>

//use thiserror::Error;

/// An error related to RedJubJub signatures.
#[derive(Debug, Copy, Clone, Eq, PartialEq)] // Error
pub enum Error {
    /// The encoding of a signing key was malformed.
    //#[error("Malformed signing key encoding.")]
    MalformedSigningKey,
    /// The encoding of a verification key was malformed.
    //#[error("Malformed verification key encoding.")]
    MalformedVerificationKey,
    /// Signature verification failed.
    //#[error("Invalid signature.")]
    InvalidSignature,
}
