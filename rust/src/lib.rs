mod player;

use gdnative::prelude::{godot_init, InitHandle};

fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
}

godot_init!(init);
