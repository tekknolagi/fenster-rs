#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use fenster_sys::*;

/// The main struct of the fenster library.
pub struct Fenster {
    inner: *mut fenster,
    buf: Vec<u32>,
    now: i64,
}

impl Fenster {
    /// Creates a window with the specified title, width, and height.
    ///
    /// # Remarks
    /// The width and height can not change after the window is created.
    ///
    /// # Panics
    /// The function will panic if:
    ///  - title contains a null byte in the middle,
    ///  - width or height is not positive,
    ///  - or, the window failed to create or open.
    pub fn new(title: &str, width: i32, height: i32) -> Self {
        let c_title = std::ffi::CString::new(title).unwrap();
        assert!(width > 0);
        assert!(height > 0);
        let mut buf: Vec<u32> = vec![0; (width * height) as usize];
        let inner = unsafe { fenster_create(c_title.as_ptr(), width, height, buf.as_mut_ptr()) };
        assert!(!inner.is_null());
        unsafe { fenster_open(inner) };
        Self {
            inner,
            buf,
            now: unsafe { fenster_time() },
        }
    }

    /// Returns a mutable reference to the pixel at the specified coordinates.
    ///
    /// # Panics
    /// Panics if x or y are out of bounds.
    pub fn pixel(&mut self, x: i32, y: i32) -> &mut u32 {
        assert!(x >= 0 && x < self.width());
        assert!(y >= 0 && y < self.height());
        let index = (y * self.width() + x) as usize;
        &mut self.buf[index]
    }

    /// Runs the event loop. 
    /// 
    /// Returns true if the loop was terminated successfully.
    pub fn loop_with_fps(&mut self, fps: i64) -> bool {
        let t = unsafe { fenster_time() };
        if t - self.now < 1000 / fps {
            unsafe { fenster_sleep(t - self.now) };
        }
        self.now = t;
        unsafe { fenster_loop(self.inner) == 0 }
    }

    /// Checks if a key is pressed.
    pub fn key(&self, c: i32) -> bool {
        if c >= 0 && c < 128 {
            unsafe { (*self.inner).keys[c as usize] != 0 }
        } else {
            false
        }
    }

    /// Returns the width of the window.
    pub fn width(&self) -> i32 {
        unsafe { (*self.inner).width }
    }

    /// Returns the height of the window.
    pub fn height(&self) -> i32 {
        unsafe { (*self.inner).height }
    }

    /// Returns the x coordinate of the mouse cursor.
    pub fn x(&self) -> i32 {
        unsafe { (*self.inner).x }
    }

    /// Returns the y coordinate of the mouse cursor.
    pub fn y(&self) -> i32 {
        unsafe { (*self.inner).y }
    }

    /// Returns the mouse button state.
    ///
    /// If button pressed, mouse will be 1.
    pub fn mouse(&self) -> i32 {
        unsafe { (*self.inner).mouse }
    }

    /// Returns the modifier key state.
    ///
    /// mod is 4 bits mask, ctrl=1, shift=2, alt=4, meta=8,
    /// if ctrl + shift is pressed, mod will be 3 (1 | 2)
    pub fn mod_(&self) -> i32 {
        unsafe { (*self.inner).mod_ }
    }

    /// Returns the pointer of the inner buffer.
    pub fn buffer(&mut self) -> *mut u32 {
        self.buf.as_mut_ptr() as *mut u32
    }
}

impl Drop for Fenster {
    fn drop(&mut self) {
        unsafe { fenster_close(self.inner) };
        unsafe { fenster_destroy(&mut self.inner) };
    }
}
