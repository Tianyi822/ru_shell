#[cfg(test)]
mod executor_test {
    use std::{env, rc::Rc};

    use ru_shell::{executor, stream::console_stream::ConsoleSteam};

    #[test]
    fn test_new_executor() {
        let console_stream = Rc::new(ConsoleSteam::new());
        executor::execute("ls -l -h -s -r", console_stream.clone());
    }

    #[test]
    fn test_ls_tree() {
        let console_stream = Rc::new(ConsoleSteam::new());
        executor::execute("ls --tree --depth=2", console_stream.clone());
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
        let console_stream = Rc::new(ConsoleSteam::new());
        executor::execute("grep -i -v -c \"col\" Cargo.toml", console_stream.clone());
        println!("======================");
        executor::execute("grep -i -v \"col\" Cargo.toml", console_stream.clone());
        println!("======================");
        executor::execute("grep -i \"col\" Cargo.toml", console_stream.clone());
    }
}
