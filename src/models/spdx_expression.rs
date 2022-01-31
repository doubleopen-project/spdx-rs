// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use spdx::Expression;

use crate::error::SpdxError;

/// <https://spdx.github.io/spdx-spec/appendix-IV-SPDX-license-expressions/>
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SPDXExpression(String);

impl Default for SPDXExpression {
    fn default() -> Self {
        Self::new("NOASSERTION")
    }
}

impl SPDXExpression {
    /// Create a new `SPDXExpression`. Does not check for the validity of the expression, all
    /// strings will succeed.
    pub fn new(expression: &str) -> Self {
        Self(expression.to_string())
    }

    /// # Errors
    ///
    /// Returns [`SpdxError`] if parsing of the epxression fails.
    pub fn expression(&self) -> Result<Expression, SpdxError> {
        Expression::parse(&self.0).map_err(|err| SpdxError::Parse(err.to_string()))
    }

    /// Get licenses from the expression.
    ///
    /// # Errors
    ///
    /// Returns [`SpdxError`] if parsing of the epxression fails.
    pub fn licenses(&self) -> Result<Vec<String>, SpdxError> {
        Ok(self
            .expression()?
            .requirements()
            .map(|req| req.req.to_string())
            .collect::<Vec<_>>())
    }
}

impl Deref for SPDXExpression {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SPDXExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test_spdx_expression {
    use super::*;

    #[test]
    fn get_separated_licenses() {
        let input_1 = SPDXExpression::new("MIT OR GPL-2.0-or-later");
        let input_2 = SPDXExpression::new("MIT OR (GPL-2.0-or-later AND ISC)");
        let input_3 = SPDXExpression::new("MIT AND (GPL-2.0-or-later AND ISC) AND BSD-3-Clause");

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

        let mut actual_1 = input_1.licenses().unwrap();
        let mut actual_2 = input_2.licenses().unwrap();
        let mut actual_3 = input_3.licenses().unwrap();

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
