use crate::*;

#[derive(Debug)]
pub enum CompletionSuggestion {
    Variable(String),
    Function(String),
}

enum Filter {
    Variables,
    Functions,
    All,
}
pub struct Completion<'a> {
    locator: LocationIterator<'a>,
    current: Token,
    scx: &'a SemanticCx,
}

impl<'a> Completion<'a> {
    pub fn new(current: Token, locator: LocationIterator<'a>, scx: &'a SemanticCx) -> Self {
        Self {
            locator,
            current,
            scx,
        }
    }
    fn find_task_uuid(&mut self) -> Option<Uuid> {
        while let Some(prev) = self.locator.prev_node() {
            if let Node::Root(Root::Task(node)) = prev.node.get_node() {
                return Some(node.uuid.clone());
            }
        }
        None
    }
    fn find_block_uuid(&mut self) -> Option<Uuid> {
        while let Some(prev) = self.locator.prev_node() {
            if let Node::Statement(Statement::Block(node)) = prev.node.get_node() {
                return Some(node.uuid.clone());
            }
        }
        None
    }
    pub fn suggest(&mut self) -> Option<Vec<CompletionSuggestion>> {
        println!(">>>>>>>>>>>>>>>>>>>> 0000 1");
        let Some(task) = self.find_task_uuid() else {
            return None;
        };
        println!(">>>>>>>>>>>>>>>>>>>> 0000 2");
        self.locator.drop();
        let Some(block) = self.find_block_uuid() else {
            return None;
        };
        println!(">>>>>>>>>>>>>>>>>>>> 0000 3");
        let Some(scope) = self.scx.tys.get_scope(&task) else {
            return None;
        };
        println!(
            ">>>>>>>>>>>>>>>> CURRENT: {} ({})",
            self.current.id(),
            self.current
        );
        self.locator.drop();
        // println!("{:?}", self.locator.parser.tokens);
        while let Some(prev) = self.locator.prev_token() {
            println!(">>>>>>>>>>>>>>>>: {} ({})", prev.token.id(), prev.token);
        }
        return None;
        // let filter = loop {
        //     let Some(prev) = self.locator.prev_token() else {
        //         break None;
        //     };
        //     println!(">>>>>>>>>>>>>>>>: {} ({})", prev.token.id(), prev.token);
        //     match prev.token.kind {
        //         Kind::Dot => match self.current.id() {
        //             KindId::Identifier => {
        //                 break Some(Filter::Functions);
        //                 // let ty = scope.find(prev.token.to_string(), &[block]);
        //                 // println!(">>>>>>>>>>>>>>>>: {ty:?}");
        //             }
        //             _ => {}
        //         },
        //         Kind::Keyword(Keyword::Let) => match self.current.id() {
        //             KindId::Identifier => {
        //                 break None;
        //             }
        //             _ => {}
        //         },
        //         Kind::Keyword(Keyword::If) => match self.current.id() {
        //             KindId::Identifier => {
        //                 break Some(Filter::All);
        //                 // let variables = scope.get_all_variables(&[block]);
        //                 return None;
        //             }
        //             _ => {}
        //         },
        //         Kind::LeftParen => {}
        //         Kind::Whitespace(..) => {}
        //         Kind::LeftBrace => {}
        //         Kind::Colon => {}
        //         _ => {}
        //     };
        // }?;
        // match filter {
        //     Filter::Variables => {
        //         let variables = scope.get_all_variables(&[block])?;
        //         Some(
        //             variables
        //                 .iter()
        //                 .map(|(name, _)| CompletionSuggestion::Variable(name.to_string()))
        //                 .collect(),
        //         )
        //     }
        //     Filter::Functions => None,
        //     Filter::All => {
        //         let mut suggestions = Vec::new();
        //         if let Some(variables) = scope.get_all_variables(&[block]) {
        //             suggestions.extend(
        //                 variables
        //                     .iter()
        //                     .map(|(name, _)| CompletionSuggestion::Variable(name.to_string()))
        //                     .collect::<Vec<CompletionSuggestion>>(),
        //             );
        //         }
        //         if suggestions.is_empty() {
        //             None
        //         } else {
        //             Some(suggestions)
        //         }
        //     }
        // }
    }
}

#[test]
fn test() {
    let mut driver = Driver::unbound(
        r#"component component_a() {
    task task_a() {
        let variable_a = 1;
        let variable_b = 1;
        let variable_c = variable_a + variable_b;
        let varibale_d = if varibale_a > 1 {
            true;
        } else {
            false;
        }
        variable.fns::sum(a);
    }
};
"#,
        true,
    );
    driver.read().unwrap();
    let mut completion = driver.completion(190, None).unwrap();
    println!("Suggestions: {:?}", completion.suggest());
}
