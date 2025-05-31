mod semantic;

use driver::Driver;
use tower_lsp::lsp_types::*;
use tower_lsp::{jsonrpc, Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
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

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> jsonrpc::Result<Option<SemanticTokensResult>> {
        let filepath = params
            .text_document
            .uri
            .to_file_path()
            .map_err(|_| jsonrpc::Error {
                message: "Fail to get filepath".into(),
                code: jsonrpc::ErrorCode::ParseError,
                data: None,
            })?;
        self.client
            .log_message(MessageType::LOG, format!("will read: {filepath:?}"))
            .await;
        let mut driver = Driver::new(filepath, true);
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
    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
