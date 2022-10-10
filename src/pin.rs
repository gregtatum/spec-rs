#![cfg(test)]
#![allow(unused_imports)]

use std::marker::PhantomPinned;
use std::mem;
use std::pin::Pin;

#[derive(Debug)]
struct SelfReferential {
    string: String,
    string_ref: *const String,
}

impl SelfReferential {
    fn new(txt: &str) -> Self {
        SelfReferential {
            string: String::from(txt),
            string_ref: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.string;
        self.string_ref = self_ref;
    }

    fn get_string(&self) -> &str {
        &self.string
    }

    unsafe fn get_string_from_ref(&self) -> &String {
        assert!(!self.string_ref.is_null(), "init must be called first");
        &*(self.string_ref)
    }
}

#[test]
fn test_mem_swap() {
    let mut x = 5;
    let mut y = 42;

    assert_eq!(x, 5);
    assert_eq!(y, 42);
    mem::swap(&mut x, &mut y);
    assert_eq!(x, 42);
    assert_eq!(y, 5);
}

#[test]
fn test_pins() {
    let mut a = SelfReferential::new("dog");
    a.init();

    let mut b = SelfReferential::new("cat");
    b.init();

    assert_eq!(a.get_string(), "dog");
    assert_eq!(unsafe { a.get_string_from_ref() }, "dog");
    assert_eq!(b.get_string(), "cat");
    assert_eq!(unsafe { b.get_string_from_ref() }, "cat");

    mem::swap(&mut a, &mut b);

    assert_eq!(
        a.get_string(),
        "cat",
        "The stack-allocated memory is swapped"
    );
    assert_eq!(
        unsafe { a.get_string_from_ref() },
        "dog",
        "The pointer now points the value of b"
    );
    assert_eq!(
        b.get_string(),
        "dog",
        "The stack-allocated memory is swapped"
    );
    assert_eq!(
        unsafe { b.get_string_from_ref() },
        "cat",
        "The pointer now points the value of b"
    );

    a.string = "Value is changed".into();
    assert_eq!(
        unsafe { b.get_string_from_ref() },
        "Value is changed",
        "a's value is reflected in b"
    );
}

// TODO - I never finished what I was experimenting with here.

// #[derive(Debug)]
// struct PinnedSelfReferential {
//     string: String,
//     string_ref: *const String,
//     _marker: PhantomPinned,
// }

// struct CodeRefs<'s> {
//     pub code: Pin<String>,
//     pub header: Option<&'s str>,
//     _marker: PhantomPinned,
// }

// impl<'s> CodeRefs<'s> {
//     fn new(code: String) -> Pin<Box<CodeRefs<'s>>> {
//         Box::pin(CodeRefs {
//             code: Pin::new(code),
//             header: None,
//             _marker: PhantomPinned,
//         })
//     }

//     fn init(mut self: Pin<&'s mut Self>) {
//         if let Some(index) = self.code.find("#header") {
//             let str = &self.as_ref().code.as_mut_ref().[index..index + 7];
//             self.as_mut().header = Some(str);
//         };
//     }
// }

// #[test]
// fn code_reference() {
//     let codeRefs = Pin::new(Box::new(CodeRefs::new("#header\nCode goes heres".into())));
//     assert_eq!(codeRefs.header.unwrap(), "#header");
// }
