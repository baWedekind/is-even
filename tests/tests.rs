#[cfg(test)]
mod tests {
    use is_even::IsEven;

    #[test]
    fn number_i8_is_even() {

        let mut _i : i8 = -1;
        assert!(!_i.is_even());

        _i=0;
        assert!(_i.is_even());

        _i=1;
        assert!(!_i.is_even());

        _i=2;
        assert!(_i.is_even());
    }

    #[test]
    fn number_u8_is_even() {

        let mut _i : u8 = 0;
        assert!(_i.is_even());

        _i=1;
        assert!(!_i.is_even());

        _i=2;
        assert!(_i.is_even());

        _i=3;
        assert!(!_i.is_even());
    }

    #[test]
    fn number_i16_is_even() {

        let mut _i : i16 = -1;
        assert!(!_i.is_even());

        _i=0;
        assert!(_i.is_even());

        _i=1;
        assert!(!_i.is_even());

        _i=2;
        assert!(_i.is_even());
    }

    #[test]
    fn number_u16_is_even() {

        let mut _i : u16 = 0;
        assert!(_i.is_even());

        _i=1;
        assert!(!_i.is_even());

        _i=2;
        assert!(_i.is_even());

        _i=3;
        assert!(!_i.is_even());
    }

    #[test]
    fn number_i32_is_even() {

        let mut _i : i32 = -2;
        assert!(_i.is_even());

        _i=-1;
        assert!(!_i.is_even());

        _i=0;
        assert!(_i.is_even());

        _i=1;
        assert!(!_i.is_even());

        _i=2;
        assert!(_i.is_even());

        _i=3;
        assert!(!_i.is_even());

        _i=10_000_000;
        assert!(_i.is_even());
    }

    #[test]
    fn number_u32_is_even() {

        let mut _i : u32 = 0;
        assert!(_i.is_even());

        _i=1;
        assert!(!_i.is_even());

        _i=2;
        assert!(_i.is_even());

        _i=3;
        assert!(!_i.is_even());
    }

    #[test]
    fn number_i64_is_even() {

        let mut _i : i64 = -2;
        assert!(_i.is_even());

        _i=-1;
        assert!(!_i.is_even());

        _i=0;
        assert!(_i.is_even());

        _i=1;
        assert!(!_i.is_even());

        _i=2;
        assert!(_i.is_even());

        _i=3;
        assert!(!_i.is_even());

        _i=10_000_000;
        assert!(_i.is_even());
    }

    #[test]
    fn number_u64_is_even() {

        let mut _i : u64 = 0;
        assert!(_i.is_even());

        _i=1;
        assert!(!_i.is_even());

        _i=2;
        assert!(_i.is_even());

        _i=3;
        assert!(!_i.is_even());
    }
}