use serde_json::Value;

const RED: &str = "red";

fn sum_numbers(v: &Value) -> i64 {
    let mut sum = 0;
    sum += match v {
        Value::Number(n) => n.as_i64().expect("Invalid number"),
        Value::Array(arr) => arr.iter().map(sum_numbers).sum::<i64>(),
        Value::Object(obj) => obj.values().map(sum_numbers).sum::<i64>(),
        Value::Bool(_) | Value::Null | Value::String(_) => 0,
    };
    sum
}

fn sum_non_reds(v: &Value) -> i64 {
    let mut sum = 0;
    sum += match v {
        Value::Number(n) => n.as_i64().expect("Invalid number"),
        Value::Array(arr) => arr.iter().map(sum_non_reds).sum::<i64>(),
        Value::Object(obj) => {
            if obj.values().any(|value| value == RED) {
                0
            } else {
                obj.values().map(sum_non_reds).sum::<i64>()
            }
        }
        Value::Bool(_) | Value::Null | Value::String(_) => 0,
    };
    sum
}

fn read_doc(s: &str, allow_reds: bool) -> i64 {
    let doc: Value = serde_json::from_str(s).expect("Invalid JSON");
    if allow_reds {
        sum_numbers(&doc)
    } else {
        sum_non_reds(&doc)
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use super::*;

    #[test]
    fn test_examples_1() {
        assert_eq!(read_doc(r"[1,2,3]", true), 6);
        assert_eq!(read_doc(r#"{"a":2,"b":4}"#, true), 6);
        assert_eq!(read_doc(r"[[[3]]]", true), 3);
        assert_eq!(read_doc(r#"{"a":{"b":4},"c":-1}"#, true), 3);
        assert_eq!(read_doc(r#"{"a":[-1,1]}"#, true), 0);
        assert_eq!(read_doc(r#"[-1,{"a":1}]"#, true), 0);
        assert_eq!(read_doc(r"[]", true), 0);
        assert_eq!(read_doc(r"{}", true), 0);
    }

    #[test]
    fn test_part_1() {
        let path = PathBuf::from_str("../inputs/2015/day12.txt").expect("path");
        let input = std::fs::read_to_string(path).unwrap();
        assert_eq!(read_doc(&input, true), 156366);
    }

    #[test]
    fn test_examples_2() {
        assert_eq!(read_doc(r#"[1,{"c":"red","b":2},3]"#, false), 4);
        assert_eq!(read_doc(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, false), 0);
        assert_eq!(read_doc(r#"[1,"red",5]"#, false), 6);
    }

    #[test]
    fn test_part_2() {
        let path = PathBuf::from_str("../inputs/2015/day12.txt").expect("path");
        let input = std::fs::read_to_string(path).unwrap();
        assert_eq!(read_doc(&input, false), 96852);
    }
}
