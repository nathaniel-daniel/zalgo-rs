extern crate zalgo;
use zalgo::{
    RandOrStatic,
    Zalgoifier,
};

#[test]
fn zalgoify() {
    println!("{}", zalgo::zalgoify("Hello World!"));
}

#[test]
fn zalgoify_struct() {
    let mut zalgoifier = Zalgoifier::new();
    zalgoifier.set_up(RandOrStatic::Rand(100));
    zalgoifier.set_down(RandOrStatic::Static(0));
    zalgoifier.set_mid(RandOrStatic::Static(0));

    println!("{}", zalgoifier.zalgoify("Hello World!"));
}

#[test]
fn no_zalgo_op() {
    let mut zalgoifier = Zalgoifier::new();
    zalgoifier.set_up(RandOrStatic::Static(0));
    zalgoifier.set_down(RandOrStatic::Static(0));
    zalgoifier.set_mid(RandOrStatic::Static(0));

    let test = "Hello World!";
    assert_eq!(test, zalgoifier.zalgoify("Hello World!"));
}
