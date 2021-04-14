// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// A FFI to get DateFormat information

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

use icu_datetime::{options::length, DateTimeFormat};
use icu_locid::Locale;
use icu_locid_macros::langid;
use icu_provider_fs::FsDataProvider;
use std::path::PathBuf;

#[no_mangle]
pub extern fn GetDateFormat(locale: *const i8, value: *const i8, value_length: i32) -> usize {
    // if locale == 0 && value == 0
    // {
    //     return 0
    // }

    let locale: Locale = langid!("en").into();

    let provider = get_provider();

    let options = length::Bag {
        date: Some(length::Date::Medium),
        time: Some(length::Time::Short),
        ..Default::default()
    };

    let dtf = DateTimeFormat::try_new(locale, &provider, &options.into())
        .expect("Failed to create DateTimeFormat instance.");

    return dtf.symbols.months.format.wide.0[7].len();
}

#[no_mangle]
pub extern fn GetDateFormat2(value_length: usize) -> usize {
    // if locale == 0 && value == 0
    // {
    //     return 0
    // }

    let locale: Locale = langid!("en").into();

    let provider = get_provider();

    let options = length::Bag {
        date: Some(length::Date::Medium),
        time: Some(length::Time::Short),
        ..Default::default()
    };

    let dtf = DateTimeFormat::try_new(locale, &provider, &options.into())
        .expect("Failed to create DateTimeFormat instance.");

    return dtf.symbols.months.format.wide.0[7].len() + value_length;
}

/// Get a `DataProvider` loading from test data. Panics if unable to load the data.
pub fn get_provider() -> FsDataProvider {
    let path: PathBuf = match std::env::var_os("ICU4X_TESTDATA_DIR") {
        Some(val) => val.into(),
        None => panic!("ERIC: The env var couldn't be read"),
    };

    FsDataProvider::try_new(&path).unwrap_or_else(|err| {
        panic!(
            "The test data directory was unable to be opened: {}: {:?}",
            err, path
        )
    })
}
