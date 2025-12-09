use seq_macro::seq;
seq!(N in 1..10 {
    pub mod day0~N;
});
seq!(N in 10..=11 {
    pub mod day~N;
});

#[macro_export]
macro_rules! default_input_path {
    () => {{
        let file_name = ::std::path::PathBuf::from(file!())
            .with_extension("txt")
            .file_name()
            .expect("file name")
            .to_owned();
        ::std::path::PathBuf::from("../inputs/2019/").join(file_name)
    }};
}
