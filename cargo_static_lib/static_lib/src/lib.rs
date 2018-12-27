mod func {
    #[allow(dead_code)]
	#[link(name = "func", kind = "static")]
    extern "C" {
        pub fn f(i: i32) -> i32;
    }
}

pub fn f(i: i32) -> i32 {
	unsafe { func::f(i) }
}