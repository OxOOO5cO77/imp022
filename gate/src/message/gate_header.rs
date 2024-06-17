use std::mem::size_of;
use shared_data::types::{AuthType, NodeType, UserType};
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GateHeader {
    pub vagabond: NodeType,
    pub user: UserType,
    pub auth: AuthType,
}

impl GateHeader {
    pub fn new(vagabond: NodeType, user: UserType, auth: AuthType) -> Self {
        Self {
            vagabond,
            user,
            auth,
        }
    }
}

impl Bufferable for GateHeader {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.vagabond.push_into(buf);
        self.user.push_into(buf);
        self.auth.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        Self {
            vagabond: NodeType::pull_from(buf),
            user: UserType::pull_from(buf),
            auth: AuthType::pull_from(buf),
        }
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<NodeType>() + size_of::<UserType>() + size_of::<AuthType>()
    }
}

#[cfg(test)]
mod test {
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;
    use crate::message::gate_header::GateHeader;

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
