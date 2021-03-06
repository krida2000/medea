//! WebAPI objects mocks.

pub mod media_devices;
pub mod websocket;

use super::Window;

pub use self::{media_devices::MediaDevices, websocket::WebSocket};

/// Instantiates all the required mocks in the provided [`Window`].
pub async fn instantiate_mocks(window: &Window) {
    WebSocket::instantiate(window).await;
    MediaDevices::instantiate(window).await;
}

impl Window {
    /// Returns a `WebSocket` object mock for this [`Window`].
    #[must_use]
    pub fn websocket_mock(&self) -> WebSocket<'_> {
        WebSocket(self)
    }

    /// Returns `MediaDevices` interface mock for this [`Window`].
    #[must_use]
    pub fn media_devices_mock(&self) -> MediaDevices<'_> {
        MediaDevices(self)
    }
}
