pub mod algorithms;
pub mod cards;
pub mod coords;
pub mod file;
pub mod grid;
pub mod iterators;
pub mod math;
pub mod range;
pub mod v2;

pub fn trim_lines(input: impl AsRef<str>) -> String {
    input
        .as_ref()
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .join("\n")
}

pub trait Solver<'a> {
    type Output: std::fmt::Debug;

    fn solve(&self, input: &'a str) -> Self::Output;

    #[expect(clippy::unnecessary_literal_bound)]
    fn file_chunk_separator(&self) -> &str {
        "------"
    }

    fn file_path(&self) -> std::path::PathBuf {
        let path = std::path::PathBuf::from(file!()).with_extension("txt");
        let src_index = path
            .components()
            .position(|c| c.as_os_str() == "src")
            .unwrap_or(0);
        let path = path.components().skip(src_index).collect();
        path
    }

    fn read_file(&self) -> Result<String, std::io::Error> {
        let path = self.file_path();
        let content = std::fs::read_to_string(&path)
            .inspect(|_| tracing::debug!("Opened file {path:?}"))
            .inspect_err(|e| tracing::error!("Failed to open {path:?}: {e}"))?;
        Ok(content)
    }

    fn read_file_chunk(&self, chunk_index: usize) -> Result<String, std::io::Error> {
        let content = self.read_file()?;
        content
            .split(self.file_chunk_separator())
            .nth(chunk_index)
            .map_or_else(
                || {
                    let msg = format!("Chunk index {chunk_index} out of bounds");
                    tracing::error!(msg);
                    Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, msg))
                },
                |chunk| Ok(chunk.trim().to_string()),
            )
    }
}

#[macro_export]
macro_rules! expect_solution {
    ($obj:expr, $index:literal, $expected:expr) => {{
        let input = $obj.read_file_chunk($index).unwrap();
        assert_eq!($obj.solve(&input), $expected);
    }};
}
