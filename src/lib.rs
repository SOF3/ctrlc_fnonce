/*
 * ctrlc_fnonce
 *
 * Copyright (C) 2019 SOFe
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

extern crate ctrlc;

use std::process;
use std::sync::Mutex;

/// Register a shutdown handler that activates upon Ctrl-C.
///
/// This function accepts `FnOnce`, allowing values to be moved into the handler.
///
/// This function only provides thread-safe FnOnce guarantee
/// and calls `process::exit` upon shutdown.
/// Other semantics are identical to the [`ctrlc`](https://docs.rs/ctrlc) `set_handler` function.
pub fn set_ctrlc_handler<F>(f: F) -> Result<(), ctrlc::Error>
    where F: FnOnce() -> i32 + Send + 'static {
    let f = Mutex::new(Some(f));
    ctrlc::set_handler(move || {
        if let Ok(mut guard) = f.lock() {
            let f = guard.take().expect("f can only be taken once");
            let code = f();
            process::exit(code);
        }
    })
}

// as long as this compiles, it works. There is nothing to test.
