use tokio::sync::mpsc::UnboundedSender;

use hall_lib::message::CommandMessage;
use shared_net::{op, Bufferable, NodeType, RoutedMessage, SizedBuffer};

use crate::HallError;

pub(crate) fn send_routed_message<T: CommandMessage>(message: &T, gate: NodeType, vagabond: NodeType, tx: &UnboundedSender<RoutedMessage>) -> Result<(), HallError> {
    let route = op::Route::One(gate);
    let command = T::COMMAND;

    let mut out = SizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + vagabond.size_in_buffer() + message.size_in_buffer());

    out.push(&route).map_err(|e| HallError::SizedBuffer("route", e))?;
    out.push(&command).map_err(|e| HallError::SizedBuffer("command", e))?;
    out.push(&vagabond).map_err(|e| HallError::SizedBuffer("vagabond", e))?;
    out.push(message).map_err(|e| HallError::SizedBuffer("message", e))?;

    tx.send(out.into()).map_err(HallError::Send)
}
