pub trait Execute {
    fn execute(&self, path: &String) -> Result<(), String>;
}