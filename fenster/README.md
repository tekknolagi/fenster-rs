# fenster

Rust bindings to zserge/fenster, one of the most minimal cross-platform GUI library. 

## Examples

```rust
use fenster::*;

fn main() {
    let mut fenster = Fenster::new("Minimal Fenster", 320, 240);

    let mut t = 0;
    while fenster.loop_with_fps(60) {
        for i in 0..fenster.width() {
            for j in 0..fenster.height() {
                *fenster.pixel(i, j) = ((i ^ j) * t) as u32;
            }
        }
        t = t + 1;
    }
}
```