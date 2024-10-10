use crate::elements::{Command, Element, InnersGetter};
impl InnersGetter for Command {
    fn get_inners(&self) -> Vec<&Element> {
        self.elements.iter().collect()
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{Command, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../../tests/reading/command.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let origins = include_str!("../../../tests/reading/command.sibs")
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Command::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};")),
                        "line {}",
                        count + 1
                    );
                    assert_eq!(
                        origins[count],
                        trim_carets(&format!("{entity};")),
                        "line {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, 130);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../../tests/reading/command.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Command::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&entity.to_string()),
                        reader.get_fragment(&entity.token)?.content
                    );
                    for el in entity.elements.iter() {
                        assert_eq!(
                            el.to_string().replace('\n', ""),
                            reader.get_fragment(&el.token())?.content
                        );
                    }
                    count += 1;
                }
                assert_eq!(count, 130);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

use crate::test_block;

test_block!(
    success,
    r#"
            $status = `./target/debug/exit 0 100 200 10`;
            $status.is_success();
        "#,
    true
);

test_block!(
    fail,
    r#"
            $status = `./target/debug/exit 1 100 200 10`;
            $status.is_fail();
        "#,
    true
);
