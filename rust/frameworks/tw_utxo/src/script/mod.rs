// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

use crate::encode::stream::Stream;
use crate::encode::Encodable;
use tw_memory::Data;

#[derive(Clone, Debug, Default)]
pub struct Script {
    bytes: Data,
}

impl Encodable for Script {
    fn encode(&self, stream: &mut Stream) {
        stream.append(&self.bytes);
    }
}

impl Script {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn as_data(&self) -> &Data {
        &self.bytes
    }
}

impl From<Data> for Script {
    fn from(bytes: Data) -> Self {
        Script { bytes }
    }
}