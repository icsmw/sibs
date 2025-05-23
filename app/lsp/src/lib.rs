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
                    trigger_characters: Some(vec![":".into(), "::".into()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
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
