use chan::Sender;

use super::Signal;

#[doc(hidden)]
pub fn _notify_on(chan: &Sender<Signal>, signal: Signal) {
    // TODO
}

#[doc(hidden)]
pub fn _block(signals: &[Signal]) {
    // TODO
}

#[doc(hidden)]
pub fn _block_all_subscribable() {
    // TODO
}