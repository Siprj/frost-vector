use std::mem::size_of;

pub trait Gpu {}

impl Gpu for u16 {}
impl Gpu for u32 {}
impl Gpu for f32 {}
impl Gpu for f64 {}

pub trait Raw {
    fn get_raw(&self) -> &[u8];
}

impl<T: Sized + Gpu> Raw for T {
    fn get_raw(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts((self as *const T) as *const u8, size_of::<T>()) }
    }
}

impl<T: Sized + Gpu> Raw for [T] {
    fn get_raw(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self as *const [T]) as *const u8,
                size_of::<T>() * self.len(),
            )
        }
    }
}

impl<T: Sized + Gpu> Raw for Vec<T> {
    fn get_raw(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self.as_slice() as *const [T]) as *const u8,
                size_of::<T>() * self.len(),
            )
        }
    }
}
