#[cfg(test)]
mod tests {
    use crate::{eval_to, evaluators::eval_to_string};
    #[test]
    fn eval_to_string_test() {
        assert_eq!(eval_to_string("1+2"), Some("3".to_string()));
    }

    #[test]
    #[should_panic]
    fn eval_to_string_nil() {
        eval_to_string("nil").unwrap();
    }

    #[test]
    fn eval_to_i32_test() {
        assert_eq!(eval_to!(i32,"500+500").unwrap(), 1000);
    }

    #[test]
    #[should_panic]
    fn eval_to_u8_test() {
        eval_to!(u8,"500+500").unwrap();
    }
}
