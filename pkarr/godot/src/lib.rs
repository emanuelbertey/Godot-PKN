use godot::prelude::*;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}

mod keytool;
mod pkarrgodot;
mod state;