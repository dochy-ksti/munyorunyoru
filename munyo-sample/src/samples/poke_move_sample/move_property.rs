use bitflags::bitflags;

#[derive(Debug, strum::EnumString)]
pub(crate) enum MovePropertySyntax {
    音,
}

bitflags! {
    #[derive(Debug)]
    pub(crate) struct MoveProperty:u32{
        const 音 = 0b0000001;
    }
}

impl From<MovePropertySyntax> for MoveProperty {
    fn from(value: MovePropertySyntax) -> Self {
        match value {
            MovePropertySyntax::音 => MoveProperty::音,
        }
    }
}
