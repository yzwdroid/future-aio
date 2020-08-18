#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! cfg_std {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "std")]
            $item
        )*
    }
}

cfg_std! {
    pub mod fs;
    pub mod sync;
    pub mod task;
    pub mod timer;
    pub mod actor;

    #[cfg(unix)]
    pub mod zero_copy;
}

pub mod io;


#[cfg(any(test,feature = "fixture"))]
mod test_util;


#[cfg(any(test,feature = "fixture"))]
pub use async_test_derive::test_async;

pub mod net;


pub mod bytes {
    pub use bytes::Bytes;
    pub use bytes::BytesMut;
    pub use bytes::BufMut;
}


pub mod util {
    pub use flv_util::*;
}

