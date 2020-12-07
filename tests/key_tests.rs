use miau::configuration::Key;

#[test]
fn test_key_succesfull_unwrap() {
    let key_map = Key::Map("A".into());
    let key_arr = Key::Array(1);

    assert_eq!("A".to_string(), key_map.unwrap_map());
    assert_eq!(1, key_arr.unwrap_array());
}

#[test]
#[should_panic]
fn test_key_map_unwrap_into_array_panics() {
    let key_map = Key::Map("A".into());

    let _ = key_map.unwrap_array();
}

#[test]
#[should_panic]
fn test_key_array_unwrap_into_map_panics() {
    let key_arr = Key::Array(1);

    let _ = key_arr.unwrap_map();
}
