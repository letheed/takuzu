/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

macro_rules! println_err {
    ($($arg: tt)*) => ({
        use std::io::{Write, stderr};
        write!(&mut stderr(), "{}\n", format!($($arg)*)).unwrap();
    })
}
