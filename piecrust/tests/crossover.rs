// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use piecrust::{contract_bytecode, ContractData, Error, SessionData, VM};
use piecrust_uplink::ContractId;

const OWNER: [u8; 32] = [0u8; 32];

const CROSSOVER_ONE: ContractId = {
    let mut bytes = [0; 32];
    bytes[0] = 0x01;
    ContractId::from_bytes(bytes)
};

const CROSSOVER_TWO: ContractId = {
    let mut bytes = [0; 32];
    bytes[0] = 0x02;
    ContractId::from_bytes(bytes)
};

#[test]
fn crossover() -> Result<(), Error> {
    let vm = VM::ephemeral()?;

    let mut session = vm.session(SessionData::builder())?;
    session.set_point_limit(u64::MAX / 100);

    session.deploy(
        contract_bytecode!("crossover"),
        ContractData::builder(OWNER).contract_id(CROSSOVER_ONE),
    )?;
    session.deploy(
        contract_bytecode!("crossover"),
        ContractData::builder(OWNER).contract_id(CROSSOVER_TWO),
    )?;

    // These value should not be set to `INITIAL_VALUE` in the contract.
    const CROSSOVER_TO_SET: i32 = 42;
    const CROSSOVER_TO_SET_FORWARD: i32 = 314;
    const CROSSOVER_TO_SET_BACK: i32 = 272;

    // This call will fail if the state is inconsistent. Check the contract for
    // more details.
    session.call::<_, ()>(
        CROSSOVER_ONE,
        "check_consistent_state_on_errors",
        &(
            CROSSOVER_TWO,
            CROSSOVER_TO_SET,
            CROSSOVER_TO_SET_FORWARD,
            CROSSOVER_TO_SET_BACK,
        ),
    )?;

    assert_eq!(
        session.call::<_, i32>(CROSSOVER_ONE, "crossover", &())?.data,
        CROSSOVER_TO_SET,
        "The crossover should still be set even though the other contract panicked"
    );

    Ok(())
}
