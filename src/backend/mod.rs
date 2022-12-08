//!
//! Contains backend specific types.
//! Most of the type user would use re-exports in the crate root.
//!

macro_rules! if_backend_than {
    ($back:ident ($check:meta) => $($than:tt)*) => {
        #[$check]
        mod $back {
            $($than)*
        }
    };
}

macro_rules! single_backed_check {
    ($( $back:ident ($check:meta) )*) => {
        mod check {
            single_backed_check!(impl $($back ($check))*);
        }
    };

    (impl) => {};

    (impl $head_back:ident ($head_check:meta) $($tail_back:ident ($tail_check:meta))*) => {
        if_backend_than!($head_back ($head_check) => $(#[$tail_check] compile_error!(concat!("Only one backed can be enabled. Attempt to enable '", stringify!($head_back), "' and '", stringify!($tail_back), "'")); )*);
        single_backed_check!(impl $($tail_back ($tail_check))*);
    };
}

single_backed_check! {
    vulkan (cfg(feature = "vulkan"))
    webgpu (cfg(feature = "webgpu"))
    dx12 (cfg(feature = "dx12"))
}

#[cfg(feature = "vulkan")]
mod vulkan;

#[cfg(feature = "vulkan")]
pub use vulkan::*;

#[cfg(feature = "webgpu")]
mod webgpu;

#[cfg(feature = "webgpu")]
pub use webgpu::*;
