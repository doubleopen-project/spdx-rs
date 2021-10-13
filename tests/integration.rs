// SPDX-FileCopyrightText: 2021 HH Partners
//
// SPDX-License-Identifier: MIT

use std::fs::read_to_string;

use spdx_rs::{error::SpdxError, models::SPDX, parsers::spdx_from_tag_value};

#[test]
fn deserialize_json() -> Result<(), SpdxError> {
    let spdx_file = read_to_string("tests/data/SPDXJSONExample-v2.2.spdx.json")?;
    let spdx_document: SPDX = serde_json::from_str(&spdx_file)?;

    assert_eq!(
        spdx_document.document_creation_information.spdx_identifier,
        "SPDXRef-DOCUMENT"
    );
    Ok(())
}

#[test]
fn deserialize_tag_value() -> Result<(), SpdxError> {
    let spdx_file = read_to_string("tests/data/SPDXTagExample-v2.2.spdx")?;
    let spdx_document: SPDX = spdx_from_tag_value(&spdx_file)?;

    assert_eq!(
        spdx_document.document_creation_information.spdx_identifier,
        "SPDXRef-DOCUMENT"
    );
    Ok(())
}
