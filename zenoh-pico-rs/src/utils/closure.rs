use core::ffi::c_void;

pub(crate) trait IntoFFIClosure<Args, Ret> {
    type Closure;

    fn to_ffi(&mut self) -> (*mut c_void, Self::Closure);
}

macro_rules! impl_into_ffi_closure {
    ($( $num:tt: ( $( $ty:ident ),* ) ),* $(,)?) => {
        $(
            impl<Func, Ret, $( $ty ),*> IntoFFIClosure<($( $ty, )*), Ret> for Func
            where
                Func: FnMut($( $ty ),*) -> Ret,
            {
                type Closure = unsafe extern "C" fn($( $ty ),*, *mut c_void) -> Ret;

                fn to_ffi(&mut self) -> (*mut c_void, Self::Closure) {
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn trampoline<T, Ret, $( $ty ),*>( $( $ty: $ty ),*, ptr: *mut c_void) -> Ret
                    where
                        T: FnMut($( $ty ),*) -> Ret,
                    {
                        debug_assert!(!ptr.is_null());

                        let callback: &mut T = unsafe { &mut *(ptr as *mut T) };

                        callback($( $ty ),*)
                    }

                    (self as *mut Func as *mut c_void, trampoline::<Func, Ret, $( $ty ),*>)
                }
            }
        )*
    };
}

impl_into_ffi_closure!(
    1: (A),
    2: (A, B),
    3: (A, B, C),
    4: (A, B, C, D),
    5: (A, B, C, D, E),
    6: (A, B, C, D, E, F),
);
