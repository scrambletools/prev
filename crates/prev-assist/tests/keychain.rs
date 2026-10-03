//! Keeps, reads and removes a key in the system's keychain. Ignored by
//! default, as it needs a keychain (the Secret Service on Linux) and may
//! ask the user to unlock it:
//!
//!     cargo test -p prev-assist --test keychain -- --ignored

use prev_assist::keys;

#[test]
#[ignore = "uses the system keychain"]
fn keys_are_kept_read_and_removed() {
    let (service, id) = ("prev-test", "keychain-round-trip");
    keys::delete(service, id).unwrap();
    assert_eq!(keys::get(service, id).unwrap(), None);
    keys::set(service, id, "first").unwrap();
    assert_eq!(keys::get(service, id).unwrap().as_deref(), Some("first"));
    keys::set(service, id, "second").unwrap();
    assert_eq!(keys::get(service, id).unwrap().as_deref(), Some("second"));
    keys::delete(service, id).unwrap();
    assert_eq!(keys::get(service, id).unwrap(), None);
    // Removing a key that is not there is fine.
    keys::delete(service, id).unwrap();
}
