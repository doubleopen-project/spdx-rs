// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// <https://spdx.github.io/spdx-spec/appendix-IV-SPDX-license-expressions/>
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SPDXExpression(pub String);

impl SPDXExpression {

    /// Get licenses from the expression.
    pub fn licenses(&self) -> Vec<String> {
        let licenses = self.0.split_ascii_whitespace();
        let licenses = licenses.filter(|&i| i != "OR" && i != "AND" && i != "WITH");
        let licenses = licenses.map(|i| i.replace("(", "").replace(")", ""));
        licenses.collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test_spdx_expression {
    use super::*;

    #[test]
    fn get_separated_licenses() {
        let input_1 = SPDXExpression("MIT OR GPL-2.0-or-later".into());
        let input_2 = SPDXExpression("MIT OR (GPL-2.0-or-later AND ISC)".into());
        let input_3 = SPDXExpression("MIT AND (GPL-2.0-or-later AND ISC) AND BSD-3-Clause".into());

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
