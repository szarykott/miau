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

        if trimmed_key.starts_with("[") && trimmed_key.ends_with("]") {
            let trimmed_key = trimmed_key[1..trimmed_key.len() - 1].trim();
            match trimmed_key.parse::<usize>() {
                Ok(i) => result.push(Key::Array(i)),
                Err(e) => return Err(ErrorCode::ParsingError(e.to_string()).into()),
            }
        } else {
            result.push(Key::Map(trimmed_key.to_owned()))
        }
    }

    Ok(result)
}
