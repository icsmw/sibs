use crate::*;

impl Interpret for InterpolatedStringPart {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Self::Literal(tk) => Ok(RtValue::Str(tk.to_string())),
            Self::Expression(_, n, _) => n.interpret(rt, cx).await,
            Self::Open(..) | Self::Close(..) => Ok(RtValue::Str(String::new())),
        }
    }
}

impl Interpret for InterpolatedString {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let mut vls = Vec::new();
        for p in self.nodes.iter() {
            vls.push(
                p.interpret(rt.clone(), cx.clone())
                    .await?
                    .as_string()
                    .ok_or(LinkedErr::from(E::CannotBeConvertedToString, self))?,
            );
        }
        Ok(RtValue::Str(vls.join("")))
    }
}
