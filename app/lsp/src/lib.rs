mod semantic;

use std::collections::HashMap;

use driver::Driver;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::*;
use tower_lsp::{jsonrpc, Client, LanguageServer, LspService, Server};

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
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![":".into(), "::".into(), ".".into()]),
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
        self.docs.write().await.insert(uri, text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let changes = &params.content_changes;
        if let Some(change) = changes.last() {
            self.docs.write().await.insert(uri, change.text.clone());
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

    async fn completion(&self, _: CompletionParams) -> jsonrpc::Result<Option<CompletionResponse>> {
        let items = vec![
            CompletionItem::new_simple("component".into(), "Component detection".into()),
            CompletionItem::new_simple("task".into(), "Task detection".into()),
            CompletionItem::new_simple("mod".into(), "Module declaration".into()),
            CompletionItem::new_simple("include".into(), "Including".into()),
        ];
        Ok(Some(CompletionResponse::Array(items)))
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
