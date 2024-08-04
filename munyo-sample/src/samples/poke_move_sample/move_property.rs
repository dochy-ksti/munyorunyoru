use bitflags::bitflags;

#[derive(Debug, strum::EnumString)]
pub(crate) enum MovePropertySyntax {
    Sound,
}

bitflags! {
    #[derive(Debug)]
    pub(crate) struct MoveProperty:u32{
        const Sound = 0b0000001;
    }
}

impl From<MovePropertySyntax> for MoveProperty {
    fn from(value: MovePropertySyntax) -> Self {
        match value {
            MovePropertySyntax::Sound => MoveProperty::Sound,
        }
    }
}
