pub mod algorithms;
pub mod cards;
pub mod coords;
pub mod file;
pub mod grid;
pub mod iterators;
pub mod math;
pub mod range;
pub mod v2;

/// A trait for solving problems that are defined by an input text file.
///
/// The lifetime parameter `'a` allows the solver to return references to the input string
/// if needed.
pub trait Solver<'a> {
    /// The output type of the solver. Must implement [`std::fmt::Debug`
    type Output: std::fmt::Debug;

    /// The main solving function. Takes the input as a string slice and returns the output.
    /// Must be implemented by the user. Supports any lifetime `'a`, but if the output
    /// contains references to the input, ensure that [`Self::Output`] is defined with
    /// the same lifetime `'a` (as well as [`Solver<'a>`] itself).
    fn solve(&self, input: &'a str) -> Self::Output;

    /// The separator used to split the input file into chunks. Defaults to "------".
    ///
    /// If each problem chunk is on a new line, consider overriding this method to return "\n".
    #[expect(clippy::unnecessary_literal_bound)]
    fn file_chunk_separator(&self) -> &str {
        "------"
    }

    /// Constructs the file path for the input file based on the source file location.
    ///
    /// Overrides the default behavior to specify the input file path. By default,
    /// it replaces the source file extension with ".txt" and adjusts the path to start
    /// from the "src" directory.
    fn file_path(&self) -> std::path::PathBuf {
        let path = std::path::PathBuf::from(file!()).with_extension("txt");
        let src_index = path
            .components()
            .position(|c| c.as_os_str() == "src")
            .unwrap_or(0);
        let path = path.components().skip(src_index).collect();
        path
    }

    /// Reads the entire input file and returns its content as a `String`.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the file cannot be read / opened.
    fn read_file(&self) -> Result<String, std::io::Error> {
        let path = self.file_path();
        let content = std::fs::read_to_string(&path)
            .inspect(|_| tracing::debug!("Opened file {path:?}"))
            .inspect_err(|e| tracing::error!("Failed to open {path:?}: {e}"))?;
        Ok(content)
    }

    /// Reads a specific chunk of the input file, split by whatever is returned by
    /// [`Self::file_chunk_separator`]. Returns an error if the chunk index is out of bounds
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the file cannot be read / opened,
    /// or if the chunk index is out of bounds.
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

/// A shorthand macro to avoid the common boilerplate when testing the solution of a given solver
/// for a specific input chunk. Equivalent to:
/// ```ignore
/// assert_eq!(solver.solve(&solver.read_file_chunk(index).unwrap()), expected);
/// ```
#[macro_export]
macro_rules! expect_solution {
    ($obj:expr, $index:literal, $expected:expr) => {{
        let input = $obj.read_file_chunk($index).unwrap();
        assert_eq!($obj.solve(&input), $expected);
    }};
}
