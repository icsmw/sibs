use crate::elements::{Element, InnersGetter, VariableVariants};
impl InnersGetter for VariableVariants {
    fn get_inners(&self) -> Vec<&Element> {
        Vec::new()
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::VariableVariants,
        error::LinkedErr,
        inf::Configuration,
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let samples = include_str!("../../../tests/reading/variants.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, src: &mut Sources| {
                    assert!(src.report_err_if(VariableVariants::dissect(reader)).is_ok());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../../tests/error/variants.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    let result = VariableVariants::dissect(reader);
                    assert!(result.is_err());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }
}
