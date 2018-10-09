rustc func1.rs --crate-type lib
rustc func2.rs --crate-type lib
rustc main.rs --extern func1=libfunc1.rlib --extern func2=libfunc2.rlib