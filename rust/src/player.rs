use gdnative::api::*;
use gdnative::prelude::*;

type Inherits = KinematicBody;

#[derive(NativeClass)]
#[inherit(Inherits)]
#[register_with(Self::register_builder)]
pub struct Player {
    name: String,
}

#[methods]
impl Player {
    fn register_builder(_builder: &ClassBuilder<Self>) {}

    fn new(_owner: &Inherits) -> Self {
        Player {
            name: "Player".to_string(),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, _owner: &Inherits) {
        self.name = "Player".to_string();
        godot_print!("{} is ready!", self.name);
    }

    #[export]
    unsafe fn _process(&self, _owner: &Inherits, delta: f64) {
        godot_print!("Inside {} _process(), delta is {}", self.name, delta);
    }

    #[export]
    unsafe fn _physics_process(&self, _owner: &Inherits, delta: f64) {
        godot_print!(
            "Inside {} _physics_process(), delta is {}",
            self.name,
            delta
        );
    }
}
