use crate::{
    elements::{Comparing, Element, ElementRef, InnersGetter},
    test_reading_el_by_el,
};

impl InnersGetter for Comparing {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.left.as_ref(), self.right.as_ref()]
    }
}

// test_reading_el_by_el!(
//     reading,
//     &include_str!("../../../tests/reading/comparing.sibs"),
//     ElementRef::Comparing,
//     2
// );
// "!" isn't included into token

#[cfg(test)]
mod reading {
    use crate::{
        elements::{Comparing, Element, ElementRef, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let content = include_str!("../../../tests/reading/comparing.sibs")
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut count = 0;
        for str in content.iter() {
            count += read_string!(
                &Configuration::logs(false),
                str,
                |reader: &mut Reader, src: &mut Sources| {
                    let entity =
                        src.report_err_if(Element::include(reader, &[ElementRef::Comparing]))?;
                    assert!(entity.is_some(), "Line: {}", count + 1);
                    let entity = entity.unwrap();
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity}")),
                        "Line: {}",
                        count + 1
                    );
                    assert!(reader.rest().trim().is_empty(), "Line: {}", count + 1);
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, content.len());
    }

    #[tokio::test]
    async fn tokens() {
        let content = include_str!("../../../tests/reading/comparing.sibs")
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut count = 0;
        for str in content.iter() {
            count += read_string!(
                &Configuration::logs(false),
                str,
                |reader: &mut Reader, src: &mut Sources| {
                    let entity =
                        src.report_err_if(Element::include(reader, &[ElementRef::Comparing]))?;
                    assert!(entity.is_some(), "Line: {}", count + 1);
                    let entity = entity.unwrap();
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        reader.get_fragment(&entity.token())?.lined
                    );
                    if let Element::Comparing(entity, _) = entity {
                        assert_eq!(
                            trim_semicolon(&trim_carets(&entity.left.to_string())),
                            trim_semicolon(&trim_carets(&format!(
                                "{}{}",
                                if entity.left.get_metadata().inverting {
                                    chars::EXCLAMATION.to_string()
                                } else {
                                    String::new()
                                },
                                reader.get_fragment(&entity.left.token())?.lined
                            ))),
                        );
                        assert_eq!(
                            trim_semicolon(&trim_carets(&entity.right.to_string())),
                            trim_semicolon(&trim_carets(&format!(
                                "{}{}",
                                if entity.right.get_metadata().inverting {
                                    chars::EXCLAMATION.to_string()
                                } else {
                                    String::new()
                                },
                                reader.get_fragment(&entity.right.token())?.lined
                            ))),
                        );
                    } else {
                        panic!("Fail to extract Element::Comparing")
                    }
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, content.len());
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../../tests/error/comparing.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    let cmp = Comparing::dissect(reader);
                    assert!(cmp.is_err() || matches!(cmp, Ok(None)));
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }
}
