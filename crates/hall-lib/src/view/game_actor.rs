use crate::core::{ActorIdType, AuthLevel};
use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameActorPlayerView {
    pub id: ActorIdType,
    pub auth_level: AuthLevel,
}

impl Bufferable for GameActorPlayerView {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let mut pushed = 0;
        pushed += self.id.push_into(buf)?;
        let auth_level: u8 = self.auth_level.into();
        pushed += auth_level.push_into(buf)?;
        Ok(pushed)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let id = ActorIdType::pull_from(buf)?;
        let auth_level = u8::pull_from(buf)?.into();
        let result = Self {
            id,
            auth_level,
        };
        Ok(result)
    }

    fn size_in_buffer(&self) -> usize {
        self.id.size_in_buffer() + size_of::<u8>()
    }
}

#[cfg(test)]
impl GameActorPlayerView {
    pub(crate) fn test_default() -> Self {
        Self {
            id: 123456789,
            auth_level: AuthLevel::Admin,
        }
    }
}

#[cfg(test)]
mod test {
    use super::GameActorPlayerView;
    use shared_net::{SizedBuffer, SizedBufferError};

    #[test]
    fn test_actor_view() -> Result<(), SizedBufferError> {
        let orig = GameActorPlayerView::test_default();

        let mut buf = SizedBuffer::from(&orig)?;
        let new = buf.pull::<GameActorPlayerView>()?;

        assert_eq!(orig, new);
        Ok(())
    }
}
