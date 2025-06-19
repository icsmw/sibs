mod semantic;

use diagnostics::ErrorCode;
use driver::{CompletionMatch, Driver};
use std::collections::HashMap;
use std::fs;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::*;
use tower_lsp::{jsonrpc, Client, LanguageServer, LspService, Server};
use tracing::{debug, error};

#[derive(Debug)]
struct Backend {
    client: Client,
    docs: RwLock<HashMap<Url, String>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            docs: RwLock::new(HashMap::new()),
        }
    }
    pub async fn get_driver_by_url(&self, uri: &Url) -> jsonrpc::Result<Driver> {
        if let Some(content) = self.docs.read().await.get(uri) {
            self.client
                .log_message(
                    MessageType::LOG,
                    format!("document ({}) found in cache", uri.path()),
                )
                .await;
            Ok(Driver::unbound(content, true))
        } else {
            let filepath = uri.to_file_path().map_err(|_| jsonrpc::Error {
                message: "Fail to get filepath".into(),
                code: jsonrpc::ErrorCode::ParseError,
                data: None,
            })?;
            self.client
                .log_message(
                    MessageType::LOG,
                    format!("document ({filepath:?}) will be read from disk"),
                )
                .await;
            Ok(Driver::new(filepath, true))
        }
    }
    pub async fn get_abs_pos(&self, uri: &Url, pos: Position) -> jsonrpc::Result<usize> {
        fn get_pos(content: &str, pos: Position) -> usize {
            let mut abs = 0;
            let mut lines = content.lines();

            for _ in 0..pos.line {
                if let Some(line) = lines.next() {
                    abs += line.len() + 1;
                } else {
                    return content.len();
                }
            }

            if let Some(line) = lines.next() {
                abs + (pos.character as usize).min(line.len())
            } else {
                content.len()
            }
        }
        if let Some(content) = self.docs.read().await.get(uri) {
            Ok(get_pos(content, pos))
        } else {
            let filepath = uri.to_file_path().map_err(|_| jsonrpc::Error {
                message: "Fail to get filepath".into(),
                code: jsonrpc::ErrorCode::ParseError,
                data: None,
            })?;
            self.docs.write().await.insert(
                uri.clone(),
                fs::read_to_string(filepath).map_err(|err| jsonrpc::Error {
                    code: jsonrpc::ErrorCode::InternalError,
                    message: format!("Fail read {}: {err}", uri.path()).into(),
                    data: None,
                })?,
            );
            let reader = self.docs.read().await;
            let content = reader.get(uri).ok_or(jsonrpc::Error {
                code: jsonrpc::ErrorCode::InternalError,
                message: format!("Cannot access to {}", uri.path()).into(),
                data: None,
            })?;
            Ok(get_pos(content, pos))
        }
    }
    async fn get_diagnostics<S: ToString>(&self, content: S) -> jsonrpc::Result<Vec<Diagnostic>> {
        let mut driver = Driver::unbound(content, true);
        driver.read().map_err(|err| jsonrpc::Error {
            message: format!("Fail to parse source code: {err}").into(),
            code: jsonrpc::ErrorCode::ParseError,
            data: None,
        })?;
        let Some(errors) = driver.errors() else {
            return Ok(Vec::new());
        };
        Ok(errors
            .map(|err| {
                let link = err.err.link();
                Diagnostic {
                    range: Range {
                        start: Position {
                            line: link.from.ln as u32,
                            character: link.from.col as u32,
                        },
                        end: Position {
                            line: link.to.ln as u32,
                            character: link.to.col as u32,
                        },
                    },
                    code: Some(NumberOrString::String(err.err.formattable())),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: err.err.to_string(),
                    ..Default::default()
                }
            })
            .collect())
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
                    retrigger_characters: None,
                    work_done_progress_options: Default::default(),
                }),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![
                        "=".into(),
                        "==".into(),
                        "!=".into(),
                        ">=".into(),
                        "<=".into(),
                        ">".into(),
                        "<".into(),
                        ":".into(),
                        "::".into(),
                        ".".into(),
                    ]),
                    ..Default::default()
                }),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        will_save: None,
                        will_save_wait_until: None,
                        save: None,
                    },
                )),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            legend: SemanticTokensLegend {
                                token_types: vec![
                                    SemanticTokenType::KEYWORD,
                                    SemanticTokenType::FUNCTION,
                                    SemanticTokenType::VARIABLE,
                                    SemanticTokenType::STRING,
                                    SemanticTokenType::NAMESPACE,
                                    SemanticTokenType::PARAMETER,
                                    SemanticTokenType::TYPE,
                                    SemanticTokenType::METHOD,
                                    SemanticTokenType::NUMBER,
                                    SemanticTokenType::OPERATOR,
                                    SemanticTokenType::EVENT,
                                    SemanticTokenType::COMMENT,
                                ],
                                token_modifiers: vec![],
                            },
                            ..Default::default()
                        },
                    ),
                ),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.client
            .log_message(
                MessageType::INFO,
                format!("document ({}) has been opened", uri.path()),
            )
            .await;
        let diagnostics = self.get_diagnostics(&text).await;
        self.docs.write().await.insert(uri.clone(), text);
        match diagnostics {
            Ok(diagnostics) => {
                self.client
                    .publish_diagnostics(uri, diagnostics, None)
                    .await;
            }
            Err(err) => {
                error!("fail to get diagnostics: {err}");
            }
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let changes = &params.content_changes;
        if let Some(change) = changes.last() {
            let diagnostics = self.get_diagnostics(&change.text).await;
            self.docs
                .write()
                .await
                .insert(uri.clone(), change.text.clone());
            match diagnostics {
                Ok(diagnostics) => {
                    self.client
                        .publish_diagnostics(uri, diagnostics, None)
                        .await;
                }
                Err(err) => {
                    error!("fail to get diagnostics: {err}");
                }
            }
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "document ({}) closed and will be removed",
                    params.text_document.uri.path()
                ),
            )
            .await;
        self.docs.write().await.remove(&params.text_document.uri);
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> jsonrpc::Result<Option<SemanticTokensResult>> {
        let mut driver = self.get_driver_by_url(&params.text_document.uri).await?;
        driver.read().map_err(|err| jsonrpc::Error {
            message: format!("Fail to parse source code: {err}").into(),
            code: jsonrpc::ErrorCode::ParseError,
            data: None,
        })?;
        let mut tokens = driver.get_semantic_tokens();
        tokens.sort_by(|a, b| a.position.from.abs.cmp(&b.position.from.abs));
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: semantic::to_lsp_tokens(&tokens),
        })))
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "SIBS language server initialized")
            .await;
    }

    async fn signature_help(
        &self,
        params: SignatureHelpParams,
    ) -> jsonrpc::Result<Option<SignatureHelp>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let pos = self
            .get_abs_pos(uri, params.text_document_position_params.position)
            .await?;
        self.client
            .log_message(
                MessageType::LOG,
                format!("will find signature_help for position {pos}"),
            )
            .await;
        debug!("will find signature_help for position {pos}");
        let mut driver = self.get_driver_by_url(uri).await?;
        driver.read().map_err(|err| jsonrpc::Error {
            message: format!("Fail to parse source code: {err}").into(),
            code: jsonrpc::ErrorCode::ParseError,
            data: None,
        })?;
        let Some(signature) = driver.signature(pos, None) else {
            return Ok(None);
        };
        let sig = SignatureInformation {
            label: signature.signature,
            documentation: signature.docs.map(|value| {
                Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value,
                })
            }),
            parameters: Some(
                signature
                    .args
                    .into_iter()
                    .map(|arg| ParameterInformation {
                        label: ParameterLabel::Simple(arg),
                        documentation: None,
                    })
                    .collect(),
            ),
            active_parameter: Some(0),
        };
        Ok(Some(SignatureHelp {
            signatures: vec![sig],
            active_parameter: Some(0),
            active_signature: Some(0),
        }))
    }

    async fn completion(
        &self,
        params: CompletionParams,
    ) -> jsonrpc::Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let pos = self
            .get_abs_pos(uri, params.text_document_position.position)
            .await?;
        self.client
            .log_message(
                MessageType::LOG,
                format!("will find suggestions for position {pos}"),
            )
            .await;
        debug!("will find suggestions for position {pos}");
        let mut driver = self.get_driver_by_url(uri).await?;
        driver.read().map_err(|err| jsonrpc::Error {
            message: format!("Fail to parse source code: {err}").into(),
            code: jsonrpc::ErrorCode::ParseError,
            data: None,
        })?;
        let Some(mut completion) = driver.completion(pos, None) else {
            debug!("no completion has been gotten for pos: {pos}");
            return Ok(None);
        };
        let suggestions = completion.suggest().map_err(|err| jsonrpc::Error {
            code: jsonrpc::ErrorCode::InternalError,
            message: format!("Fail to get suggestions for ({}): {err}", uri.path()).into(),
            data: None,
        })?;
        let Some(mut suggestions) = suggestions else {
            debug!("no suggestions has been gotten for pos: {pos}");
            return Ok(None);
        };
        debug!(
            "for pos: {pos} has been found {} suggestions",
            suggestions.suggestions.len()
        );
        suggestions
            .suggestions
            .sort_by(|a, b| a.score.cmp(&b.score));
        let items = suggestions
            .suggestions
            .into_iter()
            .map(|suggestion| match &suggestion.target {
                CompletionMatch::Function(name, docs, ..) => {
                    let detail = if let Some(docs) = &docs {
                        docs.split('\n').next().map(|str| str.to_owned())
                    } else {
                        None
                    };
                    CompletionItem {
                        label: name.to_owned(),
                        kind: Some(CompletionItemKind::FUNCTION),
                        detail,
                        text_edit: suggestions.replacement.map(|(start, end)| {
                            CompletionTextEdit::Edit(TextEdit {
                                new_text: name.to_owned(),
                                range: Range {
                                    start: Position {
                                        line: start.ln as u32,
                                        character: start.col as u32,
                                    },
                                    end: Position {
                                        line: end.ln as u32,
                                        character: end.col as u32,
                                    },
                                },
                            })
                        }),
                        // documentation: docs.clone().map(|docs| {
                        //     Documentation::MarkupContent(MarkupContent {
                        //         kind: MarkupKind::Markdown,
                        //         value: docs,
                        //     })
                        // }),
                        ..Default::default()
                    }
                }
                CompletionMatch::Variable(name, ..) => CompletionItem {
                    label: name.to_owned(),
                    kind: Some(CompletionItemKind::VARIABLE),
                    ..Default::default()
                },
            })
            .collect::<Vec<CompletionItem>>();
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> jsonrpc::Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let pos = self
            .get_abs_pos(uri, params.text_document_position_params.position)
            .await?;
        self.client
            .log_message(
                MessageType::LOG,
                format!("will process hover for position {pos}"),
            )
            .await;
        debug!("will process hover for position {pos}");
        let mut driver = self.get_driver_by_url(uri).await?;
        driver.read().map_err(|err| jsonrpc::Error {
            message: format!("Fail to parse source code: {err}").into(),
            code: jsonrpc::ErrorCode::ParseError,
            data: None,
        })?;
        let Some(signature) = driver.signature(pos, None) else {
            return Ok(None);
        };
        let Some(docs) = signature.docs else {
            return Ok(None);
        };
        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: docs,
            }),
            range: None,
        }))
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }
}

pub async fn run() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
