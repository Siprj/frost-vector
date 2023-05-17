use std::mem::size_of;

pub trait Raw {
    fn get_raw(&self) -> &[u8];
}

impl<T: Sized> Raw for T {
    fn get_raw(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts((self as *const T) as *const u8, size_of::<T>()) }
    }
}

impl<T: Sized> Raw for [T] {
    fn get_raw(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self as *const [T]) as *const u8,
                size_of::<T>() * self.len(),
            )
        }
    }
}
