use crate::*;

#[derive(Debug)]
pub enum Cursor {
    /// Uses if token doesn't bellow to namespace reference
    /// * `Token` - cursor token
    /// * `isize` - `idx` of token in tokens flow
    Token(Token, isize),
    /// Uses if token is a part of namespace reference (fs::create)
    /// * `String` - parts of namespace reference; for example `fs::create`
    /// * `isize` - is `idx` of token in tokens flow (from left, meaning the first one)
    /// * `TextPosition` - `from` position of the first identifier
    /// * `TextPosition` - `to` position of the last identifier
    Path(String, isize, TextPosition, TextPosition),
}
#[derive(Debug)]
pub enum Ownership {
    Task(Uuid),
    /// * `Uuid` - Uuid of function declaration node
    /// * `Option<Uuid>` - Uuid of task, if function is declared in context of task
    Function(Uuid, Option<Uuid>),
}

#[derive(Debug)]
pub struct Location {
    pub ownership: Ownership,
    pub blocks: Vec<Uuid>,
    /// Location in modules
    pub mods: Vec<String>,
    pub cursor: Cursor,
    /// IDX of token in tokens flow
    pub idx: isize,
    pub before_token: Option<Token>,
    pub before_node: Option<Uuid>,
}

impl Location {
    pub fn detect(locator: &mut LocationIterator) -> Result<Option<Location>, E> {
        let Some(current) = locator.prev_token() else {
            locator.drop();
            return Ok(None);
        };
        let cursor = current.token.clone();
        let idx = current.idx;
        drop(current);
        debug!("Cursor token: {}", cursor.id());
        let tree = locator.get_ownership_tree(cursor.pos.from.abs);
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

        let cursor = if matches!(
            cursor.id(),
            KindId::Identifier
                | KindId::Whitespace
                | KindId::LF
                | KindId::CR
                | KindId::CRLF
                | KindId::EOF
        ) {
            let mut path = Vec::new();
            let mut idx = idx - 1;
            path = if let Kind::Identifier(ident) = &cursor.kind {
                vec![ident.to_owned()]
            } else {
                Vec::new()
            };
            let mut from = None;
            let mut to = None;
            while let Some(token) = locator.nth_token(idx) {
                idx -= 1;
                match &token.kind {
                    Kind::Colon => {
                        if let Some(token) = locator.nth_token(idx) {
                            if !matches!(token.id(), KindId::Colon) {
                                break;
                            }
                        } else {
                            break;
                        }
                        idx -= 1;
                    }
                    Kind::Identifier(ident) => {
                        path.push(ident.to_owned());
                        if to.is_none() {
                            to = Some(token.pos.to.clone());
                        }
                        from = Some(token.pos.from.clone());
                    }
                    _ => {
                        break;
                    }
                }
            }
            path.reverse();
            if !path.is_empty() && from.is_some() && to.is_some() {
                locator.set_idx(idx);
                Cursor::Path(path.join("::"), idx, from.unwrap(), to.unwrap())
            } else {
                Cursor::Token(cursor, locator.idx)
            }
        } else {
            Cursor::Token(cursor, locator.idx)
        };
        let restore = locator.pin();
        let before_token = locator.prev().map(|prev| prev.token.clone());
        restore(locator);
        let before_node = locator.prev_node().map(|prev| prev.node.uuid().to_owned());
        locator.drop();
        Ok(ownership.map(|ownership| Location {
            ownership,
            idx,
            blocks,
            mods,
            cursor,
            before_token,
            before_node,
        }))
    }

    pub fn get_scx_uuid(&self) -> &Uuid {
        match &self.ownership {
            Ownership::Task(uuid) => uuid,
            Ownership::Function(_, Some(uuid)) => uuid,
            Ownership::Function(uuid, None) => uuid,
        }
    }
}
