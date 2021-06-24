use std::ops::Deref;

struct WhatThe<F>(F);

impl<F> WhatThe<F> {
    fn new(x: F) -> WhatThe<F> {
        WhatThe(x)
    }
}

impl<F> Deref for WhatThe<F> {
    type Target = F;

    fn deref(&self) -> &F {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let a = WhatThe::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(5, y); -> 요건 integer이랑 &integer이랑 비교하는 꼴이여서 안 됨.
    assert_eq!(5, *z);
    // assert_eq!(5, z); -> 요건 integer이랑 Box<integer>이랑 비교하는 꼴... 역시 안 됨.
    assert_eq!(5, *a);
}
