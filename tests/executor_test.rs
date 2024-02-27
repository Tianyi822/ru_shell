#[cfg(test)]
mod executor_test {
    use std::{env, rc::Rc};

    use ru_shell::{executor, stream::console_stream::ConsoleStream};
    use ru_shell::stream::Stream;

    #[test]
    fn test_new_executor() {
        let console_stream = Rc::new(ConsoleStream::new());
        executor::execute("ls -l -h -s -r", console_stream.clone());
        console_stream.output();
    }

    #[test]
    fn test_ls_tree() {
        let console_stream = Rc::new(ConsoleStream::new());
        executor::execute("ls --tree --depth=2", console_stream.clone());
        console_stream.output();
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
        let console_stream = Rc::new(ConsoleStream::new());
        executor::execute("grep -i -v -c \"col\" Cargo.toml", console_stream.clone());
        console_stream.output();
        println!("======================");
        executor::execute("grep -i -v \"col\" Cargo.toml", console_stream.clone());
        console_stream.output();
        println!("======================");
        executor::execute("grep -i \"col\" Cargo.toml", console_stream.clone());
        console_stream.output();
    }

    #[test]
    fn test_cat_cmd() {
        let console_stream = Rc::new(ConsoleStream::new());
        executor::execute("cat Cargo.toml", console_stream.clone());
        console_stream.output();
    }

    #[test]
    fn test_cat_cmd_with_line_number() {
        let console_stream = Rc::new(ConsoleStream::new());
        executor::execute("cat -n Cargo.toml", console_stream.clone());
        console_stream.output();
    }

    #[test]
    fn test_ls_pipeline_grep_cmd() {
        let console_stream = Rc::new(ConsoleStream::new());
        executor::execute("ls -l | grep -i \"lock\"", console_stream.clone());
        console_stream.output();
    }
}
