extern crate zalgo;
use zalgo::{
    RandOrStatic,
    Zalgoifier,
};

#[test]
fn zalgoify() {
    let ret = zalgo::zalgoify("Hello World!");
    println!("{}", ret);
    assert!(!ret.is_empty());
}

#[test]
fn zalgoify_struct() {
    let mut zalgoifier = Zalgoifier::new();
    zalgoifier.set_up(RandOrStatic::Rand(100));
    zalgoifier.set_down(RandOrStatic::Static(0));
    zalgoifier.set_mid(RandOrStatic::Static(0));

    let ret = zalgoifier.zalgoify("Hello World!");

    println!("{}", ret);
    assert!(!ret.is_empty());
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
