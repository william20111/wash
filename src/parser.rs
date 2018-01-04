struct CmdTable {
    command_table: Vec<String>
}

impl CmdTable {
    fn parse(input: String) -> Vec<String> {
        input.trim().split(" ").map(|s| s.to_string()).collect()
    }
}