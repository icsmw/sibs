use crate::elements::{Element, For, InnersGetter};
impl InnersGetter for For {
    fn get_inners(&self) -> Vec<&Element> {
        vec![
            self.block.as_ref(),
            self.index.as_ref(),
            self.target.as_ref(),
        ]
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{For, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../../tests/reading/for.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(For::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};"))
                    );
                    count += 1;
                }
                assert_eq!(count, 5);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../../tests/reading/for.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(For::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        trim_carets(&reader.get_fragment(&entity.token)?.lined),
                    );
                    assert_eq!(
                        trim_carets(&entity.block.to_string()),
                        trim_carets(&reader.get_fragment(&entity.block.token())?.lined),
                    );
                    count += 1;
                }
                assert_eq!(count, 5);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod processing {
    use crate::test_block;

    test_block!(
        no_steps,
        r#"
            for $n in 0..0 {
                print($n);
            };
            $n;
        "#,
        0isize
    );

    test_block!(
        increase_index,
        r#"
            for $n in 0..10 {
                print($n);
            };
            true;
        "#,
        true
    );

    test_block!(
        reduce_index,
        r#"
            for $n in 10..0 {
                print($n);
            };
            true;
        "#,
        true
    );

    test_block!(
        increase_index_break,
        r#"
            for $n in 0..10 {
                if $n == 5 {
                    break;
                };
                print($n);
            };
            if $n == 5 {
                true;
            } else {
                false;
            };
        "#,
        true
    );

    test_block!(
        reduce_index_and_incrementer,
        r#"
            $i = 10;
            for $n in 10..0 {
                print($n);
                $i -= 1;
            };
            if $i == 0 {
                true;
            } else {
                false;
            };
        "#,
        true
    );

    test_block!(
        increase_index_and_incrementer,
        r#"
            $i = 0;
            for $n in 0..10 {
                print($n);
                $i += 1;
            };
            if $i == 10 {
                true;
            } else {
                false;
            };
        "#,
        true
    );

    test_block!(
        reduce_index_break,
        r#"
            for $n in 10..0 {
                if $n == 5 {
                    break;
                };
                print($n);
            };
            if $n == 5 {
                true;
            } else {
                false;
            };
        "#,
        true
    );

    test_block!(
        iteration,
        r#"
            for $el in ("one", "two", "three") {
                print($el);
            };
            true;
        "#,
        true
    );

    test_block!(
        iteration_from_var,
        r#"
            $els = ("one", "two", "three");
            for $el in $els {
                print($el);
            };
            true;
        "#,
        true
    );
}
