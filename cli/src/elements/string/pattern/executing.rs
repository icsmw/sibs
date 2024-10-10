use crate::{
    elements::{Element, PatternString},
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute, Value,
    },
};

impl Processing for PatternString {}

impl TryExecute for PatternString {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let mut output = String::new();
            for element in self.elements.iter() {
                if let Element::SimpleString(el, _) = element {
                    output = format!("{output}{el}");
                } else {
                    output = format!(
                        "{output}{}",
                        element
                            .execute(cx.clone())
                            .await?
                            .as_string()
                            .ok_or(E::FailToGetValueAsString)?
                    );
                }
            }
            Ok(Value::String(output))
        })
    }
}
