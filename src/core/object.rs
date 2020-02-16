
pub trait Object {
    fn id(&self) -> u32;
    fn name(&self) -> &String;
}
