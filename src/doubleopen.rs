// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use crate::{license_list::LicenseList, SPDXExpression};

/// Parse list of Double Open's license conclusions from Fossology to an SPDX expression.
pub fn parse_doubleopen_license(licenses: Vec<String>) -> String {
    let mut or_operator_list: Vec<String> = Vec::new();
    let mut other_licenses_list: Vec<String> = Vec::new();

    for license in licenses {
        if is_do_license(&license) && is_or_license(&license) {
            let license = dolicense_to_spdx(license);
            or_operator_list.push(license);
        } else {
            let license = dolicense_to_spdx(license);
            other_licenses_list.push(license);
        }
    }

    let or_licenses = or_operator_list.join(" ");
    let other_licenses = other_licenses_list.join(" AND ");
    if or_licenses.is_empty() {
        other_licenses
    } else {
        format!("{} {}", or_licenses, other_licenses)
    }
}

/// Convert Double Open's custom Fossology license to SPDX expression.
fn dolicense_to_spdx(license: String) -> String {
    if is_do_license(&license) {
        // Remove prefix.
        let license = license.strip_prefix("DOLicense-").expect("Always exists.");

        // Process parentheses.
        let license = license.replace("paro-", "(");
        let license = license.replace("-parc", ")");

        // Process -OR- and -AND-.
        let license = license.replace("-OR-", " OR ").replace("-AND-", " AND ");

        // Process -OR license.
        let license = if is_or_license(&license) {
            let license = license.strip_suffix("-OR").expect("Always exists.");
            format!("{} OR", license)
        } else {
            license
        };

        // Process DO Exceptions.
        let license = if is_do_exception_license(&license) {
            let license = license
                .strip_prefix("SPDXException-")
                .expect("Always exists.")
                .to_string();
            license.replace("-with-", " WITH ")
        } else {
            license
        };

        gpl_or_later_conversion(license)
    } else {
        license
    }
}

/// Convert deprecated license ids.
pub fn gpl_or_later_conversion(license: String) -> String {
    license
        .replace("AGPL-1.0+", "AGPL-1.0-or-later")
        .replace("AGPL-3.0+", "AGPL-3.0-or-later")
        .replace("GFDL-1.1-invariants+", "GFDL-1.1-invariants-or-later")
        .replace("GFDL-1.1-no-invariants+", "GFDL-1.1-no-invariants-or-later")
        .replace("GFDL-1.1+", "GFDL-1-1-or-later")
        .replace("GFDL-1.2-invariants+", "GFDL-1.2-invariants-or-later")
        .replace("GFDL-1.2-no-invariants+", "GFDL-1.2-no-invariants-or-later")
        .replace("GFDL-1.2+", "GFDL-1-2-or-later")
        .replace("GFDL-1.3-invariants+", "GFDL-1.3-invariants-or-later")
        .replace("GFDL-1.3-no-invariants+", "GFDL-1.3-no-invariants-or-later")
        .replace("GFDL-1.3+", "GFDL-1-3-or-later")
        .replace("GPL-1.0+", "GPL-1.0-or-later")
        .replace("gpl-2.0+", "GPL-2.0-or-later")
        .replace("GPL-2.0+", "GPL-2.0-or-later")
        .replace("gpl-3.0+", "GPL-3.0-or-later")
        .replace("GPL-3.0+", "GPL-3.0-or-later")
        .replace("LGPL-2.0+", "LGPL-2.0-or-later")
        .replace("LGPL-2.1+", "LGPL-2.1-or-later")
        .replace("LGPL-3.0+", "LGPL-3.0-or-later")
}

/// Check if the string is Double Open's custom Fossology license.
pub fn is_do_license(license: &str) -> bool {
    license.starts_with("DOLicense-")
}

/// Check if the string is Double Open's OR license.
fn is_or_license(license: &str) -> bool {
    license.ends_with("-OR")
}

/// Check if the string is Double Open's license with SPDX exception.
fn is_do_exception_license(license: &str) -> bool {
    license.starts_with("SPDXException-")
}

/// Convert Fossology's conclusions to SPDX Expression.
pub fn fossology_conclusions_to_spdx_expression(
    conclusions: Vec<String>,
    license_list: &LicenseList,
) -> SPDXExpression {
    // Convert all conclusions to be SPDX compliant.
    let conclusions: Vec<String> = conclusions
        .into_iter()
        .map(|lic| gpl_or_later_conversion(lic))
        .map(|lic| {
            if license_list.includes_license(&lic)
                || license_list.includes_exception(&lic)
                || is_do_license(&lic)
                || lic.starts_with("LicenseRef-")
                || lic == "Dual-license"
                || lic == "NOASSERTION"
                || lic == "NONE"
            {
                lic
            } else {
                format!("LicenseRef-{}", lic)
            }
        })
        .collect();

    // Join licenses with AND or OR.
    let expression = if (conclusions.len() == 2
        || (conclusions.len() == 3 && conclusions.contains(&"Dual-license".to_string())))
        && conclusions
            .iter()
            .any(|lic| license_list.includes_exception(&lic))
    {
        let mut sorted_conclusions: Vec<String> = Vec::new();
        for lic in conclusions {
            if license_list.includes_exception(&lic) {
                sorted_conclusions.push(lic)
            } else {
                sorted_conclusions.insert(0, lic)
            }
        }
        filter_dual_license(sorted_conclusions).join(" WITH ")
    } else if conclusions.len() == 3 && conclusions.contains(&"Dual-license".to_string()) {
        let conclusions = filter_dual_license(conclusions);
        conclusions.join(" OR ")
    } else {
        let conclusions = filter_dual_license(conclusions);
        let conclusions = add_licenserefs(conclusions, &license_list);
        parse_doubleopen_license(conclusions)
    };

    SPDXExpression(expression)
}

/// Filter Fossology's Dual-license from the list of licenses.
fn filter_dual_license(conclusions: Vec<String>) -> Vec<String> {
    conclusions
        .into_iter()
        .filter(|lic| lic != "Dual-license")
        .collect::<Vec<String>>()
}

/// Add SPDX's LicenseRef to a license if it's not on the SPDX license list.
fn add_licenserefs(conclusions: Vec<String>, license_list: &LicenseList) -> Vec<String> {
    conclusions
        .into_iter()
        .map(|lic| {
            if license_list.includes_license(&lic)
                || lic.starts_with("LicenseRef-")
                || is_do_license(&lic)
                || lic == "NOASSERTION"
                || lic == "NONE"
            {
                lic
            } else {
                format!("LicenseRef-{}", lic)
            }
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[cfg(test)]
    mod conclusions_to_spdx_expression {
        use super::*;

        #[test]
        fn single_licenses_are_converted_correctly() {
            let input1 = vec!["MIT".to_string()];
            let input2 = vec!["CustomLicense".to_string()];
            let input3 = vec!["Autoconf-exception-2.0".to_string()];
            let input4 = vec!["NONE".to_string()];
            let input5 = vec!["NOASSERTION".to_string()];

            let license_list = LicenseList::from_github();

            let result1 = fossology_conclusions_to_spdx_expression(input1, &license_list);
            let result2 = fossology_conclusions_to_spdx_expression(input2, &license_list);
            let result3 = fossology_conclusions_to_spdx_expression(input3, &license_list);
            let result4 = fossology_conclusions_to_spdx_expression(input4, &license_list);
            let result5 = fossology_conclusions_to_spdx_expression(input5, &license_list);

            assert_eq!(result1, SPDXExpression("MIT".to_string()));
            assert_eq!(
                result2,
                SPDXExpression("LicenseRef-CustomLicense".to_string())
            );
            assert_eq!(
                result3,
                SPDXExpression("LicenseRef-Autoconf-exception-2.0".to_string())
            );
            assert_eq!(result4, SPDXExpression("NONE".to_string()));
            assert_eq!(result5, SPDXExpression("NOASSERTION".to_string()));
        }

        #[test]
        fn simple_and_licenses_are_converted_correctly() {
            let input1 = vec!["MIT".to_string(), "Apache-2.0".to_string()];
            let input2 = vec!["CustomLicense".to_string(), "MIT".to_string()];
            let input3 = vec!["Autoconf-exception-2.0".to_string(), "MIT".to_string()];

            let license_list = LicenseList::from_github();

            let result1 = fossology_conclusions_to_spdx_expression(input1, &license_list);
            let result2 = fossology_conclusions_to_spdx_expression(input2, &license_list);
            let result3 = fossology_conclusions_to_spdx_expression(input3, &license_list);

            assert_eq!(result1, SPDXExpression("MIT AND Apache-2.0".to_string()));
            assert_eq!(
                result2,
                SPDXExpression("LicenseRef-CustomLicense AND MIT".to_string())
            );
            assert_eq!(
                result3,
                SPDXExpression("MIT WITH Autoconf-exception-2.0".to_string())
            );
        }

        #[test]
        fn simple_or_licenses_are_converted_correctly() {
            let input1 = vec![
                "MIT".to_string(),
                "Apache-2.0".to_string(),
                "Dual-license".to_string(),
            ];
            let input2 = vec![
                "CustomLicense".to_string(),
                "MIT".to_string(),
                "Dual-license".to_string(),
            ];
            let input3 = vec![
                "Autoconf-exception-2.0".to_string(),
                "MIT".to_string(),
                "Dual-license".to_string(),
            ];

            let license_list = LicenseList::from_github();

            let result1 = fossology_conclusions_to_spdx_expression(input1, &license_list);
            let result2 = fossology_conclusions_to_spdx_expression(input2, &license_list);
            let result3 = fossology_conclusions_to_spdx_expression(input3, &license_list);

            assert_eq!(result1, SPDXExpression("MIT OR Apache-2.0".to_string()));
            assert_eq!(
                result2,
                SPDXExpression("LicenseRef-CustomLicense OR MIT".to_string())
            );
            assert_eq!(
                result3,
                SPDXExpression("MIT WITH Autoconf-exception-2.0".to_string())
            );
        }

        #[test]
        fn or_licenses_with_three_are_converted_to_and() {
            let input1 = vec![
                "MIT".to_string(),
                "Apache-2.0".to_string(),
                "ISC".to_string(),
                "Dual-license".to_string(),
            ];
            let input2 = vec![
                "CustomLicense".to_string(),
                "MIT".to_string(),
                "Dual-license".to_string(),
                "GPL-2.0-or-later".to_string(),
            ];

            let license_list = LicenseList::from_github();

            let result1 = fossology_conclusions_to_spdx_expression(input1, &license_list);
            let result2 = fossology_conclusions_to_spdx_expression(input2, &license_list);

            assert_eq!(
                result1,
                SPDXExpression("MIT AND Apache-2.0 AND ISC".to_string())
            );
            assert_eq!(
                result2,
                SPDXExpression("LicenseRef-CustomLicense AND MIT AND GPL-2.0-or-later".to_string())
            );
        }

        #[test]
        fn doubleopen_license_is_converted_correctly() {
            let license_list = LicenseList::from_github();

            let input_1 = vec![
                "DOLicense-LGPL-2.1-AND-Zlib-OR".to_string(),
                "DOLicense-SPDXException-GPL-2.0+-with-Autoconf-exception".to_string(),
                "MIT".to_string(),
                "DOLicense-BSD-3-Clause-AND-GPL-2.0-OR".to_string(),
            ];
            let expected_1 = SPDXExpression("LGPL-2.1 AND Zlib OR BSD-3-Clause AND GPL-2.0 OR GPL-2.0-or-later WITH Autoconf-exception AND MIT".to_string());
            assert_eq!(
                fossology_conclusions_to_spdx_expression(input_1, &license_list),
                expected_1
            );

            let input_2 = vec![
                "DOLicense-LGPL-2.1-OR".to_string(),
                "BSD-3-Clause".to_string(),
                "MIT".to_string(),
            ];
            let expected_2 = SPDXExpression("LGPL-2.1 OR BSD-3-Clause AND MIT".to_string());
            assert_eq!(
                fossology_conclusions_to_spdx_expression(input_2, &license_list),
                expected_2
            );

            let input_3 = vec![
                "DOLicense-paro-LGPL-2.1-OR-BSD-3-Clause-parc".to_string(),
                "MIT".to_string(),
            ];
            let expected_3 = SPDXExpression("(LGPL-2.1 OR BSD-3-Clause) AND MIT".to_string());
            assert_eq!(
                fossology_conclusions_to_spdx_expression(input_3, &license_list),
                expected_3
            );
        }

        #[test]
        fn with_licenses_are_converted_correctly() {
            let input1 = vec!["Bison-exception-2.2".to_string(), "GPL-3.0+".to_string()];
            let input2 = vec!["GPL-3.0+".to_string(), "Bison-exception-2.2".to_string()];

            let license_list = LicenseList::from_github();

            let result1 = fossology_conclusions_to_spdx_expression(input1, &license_list);
            let result2 = fossology_conclusions_to_spdx_expression(input2, &license_list);

            assert_eq!(
                result1,
                SPDXExpression("GPL-3.0-or-later WITH Bison-exception-2.2".to_string())
            );
            assert_eq!(
                result2,
                SPDXExpression("GPL-3.0-or-later WITH Bison-exception-2.2".to_string())
            );
        }
    }

    #[test]
    fn is_do_license_works() {
        let input_1 = "DOLicense-LGPL-2.1-AND-Zlib-OR";
        let input_2 = "DOLicense-SPDXException-GPL-2.0+-with-Autoconf-exception";
        let input_3 = "MIT";
        let input_4 = "DOLicense-BSD-3-Clause-AND-GPL-2.0-OR";

        assert!(is_do_license(input_1));
        assert!(is_do_license(input_2));
        assert!(!is_do_license(input_3));
        assert!(is_do_license(input_4));
    }

    #[test]
    fn is_or_license_works() {
        let input_1 = "DOLicense-LGPL-2.1-AND-Zlib-OR";
        let input_2 = "DOLicense-SPDXException-GPL-2.0+-with-Autoconf-exception";
        let input_3 = "MIT";
        let input_4 = "DOLicense-BSD-3-Clause-AND-GPL-2.0-OR";

        assert!(is_or_license(input_1));
        assert!(!is_or_license(input_2));
        assert!(!is_or_license(input_3));
        assert!(is_or_license(input_4));
    }

    #[test]
    fn format_dolicense_to_spdx_works() {
        let input_1 = "DOLicense-paro-LGPL-2.1-OR-BSD-3-Clause-parc";
        let expected_1 = "(LGPL-2.1 OR BSD-3-Clause)";

        let input_2 = "DOLicense-LGPL-2.1-AND-Zlib-OR";
        let expected_2 = "LGPL-2.1 AND Zlib OR";

        let input_3 = "DOLicense-SPDXException-GPL-2.0+-with-Autoconf-exception";
        let expected_3 = "GPL-2.0-or-later WITH Autoconf-exception";

        let input_4 = "DOLicense-LGPL-2.1-OR";
        let expected_4 = "LGPL-2.1 OR";

        assert_eq!(dolicense_to_spdx(input_1.into()), expected_1.to_string());
        assert_eq!(dolicense_to_spdx(input_2.into()), expected_2.to_string());
        assert_eq!(dolicense_to_spdx(input_3.into()), expected_3.to_string());
        assert_eq!(dolicense_to_spdx(input_4.into()), expected_4.to_string());
    }

    #[test]
    fn parse_doubleopen_license_works() {
        let input_1 = vec![
            "DOLicense-LGPL-2.1-AND-Zlib-OR".to_string(),
            "DOLicense-SPDXException-GPL-2.0+-with-Autoconf-exception".to_string(),
            "MIT".to_string(),
            "DOLicense-BSD-3-Clause-AND-GPL-2.0-OR".to_string(),
        ];
        let expected_1 = "LGPL-2.1 AND Zlib OR BSD-3-Clause AND GPL-2.0 OR GPL-2.0-or-later WITH Autoconf-exception AND MIT".to_string();
        assert_eq!(parse_doubleopen_license(input_1), expected_1);

        let input_2 = vec![
            "DOLicense-LGPL-2.1-OR".to_string(),
            "BSD-3-Clause".to_string(),
            "MIT".to_string(),
        ];
        let expected_2 = "LGPL-2.1 OR BSD-3-Clause AND MIT".to_string();
        assert_eq!(parse_doubleopen_license(input_2), expected_2);

        let input_3 = vec![
            "DOLicense-paro-LGPL-2.1-OR-BSD-3-Clause-parc".to_string(),
            "MIT".to_string(),
        ];
        let expected_3 = "(LGPL-2.1 OR BSD-3-Clause) AND MIT".to_string();
        assert_eq!(parse_doubleopen_license(input_3), expected_3);
    }
}
