# Create your own rust library and import it within the module system.

If you would prefer to reuse this library in other projects I recommend checking it into git and tossing it up on github. If you only will use this library internal please scroll down to that section.

# Below are instructions how to build an internal "library" just for one project

    cargo new leonhard --lib     # your new library name
    cdd                          # cd into newest created folder

`src/lib.rs is` where you add this code:

    pub mod file_io {
        // #[allow(dead_code)]
        pub fn some_function() {
            println!("# some_function called");
        }
    }

In order for the module to be accessible it must be pub. Same goes for the method within the module.

# Cargo.toml

Requires the absolute or relative path to the dependency, in this case the library called `leonhard`.

    [dependencies]
    leonhard = {path = "../leonhard"}

Because all of your rust projects should live at the top level `~/code/rust/` you can then easily use relative paths, reuse a ton of code and be setup nicely for git and a github repository for your library.
And with that our rather simple library is ready for use in some project

    cargo new create_use_library # your project folder name
    cdd                          # cd into newest created folder

# main.rs

    use leonhard::file_io;

    fn main() {
        file_io::some_function();
    }

Here we import the library called `leonhard` but only the module `file_io`, so if there are more modules they are intentionally not accessible for now.

Calling `file_io::some_function();` to execute when the program runs.

# Run

To both build and run you can use:

    cargo run

# Build, Test, Run

    cargo test; cargo run;

Note that you do not need to navigate down the tree to your library folder `leonhard`, when build or run occur the library will be complied. Thus adding more methods to lib.rs you save some navigation effort too.

---

---

## Internal "library"

Below are instructions how to use a library for just for one project, aka not reusing code in multiple projects, just this one:

    cargo new create_use_library # your project folder name
    cdd                          # cd into newest created folder
    cd src                       # where your library folder lives
    cargo new leonhard --lib     # your new library name

`src/lib.rs is` where you add this code:

    pub mod file_io {
        // #[allow(dead_code)]
        pub fn some_function() {
            println!("# some_function called");
        }
    }

In order for the module to be accessible it must be pub. Same goes for the method within the module.

# Cargo.toml

Requires the absolute or relative path to the dependency, in this case the library called `leonhard`.

# main.rs

    use leonhard::file_io;

    fn main() {
        file_io::some_function();
    }

Here we import the library called `leonhard` but only the module `file_io`, so if there are more modules they are intentionally not accessible for now.

Calling `file_io::some_function();` to execute when the program runs.

# Run

To both build and run you can use:

    cargo run

# Build, Test, Run

    cargo test; cargo run;

Note that you do not need to navigate down the tree to your library folder `leonhard`, when build or run occur the library will be complied. Thus adding more methods to lib.rs you save some navigation effort too.

This however is not the case for `cargo test` from the top level. That will not run your module level unit tests (aka `leonhard`) has some tests that are ran if you instead run `cargo test --package leonhard` which will run only the tests from that package, from the top level without navigating down.
