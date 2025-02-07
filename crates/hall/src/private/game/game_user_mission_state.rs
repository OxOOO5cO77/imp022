use std::collections::HashSet;

use hall::core::{AuthLevel, MissionNodeIdType, TickType, Token, TokenKind};
use hall::message::UpdateTokenMessage;

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

    pub fn expire_tokens(&mut self, tick: TickType) -> Vec<UpdateTokenMessage> {
        let messages = self.tokens.iter().filter(|t| t.expiry < tick).map(|t| UpdateTokenMessage::Expire(t.clone())).collect();
        self.tokens.retain(|t| t.expiry >= tick);
        messages
    }

    pub fn upgrade_cred_to_auth(&mut self) -> Vec<UpdateTokenMessage> {
        let mut messages = Vec::new();
        for token in self.tokens.iter_mut() {
            if let TokenKind::Credentials(level) = token.kind {
                let from = token.clone();
                token.kind = TokenKind::Authorization(level);
                let to = token.clone();
                messages.push(UpdateTokenMessage::Convert(from, to))
            }
        }
        messages
    }

    pub fn add_token(&mut self, token: Token) -> UpdateTokenMessage {
        self.tokens.push(token.clone());
        UpdateTokenMessage::Add(token)
    }

    pub fn any_auth(&self) -> bool {
        self.tokens.iter().any(|t| matches!(t.kind, TokenKind::Authorization(_)))
    }

    pub(crate) fn max_auth_level(&self) -> AuthLevel {
        self.tokens.iter().filter(|t| matches!(t.kind, TokenKind::Authorization(_))).max().map_or(AuthLevel::Anonymous, |t| t.kind.level())
    }
}
