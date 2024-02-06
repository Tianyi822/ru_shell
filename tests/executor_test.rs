#[cfg(test)]
mod executor_test {
    use std::env;

    use ru_shell::executor::*;

    #[test]
    fn test_new_executor() {
        executor::execute("ls -l -h -s -r");
    }

    #[test]
    fn test_ls_tree() {
        executor::execute("ls --tree --depth=2");
    }

    #[test]
    fn test_cur_path() {
        match env::current_dir() {
            Ok(path) => {
                println!("Current directory is: {:?}", path);
            }
            Err(e) => {
                println!("Failed to get current directory: {}", e);
            }
        }
    }

    #[test]
    fn test_grep_cmd() {
        executor::execute("grep -i -v -c \"col\" Cargo.toml");
    }
}
