use crate::elements::{Comment, Element, InnersGetter};

impl InnersGetter for Comment {
    fn get_inners(&self) -> Vec<&Element> {
        Vec::new()
    }
}

use crate::{
    elements::Task,
    error::LinkedErr,
    inf::Configuration,
    read_string,
    reader::{chars, Dissect, Reader, Sources, E},
};

#[tokio::test]
async fn reading() {
    read_string!(
        &Configuration::logs(false),
        &include_str!("../../tests/reading/comments.sibs"),
        |reader: &mut Reader, src: &mut Sources| {
            while let Some(entity) = src.report_err_if(Task::dissect(reader))? {
                let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                if let Element::Block(block, _) = entity.block.as_ref() {
                    for el in block.elements.iter() {
                        assert_eq!(el.get_metadata().comments().len(), 2);
                    }
                } else {
                    panic!("Fail to get task's block");
                }
            }
            assert!(reader.rest().trim().is_empty());
            Ok::<(), LinkedErr<E>>(())
        }
    );
}
