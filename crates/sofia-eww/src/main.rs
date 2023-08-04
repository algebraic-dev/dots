//! This is a crate for my widgets with eww. It contains a bunch of widgets for:
//!
//! - Workspaces
//! - Battery
//!

pub mod battery;

pub use battery::battery;

use std::str::FromStr;

use clap::Parser;
use smol::io;

/// Widget to display
#[derive(Clone)]
pub enum Widget {
    Workspace,
    Battery,
}

impl FromStr for Widget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "workspace" => Ok(Self::Workspace),
            "battery" => Ok(Self::Battery),
            _ => Err(format!("Unknown widget: {}", s)),
        }
    }
}

/// CLI for the widgets
#[derive(Parser, Debug)]
struct WidgetsCli {
    widget: String,
}

fn main() -> io::Result<()> {
    smol::block_on(async {
        let widgets = WidgetsCli::parse();

        match widgets.widget.parse::<Widget>() {
            Ok(Widget::Workspace) => {
                todo!()
            }
            Ok(Widget::Battery) => battery().await,
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    })
}
