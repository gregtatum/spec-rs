#![cfg(test)]

use std::rc::Rc;

/// A bad vector library for demonstration purposes.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2(usize, usize);

#[test]
fn test_rc() {
    let a1 = Vec2(2, 4);
    let a2 = Vec2(2, 4);
    let b = Vec2(0, 0);

    assert_eq!(a1, a2, "The base vectors evaluate as equal.");
    assert_ne!(a1, b, "The base vectors can be unequal.");

    let a1 = Rc::new(Vec2(2, 4));
    let a2 = a1.clone();
    let a3 = Rc::new(Vec2(2, 4));
    let b = Rc::new(Vec2(0, 0));

    assert_eq!(a1, a2, "Equality points to the underlying data");
    assert_eq!(a1, a3, "Equality points to the underlying data");
    assert_eq!(a2, a3, "Equality points to the underlying data");
    assert_ne!(a1, b, "Equality points to the underlying data");

    assert!(
        Rc::ptr_eq(&a1, &a2),
        "The pointers can also be checked as the same"
    );
    assert!(
        !Rc::ptr_eq(&a1, &a3),
        "The pointers can also be checked as the same"
    );
}
