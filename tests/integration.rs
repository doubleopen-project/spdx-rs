// SPDX-FileCopyrightText: 2021 HH Partners
//
// SPDX-License-Identifier: MIT

use anyhow::Error;
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde_json::from_str as json_from_str;
use serde_json::Error as SerdeJsonError;
use spdx_rs::models::SPDX;
use spdx_rs::parsers::spdx_from_tag_value;
use std::fs::read_to_string;
use std::path::Path;
use std::result::Result as StdResult;

#[test]
fn deserialize_json_v2_2() -> Result<()> {
    Ok(load_spdx("tests/data/SPDXJSONExample-v2.2.spdx.json", spdx_from_json).map(ignore)?)
}

#[test]
fn deserialize_json_v2_3() -> Result<()> {
    Ok(load_spdx("tests/data/SPDXJSONExample-v2.3.spdx.json", spdx_from_json).map(ignore)?)
}

#[test]
fn deserialize_tag_value_v2_2() -> Result<()> {
    Ok(load_spdx("tests/data/SPDXTagExample-v2.2.spdx", spdx_from_tag_value).map(ignore)?)
}

#[test]
fn deserialize_tag_value_v2_3() -> Result<()> {
    Ok(load_spdx("tests/data/SPDXTagExample-v2.3.spdx", spdx_from_tag_value).map(ignore)?)
}

/// Helper function for ignoring a value.
fn ignore<T>(_: T) {
    ()
}

/// Helper function which tightens the trait bounds on serde_json::from_str.
fn spdx_from_json<T>(s: &str) -> StdResult<T, SerdeJsonError>
where
    T: DeserializeOwned,
{
    json_from_str(s)
}

/// Helper function to try calling the given parsing function on the contents of the given file.
fn load_spdx<P, F, E>(path: P, parser: F) -> Result<SPDX, Error>
where
    P: AsRef<Path>,
    F: FnOnce(&str) -> Result<SPDX, E>,
    Error: From<E>,
{
    let result: SPDX = parser(&read_to_string(path.as_ref())?)?;
    Ok(result)
}
