use async_i3ipc::I3;
use tokio;
use async_i3ipc::reply::{Workspaces, Workspace};
use anyhow::Result;
use clap::{AppSettings, Clap};
use std::process;

#[derive(Clap, Clone)]
#[clap(version = "0.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(long)]
    screen: Option<String>,

    #[clap(long)]
    visible: Option<bool>,

    #[clap(long)]
    num: Option<usize>,

    #[clap(long)]
    name: Option<String>,
}

#[tokio::main]
pub async fn main() {
    let opts: Opts = Opts::parse();
    let workspaces = match get_workspaces().await {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Failed to connect to i3 and get workspaces. {}", e);
            process::exit(1);
        }
    };

    for workspace in workspaces {
        if filter(opts.clone(), &workspace) {
            println!("{:?}", workspace)
        }
    }
}

fn filter(opts: Opts, workspace: &Workspace) -> bool {
    if matches!(opts.screen, Some(screen) if screen != workspace.output) ||
        matches!(opts.num, Some(num) if num != workspace.num) ||
        matches!(opts.visible, Some(visible) if visible != workspace.visible) ||
        matches!(opts.name, Some(name) if name != workspace.name)
    {
        return false;
    }

    return true;
}

async fn get_workspaces() -> Result<Workspaces> {
    let mut i3 = I3::connect().await?;
    let workspaces = i3.get_workspaces().await?;
    Ok(workspaces)
}
