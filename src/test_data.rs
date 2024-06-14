use std::path::Path;

use enostr::{FullKeypair, Pubkey, RelayPool};
use nostrdb::ProfileRecord;

use crate::{account_manager::UserAccount, Damus};

#[allow(unused_must_use)]
pub fn sample_pool() -> RelayPool {
    let mut pool = RelayPool::new();
    let wakeup = move || {};

    pool.add_url("wss://relay.damus.io".to_string(), wakeup);
    pool.add_url("wss://eden.nostr.land".to_string(), wakeup);
    pool.add_url("wss://nostr.wine".to_string(), wakeup);
    pool.add_url("wss://nos.lol".to_string(), wakeup);
    pool.add_url("wss://test_relay_url_long_00000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(), wakeup);

    for _ in 0..20 {
        pool.add_url("tmp".to_string(), wakeup);
    }

    pool
}

// my (jb55) profile
const TEST_PROFILE_DATA: [u8; 448] = [
    0x04, 0x00, 0x00, 0x00, 0x54, 0xfe, 0xff, 0xff, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xd6, 0xd9, 0xc6, 0x65, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0a, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x66, 0x69, 0x78, 0x6d,
    0x65, 0x00, 0x00, 0x00, 0x78, 0x01, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xda, 0xff, 0xff, 0xff,
    0x64, 0x01, 0x00, 0x00, 0x50, 0x01, 0x00, 0x00, 0x34, 0x01, 0x00, 0x00, 0x08, 0x01, 0x00, 0x00,
    0xec, 0x00, 0x00, 0x00, 0xdc, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x1c, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x16, 0x00, 0x24, 0x00, 0x18, 0x00, 0x14, 0x00, 0x20, 0x00, 0x0c, 0x00, 0x1c, 0x00,
    0x04, 0x00, 0x00, 0x00, 0x10, 0x00, 0x08, 0x00, 0x52, 0x00, 0x00, 0x00, 0x49, 0x20, 0x6d, 0x61,
    0x64, 0x65, 0x20, 0x64, 0x61, 0x6d, 0x75, 0x73, 0x2c, 0x20, 0x6e, 0x70, 0x75, 0x62, 0x73, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x7a, 0x61, 0x70, 0x73, 0x2e, 0x20, 0x62, 0x61, 0x6e, 0x6e, 0x65, 0x64,
    0x20, 0x62, 0x79, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x65, 0x20, 0x26, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x63, 0x63, 0x70, 0x2e, 0x20, 0x6d, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x65, 0x73, 0x20, 0x61, 0x72,
    0x65, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x61, 0x6c, 0x65, 0x00, 0x00,
    0x5a, 0x00, 0x00, 0x00, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x6e, 0x6f, 0x73, 0x74,
    0x72, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2f, 0x69, 0x2f, 0x33, 0x64, 0x36, 0x66, 0x32, 0x32,
    0x64, 0x34, 0x35, 0x64, 0x39, 0x35, 0x65, 0x63, 0x63, 0x32, 0x63, 0x31, 0x39, 0x62, 0x31, 0x61,
    0x63, 0x64, 0x65, 0x63, 0x35, 0x37, 0x61, 0x61, 0x31, 0x35, 0x66, 0x32, 0x64, 0x62, 0x61, 0x39,
    0x63, 0x34, 0x32, 0x33, 0x62, 0x35, 0x33, 0x36, 0x65, 0x32, 0x36, 0x66, 0x63, 0x36, 0x32, 0x37,
    0x30, 0x37, 0x63, 0x31, 0x32, 0x35, 0x66, 0x35, 0x35, 0x37, 0x2e, 0x6a, 0x70, 0x67, 0x00, 0x00,
    0x04, 0x00, 0x00, 0x00, 0x6a, 0x62, 0x35, 0x35, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
    0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x64, 0x61, 0x6d, 0x75, 0x73, 0x2e, 0x69, 0x6f,
    0x00, 0x00, 0x00, 0x00, 0x23, 0x00, 0x00, 0x00, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f,
    0x63, 0x64, 0x6e, 0x2e, 0x6a, 0x62, 0x35, 0x35, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x69, 0x6d, 0x67,
    0x2f, 0x72, 0x65, 0x64, 0x2d, 0x6d, 0x65, 0x2e, 0x6a, 0x70, 0x67, 0x00, 0x11, 0x00, 0x00, 0x00,
    0x6a, 0x62, 0x35, 0x35, 0x40, 0x73, 0x65, 0x6e, 0x64, 0x73, 0x61, 0x74, 0x73, 0x2e, 0x6c, 0x6f,
    0x6c, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x5f, 0x40, 0x6a, 0x62, 0x35, 0x35, 0x2e, 0x63,
    0x6f, 0x6d, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x57, 0x69, 0x6c, 0x6c, 0x00, 0x00, 0x00, 0x00,
    0x0c, 0x00, 0x24, 0x00, 0x04, 0x00, 0x0c, 0x00, 0x1c, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const TEST_PUBKEY: [u8; 32] = [
    0x32, 0xe1, 0x82, 0x76, 0x35, 0x45, 0x0e, 0xbb, 0x3c, 0x5a, 0x7d, 0x12, 0xc1, 0xf8, 0xe7, 0xb2,
    0xb5, 0x14, 0x43, 0x9a, 0xc1, 0x0a, 0x67, 0xee, 0xf3, 0xd9, 0xfd, 0x9c, 0x5c, 0x68, 0xe2, 0x45,
];

pub fn test_pubkey() -> &'static [u8; 32] {
    &TEST_PUBKEY
}

pub fn test_profile_record() -> ProfileRecord<'static> {
    ProfileRecord::new_owned(&TEST_PROFILE_DATA).unwrap()
}

const TEN_ACCOUNT_HEXES: [&str; 10] = [
    "3efdaebb1d8923ebd99c9e7ace3b4194ab45512e2be79c1b7d68d9243e0d2681",
    "32e1827635450ebb3c5a7d12c1f8e7b2b514439ac10a67eef3d9fd9c5c68e245",
    "bd1e19980e2c91e6dc657e92c25762ca882eb9272d2579e221f037f93788de91",
    "5c10ed0678805156d39ef1ef6d46110fe1e7e590ae04986ccf48ba1299cb53e2",
    "4c96d763eb2fe01910f7e7220b7c7ecdbe1a70057f344b9f79c28af080c3ee30",
    "edf16b1dd61eab353a83af470cc13557029bff6827b4cb9b7fc9bdb632a2b8e6",
    "3efdaebb1d8923ebd99c9e7ace3b4194ab45512e2be79c1b7d68d9243e0d2681",
    "32e1827635450ebb3c5a7d12c1f8e7b2b514439ac10a67eef3d9fd9c5c68e245",
    "32e1827635450ebb3c5a7d12c1f8e7b2b514439ac10a67eef3d9fd9c5c68e245",
    "32e1827635450ebb3c5a7d12c1f8e7b2b514439ac10a67eef3d9fd9c5c68e245",
];

pub fn get_test_accounts() -> Vec<UserAccount> {
    TEN_ACCOUNT_HEXES
        .iter()
        .map(|account_hex| {
            let mut kp = FullKeypair::generate().to_keypair();
            kp.pubkey = Pubkey::from_hex(account_hex).unwrap();
            kp
        })
        .collect()
}

pub fn test_app(is_mobile: bool) -> Damus {
    let db_dir = Path::new(".");
    let path = db_dir.to_str().unwrap();
    let mut app = Damus::mock(path, is_mobile);

    let accounts = get_test_accounts();
    for account in accounts {
        app.account_manager.add_account(account);
    }

    app
}
