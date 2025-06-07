import * as path from "path";
import * as vscode from "vscode";
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    const output = vscode.window.createOutputChannel("SIBS");
    output.show();
    const item = vscode.window.createStatusBarItem(
        vscode.StatusBarAlignment.Left,
        100
    );
    item.text = "🛠️ SIBS";
    item.tooltip = "SIBS Language Server";
    item.command = undefined;
    item.show();
    const command = "/storage/projects/private/sibs/app/target/release/cli";

    const serverOptions: ServerOptions = {
        run: { command },
        debug: { command, args: ["--lsp"] },
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: "sibs" }],
        workspaceFolder: vscode.workspace.workspaceFolders?.[0],
    };

    client = new LanguageClient(
        "sibsLanguageServer",
        "SIBS Language Server",
        serverOptions,
        clientOptions
    );
    output.appendLine(`All ready to start`);

    client.start();
    output.appendLine(`Started`);
}

export function deactivate(): Thenable<void> | undefined {
    // return Promise.resolve();
    return client?.stop();
}
