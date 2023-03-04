use std::{os::unix::process::CommandExt, process::Command};
use xtask_wasm::{anyhow::Result, clap};

#[derive(clap::Parser)]
enum Opt {
    Dist(xtask_wasm::Dist),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
}

fn build_tailwind() -> Command {
    let mut command = Command::new("sh");
    command.args(["-c", "pnpm build"]);
    command.exec();
    command
}

fn main() -> Result<()> {
    let opt: Opt = clap::Parser::parse();

    match opt {
        Opt::Dist(dist) => {
            dist.dist_dir_path("apps/web/dist")
                .static_dir_path("apps/web/public")
                .app_name("web")
                .build_command(build_tailwind())
                .run_in_workspace(true)
                .run("web")?;
        }
        Opt::Watch(watch) => {
            let mut command = Command::new("cargo");
            command.arg("check");

            watch.run(command)?;
        }
        Opt::Start(dev_server) => {
            log::info!("App ready at: {}", "127.0.0.1:8000");
            dev_server
                .arg("dist")
                .start("apps/web/dist")
                .expect("Error");
        }
    }

    Ok(())
}
