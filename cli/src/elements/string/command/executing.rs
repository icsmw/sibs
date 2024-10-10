use crate::{
    elements::{Command, Element},
    inf::{
        operator::E, spawner, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute,
        Value,
    },
};

impl Processing for Command {}

impl TryExecute for Command {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let mut command = String::new();
            for element in self.elements.iter() {
                if let Element::SimpleString(el, _) = element {
                    command.push_str(&el.to_string());
                } else {
                    command.push_str(
                        &element
                            .execute(cx.clone())
                            .await?
                            .as_string()
                            .ok_or(E::FailToGetValueAsString.by(self))?,
                    );
                }
            }
            let cwd = cx.sc.get_cwd().await?.clone();
            Ok(Value::SpawnStatus(
                spawner::run(cx.token.clone(), &command, &cwd, cx.cx.clone()).await?,
            ))
        })
    }
}
