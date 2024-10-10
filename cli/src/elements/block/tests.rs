use crate::elements::{Block, Element, InnersGetter};

#[cfg(test)]
impl InnersGetter for Block {
    fn get_inners(&self) -> Vec<&Element> {
        self.elements.iter().collect()
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Block,
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &format!(
                "{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}",
                include_str!("../../tests/reading/if.sibs"),
                include_str!("../../tests/reading/variable_assignation.sibs"),
                include_str!("../../tests/reading/function.sibs"),
                include_str!("../../tests/reading/optional.sibs"),
                include_str!("../../tests/reading/each.sibs"),
                include_str!("../../tests/reading/refs.sibs")
            ),
            |reader: &mut Reader, src: &mut Sources| {
                while let Some(entity) = src.report_err_if(Block::dissect(reader))? {
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&entity.to_string())
                    );
                }
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &format!(
                "{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}",
                include_str!("../../tests/reading/if.sibs"),
                include_str!("../../tests/reading/variable_assignation.sibs"),
                include_str!("../../tests/reading/function.sibs"),
                include_str!("../../tests/reading/optional.sibs"),
                include_str!("../../tests/reading/each.sibs"),
                include_str!("../../tests/reading/refs.sibs")
            ),
            |reader: &mut Reader, src: &mut Sources| {
                while let Some(entity) = src.report_err_if(Block::dissect(reader))? {
                    assert_eq!(
                        trim_carets(&entity.to_string()),
                        reader.get_fragment(&entity.token)?.lined
                    );
                }
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}
