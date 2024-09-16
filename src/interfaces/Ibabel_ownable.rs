use super::babel_core::BabelCore;

pub trait IBabelOwnable {
    fn babel_core(&self) -> &dyn BabelCore;
    fn owner(&self) -> &str;
    fn guardian(&self) -> &str;
}
