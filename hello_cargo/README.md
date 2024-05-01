# Let’s recap what we’ve learned so far about Cargo:

- We can create a project using `cargo new`.
- We can build a project using `cargo build`.
- We can build and run a project in one step using `cargo run`.
- We can build a project without producing a binary to check for errors using `cargo check`.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the *target/debug* directory.

An additional advantage of using Cargo is that the commands are the same no matter which operating system you’re working on.
So, at this point, we’ll no longer provide specific instructions for Linux and macOS versus Windows.

## Building for Release

When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations.
This command will create an executable in *target/release* instead of *target/debug*. The optimizations make your Rust code run faster,
but turning them on lengthens the time it takes for your program to compile.
This is why there are two different profiles: one for development, when you want to rebuild quickly and often,
and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible.
If you’re benchmarking your code’s running time, be sure to run cargo build --release and benchmark with the executable in target/release.