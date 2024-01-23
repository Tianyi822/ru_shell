#[cfg(test)]
mod executor_test {
    use ru_shell::executor::executor::Executor;

    #[test]
    fn test_new_executor() {
        let exe = Executor::new("ls -l -a");

        assert_eq!(exe.get_cmds().len(), 1);
        println!("{:#?}", exe.get_cmds()[0]);
    }
}
