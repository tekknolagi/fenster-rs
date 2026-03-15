use fenster_sys::*;

fn main() {
    let mut buf = [0u32; 320 * 240];
    let mut f = unsafe { fenster_create(c"Hello, world!".as_ptr(), 320, 240, buf.as_mut_ptr()) };

    unsafe {
        fenster_open(f);

        let mut t = 0;
        let mut now = fenster_time();
        while fenster_loop(f) == 0 {
            t = t + 1;
            for i in 0..320 {
                for j in 0..240 {
                    let width = (*f).width;
                    let index = (j * width + i) as usize;
                    buf[index] = ((i ^ j) * t) as u32;
                }
            }
            let time = fenster_time();
            if time - now < 1000 / 60 {
                fenster_sleep(time - now);
            }
            now = time;
        }
        fenster_close(f);
        fenster_destroy(&mut f);
    }
}
