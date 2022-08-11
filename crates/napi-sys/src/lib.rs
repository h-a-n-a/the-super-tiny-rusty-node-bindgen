pub type napi_env = *mut napi_env__;

#[repr(C)]
struct napi_env__ {
    _unused: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type napi_status = i32;

extern "C" {
    pub fn napi_get_version(env: napi_env, version: *mut u32) -> napi_status;
}
