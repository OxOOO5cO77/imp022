use num_enum::{FromPrimitive, IntoPrimitive};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, IntoPrimitive)]
pub enum ForumSubCommand {
    #[num_enum(default)]
    Chat,
    DM,
}
