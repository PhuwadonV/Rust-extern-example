mod func {
    #[link(name = "lib/func", kind = "static")]
    extern "C" {
        pub fn f(i: i32) -> i32;
    }
}

fn main() {
    println!("{}", unsafe { func::f(5) });
}
