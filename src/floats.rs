#[cfg(test)]
mod float {

    fn to_u32(number: f32) -> u32 {
        unsafe { std::mem::transmute::<f32, u32>(number) }
    }

    fn assert(binary: u32, float: f32) {
        let float_u32 = to_u32(float);
        assert_eq!(
            binary, float_u32,
            "binary expected: {:032b}\nfloat value: {:032b}",
            binary, float_u32
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_floats() {
        //           S EXPONENT_ FRACTION______________

        assert(0b0_111111111_0000000000000000000000, f32::NAN);
        assert(0b1_111111111_0000000000000000000000, -f32::NAN);
        assert(0b0_111111110_0000000000000000000000, f32::INFINITY);
        assert(0b1_111111110_0000000000000000000000, f32::NEG_INFINITY);
        assert(0b1_111111110_0000000000000000000000, -f32::INFINITY);
        assert(0b0_011111100_0000000000000000000000, 0.5);
        assert(0b0_011111110_0000000000000000000000, 1.0);
        assert(0b0_100000000_0000000000000000000000, 2.0);
        assert(0b0_100000100_1000000000000000000000, 10.0);
    }
}
