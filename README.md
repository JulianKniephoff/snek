This is Snek!
=============

This project is an implementation of the classic Snake game,
probably most prominently found on 90's-era Nokia cell phones.
The twist is that it is written in the Rust programming language,
and also that it still manages to run in your browser!
This is achieved through the magic of WebAssembly.

This projects serves multiple purposes:

- I wanted to familiarize myself
  with compile-to-WASM technology.
- I wanted a small game project
  that I could actually complete
  in a reasonable amount of time.
- When it **is** complete,
  it can also serve as a playground
  to try out a few things on it,
  both gameplay- and presentation-wise.

Running it
----------

If you want to play around with this on your machine,
you will need the following things:

- An installation of the Rust programming language,
  including Cargo
- `wasm-pack` available in your PATH
- `make`
- Python 3 (for a simple local server)

Unfortunately, I can't give you exact version requirements for these,
since the build is currently only tested on my machine.
Be adviced, though, that I currently do not flinch
from using unstable features. 😉

If you have `rustup` installed,
which is the recommended way to install Rust,
you can just say

    rustup override set nightly

and the build process we are about to start
should do the right thing.

With all this out of the way,
you should be able to run

  make start

which runs `wasm-pack`
and starts a local development server,
on [`http://localhost:8000`](http://localhost:8000).

If you change Rust source files,
stop and rerun `make start`.

---

License
-------

Licensed under either of

- Apache License, Version 2.0,
  (see [LICENSE-APACHE](LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  (see [LICENSE-MIT](LICENSE-MIT)
  or http://opensource.org/licenses/MIT)

at your option.

    SPDX-License-Identifier: Apache-2.0 OR MIT

### Contribution

Unless you explicitly state otherwise,
any contribution intentionally submitted
for inclusion in the work by you,
as defined in the Apache-2.0 license,
shall be dual licensed as above,
without any additional terms or conditions.
