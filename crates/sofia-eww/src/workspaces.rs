//! Workspaces widget

use hyprland::shared::HyprDataActive;
use hyprland::{async_closure, data::Workspace, event_listener::AsyncEventListener};

use tokio::io;

pub fn render(id: String) {
    fn single_box(id: String, is_active: bool) {
        print!(
            "(box :class \"workspace {}\" \"{}\")",
            if is_active { "active" } else { "" },
            id,
        );
    }

    print!("(box :class \"bar_item workspaces\"");

    for i in 1..=9 {
        print!(" ");
        single_box(i.to_string(), i.to_string() == id);
    }

    println!(")");
}

pub async fn workspaces() -> io::Result<()> {
    let mut event_listener = AsyncEventListener::new();

    let work = Workspace::get_active_async().await.unwrap();

    render(work.id.to_string());

    event_listener.add_workspace_change_handler(async_closure! {|id| {
        render(id.to_string());
    }});

    event_listener.start_listener_async().await.unwrap();

    Ok(())
}
