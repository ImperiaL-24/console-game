pub trait Named {
    fn name(&self) -> String;
}

pub trait Value {
    fn get_val(&self) -> i32;
    fn set_val(&mut self, value: i32);
}
