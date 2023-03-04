use std::{process::Command, os::unix::process::CommandExt};
use xtask_wasm::{anyhow::Result, clap, default_dist_dir};

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
            dist
                .dist_dir_path("apps/web/dist")
                .static_dir_path("apps/web/public")
                .app_name("web")
                .run_in_workspace(true)
                // .build_command(build_tailwind())
                .run("web")?;

                build_tailwind();
        }
        Opt::Watch(watch) => {
            let mut command = Command::new("cargo");
            command.arg("check");

            watch.run(command)?;
        }
        Opt::Start(dev_server) => {
            dev_server.arg("dist").start(default_dist_dir(false))?;
        }
    }

    Ok(())
}
