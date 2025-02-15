use std::collections::HashMap;

use tokio::sync::mpsc::UnboundedSender;
use tracing::error;

use hall_lib::message::CommandMessage;
use shared_net::{NodeType, RoutedMessage, UserIdType};

use crate::network::util::send_routed_message;

pub(crate) struct Broadcaster {
    pub(crate) local_tx: UnboundedSender<RoutedMessage>,
    pub(crate) gate_map: HashMap<UserIdType, (NodeType, NodeType)>,
}

impl Broadcaster {
    pub(crate) fn new(local_tx: UnboundedSender<RoutedMessage>) -> Self {
        Self {
            local_tx,
            gate_map: HashMap::new(),
        }
    }

    pub(crate) fn broadcast<T: CommandMessage>(&mut self, message: T) {
        let mut errors = vec![];
        for (id, (gate, vagabond)) in &self.gate_map {
            let result = send_routed_message(&message, *gate, *vagabond, &self.local_tx);
            if result.is_err() {
                error!(?result, id, gate, vagabond);
                errors.push(*id);
            }
        }

        for id in &errors {
            self.gate_map.remove(id);
        }
    }

    pub(crate) fn send_to_user<T: CommandMessage>(&mut self, id: &UserIdType, message: &T) {
        if let Some((gate, vagabond)) = self.gate_map.get(id) {
            let result = send_routed_message(message, *gate, *vagabond, &self.local_tx);
            if result.is_err() {
                error!(?result, id, gate, vagabond);
                self.gate_map.remove(id);
            }
        }
    }

    pub(crate) fn track(&mut self, id: UserIdType, target: (NodeType, NodeType)) {
        self.gate_map.insert(id, target);
    }
}
