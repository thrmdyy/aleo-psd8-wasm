// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

use anyhow::Error;
use once_cell::sync::OnceCell;
use snarkvm::{
    circuit::IndexMap,
    prelude::{
        Address, ComputeKey, CryptoRng, Field, Group, Identifier, Literal, Network, Plaintext,
        PrivateKey, Rng, Scalar, Testnet3, ToFields, Uniform, GENERATOR_G, U128, U64, U8,
    },
};
use wasm_bindgen::prelude::*;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

pub fn get_quote_struct_literal(
    token1: u64,
    token2: u64,
    version: u8,
) -> Result<Plaintext<Testnet3>, Error> {
    let value = Plaintext::<Testnet3>::Struct(
        IndexMap::from_iter(
            vec![
                (
                    Identifier::try_from("token1")?,
                    Plaintext::<Testnet3>::Literal(Literal::U64(U64::new(token1)), OnceCell::new()),
                ),
                (
                    Identifier::try_from("token2")?,
                    Plaintext::<Testnet3>::Literal(Literal::U64(U64::new(token2)), OnceCell::new()),
                ),
                (
                    Identifier::try_from("version")?,
                    Plaintext::<Testnet3>::Literal(Literal::U8(U8::new(version)), OnceCell::new()),
                ),
            ]
            .into_iter(),
        ),
        OnceCell::new(),
    );
    Ok(value)
}

#[wasm_bindgen]
pub fn calc_psd8(token1: u64, token2: u64, version: u8) -> String {
    let value = get_quote_struct_literal(token1, token2, version).unwrap();
    let hash = Testnet3::hash_psd8(&value.to_fields().unwrap()).unwrap();

    return hash.to_string();
}
