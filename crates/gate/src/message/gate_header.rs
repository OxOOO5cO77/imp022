use shared_net::{AuthType, Bufferable, NodeType, UserIdType, VSizedBuffer};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GateHeader {
    pub vagabond: NodeType,
    pub user: UserIdType,
    pub auth: AuthType,
}

impl GateHeader {
    pub fn new(vagabond: NodeType, user: UserIdType, auth: AuthType) -> Self {
        Self {
            vagabond,
            user,
            auth,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::message::gate_header::GateHeader;
    use shared_net::{Bufferable, VSizedBuffer};

    #[test]
    fn test_gate_header() {
        let orig = GateHeader {
            vagabond: 3,
            user: 1234567890,
            auth: 9876543210,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GateHeader>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
