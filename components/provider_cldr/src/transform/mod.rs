// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod dates;
mod likelysubtags;
mod numbers;
mod plurals;
mod timezones;

pub use dates::DatesProvider;
pub use likelysubtags::LikelySubtagsProvider;
pub use numbers::NumbersProvider;
pub use plurals::PluralsProvider;

use crate::support::LazyCldrProvider;
use crate::CldrPaths;
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use icu_provider::serde::SerdeSeDataStruct;

use self::timezones::TimeZonesProvider;

/// Returns a list of all ResourceKeys that this provider can produce.
pub fn get_all_resc_keys() -> Vec<ResourceKey> {
    let mut result: Vec<ResourceKey> = vec![];
    result.extend(&dates::ALL_KEYS);
    result.extend(&likelysubtags::ALL_KEYS);
    result.extend(&numbers::ALL_KEYS);
    result.extend(&plurals::ALL_KEYS);
    result.extend(&timezones::ALL_KEYS);
    result
}

#[derive(Debug)]
pub struct CldrJsonDataProvider<'a, 'd> {
    pub cldr_paths: &'a dyn CldrPaths,
    dates: LazyCldrProvider<DatesProvider<'d>>,
    likelysubtags: LazyCldrProvider<LikelySubtagsProvider<'d>>,
    numbers: LazyCldrProvider<NumbersProvider>,
    plurals: LazyCldrProvider<PluralsProvider<'d>>,
    timezones: LazyCldrProvider<TimeZonesProvider<'d>>,
}

impl<'a, 'd> CldrJsonDataProvider<'a, 'd> {
    pub fn new(cldr_paths: &'a dyn CldrPaths) -> Self {
        CldrJsonDataProvider {
            cldr_paths,
            dates: Default::default(),
            likelysubtags: Default::default(),
            numbers: Default::default(),
            plurals: Default::default(),
            timezones: Default::default(),
        }
    }
}

impl<'a, 'd, 's: 'd> DataProvider<'d, dyn SerdeSeDataStruct<'s> + 's>
    for CldrJsonDataProvider<'a, 'd>
{
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, dyn SerdeSeDataStruct<'s> + 's>, DataError> {
        if let Some(result) = self.dates.try_load_serde(req, self.cldr_paths)? {
            return Ok(result);
        }
        if let Some(result) = self.likelysubtags.try_load_serde(req, self.cldr_paths)? {
            return Ok(result);
        }
        if let Some(result) = self.numbers.try_load_serde(req, self.cldr_paths)? {
            return Ok(result);
        }
        if let Some(result) = self.plurals.try_load_serde(req, self.cldr_paths)? {
            return Ok(result);
        }
        if let Some(result) = self.timezones.try_load_serde(req, self.cldr_paths)? {
            return Ok(result);
        }
        Err(DataError::UnsupportedResourceKey(req.resource_path.key))
    }
}

impl<'a, 'd> IterableDataProviderCore for CldrJsonDataProvider<'a, 'd> {
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        if let Some(resp) = self
            .dates
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(resp);
        }
        if let Some(resp) = self
            .likelysubtags
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(resp);
        }
        if let Some(resp) = self
            .numbers
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(resp);
        }
        if let Some(resp) = self
            .plurals
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(resp);
        }
        if let Some(resp) = self
            .timezones
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(resp);
        }
        Err(DataError::UnsupportedResourceKey(*resc_key))
    }
}

impl<'a, 'd> KeyedDataProvider for CldrJsonDataProvider<'a, 'd> {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        PluralsProvider::supports_key(resc_key)
            .or_else(|err| DatesProvider::or_else_supports_key(err, resc_key))
    }
}
