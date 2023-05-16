use std::mem::size_of;

pub unsafe fn make_raw<T: Sized>(data: &T) -> &[u8] {
    core::slice::from_raw_parts((data as *const T) as *const u8, size_of::<T>())
}

pub unsafe fn make_raw_slice<T: Sized>(data: &[T]) -> &[u8] {
    core::slice::from_raw_parts(
        (data as *const [T]) as *const u8,
        size_of::<T>() * data.len(),
    )
}

impl Raw for [u16] {
    fn get_raw(&self) -> &[u8] {
        unsafe { make_raw_slice(self) }
    }
}

pub trait Raw {
    fn get_raw(&self) -> &[u8];
}
