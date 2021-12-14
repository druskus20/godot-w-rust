use gdnative::api::*;
use gdnative::godot_wrap_method;
use gdnative::prelude::*;

type Inherits = KinematicBody;

#[derive(NativeClass)]
#[inherit(Inherits)]
#[register_with(Self::register_builder)]
pub struct Player {
    name: String,
    velocity: Vector3,
    speed: f32,
    fall_acceleration: f32,
}

#[methods]
impl Player {
    fn register_builder(_builder: &ClassBuilder<Self>) {}

    fn new(_owner: &Inherits) -> Self {
        Player {
            name: "Player".to_string(),
            velocity: Vector3::zero(),
            fall_acceleration: 9.8,
            speed: 10.0,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: &Inherits) {}

    #[export]
    unsafe fn _process2(&self, owner: &Inherits, delta: f32) {}

    #[export]
    fn _process(&mut self, owner: &Inherits, delta: f32) {
        let input = Input::godot_singleton();
        let mut direction = Vector3::zero();

        if input.is_action_pressed(GodotString::from_str("move_right")) {
            direction.x += 1.0
        }
        if Input::is_action_pressed(input, GodotString::from_str("move_left")) {
            direction.x -= 1.0
        }
        if Input::is_action_pressed(input, GodotString::from_str("move_forward")) {
            direction.z -= 1.0
        }
        if Input::is_action_pressed(input, GodotString::from_str("move_backward")) {
            direction.z += 1.0
        }

        if direction.length() > 0.0 {
            direction = direction.normalize() * self.speed;
        }

        if direction != Vector3::zero() {
            direction = direction.normalize();
            owner.look_at(owner.translation() + direction, Vector3::new(0.0, 1.0, 0.0));
        }

        // Ground velocity
        self.velocity.x = direction.x * self.speed;
        self.velocity.z = direction.z * self.speed;
        // Vertical velocity
        self.velocity.y -= self.fall_acceleration * delta;
        // Moving the character
        self.velocity = owner.move_and_slide(
            self.velocity,
            Vector3::new(0.0, 0.0, 0.0),
            false,
            4,
            std::f64::consts::FRAC_PI_4,
            true,
        );
        //let change = direction * delta * self.speed * self.velocity;
        //let position = owner.global_position() + change;
    }
}
