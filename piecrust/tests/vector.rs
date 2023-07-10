// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use piecrust::{contract_bytecode, ContractData, Error, SessionData, VM};

const OWNER: [u8; 32] = [0u8; 32];

#[test]
pub fn vector_push_pop() -> Result<(), Error> {
    let vm = VM::ephemeral()?;

    let mut session = vm.session(SessionData::builder())?;

    let id = session
        .deploy(contract_bytecode!("vector"), ContractData::builder(OWNER))?;

    const N: usize = 128;

    for i in 0..N {
        session.call::<_, ()>(id, "push", &(i as i16))?;
    }

    for i in 0..N {
        let popped: Option<i16> = session.call(id, "pop", &())?.data;

        assert_eq!(popped, Some((N - i - 1) as i16));
    }

    let popped: Option<i16> = session.call(id, "pop", &())?.data;

    assert_eq!(popped, None);

    Ok(())
}
