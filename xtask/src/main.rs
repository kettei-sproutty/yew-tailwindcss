use std::process;
use xtask_wasm::{anyhow::Result, clap, default_build_command};

#[derive(clap::Parser)]
struct Opt {
    #[clap(long = "log", default_value = "Info")]
    log_level: log::LevelFilter,
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(clap::Parser)]
enum Command {
    Dist(Build),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
}

#[derive(clap::Parser)]
struct Build {
    /// Optimize the generated package using `wasm-opt`.
    #[clap(long)]
    optimize: bool,

    #[clap(flatten)]
    base: xtask_wasm::Dist,
}

fn main() -> Result<()> {
    let opt: Opt = clap::Parser::parse();

    env_logger::builder()
        .filter(Some("xtask"), opt.log_level)
        .init();

    let mut build_tailwind_command = process::Command::new("sh");
    build_tailwind_command.args(["-c", "pnpm build"]);
    build_tailwind_command.output().expect("failed to execute process");

    match opt.cmd {
        Command::Dist(arg) => {
            log::info!("Generating package...");

            let dist_result = arg
                .base
                .dist_dir_path("apps/web/dist")
                .static_dir_path("apps/web/public")
                .build_command(build_tailwind_command)
                .build_command(default_build_command())
                .app_name("web")
                .run_in_workspace(true)
                .run("web")?;

            if arg.optimize {
                xtask_wasm::WasmOpt::level(1)
                    .shrink(2)
                    .optimize(dist_result.wasm)?;
            }
        }
        Command::Watch(arg) => {
            log::info!("Watching for changes and check...");

            let mut command = process::Command::new("cargo");
            command.arg("check");

            arg.run(command)?;
        }
        Command::Start(arg) => {
            log::info!("App ready at: {}", "127.0.0.1:8000");
            arg.arg("dist")
                .start("apps/web/dist")
                .expect("failed to execute process");
        }
    }

    Ok(())
}
