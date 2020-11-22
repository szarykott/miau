use crate::{
    configuration::{CompoundKey, Key},
    error::{ConfigurationError, ErrorCode},
};

pub(crate) fn str_to_key(input: &str) -> Result<CompoundKey, ConfigurationError> {
    let mut result = Vec::new();

    if input.is_empty() {
        return Ok(result.into());
    }

    // accept string is a format key1:[index1]:key2:key3:[index2]
    for potential_key in input.split_terminator(':') {
        let trimmed_key = potential_key.trim();

        if trimmed_key.starts_with('[') && trimmed_key.ends_with(']') {
            let trimmed_key = trimmed_key[1..trimmed_key.len() - 1].trim();
            match trimmed_key.parse::<usize>() {
                Ok(i) => result.push(Key::Array(i)),
                Err(e) => return Err(ErrorCode::ParsingError(format!("Error occured while parsing `{}` : {}", trimmed_key, e.to_string())).into()),
            }
        } else {
            result.push(Key::Map(trimmed_key.to_owned()))
        }
    }

    Ok(result.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::Key;
    use rstest::rstest;

    #[rstest(input, exp, 
        // success cases
        case("", Vec::new()),
        case("key", vec![Key::Map("key".into())]),
        case("[1]", vec![Key::Array(1)]),
        case("key:[1]", vec![Key::Map("key".into()), Key::Array(1)]),
        case("key:[1]:key2", vec![Key::Map("key".into()), Key::Array(1), Key::Map("key2".into())]),
        case("[1]:key:[2]", vec![Key::Array(1), Key::Map("key".into()), Key::Array(2)]),
        // strange cases
        case("1]", vec![Key::Map("1]".into())]),
        case("1]:[2", vec![Key::Map("1]".into()), Key::Map("[2".into())]),
    )]
    fn test_key_to_str_success(input: &str, exp: Vec<Key>) {
        let parsed = str_to_key(input).unwrap();
        assert!(exp.iter().eq(parsed.iter()))
    }

    #[test]
    fn test_key_to_str_failure() {
        let input = "[A]";
        let parsed = str_to_key(input);
        assert!(parsed.is_err());
    }
}
