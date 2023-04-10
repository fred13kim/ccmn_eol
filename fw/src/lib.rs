#![feature(asm_experimental_arch)]
#![feature(asm_const)]

mod imports {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    pub mod ember_tasking {
        include!(concat!(env!("OUT_DIR"), "/ember_tasking.rs"));
    }

    pub mod opencan {
        pub mod rx {
            include!(concat!(env!("OUT_DIR"), "/opencan_rx.rs"));
        }

        pub mod tx {
            include!(concat!(env!("OUT_DIR"), "/opencan_tx.rs"));
        }

        pub mod callbacks {
            include!(concat!(env!("OUT_DIR"), "/opencan_callbacks.rs"));
        }

        macro_rules! canrx {
            ($signal:ident) => {
                paste::paste! {
                    unsafe { crate::opencan::rx::[<CANRX_get_ $signal>]() }
                }
            };
        }

        pub(crate) use canrx;

        macro_rules! canrx_is_node_ok {
            ($node:ident) => {
                paste::paste! {
                    unsafe { crate::opencan::rx::[<CANRX_is_node_ $node _ok>]() }
                }
            };
        }

        pub(crate) use canrx_is_node_ok;
    }

    pub mod pins {
        include!(concat!(env!("OUT_DIR"), "/node_pins.rs"));
    }
}

use imports::{ember_tasking, opencan, pins};

mod sys;
pub use sys::*;

mod entry;

mod leds;
mod util;