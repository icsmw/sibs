mod search;
mod suggestions;

use crate::*;
use suggestions::*;

enum Filter {
    Variables,
    Functions,
    All,
}
#[derive(Debug)]
enum Ownership {
    Task(Uuid),
    /// * `Uuid` - Uuid of function declaration node
    /// * `Option<Uuid>` - Uuid of task, if function is declared in context of task
    Function(Uuid, Option<Uuid>),
}

#[derive(Debug)]
struct LocationData {
    ownership: Ownership,
    blocks: Vec<Uuid>,
    /// Location in modules
    mods: Vec<String>,
    token: Token,
}

impl LocationData {
    pub fn get_scx_uuid(&self) -> &Uuid {
        match &self.ownership {
            Ownership::Task(uuid) => uuid,
            Ownership::Function(_, Some(uuid)) => uuid,
            Ownership::Function(uuid, None) => uuid,
        }
    }
}

pub struct Completion<'a> {
    locator: LocationIterator<'a>,
    scx: &'a SemanticCx,
    fragment: String,
}

impl<'a> Completion<'a> {
    pub fn new(locator: LocationIterator<'a>, scx: &'a SemanticCx, fragment: String) -> Self {
        Self {
            locator,
            scx,
            fragment,
        }
    }
    fn get_location_data(&mut self) -> Result<Option<LocationData>, E> {
        let Some(current) = self.locator.prev_token() else {
            return Ok(None);
        };
        let token = current.token.clone();
        drop(current);
        let tree = self.locator.get_ownership_tree(token.pos.from);
        let mut blocks = Vec::new();
        let mut mods = Vec::new();
        let mut ownership = None;
        for node in tree.iter().rev().into_iter() {
            match node.get_node() {
                Node::Statement(Statement::Block(..)) => {
                    blocks.push(*node.uuid());
                }
                Node::Declaration(Declaration::FunctionDeclaration(..)) => {
                    if ownership.is_none() {
                        ownership = Some(Ownership::Function(*node.uuid(), None));
                    } else {
                        return Err(E::TaskInsideFuncDeclaration(*node.uuid()));
                    }
                }
                Node::Root(Root::Module(module)) => {
                    if let Some(name) = module.get_name() {
                        mods.insert(0, name.to_owned());
                    }
                }
                Node::Root(Root::Task(..)) => {
                    if let Some(Ownership::Function(uuid, task)) = ownership {
                        if task.is_some() {
                            return Err(E::NestedTasks(*node.uuid()));
                        }
                        ownership = Some(Ownership::Function(uuid, Some(*node.uuid())));
                    } else {
                        ownership = Some(Ownership::Task(*node.uuid()));
                    }
                    break;
                }
                _ => {}
            }
        }
        self.locator.drop();
        if let Some(ownership) = ownership {
            Ok(Some(LocationData {
                ownership,
                blocks,
                mods,
                token,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn suggest(&mut self) -> Result<Option<Vec<CompletionSuggestion>>, E> {
        let Some(info) = self.get_location_data()? else {
            return Ok(None);
        };
        let Some(scope) = self.scx.tys.get_scope(&*info.get_scx_uuid()) else {
            return Ok(None);
        };
        let filter = loop {
            let Some(prev) = self.locator.prev_token() else {
                break None;
            };
            let point_kind = info.token.id();
            let prev_kind = prev.token.kind.clone();
            drop(prev);
            match prev_kind {
                Kind::Dot => match point_kind {
                    KindId::Identifier => {
                        break Some(Filter::Functions);
                    }
                    _ => {}
                },
                Kind::Keyword(Keyword::Let) => match point_kind {
                    KindId::Identifier => {
                        break None;
                    }
                    _ => {}
                },
                Kind::Keyword(Keyword::If) => match point_kind {
                    KindId::Identifier => {
                        break Some(Filter::All);
                    }
                    _ => {}
                },
                Kind::LeftParen => {}
                Kind::Whitespace(..) => {}
                Kind::LeftBrace => {}
                Kind::Colon => {}
                Kind::Comma => {}
                Kind::Semicolon => {
                    break Some(Filter::All);
                }
                Kind::Equals => {
                    break Some(Filter::All);
                }
                _ => {}
            };
        };
        let Some(filter) = filter else {
            return Ok(None);
        };
        let suggestion = match filter {
            Filter::Variables => Some(vars::collect(&scope, &info.blocks, &self.fragment, None)),
            Filter::Functions => Some(funcs::collect(
                &self.scx.fns,
                &info.mods.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                &self.fragment,
                None,
            )),
            Filter::All => {
                let mut suggestions = vars::collect(&scope, &info.blocks, &self.fragment, None);
                suggestions.extend(funcs::collect(
                    &self.scx.fns,
                    &info.mods.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                    &self.fragment,
                    None,
                ));
                if suggestions.is_empty() {
                    None
                } else {
                    Some(suggestions)
                }
            }
        };
        Ok(suggestion)
    }
}

#[test]
fn test() {
    let mut driver = Driver::unbound(
        r#"
mod aaa {
    fn sum(a: num, b: num) {
        a + b;
    };
};      
component component_a() {
    task task_a() {
        let eeevariaeee = 2;
        sumvariable.sum;
        let variable_a = 1;
        let variable_b = 1;
        let variable_c = variable_a + variable_b;
        let varibale_d = if eeevariaeee > 1 {
            let sub_var = env;
            variable_a;
        } else {
            variable_b;
        }
        variable.fns::sum(a);
    }
};
"#,
        true,
    );
    driver.read().unwrap();
    driver.print_errs().unwrap();
    let mut completion = driver.completion(168, None).unwrap();
    println!("Suggestions: {:?}", completion.suggest());
}
