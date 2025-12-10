mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

#[macro_export]
macro_rules! default_input_path {
    () => {{
        let file_name = ::std::path::PathBuf::from(file!())
            .with_extension("txt")
            .file_name()
            .expect("file name")
            .to_owned();
        ::std::path::PathBuf::from("../inputs/2025/").join(file_name)
    }};
}
