mod intcode;
advent_of_code_macros::all_the_days!(16);

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
