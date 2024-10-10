use crate::{
    elements::Task,
    inf::{operator::E, Execute, ExecuteContext, ExecutePinnedResult, Processing, TryExecute},
};

impl Processing for Task {}

impl TryExecute for Task {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            if self.declarations.len() != cx.args.len() {
                Err(E::DismatchTaskArgumentsCount(
                    self.declarations.len(),
                    self.declarations
                        .iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    cx.args.len(),
                    cx.args
                        .iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                )
                .by(self))?;
            }
            for (i, el) in self.declarations.iter().enumerate() {
                el.execute(cx.clone().args(&[cx.args[i].to_owned()]))
                    .await?;
            }
            for dependency in self.dependencies.iter() {
                dependency.execute(cx.clone().args(&[])).await?;
            }
            let result = self.block.execute(cx.clone()).await?;
            Ok(cx.sc.get_retreat().await?.unwrap_or(result))
        })
    }
}
