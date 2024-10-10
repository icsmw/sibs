use crate::elements::{Element, InnersGetter, VariableName};

impl InnersGetter for VariableName {
    fn get_inners(&self) -> Vec<&Element> {
        Vec::new()
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::VariableName,
        error::LinkedErr,
        inf::Configuration,
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let samples = include_str!("../../../tests/reading/variable_name.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, src: &mut Sources| {
                    src.report_err_if(VariableName::dissect(reader))?.unwrap();
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }

    #[tokio::test]
    async fn tokens() {
        let samples = include_str!("../../../tests/reading/variable_name.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, src: &mut Sources| {
                    let variable_name = src.report_err_if(VariableName::dissect(reader))?.unwrap();
                    let fragment = reader.get_fragment(&reader.token()?.id)?.content;
                    assert_eq!(format!("${}", variable_name.name), fragment);
                    assert_eq!(fragment, variable_name.to_string());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../../tests/error/variable_name.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    assert!(VariableName::dissect(reader).is_err());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }
}
