// run-rustfix

#![warn(clippy::small_underscore_scope)]

struct NewType(u8);

#[allow(dead_code)]
struct Foo {
    x: u8,
    y: u8,
}

fn main() {
    let n = NewType(24);
    let NewType(_) = n;
    let NewType(..) = n;

    let t = (4, 5);
    let (_, _) = t;
    let (..) = t;

    let t2 = (6, (7, 8));
    let (_x, (_, _)) = t2;

    let f = Foo { x: 3, y: 7 };
    let Foo { .. } = f;
}
