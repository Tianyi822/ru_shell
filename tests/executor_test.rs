#[cfg(test)]
mod executor_test {
    use std::env;

    use ru_shell::executor::executor::Executor;

    #[test]
    fn test_new_executor() {
        let exe = Executor::new("ls -l -h -s -r");

        // println!("{:#?}", exe);
        assert_eq!(exe.get_cmds().len(), 1);

        exe.execute();
    }

    #[test]
    fn test_ls_tree() {
        let exe = Executor::new("ls --tree --depth=2");

        assert_eq!(exe.get_cmds().len(), 1);

        exe.execute();
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
}
