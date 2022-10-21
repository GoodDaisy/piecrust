// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use piecrust::{module_bytecode, Error, VM};

#[test]
fn fallible_read_write_panic() -> Result<(), Error> {
    let mut vm = VM::ephemeral()?;
    let id = vm.deploy(module_bytecode!("fallible_counter"))?;

    let mut session = vm.session();

    session.transact::<bool, ()>(id, "increment", false)?;

    assert_eq!(session.query::<(), i64>(id, "read_value", ())?, 0xfd);

    let err = session.transact::<bool, ()>(id, "increment", true).is_err();

    assert!(err, "execution failed");

    assert_eq!(
        session.query::<(), i64>(id, "read_value", ())?,
        0xfd,
        "should remain unchanged, since the panic happened"
    );

    Ok(())
}
