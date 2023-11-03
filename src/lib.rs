use godot::prelude::*;

mod serial;

struct SerialLib;

#[gdextension(entry_point=serial_ext_init)]
unsafe impl ExtensionLibrary for SerialLib {}
