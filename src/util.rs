use std::fs;

pub(crate) fn get_input(name: &str) -> String {
    fs::read_to_string(format!("./inputs/{}.txt", name))
        .map_err(|e| format!("no input file with name {}.txt; caused by: {}", name, e))
        .unwrap()
}
