#[macro_use]
extern crate withers_derive;

#[derive(Default, Withers, Debug, Eq, PartialEq)]
struct Foo {
    foo: u32,
    bar: Bar,
    baz: Option<bool>
}

#[derive(Default, Debug, Eq, PartialEq)]
struct Bar(u32, bool);

#[test]
fn internal() {
    let foo = Foo::default().with_foo(234).with_bar(Bar(12, false));

    assert_eq!(
        foo,
        Foo {
            foo: 234,
            bar: Bar(12, false),
            baz: None
        }
    );
}
