//! Keyboard state events

use rmk_macro::event;
use rmk_types::led_indicator::LedIndicator;

/// Active layer changed event
#[event(channel_size = crate::LAYER_CHANGE_EVENT_CHANNEL_SIZE, pubs = crate::LAYER_CHANGE_EVENT_PUB_SIZE, subs = crate::LAYER_CHANGE_EVENT_SUB_SIZE)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LayerChangeEvent(pub u8);

impl LayerChangeEvent {
    pub fn new(layer: u8) -> Self {
        Self(layer)
    }
}

impl_payload_wrapper!(LayerChangeEvent, u8);

/// Default layout (base/default layer) changed event.
///
/// Published when the default layer is changed at runtime (e.g. by the
/// Windows/macOS layout toggle). The payload is the new default layer index.
/// Unlike [`LayerChangeEvent`], which fires for any momentary layer change,
/// this fires only when the persistent default layer is updated.
#[event(channel_size = 1, pubs = 1, subs = 1)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DefaultLayoutChangeEvent(pub u8);

impl DefaultLayoutChangeEvent {
    pub fn new(layer: u8) -> Self {
        Self(layer)
    }
}

impl_payload_wrapper!(DefaultLayoutChangeEvent, u8);

/// Request to briefly show the battery-level gauge on the RGB strip.
///
/// Published on the central when the "show battery" user key is pressed
/// (see `process_user`), and forwarded to the peripheral over the split link
/// (`SplitMessage::ShowBattery`) so each half flashes its own gauge. The RGB
/// processors subscribe and light their five gauge LEDs for a few seconds.
///
/// Carries no payload — it is a one-shot pulse. `subs = 3` covers the central's
/// two subscribers (RGB processor + split driver forwarder); the peripheral
/// only has one (its RGB processor).
#[event(channel_size = 2, pubs = 1, subs = 3)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ShowBatteryEvent;

/// Toggle whether the RGB charging indicator is shown.
///
/// Published on the central when the "toggle charging indicator" user key is
/// pressed, and forwarded to the peripheral (`SplitMessage::ToggleChargingIndicator`)
/// so both halves flip together. Each RGB processor keeps its own enable flag
/// (not persisted — resets to on at boot); receiving this pulse inverts it.
/// Same `subs = 3` layout as [`ShowBatteryEvent`].
#[event(channel_size = 2, pubs = 1, subs = 3)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ToggleChargingIndicatorEvent;

/// WPM updated event
#[event(channel_size = crate::WPM_UPDATE_EVENT_CHANNEL_SIZE, pubs = crate::WPM_UPDATE_EVENT_PUB_SIZE, subs = crate::WPM_UPDATE_EVENT_SUB_SIZE)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct WpmUpdateEvent(pub u16);

impl WpmUpdateEvent {
    pub fn new(wpm: u16) -> Self {
        Self(wpm)
    }
}

impl_payload_wrapper!(WpmUpdateEvent, u16);

/// LED indicator state changed event
#[event(channel_size = crate::LED_INDICATOR_EVENT_CHANNEL_SIZE, pubs = crate::LED_INDICATOR_EVENT_PUB_SIZE, subs = crate::LED_INDICATOR_EVENT_SUB_SIZE)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LedIndicatorEvent(pub LedIndicator);

impl LedIndicatorEvent {
    pub fn new(indicator: LedIndicator) -> Self {
        Self(indicator)
    }
}

impl_payload_wrapper!(LedIndicatorEvent, LedIndicator);

/// Sleep state changed event
#[event(channel_size = crate::SLEEP_STATE_EVENT_CHANNEL_SIZE, pubs = crate::SLEEP_STATE_EVENT_PUB_SIZE, subs = crate::SLEEP_STATE_EVENT_SUB_SIZE)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SleepStateEvent(pub bool);

impl SleepStateEvent {
    pub fn new(sleeping: bool) -> Self {
        Self(sleeping)
    }
}

impl_payload_wrapper!(SleepStateEvent, bool);
