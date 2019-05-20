use crate::input::Input;

pub trait InputInterface {
    fn get_user_input(&self) -> Input;
}
