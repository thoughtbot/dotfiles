use std::env;

use anyhow::{bail, Result};
use clap::Parser;

use agent_sync::cli::{Cli, Command, PublishArgs, PullArgs};
use agent_sync::config::Config;
use agent_sync::publish::{self, PublishConfig};
use agent_sync::pull::{self, PullConfig};
use agent_sync::{doctor, list, migrate, sync, verify};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Sync(args) => {
            let config = Config::discover(args.root)?;
            sync::run_with_source(&config, args.dry_run, args.source)
        }
        Command::Verify(args) => {
            let config = Config::discover(args.root)?;
            if verify::run(&config)? {
                Ok(())
            } else {
                bail!("verification failed")
            }
        }
        Command::List(args) => {
            let config = Config::discover(args.root)?;
            list::run(&config)
        }
        Command::Migrate(args) => {
            let config = Config::discover(args.root.clone())?;
            migrate::run(&config, &args)
        }
        Command::Doctor(args) => {
            let config = Config::discover(args.root)?;
            doctor::run(&config, args.fix)
        }
        Command::Publish(args) => run_publish(args),
        Command::Pull(args) => run_pull(args),
    }
}

fn run_pull(args: PullArgs) -> Result<()> {
    let home = env::var_os("HOME").ok_or_else(|| anyhow::anyhow!("HOME is not set"))?;
    let base_url = args
        .base_url
        .unwrap_or_else(|| "http://localhost:8081".to_owned());
    let status = pull::pull(&PullConfig {
        home: home.into(),
        base_url,
        channel: args.channel,
        if_stale: args.if_stale,
    })?;
    match status {
        pull::PullStatus::Updated => {}
        pull::PullStatus::NotModified => println!("PULL not modified"),
        pull::PullStatus::Skipped => {}
    }
    Ok(())
}

fn run_publish(args: PublishArgs) -> Result<()> {
    let base_url = args
        .base_url
        .unwrap_or_else(|| "http://localhost:8081".to_owned());

    let items: Vec<String> = args
        .items
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();

    publish::publish(&PublishConfig {
        platform_root: args.platform_root,
        items,
        base_url,
        dry_run: args.dry_run,
    })
}
