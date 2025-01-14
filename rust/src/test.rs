// Strict encoding library for deterministic binary serialization.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2023 by
//     Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright 2022-2023 UBIDECO Institute
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::Debug;
use std::io;
use std::io::BufRead;

use amplify::confinement::Confined;

use crate::{StrictDecode, StrictEncode, StrictReader, StrictWriter};

pub fn encode<T: StrictEncode + Debug + Eq>(val: &T) -> Vec<u8> {
    const MAX: usize = u16::MAX as usize;

    let ast_data = StrictWriter::in_memory(MAX);
    let data = val.strict_encode(ast_data).unwrap().unbox();
    Confined::<Vec<u8>, 0, MAX>::try_from(data)
        .unwrap()
        .into_inner()
}

pub fn decode<T: StrictDecode + Debug + Eq>(data: impl AsRef<[u8]>) -> T {
    const MAX: usize = u16::MAX as usize;

    let cursor = io::Cursor::new(data);
    let mut reader = StrictReader::with(MAX, cursor);
    let val2 = T::strict_decode(&mut reader).unwrap();
    let mut cursor = reader.unbox();
    assert!(!cursor.fill_buf().unwrap().is_empty(), "data not entirely consumed");

    val2
}

#[allow(dead_code)]
pub fn encoding_roundtrip<T: StrictEncode + StrictDecode + Debug + Eq>(val: &T) {
    let data = encode(val);
    let val2: T = decode(data);
    assert_eq!(val, &val2);
}

#[allow(dead_code)]
pub fn encoding<T: StrictEncode + StrictDecode + Debug + Eq>(val: &T, expect: impl AsRef<[u8]>) {
    let data = encode(val);
    assert_eq!(&data[..], expect.as_ref());
    let val2: T = decode(data);
    assert_eq!(val, &val2);
}
