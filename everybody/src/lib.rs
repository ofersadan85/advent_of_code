mod ducks_and_dragons;
mod quest01;
mod quest02;
mod quest03;
mod quest04;
mod quest05;
mod quest06;

#[macro_export]
macro_rules! default_input_path {
    () => {{
        let path = ::std::path::PathBuf::from(file!()).with_extension("txt");
        let src_index = path
            .components()
            .position(|c| c.as_os_str() == "src")
            .unwrap_or(0);
        let path = path.components().skip(src_index).collect();
        path
    }};
}
