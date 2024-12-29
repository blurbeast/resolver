use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ClapperArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

// ---------------
// Valid Commands: get | scaffold
// ---------------
#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Clones projects boilerplate for diamond standard (JavaScript, TypeScript and Foundry) and NestJs
    Get(GetCommand),
    /// Scaffolds projects for any development tool
    Scaffold(ScaffoldCommand),
    /// Installs dependencies and software development tools
    Install(InstallCommand),
}

// ----------------
// GetCommand Args
// ----------------
#[derive(Debug, Args)]
pub struct GetCommand {
    #[clap(subcommand)]
    pub command: GetSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum GetSubCommand {
    /// Clones a diamond standard JavaScript project
    Dhjs(GetDir),
    /// Clones a diamond standard TypeScript project
    Dhts(GetDir),
    /// Clones a diamond standard Foundry project
    Dfd(GetDir),
    /// Clones a NestJS project boilerplate
    Nestjs(GetDir),
}

// --------------------
// ScaffoldCommand Args
// --------------------
#[derive(Debug, Args)]
pub struct ScaffoldCommand {
    #[clap(subcommand)]
    pub command: ScaffoldSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ScaffoldSubCommand {
    /// Scaffolds a create-react-app JavaScript project
    Reactjs(GetDir),
    /// Scaffolds a create-react-app TypeScript project
    Reactts(GetDir),
    /// Scaffolds a Hardhat project
    Hardhat(GetDir),
    /// Scaffolds a NestJS project
    Nestjs(GetDir),
    /// Scaffolds a Laravel project
    Laravel(GetDir),
    /// Scaffolds a Next project
    Nextjs(GetDir),
    /// Scaffolds a Foundry project
    Foundry(GetDir),
    /// Scaffold a Vue.js project
    Vue(GetDir),
    /// Scaffold (Vanilla TypeScript, Vue, React, Preact, Lit, Svelte) projects using Vite
    Vite(GetDir),
    /// Scaffold a Noir project
    Noir(GetDir),
    /// Scaffold a Starknet Foundry project
    Snforge(GetDir),
    /// Scaffold a RainbowKit + Wagmi + Next.js project
    RainbowKit(GetDir),
    /// Scafold a React-Native Expo project
    ReactNativeExpo(GetDir),
    /// Scafold an Adonis.js Project
    Adonis(GetDir),
    /// Scaffold an Anchor project with TypeScript Tests
    AnchorTS(GetDir),
    /// Scaffold an Anchor project with Rust Tests
    AnchorRust(GetDir),
    // scaffold a react project
    React(CreateReactSubCommand),
}

// ----------------
// InstallCommand Args
// ----------------
#[derive(Debug, Args)]
pub struct InstallCommand {
    #[clap(subcommand)]
    pub command: InstallSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum InstallSubCommand {
    /// Installs Homebrew
    Brew,
    /// Installs Chocolatey
    Choco,
    /// Installs Node.js
    Node,
    /// Installs Scarb
    Scarb,
    /// Installs Forge
    Forge,
    /// Installs Starkli
    Starkli,
    /// Installs Nargo
    Noir,
    // installs Starknet Foundry
    Snfoundry(Version),
    /// Installs the Solana CLI
    Solana,
    /// Installs Anchor
    Anchor,
}

// --------------------------------------
// GetDir: For passing the directory name
// --------------------------------------
#[derive(Debug, Args,)]
pub struct GetDir {
    /// Specifies the name of the project directory to initialize
    pub dir_name: String,
}


// for passing both the directory name and the language of choice
// the args is an inbuilt library from clap used to take input from the terminal
#[derive(Debug, Args,)]
pub struct CreateReactSubCommand {
    pub dir_name: String, // Specifies the name of the project directory to initialize

    #[clap(long , short)] // this is used purposely for explicit choice when dealing with the argument via terminal hence --lan t or --lan=t
    pub lan: ReactVariants, // specifies the language of choice for the project
}

// to choose the language variants
// the valueEnum , just like the Args is a library from clap used to take argument from the terminal
#[derive(Debug, ValueEnum, Clone)]
pub enum ReactVariants {
    J, // javascript
    T, // typescript
}


#[derive(Debug, Args)]
pub struct Version {
    pub version_name: String,
}
