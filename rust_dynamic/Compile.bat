rustc func1.rs --crate-type dylib -C prefer-dynamic
rustc func2.rs --crate-type dylib -C prefer-dynamic
rustc main.rs --extern func1=func1.dll --extern func2=func2.dll