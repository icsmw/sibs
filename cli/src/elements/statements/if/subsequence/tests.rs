use crate::{
    elements::{Element, ElementRef, IfSubsequence, InnersGetter},
    test_reading_el_by_el,
};

impl InnersGetter for IfSubsequence {
    fn get_inners(&self) -> Vec<&Element> {
        self.subsequence.iter().collect()
    }
}

// test_reading_el_by_el!(
//     reading,
//     &include_str!("../../../../tests/reading/subsequence.sibs"),
//     ElementRef::IfSubsequence,
//     7
// );
//
// Needs runner one line by one line

#[cfg(test)]
mod reading {
    use crate::{
        elements::{IfSubsequence, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let content = include_str!("../../../../tests/reading/subsequence.sibs")
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut count = 0;
        for str in content.iter() {
            count += read_string!(
                &Configuration::logs(false),
                str,
                |reader: &mut Reader, src: &mut Sources| {
                    let entity = src.report_err_if(IfSubsequence::dissect(reader))?;
                    assert!(entity.is_some(), "Line: {}", count + 1);
                    let entity = entity.unwrap();
                    assert_eq!(
                        trim_carets(str),
                        trim_carets(&format!("{entity}")),
                        "Line: {}",
                        count + 1
                    );
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, content.len());
    }

    #[tokio::test]
    async fn tokens() {
        let content = include_str!("../../../../tests/reading/subsequence.sibs")
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for (count, str) in content.iter().enumerate() {
            read_string!(
                &Configuration::logs(false),
                str,
                |reader: &mut Reader, src: &mut Sources| {
                    let entity = src.report_err_if(IfSubsequence::dissect(reader))?;
                    assert!(entity.is_some(), "Line: {}", count + 1);
                    let entity = entity.unwrap();
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        trim_carets(&reader.get_fragment(&entity.token)?.lined),
                        "Line: {}",
                        count + 1
                    );
                    for el in entity.subsequence.iter() {
                        assert_eq!(
                            trim_carets(&format!("{el}")),
                            trim_carets(&reader.get_fragment(&el.token())?.lined),
                            "Line: {}",
                            count + 1
                        );
                    }
                    Ok::<(), LinkedErr<E>>(())
                }
            );
        }
    }
}
