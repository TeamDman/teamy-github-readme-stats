pub mod cache;
pub mod docs;
pub mod facet_shape;
pub mod fetch;
pub mod generate;
pub mod global_args;
pub mod home;
pub mod serve;

use crate::cli::cache::CacheArgs;
use crate::cli::docs::DocsArgs;
use crate::cli::fetch::FetchArgs;
use crate::cli::generate::GenerateArgs;
use crate::cli::global_args::GlobalArgs;
use crate::cli::home::HomeArgs;
use crate::cli::serve::ServeArgs;
use arbitrary::Arbitrary;
use eyre::Context;
use facet::Facet;
use figue::FigueBuiltins;
use figue::{self as args};

/// Fetch GitHub profile data and generate README stat cards.
#[derive(Facet, Arbitrary, Debug)]
pub struct Cli {
    /// Global arguments (`debug`, `log_filter`, `log_file`).
    #[facet(flatten)]
    pub global_args: GlobalArgs,

    /// Standard CLI options (help, version, completions).
    #[facet(flatten)]
    #[arbitrary(default)]
    pub builtins: FigueBuiltins,

    /// The command to run.
    #[facet(args::subcommand)]
    pub command: Command,
}

impl PartialEq for Cli {
    fn eq(&self, other: &Self) -> bool {
        // Ignore builtins in comparison since FigueBuiltins doesn't implement PartialEq
        self.global_args == other.global_args && self.command == other.command
    }
}

impl Cli {
    /// # Errors
    ///
    /// This function will return an error if the tokio runtime cannot be built or if the command fails.
    pub fn invoke(self) -> eyre::Result<()> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .wrap_err("Failed to build tokio runtime")?;
        runtime.block_on(async move { self.command.invoke().await })?;
        Ok(())
    }
}

/// Fetch GitHub profile data and generate README stat cards.
#[derive(Facet, Arbitrary, Debug, PartialEq)]
#[repr(u8)]
pub enum Command {
    /// Cache-related commands.
    Cache(CacheArgs),
    /// Docs-related commands.
    Docs(DocsArgs),
    /// Fetch GitHub stats and persist them into the local cache.
    Fetch(FetchArgs),
    /// Generate a stats card from cached or freshly fetched data.
    Generate(GenerateArgs),
    /// Home-related commands.
    Home(HomeArgs),
    /// Serve stats cards over HTTP.
    Serve(ServeArgs),
}

impl Command {
    /// # Errors
    ///
    /// This function will return an error if the subcommand fails.
    pub async fn invoke(self) -> eyre::Result<()> {
        match self {
            Command::Cache(args) => args.invoke().await,
            Command::Docs(args) => args.invoke().await,
            Command::Fetch(args) => args.invoke().await,
            Command::Generate(args) => args.invoke().await,
            Command::Home(args) => args.invoke().await,
            Command::Serve(args) => args.invoke().await,
        }
    }
}
