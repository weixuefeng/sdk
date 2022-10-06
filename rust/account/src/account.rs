// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

use snarkvm_wasm::{
    account::{Address as AleoAddress, PrivateKey as AleoPrivateKey, ViewKey as AleoViewKey},
    network::Testnet3,
    program::{Ciphertext as AleoCiphertext, Record as AleoRecord},
};

pub type CurrentNetwork = Testnet3;
pub type Address = AleoAddress<CurrentNetwork>;
pub type PrivateKey = AleoPrivateKey<CurrentNetwork>;
pub type ViewKey = AleoViewKey<CurrentNetwork>;
pub type Record = AleoRecord<CurrentNetwork, AleoCiphertext<CurrentNetwork>>;
