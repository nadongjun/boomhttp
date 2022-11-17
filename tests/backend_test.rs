#[cfg(test)]
mod tests {
    #[test]
    fn read_configfile_test() {
        let path = "configuration/config.json";
        backend::read_input_configfile(&path);
    }
}