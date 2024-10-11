use crate::elements::{Element, InnersGetter, Return};
impl InnersGetter for Return {
    fn get_inners(&self) -> Vec<&Element> {
        self.output
            .as_ref()
            .map(|el| vec![el.as_ref()])
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod processing {
    use crate::{inf::Value, test_block};

    test_block!(
        returning,
        r#"
            return 5;
        "#,
        5isize
    );

    test_block!(
        returning_from_block,
        r#"
            $a = 13;
            return 5;
            13;
        "#,
        5isize
    );

    test_block!(
        returning_from_nested_block,
        r#"
            $a = 13;
            if $a == 13 {
                return 5;
            } else {
                false;
            };
            true;
        "#,
        5isize
    );

    test_block!(
        returning_from_mt_nested_block,
        r#"
            $a = 13;
            if $a == 13 {
                if $a == 13 {
                    return 5;
                } else {
                    false;
                };
            } else {
                false;
            };
            true;
        "#,
        5isize
    );

    test_block!(
        returning_from_loop,
        r#"
            for $n in 0..10 {
                if $n == 5 {
                    return 5;
                };
            };
            true;
        "#,
        5isize
    );

    test_block!(
        returning_from_loop_error,
        r#"
            for $n in 0..10 {
                if $n == 5 {
                    return Error "test";
                };
            };
            true;
        "#,
        Value::Error(String::from("test"))
    );
}
