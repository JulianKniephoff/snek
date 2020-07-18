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

- A Node.js installation,
  including npm
- An installation of the Rust programming language,
  including Cargo

Unfortunately, I can't give you exact version requirements for these,
since the build is currently only tested on my machine.
Just make sure your Node.js setup is reasonably modern,
and for Rust, you are probably going to need the nightly toolchian,
as I currently do not flinch from using unstable features. :wink:

If you have `rustup` installed,
which is the recommended way to install Rust,
you can just say

    rustup override set nightly

and the build process we are about to start
should do the right thing.

With all this out of the way,
you should be able to run

    npm install

to install further dependencies
of the Node.js-based build process.

Finally, you can run

    npm start

which starts a local development server
on [`http://localhost:8080`](http://localhost:8080),
which will automatically rebuild the project
on any change to its source files.
