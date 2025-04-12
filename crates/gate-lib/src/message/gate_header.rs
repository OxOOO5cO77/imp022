use shared_net::{AuthType, Bufferable, NodeType, SizedBuffer, SizedBufferError, UserIdType};

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
    use super::GateHeader;
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_gate_header() -> Result<(), SizedBufferError> {
        let orig = GateHeader {
            vagabond: 3,
            user: 1234567890,
            auth: 9876543210,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GateHeader>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
        Ok(())
    }
}
