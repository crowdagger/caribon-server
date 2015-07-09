Caribon-server
==============

Note: if you are only interested in *using* Caribon server as a
client,
[an instance should be running here](http://vps184889.ovh.net/caribon/).

What is Caribon?
----------------

[Caribon](https://github.com/lady-segfault/caribon) is a Rust library
to detect repetitions in a text, and also a command line program that
uses this library.

What is Caribon-server?
----------------------- 

Caribon-server is a web service that uses this library. It allows for
a more user friendly interface than the command line.

Running Caribon-server
----------------------

Download the latest version with `git`:

`$ git clone https://github.com/lady-segfault/caribon-server.git`

You will then need Rust and Cargo to fetch the dependencies and build
the program. See their
[install instructions](http://www.rust-lang.org/install.html). Then
you just have to type:

`$ cargo run --release`

And `Cargo` should get all dependencies, build everything, and run the
program. Then just use your favorite browser and go to
`http://localhost:3000`.

(You can also specify the host(s) and port(s) on which you want to
listen, e.g. `cargo run --release -- localhost:8080 localhost:3000`
will listen on both `8080` and `3000` ports.)

License
-------

Caribon-server is licensed under the
[GNU General Public License](LICENSE), version 2.0 or (at your
convenience) any ulterior version.
