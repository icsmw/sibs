use crate::*;

impl Interpret for InterpolatedStringPart {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Self::Literal(s) => Ok(RtValue::Str(s.to_owned())),
            Self::Expression(n) => n.interpret(rt).await,
            Self::Open(..) | Self::Close(..) => Ok(RtValue::Str(String::new())),
        }
    }
}

impl Interpret for InterpolatedString {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let mut vls = Vec::new();
        for p in self.nodes.iter() {
            vls.push(
                p.interpret(rt.clone())
                    .await?
                    .as_string()
                    .ok_or(LinkedErr::from(E::CannotBeConvertedToString, self))?,
            );
        }
        Ok(RtValue::Str(vls.join("")))
    }
}
