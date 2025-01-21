pub mod batch;
pub mod reactive;
pub mod special;

#[derive(Debug, Clone, strum::Display, strum::EnumCount, strum::EnumIter, strum::EnumIs)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum Verdict {
    Ac,
    Wa,
    Re,
    Tle,
}
