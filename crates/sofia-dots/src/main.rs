//! Installation script for my dotfiles. This is a simple script that creates symbolic links to
//! the dotfiles in the repository :). I like the way it works because it's simple and focused on a
//! single place.

use std::{io::Error, path::PathBuf};

use yansi::Paint;

/// All the paths used in this script.
mod paths {
    use crate::PathExt;
    use std::path::PathBuf;

    /// Path to home repository
    pub fn home() -> PathBuf {
        std::env::var("HOME").unwrap().into()
    }

    /// Path to the .config directory
    pub fn config() -> PathBuf {
        home().with(".config")
    }

    /// Path to the current directory
    pub fn here() -> PathBuf {
        std::env::current_dir().unwrap().canonicalize().unwrap()
    }
}

/// This is a extension trait for `PathBuf` to make it easier to work with paths.
pub trait PathExt {
    /// Append a path to the current path.
    fn with(&self, path: &str) -> PathBuf;
}

// Extends the `PathBuf` type.
impl PathExt for PathBuf {
    fn with(&self, path: &str) -> PathBuf {
        let mut new = self.clone();
        new.push(path);
        new
    }
}

/// Create a symbolic link.
pub fn link(from: PathBuf, to: PathBuf) {
    println!(
        "     {} {:?} to {:?}",
        Paint::green("Linking").bold(),
        from,
        to
    );

    if to.exists() {
        if to.is_file() {
            std::fs::remove_file(&to).unwrap();
        } else if to.is_dir() {
            std::fs::remove_dir_all(&to).unwrap();
        } else {
            panic!("Path {:?} is not a file or directory", to);
        }
    }

    std::os::unix::fs::symlink(from, to).unwrap()
}

// Main part
fn main() {
    let dot_config = paths::config();
    let config = paths::here().with("config");
    let home = paths::home();

    println!();

    // .config files
    link(config.with("dunst"), dot_config.with("dunst"));
    link(config.with("eww"), dot_config.with("eww"));
    link(config.with("htop"), dot_config.with("htop"));
    link(config.with("hypr"), dot_config.with("hypr"));
    link(config.with("kitty"), dot_config.with("kitty"));
    link(config.with("neofetch"), dot_config.with("neofetch"));
    link(config.with("ranger"), dot_config.with("ranger"));

    // home files
    link(config.with(".zshrc"), home.with(".zshrc"));

    println!("        {} :)\n", Paint::green("Done").bold())
}
