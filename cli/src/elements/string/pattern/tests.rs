use crate::elements::{Element, InnersGetter, PatternString};

impl InnersGetter for PatternString {
    fn get_inners(&self) -> Vec<&Element> {
        self.elements.iter().collect()
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{PatternString, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../../tests/reading/pattern_string.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let origins = include_str!("../../../tests/reading/pattern_string.sibs")
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let mut count = 0;
                while let Some(entity) = src.report_err_if(PatternString::dissect(reader))? {
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&entity.to_string()),
                    );
                    assert_eq!(
                        origins[count],
                        trim_carets(&entity.to_string()),
                        "line {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, 96);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../../tests/reading/pattern_string.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(PatternString::dissect(reader))? {
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
                assert_eq!(count, 96);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}
