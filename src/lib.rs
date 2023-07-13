use godot::prelude::*;

mod serial;

struct SerialLib;

#[gdextension(entry_point=serial_lib_init)]
unsafe impl ExtensionLibrary for SerialLib {}
