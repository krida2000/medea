use std::{os::raw::c_char, ptr};

use super::{propagate_panic, utils::string_into_c_str, ForeignClass};

pub use crate::room::RoomCloseReason;

impl ForeignClass for RoomCloseReason {}

/// Returns a close reason of a [`Room`].
///
/// [`Room`]: crate::room::Room
#[no_mangle]
pub unsafe extern "C" fn RoomCloseReason__reason(
    this: ptr::NonNull<RoomCloseReason>,
) -> ptr::NonNull<c_char> {
    propagate_panic(move || string_into_c_str(this.as_ref().reason()))
}

/// Indicates whether a [`Room`] was closed by server.
///
/// [`Room`]: crate::room::Room
#[no_mangle]
pub unsafe extern "C" fn RoomCloseReason__is_closed_by_server(
    this: ptr::NonNull<RoomCloseReason>,
) -> u8 {
    propagate_panic(move || this.as_ref().is_closed_by_server().into())
}

/// Indicates whether a [`Room`]'s close reason is considered as an error.
///
/// [`Room`]: crate::room::Room
#[no_mangle]
pub unsafe extern "C" fn RoomCloseReason__is_err(
    this: ptr::NonNull<RoomCloseReason>,
) -> u8 {
    propagate_panic(move || this.as_ref().is_err().into())
}

/// Frees the data behind the provided pointer.
///
/// # Safety
///
/// Should be called when object is no longer needed. Calling this more than
/// once for the same pointer is equivalent to double free.
#[no_mangle]
pub unsafe extern "C" fn RoomCloseReason__free(
    this: ptr::NonNull<RoomCloseReason>,
) {
    propagate_panic(move || {
        drop(RoomCloseReason::from_ptr(this));
    });
}
