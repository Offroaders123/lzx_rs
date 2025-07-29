#[repr(transparent)]
pub struct LzxState;

unsafe extern "C" {
    pub fn lzx_init(window: i32) -> *mut LzxState;
    pub fn lzx_teardown(p_state: *mut LzxState) -> ();
    pub fn lzx_reset(p_state: *mut LzxState) -> ();
    pub fn lzx_decompress(
        p_state: *mut LzxState,
        inpos: *mut u8,
        outpos: *mut u8,
        inlen: i32,
        outlen: i32,
    ) -> i32;
}
