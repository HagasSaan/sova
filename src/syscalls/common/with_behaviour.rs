use crate::syscalls::common::behaviour::Behaviour;

pub trait WithBehaviour {
    fn behaviour(&self) -> Behaviour;
}