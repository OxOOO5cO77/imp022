use std::collections::HashSet;

use hall::core::{MissionNodeIdType, TickType, Token, TokenKind};

#[derive(Default)]
pub struct GameUserMissionState {
    pub current_node: MissionNodeIdType,
    pub known_nodes: HashSet<MissionNodeIdType>,
    pub tokens: Vec<Token>,
}

impl GameUserMissionState {
    pub fn current(&self) -> MissionNodeIdType {
        self.current_node
    }

    pub fn set_current(&mut self, node: MissionNodeIdType) -> bool {
        self.current_node = node;
        self.known_nodes.insert(node)
    }

    pub fn expire_tokens(&mut self, tick: TickType) {
        self.tokens.retain(|t| t.expiry >= tick);
    }

    pub fn get_token(&self, kind: TokenKind) -> Option<&Token> {
        self.tokens.iter().filter(|t| t.kind.matches(&kind)).max()
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}
