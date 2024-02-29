static mut STASH: i32 = 0;

fn f(p: &'static i32) {
    unsafe {
        STASH = *p;
    }
}

fn main() {
    let r;
    {
        let x = 1;
        r = &x;
        assert_eq!(*r, 1);
    }

    static  WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT)
    
}