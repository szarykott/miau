#[macro_export]
macro_rules! get_value {
    ($config:expr, $( $key:expr ),*) => {{
        let mut node = Result::Ok::<&ConfigurationNode, ConfigurationAccessError>(&$config.root);
        $(node = node.and_then(|node| node.get($key)); )*
        node.and_then(|node| node.get_value())
    }};
}

#[macro_export]
macro_rules! get_typed_value {
    ($config:expr, $t:ty => $( $key:expr ),*) => {{
        let mut node = Result::Ok::<&ConfigurationNode, ConfigurationAccessError>(&$config.root);
        $(node = node.and_then(|node| node.get($key)); )*
        node.and_then(|node| node.get_value::<$t>())
    }};
}
