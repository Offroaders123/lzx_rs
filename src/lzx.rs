#[repr(C)]
struct LzxState {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn lzx_init(window: i32) -> *mut LzxState;
    fn lzx_teardown(p_state: *mut LzxState) -> ();
    fn lzx_reset(p_state: *mut LzxState) -> ();
    fn lzx_decompress(
        p_state: *mut LzxState,
        inpos: *mut u8,
        outpos: *mut u8,
        inlen: i32,
        outlen: i32,
    ) -> i32;
}

#[derive(Debug)]
pub enum LzxError {
    DataFormat,
    IllegalData,
    NoMemory,
    Unknown(i32),
}

pub struct Lzx {
    state: *mut LzxState,
}

impl Lzx {
    pub fn new(window_size: i32) -> Result<Self, LzxError> {
        let state: *mut LzxState = unsafe { lzx_init(window_size) };
        if state.is_null() {
            return Err(LzxError::NoMemory);
        }
        Ok(Self { state })
    }

    pub fn reset(&mut self) {
        unsafe {
            lzx_reset(self.state);
        }
    }
}

impl Drop for Lzx {
    fn drop(&mut self) {
        unsafe {
            lzx_teardown(self.state);
        }
    }
}

impl Lzx {
    pub fn decompress(&mut self, input: &mut [u8], output: &mut [u8]) -> Result<(), LzxError> {
        let result: i32 = unsafe {
            lzx_decompress(
                self.state,
                input.as_mut_ptr(),
                output.as_mut_ptr(),
                input.len() as i32,
                output.len() as i32,
            )
        };

        match result {
            0 => Ok(()),
            1 => Err(LzxError::DataFormat),
            2 => Err(LzxError::IllegalData),
            3 => Err(LzxError::NoMemory),
            code => Err(LzxError::Unknown(code)),
        }
    }
}
