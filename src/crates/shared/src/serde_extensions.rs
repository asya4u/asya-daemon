pub fn get_json_value(content: &str, pointer: &str) -> Option<String> {
    get_value_by_pointer(
        serde_json::from_str::<serde_json::Value>(content).ok(),
        pointer,
    )
}

pub fn get_yaml_value(content: &str, pointer: &str) -> Option<String> {
    get_value_by_pointer(
        serde_yaml::from_str::<serde_json::Value>(content).ok(),
        pointer,
    )
}

fn get_value_by_pointer(value: Option<serde_json::Value>, pointer: &str) -> Option<String> {
    value.and_then(|value| {
        value
            .pointer(pointer)
            .and_then(|val| {
                serde_json::from_value(val.clone()).ok()
            })
    })
}
