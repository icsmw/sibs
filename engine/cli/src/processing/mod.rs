use std::path::Path;

use asttree::Anchor;
use interpreter::*;
use parser::*;
use runtime::*;
use semantic::*;
use uuid::Uuid;

use crate::*;

pub fn read<P: AsRef<Path>>(filepath: P) -> Result<(Anchor, SemanticCx), E> {
    let mut parser = Parser::new(filepath.as_ref())?;
    let anchor = Anchor::read(&mut parser);
    if let Err(err) = &anchor {
        eprintln!("{}", parser.report_err(err)?);
    }
    let anchor = anchor?.ok_or(E::FailExtractAnchorNodeFrom(
        filepath.as_ref().to_string_lossy().to_string(),
    ))?;
    let mut scx = SemanticCx::default();
    functions::register(&mut scx.fns.efns)?;
    if let Err(err) = anchor.initialize(&mut scx) {
        eprintln!("{}", parser.report_err(&err)?);
        return Err(err.into());
    }
    if let Err(err) = anchor.infer_type(&mut scx) {
        eprintln!("{}", parser.report_err(&err)?);
        return Err(err.into());
    }
    if let Err(err) = anchor.finalize(&mut scx) {
        eprintln!("{}", parser.report_err(&err)?);
        return Err(err.into());
    }
    Ok((anchor, scx))
}

pub async fn exec(
    params: RtParameters,
    scx: SemanticCx,
    anchor: Anchor,
    parser: Parser,
) -> Result<RtValue, RtError> {
    let rt = interpreter::runtime(params, scx)?;
    let cx = rt
        .create_cx(Uuid::new_v4(), "Test", None)
        .await
        .expect("Context created");
    let vl = anchor.interpret(rt.clone(), cx).await;
    let _ = rt.destroy().await;
    match vl {
        Ok(vl) => Ok(vl),
        Err(err) => {
            eprintln!(
                "{}",
                parser
                    .report_err(&err)
                    .map_err(|err| RtError::Other(err.to_string()))?
            );
            Err(err.e)
        }
    }
}
