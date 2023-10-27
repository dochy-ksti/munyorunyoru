use crate::{
    builder::builder::{Builder, MetaBuilder},
    error::parse_error::ParseError,
};

use super::{
    munyo_parser::{Pair, Pairs},
    state_machine::StateMachine,
};

pub(crate) fn parse_line_contents<MB, B, T>(
    pairs: Pairs,
    indent_level: usize,
    state: &mut StateMachine,
    builder: &MB,
) -> Result<(), ParseError>
where
    MB: MetaBuilder<B, T>,
    B: Builder<T>,
{
    Ok(())
}
