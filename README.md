# godot-w-rust
Godot with Rust shenanigans

## Problems encountered
Sadly there seems to be no way to do default parameters with rust, which means
that they must be hardcoded, and that function calls extend over the span of
multiple lines. Sadly I think this is a massive deal breaker.
