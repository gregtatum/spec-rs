#[cfg(test)]
mod test {
    use std::borrow::Cow;

    fn multiply_vector<'a>(input: &'a Cow<[i32]>, value: i32) -> Cow<'a, [i32]> {
        if value == 1 {
            return Cow::Borrowed(input);
        }

        let mut owned_input = input.to_owned();
        for i in 0..input.len() {
            owned_input.to_mut()[i] *= value;
        }
        owned_input
    }

    #[test]
    fn test_copy_on_write_pointers() {
        // Start by creating a "copy on write" vector.
        let cow_pointer = Cow::from(vec![0, 1, 2]);
        assert_eq!(cow_pointer.to_vec(), vec![0, 1, 2]);

        // The multiply_vector function is optimized to return the original
        // data if possible.
        let pointer_unmodified = multiply_vector(&cow_pointer, 1);
        assert_eq!(*pointer_unmodified, vec![0, 1, 2]);
        assert_eq!(cow_pointer.as_ptr(), pointer_unmodified.as_ptr());

        // Multiplying by a different number computes a new array.
        let pointer_modified = multiply_vector(&cow_pointer, 5);
        assert_eq!(*pointer_modified, vec![0, 5, 10]);
        assert_ne!(cow_pointer.as_ptr(), pointer_modified.as_ptr());
    }

    // https://doc.rust-lang.org/book/ch15-02-deref.html
    #[test]
    fn test_reference_deref() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
        // This also works:
        assert_eq!(&5, y);

        // This does not:
        // assert_eq!(5, y);
    }

    #[test]
    fn test_pointer_deref() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(Box::new(5), y);
    }

    // MyBox is a custom smart pointer.
    #[derive(PartialEq, Debug)]
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // Implementing Deref allows this to be used like a pointer.
    impl<T> std::ops::Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[test]
    fn test_custom_smart_pointer() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(MyBox::new(5), y);
    }
}
