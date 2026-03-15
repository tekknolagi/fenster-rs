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
        if fenster.key(0x1b) {
            break;
        }
        t = t + 1;
    }
}
