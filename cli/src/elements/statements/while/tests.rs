use crate::elements::{Element, InnersGetter, While};
impl InnersGetter for While {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.block.as_ref(), self.condition.as_ref()]
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{TokenGetter, While},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../../tests/reading/while.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(While::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};"))
                    );
                    count += 1;
                }
                assert_eq!(count, 1);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../../tests/reading/while.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(While::dissect(reader))? {
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
                assert_eq!(count, 1);
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
        simple,
        r#"
            $n = 0;
            while $n < 10 {
                $n += 1;
            };
            $n;
        "#,
        10isize
    );

    test_block!(
        with_break,
        r#"
            $n = 0;
            while $n < 10 {
                $n += 1;
                if $n == 5 {
                    break;
                };
            };
            $n;
        "#,
        5isize
    );
}
