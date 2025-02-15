use num_enum::{FromPrimitive, IntoPrimitive};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, IntoPrimitive)]
pub enum ArchiveSubCommand {
    #[num_enum(default)]
    InvList,
    InvGen,
}
