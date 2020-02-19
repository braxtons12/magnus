
pub trait Object: Send + Sync {
    fn id(&self) -> u32;
    fn name(&self) -> &String;
}
