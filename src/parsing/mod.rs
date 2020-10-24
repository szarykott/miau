use crate::{
    configuration::{CompoundKey, Key},
    error::{ConfigurationError, ErrorCode},
};

pub(crate) fn str_to_key(input: &str) -> Result<CompoundKey, ConfigurationError> {
    let mut result = Vec::new();

    if input.is_empty() {
        return Ok(result);
    }

    // accept string is a format key1:[index1]:key2:key3:[index2]
    for potential_key in input.split_terminator(":") {
        let trimmed_key = potential_key.trim();

        let open_bracket = trimmed_key.starts_with("[");
        let close_bracket = trimmed_key.ends_with("]");

        if open_bracket && close_bracket {
            let inner = trimmed_key
                .strip_prefix("[")
                .and_then(|s| s.strip_suffix("]"))
                .map(|s| s.trim());

            match inner {
                Some(potential_number) => match potential_number.parse::<usize>() {
                    Ok(i) => result.push(Key::Array(i)),
                    Err(e) => return Err(ErrorCode::ParsingError(e.to_string()).into()),
                },
                None => {
                    return Err(ErrorCode::ParsingError(format!(
                        "Expected integer key to start with [ and end with ]. Got {}",
                        trimmed_key
                    ))
                    .into())
                }
            }
        } else {
            result.push(Key::Map(trimmed_key.to_owned()))
        }
    }

    Ok(result)
}
