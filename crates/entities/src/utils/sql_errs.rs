use once_cell::sync::Lazy;
use regex::Regex;

static KEYERR_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r#""([^"]*)""#).unwrap());

pub fn parse_column_contraint_violation<'a>(source: &'a String) -> Option<&'a str> {
    let source = (*KEYERR_REG).find_iter(source);
    let matches: Vec<_> = source.collect();
    if matches.len() > 0 {
        let found = matches[matches.len() - 1].as_str();
        return Some(&found[1..found.len() - 1]);
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_parse_column_contraint_violation(source: &str, expected: &str) {
        let source = source.to_string();
        let result = parse_column_contraint_violation(&source);
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_parse_column_contraint_violation_none() {
        let s = "abc abc col col".to_string();
        let result = parse_column_contraint_violation(&s);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_column_contraint_violation_foreign_key_column() {
        test_parse_column_contraint_violation(
            r#"insert or update on table "bucket" violates foreign key constraint "bucket_namespace_fkey""#,
            "bucket_namespace_fkey",
        );
    }

    #[test]
    fn test_parse_column_contraint_violation_unique_value() {
        test_parse_column_contraint_violation(
            r#"insert or update on table "bucket" violates unique contraint "bucket_name_idx""#,
            "bucket_name_idx",
        );
    }
}
