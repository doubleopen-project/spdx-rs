// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use boolean_expression::Expr;
use serde::{Deserialize, Serialize};

/// https://spdx.github.io/spdx-spec/appendix-IV-SPDX-license-expressions/
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SPDXExpression(pub String);

impl SPDXExpression {
    /// Create a boolean expression from the SPDX expression.
    pub fn parse(&self) -> Result<Expr<String>, String> {
        let expression = parser::parse_spdx(&self.0).map_err(|e| e.to_string())?;
        if expression.0.is_empty() {
            Ok(expression.1)
        } else {
            Err("Parsing error".into())
        }
    }

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
    fn simple() {
        let expression = SPDXExpression("MIT".to_string());
        let expr = expression.parse().unwrap();
        let expected = Expr::Terminal("MIT".into());
        assert_eq!(expr, expected)
    }

    #[test]
    fn error() {
        let expression = SPDXExpression("&/%/'ääö".to_string());
        let expr = expression.parse();
        assert_eq!(expr, Err("Parsing error".into()))
    }

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

/// Module for parsing of SPDX expressions.
mod parser {
    use boolean_expression::Expr;

    use nom::{
        branch::alt, bytes::complete::tag, bytes::complete::take_while, sequence::separated_pair,
        IResult,
    };

    /// Test if character is legal in SPDX license identifier.
    fn is_spdx_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '+'
    }

    /// Parse SPDX license expression.
    fn parse_simple_expression(i: &str) -> IResult<&str, Expr<String>> {
        let (i, expression) = take_while(is_spdx_char)(i)?;
        let expression = Expr::Terminal(expression.into());
        Ok((i, expression))
    }

    /// Parse SPDX license expression.
    fn parse_expression(i: &str) -> IResult<&str, Expr<String>> {
        let (i, expression) = alt((parse_with, parse_simple_expression))(i)?;
        Ok((i, expression))
    }
    /// Parse SPDX idstring
    fn parse_idstring(i: &str) -> IResult<&str, String> {
        let (i, expression) = take_while(is_spdx_char)(i)?;
        Ok((i, expression.into()))
    }
    /// Parse OR expression.
    fn parse_or(i: &str) -> IResult<&str, Expr<String>> {
        let (i, (left, right)) = separated_pair(
            alt((parse_and, parse_expression)),
            tag(" OR "),
            alt((parse_or, parse_and, parse_expression)),
        )(i)?;
        let expression = Expr::or(left, right);
        Ok((i, expression))
    }

    /// Parse AND expression.
    fn parse_and(i: &str) -> IResult<&str, Expr<String>> {
        let (i, (left, right)) = separated_pair(
            parse_expression,
            tag(" AND "),
            alt((parse_and, parse_expression)),
        )(i)?;
        let expression = Expr::and(left, right);
        Ok((i, expression))
    }

    /// Parse WITH expression.
    fn parse_with(i: &str) -> IResult<&str, Expr<String>> {
        let (i, (left, right)) = separated_pair(parse_idstring, tag(" WITH "), parse_idstring)(i)?;
        let expression = Expr::Terminal(format!("{} WITH {}", left, right));
        Ok((i, expression))
    }
    /// Combine the parsers.
    pub(crate) fn parse_spdx(i: &str) -> IResult<&str, Expr<String>> {
        alt((parse_or, parse_and, parse_expression))(i)
    }

    #[cfg(test)]
    mod test_spdx_parser {
        use std::collections::HashMap;

        use super::*;

        #[test]
        fn test_parse_expression() {
            let (_, output) = parse_expression("MIT").unwrap();
            let expected = Expr::Terminal("MIT".to_string());
            assert_eq!(output, expected);

            let (_, output) = parse_expression("GPL-3.0-or-later").unwrap();
            let expected = Expr::Terminal("GPL-3.0-or-later".to_string());
            assert_eq!(output, expected);
        }

        #[test]
        fn test_simple() {
            let expected = Expr::Terminal("MIT".to_string());
            let (_, output) = parse_spdx("MIT").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_with() {
            let expected = Expr::Terminal("GPL-2.0-only WITH Classpath-exception-2.0".to_string());
            let (_, output) = parse_spdx("GPL-2.0-only WITH Classpath-exception-2.0").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_or() {
            let left = Expr::Terminal("MIT".to_string());
            let right = Expr::Terminal("GPL-2.0-only".to_string());
            let expected = Expr::or(left, right);
            let (_, output) = parse_spdx("MIT OR GPL-2.0-only").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_and() {
            let left = Expr::Terminal("MIT".to_string());
            let right = Expr::Terminal("GPL-2.0-only".to_string());
            let expected = Expr::and(left, right);
            let (_, output) = parse_spdx("MIT AND GPL-2.0-only").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_and_with() {
            let left = Expr::Terminal("GPL-2.0-only WITH Classpath-exception-2.0".to_string());
            let right = Expr::Terminal("MIT".to_string());
            let expected = Expr::and(left, right);
            let (_, output) =
                parse_spdx("GPL-2.0-only WITH Classpath-exception-2.0 AND MIT").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_and_with_three() {
            let left = Expr::Terminal("MIT".to_string());
            let right_left = Expr::Terminal("GPL-2.0-only".to_string());
            let right_right = Expr::Terminal("BSD-3-Clause".to_string());
            let right = Expr::and(right_left, right_right);
            let expected = Expr::and(left, right);
            let (_, output) = parse_spdx("MIT AND GPL-2.0-only AND BSD-3-Clause").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_or_with_three() {
            let left = Expr::Terminal("MIT".to_string());
            let right_left = Expr::Terminal("GPL-2.0-only".to_string());
            let right_right = Expr::Terminal("BSD-3-Clause".to_string());
            let right = Expr::or(right_left, right_right);
            let expected = Expr::or(left, right);
            let (_, output) = parse_spdx("MIT OR GPL-2.0-only OR BSD-3-Clause").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_or_with_three_and() {
            let left = Expr::Terminal("MIT".to_string());
            let right_left = Expr::Terminal("GPL-2.0-only".to_string());
            let right_right = Expr::Terminal("BSD-3-Clause".to_string());
            let right = Expr::and(right_left, right_right);
            let expected = Expr::or(left, right);
            let (_, output) = parse_spdx("MIT OR GPL-2.0-only AND BSD-3-Clause").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_and_or_and() {
            let left_left = Expr::Terminal("MIT".to_string());
            let left_right = Expr::Terminal("ISC".to_string());
            let right_left = Expr::Terminal("GPL-2.0-only".to_string());
            let right_right = Expr::Terminal("BSD-3-Clause".to_string());
            let left = Expr::and(left_left, left_right);
            let right = Expr::and(right_left, right_right);
            let expected = Expr::or(left, right);
            let (_, output) = parse_spdx("MIT AND ISC OR GPL-2.0-only AND BSD-3-Clause").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_evaluate() {
            let (_, output) = parse_spdx("MIT AND ISC OR GPL-2.0-only AND BSD-3-Clause").unwrap();
            let mut policy = HashMap::new();
            policy.insert("MIT".into(), true);
            assert_eq!(output.evaluate(&policy), false);
            policy.insert("ISC".into(), true);
            assert_eq!(output.evaluate(&policy), true);
            let mut policy = HashMap::new();
            policy.insert("GPL-2.0-only".into(), true);
            assert_eq!(output.evaluate(&policy), false);
            policy.insert("BSD-3-Clause".into(), true);
            assert_eq!(output.evaluate(&policy), true);
        }
    }
}
