use std::{fmt, io, path::PathBuf};
use tracing::{debug, error};

mod ducks_and_dragons;
mod quest01;
mod quest02;
mod quest03;
mod quest04;
mod quest05;
mod quest06;

#[macro_export]
macro_rules! expect_solution {
    ($obj:ident, $index:literal, $expected:expr) => {{
        let input = $obj.read_file_chunk($index).unwrap();
        assert_eq!($obj.solve(&input), $expected);
    }};
}

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

pub trait Solver<'a> {
    type Output: fmt::Debug;

    fn solve(&self, input: &'a str) -> Self::Output;

    #[expect(clippy::unnecessary_literal_bound)]
    fn file_chunk_separator(&self) -> &str {
        "------"
    }

    fn file_path(&self) -> PathBuf {
        let path = PathBuf::from(file!()).with_extension("txt");
        let src_index = path
            .components()
            .position(|c| c.as_os_str() == "src")
            .unwrap_or(0);
        let path = path.components().skip(src_index).collect();
        path
    }

    fn read_file(&self) -> Result<String, io::Error> {
        let path = self.file_path();
        let content = std::fs::read_to_string(&path)
            .inspect(|_| debug!("Opened file {path:?}"))
            .inspect_err(|e| error!("Failed to open {path:?}: {e}"))?;
        Ok(content)
    }

    fn read_file_chunk(&self, chunk_index: usize) -> Result<String, io::Error> {
        let content = self.read_file()?;
        content
            .split(self.file_chunk_separator())
            .nth(chunk_index)
            .map_or_else(
                || {
                    let msg = format!("Chunk index {chunk_index} out of bounds");
                    error!(msg);
                    Err(io::Error::new(io::ErrorKind::InvalidInput, msg))
                },
                |chunk| Ok(chunk.trim().to_string()),
            )
    }
}
