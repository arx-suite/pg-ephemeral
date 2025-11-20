use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub use windows::WindowsProgramFinder as ProgramFinderImpl;

#[cfg(unix)]
pub use unix::UnixProgramFinder as ProgramFinderImpl;

pub trait ProgramFinder {
    fn find(&self, program: &str) -> Option<PathBuf>;
    fn exists(&self, program: &str) -> bool {
        self.find(program).is_some()
    }
}

#[cfg(unix)]
mod unix {
    use std::env;
    use std::path::PathBuf;

    use super::ProgramFinder;

    pub struct UnixProgramFinder;

    impl ProgramFinder for UnixProgramFinder {
        fn find(&self, program: &str) -> Option<PathBuf> {
            let paths = env::var_os("PATH")?;
            for path in env::split_paths(&paths) {
                let full = path.join(program);
                if full.exists() && full.is_file() {
                    return Some(full);
                }
            }
            None
        }
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use std::env;
    use std::path::PathBuf;

    use super::ProgramFinder;

    pub struct WindowsProgramFinder;

    impl ProgramFinder for WindowsProgramFinder {
        fn find(&self, program: &str) -> Option<PathBuf> {
            let paths = env::var_os("PATH")?;
            let extensions = vec![".exe", ".cmd", ".bat", ""];

            for path in env::split_paths(&paths) {
                for ext in &extensions {
                    let candidate = path.join(format!("{}{}", program, ext));
                    if candidate.exists() {
                        return Some(candidate);
                    }
                }
            }

            None
        }
    }
}
