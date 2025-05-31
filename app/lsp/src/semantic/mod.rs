use semantic::{self, LinkedSemanticToken};
use tower_lsp::lsp_types::SemanticToken;
/**

                                   SemanticTokenType::KEYWORD,     0
                                   SemanticTokenType::FUNCTION,    1
                                   SemanticTokenType::VARIABLE,    2
                                   SemanticTokenType::STRING,      3
                                   SemanticTokenType::NAMESPACE,    4
                                   SemanticTokenType::PARAMETER,    5
                                   SemanticTokenType::TYPE,         6
                                   SemanticTokenType::METHOD,       7
                                   SemanticTokenType::NUMBER,       8
                                   SemanticTokenType::OPERATOR,     9
                                   SemanticTokenType::EVENT,        10
                                   SemanticTokenType::COMMENT,      11

*/
fn map_token_type(token: &semantic::SemanticToken) -> u32 {
    match token {
        semantic::SemanticToken::Keyword => 0,
        semantic::SemanticToken::Function => 1,
        semantic::SemanticToken::Variable => 2,
        semantic::SemanticToken::String => 3,
        semantic::SemanticToken::Number => 8,
        semantic::SemanticToken::Bool => 0,
        semantic::SemanticToken::Type => 6,
        semantic::SemanticToken::Parameter => 5,
        semantic::SemanticToken::Operator => 9,
        semantic::SemanticToken::Comment => 11,
        semantic::SemanticToken::Meta => 11,
        semantic::SemanticToken::Event => 3,
        semantic::SemanticToken::Class => 0,
        semantic::SemanticToken::Namespace => 4,
        semantic::SemanticToken::Delimiter => 0,
        semantic::SemanticToken::Task => 1,
        semantic::SemanticToken::Component => 1,
        semantic::SemanticToken::Module => 1,
    }
}

pub fn to_lsp_tokens(tokens: &[LinkedSemanticToken]) -> Vec<SemanticToken> {
    let mut result = Vec::with_capacity(tokens.len());
    let mut prev_ln = 0;
    let mut prev_col = 0;

    for linked in tokens {
        let pos = &linked.position.from;
        let token_len = linked.position.to.abs - pos.abs;
        let delta_line = pos.ln - prev_ln;
        let delta_start = if delta_line == 0 {
            pos.col - prev_col
        } else {
            pos.col
        };

        result.push(SemanticToken {
            delta_line: delta_line as u32,
            delta_start: delta_start as u32,
            length: token_len as u32,
            token_type: map_token_type(&linked.token),
            token_modifiers_bitset: 0,
        });

        prev_ln = pos.ln;
        prev_col = pos.col;
    }

    result
}
