pub(crate) mod line_col_lookup;
pub(crate) mod munyo_error;
pub(crate) mod parse_error;
pub(crate) mod parse_fail;

pub use munyo_error::Error;
pub type MunyoResult<T> = std::result::Result<T, Error>;
