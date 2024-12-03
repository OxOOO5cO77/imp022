use hall::message::CommandMessage;
use shared_net::sizedbuffers::Bufferable;
use shared_net::types::NodeType;
use shared_net::{op, RoutedMessage, VSizedBuffer};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::UnboundedSender;

pub(crate) fn send_routed_message<T: CommandMessage>(message: &T, gate: NodeType, vagabond: NodeType, tx: &UnboundedSender<RoutedMessage>) -> Result<(), SendError<RoutedMessage>> {
    let route = op::Route::One(gate);
    let command = T::COMMAND;

    let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + vagabond.size_in_buffer() + message.size_in_buffer());

    out.push(&route);
    out.push(&command);
    out.push(&vagabond);
    out.push(message);

    let message = RoutedMessage {
        route: op::Route::None,
        buf: out,
    };

    tx.send(message)
}
