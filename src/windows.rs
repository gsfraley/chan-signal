use chan::Sender;
use super::Signal;

#[doc(hidden)]
pub fn _notify_on(_chan: &Sender<Signal>, _signal: Signal) {
    // TODO
}

#[doc(hidden)]
pub fn _block(_signals: &[Signal]) {
    // TODO
}

#[doc(hidden)]
pub fn _block_all_subscribable() {
    // TODO
}