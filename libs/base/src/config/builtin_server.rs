//! The rendezvous/relay server and public key this client is built for.
//!
//! They go into the override settings, which win over every stored value and
//! which `Config::set_option(s)` refuses to save. So neither the settings
//! dialog, "Import server config", `--config`, `--option` nor
//! `--import-config` can point the client somewhere else.

use super::keys;
use hbb_common::config::{BUILTIN_SETTINGS, OVERWRITE_SETTINGS};
use std::sync::PoisonError;

pub const HOST: &str = "emily.siriuslabs.it";
pub const KEY: &str = "0hCCPq2v8kqOfZT1HvtTwce6Rhb3x29WqksWmT1WL60=";

const OVERRIDE: [(&str, &str); 4] = [
    (keys::OPTION_CUSTOM_RENDEZVOUS_SERVER, HOST),
    (keys::OPTION_RELAY_SERVER, HOST),
    (keys::OPTION_KEY, KEY),
    // RustDesk Server OSS has no API server.
    (keys::OPTION_API_SERVER, ""),
];

const BUILTIN: [(&str, &str); 2] = [
    (keys::OPTION_REGISTER_DEVICE, "N"),
    (keys::OPTION_HIDE_SERVER_SETTINGS, "Y"),
];

/// Runs after every `load_custom_client()`, so a `custom.txt` cannot replace
/// these values either.
pub fn lock() {
    let mut map = OVERWRITE_SETTINGS
        .write()
        .unwrap_or_else(PoisonError::into_inner);
    for (k, v) in OVERRIDE {
        map.insert(k.to_owned(), v.to_owned());
    }
    drop(map);
    let mut map = BUILTIN_SETTINGS
        .write()
        .unwrap_or_else(PoisonError::into_inner);
    for (k, v) in BUILTIN {
        map.insert(k.to_owned(), v.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hbb_common::config::{Config, PROD_RENDEZVOUS_SERVER, RENDEZVOUS_SERVERS, RS_PUB_KEY};

    #[test]
    fn locked_server_is_the_one_in_use() {
        lock();
        assert_eq!(Config::get_rendezvous_server(), format!("{HOST}:21116"));
        assert_eq!(Config::get_rendezvous_servers(), vec![HOST.to_owned()]);
        assert_eq!(Config::get_option(keys::OPTION_RELAY_SERVER), HOST);
        assert_eq!(Config::get_option(keys::OPTION_KEY), KEY);
        assert_eq!(Config::get_option(keys::OPTION_API_SERVER), "");
        assert!(Config::no_register_device());
    }

    // The hbb_common fallbacks come from .github/sirius/hbb_common.patch; a
    // build or a submodule bump without it would bring rs-ny.rustdesk.com back.
    #[test]
    fn hbb_common_fallbacks_are_the_same_server() {
        assert_eq!(RENDEZVOUS_SERVERS, &[HOST]);
        assert_eq!(RS_PUB_KEY, KEY);
        assert_eq!(*PROD_RENDEZVOUS_SERVER.read().unwrap(), HOST);
    }
}
