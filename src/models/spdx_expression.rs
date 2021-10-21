// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use std::ops::{Deref, DerefMut};

use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Serialize,
};
use spdx::Expression;

use crate::error::SpdxError;

/// <https://spdx.github.io/spdx-spec/appendix-IV-SPDX-license-expressions/>
#[derive(PartialEq, Debug, Clone)]
pub struct SPDXExpression(Expression);

impl Default for SPDXExpression {
    fn default() -> Self {
        Self(Expression::parse("NOASSERTION").expect("This will always succeed."))
    }
}

impl SPDXExpression {
    /// # Errors
    ///
    /// Returns [`SpdxError`] if parsing of the epxression fails.
    pub fn parse(expression: &str) -> Result<Self, SpdxError> {
        Ok(Self(Expression::parse(expression).map_err(|_err| {
            SpdxError::Parse("Error parsing SPDX".into())
        })?))
    }
    /// Get licenses from the expression.
    ///
    /// # Errors
    ///
    /// Returns [`SpdxError`] if parsing of the epxression fails.
    pub fn licenses(&self) -> Vec<String> {
        self.requirements()
            .map(|req| req.req.to_string())
            .collect::<Vec<_>>()
    }
}

impl Deref for SPDXExpression {
    type Target = Expression;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SPDXExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct SPDXExpressionVisitor;

impl<'de> Visitor<'de> for SPDXExpressionVisitor {
    type Value = SPDXExpression;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid SPDX Expression string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        SPDXExpression::parse(v).map_err(|_err| E::invalid_value(Unexpected::Str(v), &self))
    }
}

impl Serialize for SPDXExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SPDXExpression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(SPDXExpressionVisitor)
    }
}

#[cfg(test)]
mod test_spdx_expression {
    use super::*;

    #[test]
    fn get_separated_licenses() {
        let input_1 = SPDXExpression::parse("MIT OR GPL-2.0-or-later").unwrap();
        let input_2 = SPDXExpression::parse("MIT OR (GPL-2.0-or-later AND ISC)").unwrap();
        let input_3 =
            SPDXExpression::parse("MIT AND (GPL-2.0-or-later AND ISC) AND BSD-3-Clause").unwrap();

        let mut expected_1 = vec!["MIT".to_string(), "GPL-2.0-or-later".to_string()];
        let mut expected_2 = vec![
            "MIT".to_string(),
            "GPL-2.0-or-later".to_string(),
            "ISC".to_string(),
        ];
        let mut expected_3 = vec![
            "MIT".to_string(),
            "GPL-2.0-or-later".to_string(),
            "ISC".to_string(),
            "BSD-3-Clause".to_string(),
        ];

        let mut actual_1 = input_1.licenses();
        let mut actual_2 = input_2.licenses();
        let mut actual_3 = input_3.licenses();

        expected_1.sort();
        expected_2.sort();
        expected_3.sort();
        actual_1.sort();
        actual_2.sort();
        actual_3.sort();

        assert_eq!(actual_1, expected_1);
        assert_eq!(actual_2, expected_2);
        assert_eq!(actual_3, expected_3);
    }
}
