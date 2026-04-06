use bonds_cli::args::{Cli, Commands, ConfigAction};
use clap::Parser;

#[test]
fn parse_add_with_target() {
    let cli = Cli::try_parse_from(["bond", "add", "/src", "/tgt"]).unwrap();
    match cli.command {
        Commands::Add {
            source,
            target,
            contents: _,
            name: _,
        } => {
            assert_eq!(source.to_str().unwrap(), "/src");
            assert_eq!(target.unwrap().to_str().unwrap(), "/tgt");
        }
        _ => panic!("expected Add"),
    }
}

#[test]
fn parse_add_without_target() {
    let cli = Cli::try_parse_from(["bond", "add", "/src"]).unwrap();
    match cli.command {
        Commands::Add {
            source,
            target,
            contents: _,
            name: _,
        } => {
            assert_eq!(source.to_str().unwrap(), "/src");
            assert!(target.is_none());
        }
        _ => panic!("expected Add"),
    }
}

#[test]
fn parse_list() {
    let cli = Cli::try_parse_from(["bond", "list"]).unwrap();
    assert!(matches!(cli.command, Commands::List));
}

#[test]
fn parse_info() {
    let cli = Cli::try_parse_from(["bond", "info", "abc123"]).unwrap();
    match cli.command {
        Commands::Info { id } => assert_eq!(id, "abc123"),
        _ => panic!("expected Info"),
    }
}

#[test]
fn parse_remove_with_target_flag() {
    let cli = Cli::try_parse_from(["bond", "remove", "abc123", "--with-target"]).unwrap();
    match cli.command {
        Commands::Remove { id, with_target } => {
            assert_eq!(id, "abc123");
            assert!(with_target);
        }
        _ => panic!("expected Remove"),
    }
}

#[test]
fn parse_update_target_only() {
    let cli = Cli::try_parse_from(["bond", "update", "abc123", "--target", "/new"]).unwrap();
    match cli.command {
        Commands::Update { id, source, target, name } => {
            assert_eq!(id, "abc123");
            assert!(source.is_none());
            assert_eq!(target.unwrap().to_str().unwrap(), "/new");
            assert!(name.is_none());
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn parse_update_both() {
    let cli = Cli::try_parse_from([
        "bond", "update", "abc123", "--source", "/s", "--target", "/t",
    ])
    .unwrap();
    match cli.command {
        Commands::Update { id, source, target, name } => {
            assert_eq!(id, "abc123");
            assert_eq!(source.unwrap().to_str().unwrap(), "/s");
            assert_eq!(target.unwrap().to_str().unwrap(), "/t");
            assert!(name.is_none());
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn parse_config_set() {
    let cli = Cli::try_parse_from(["bond", "config", "set", "default", "/bonds"]).unwrap();
    match cli.command {
        Commands::Config {
            action: ConfigAction::Set { key, value },
        } => {
            assert_eq!(key, "default");
            assert_eq!(value, "/bonds");
        }
        _ => panic!("expected Config Set"),
    }
}

#[test]
fn parse_config_get() {
    let cli = Cli::try_parse_from(["bond", "config", "get", "default"]).unwrap();
    match cli.command {
        Commands::Config {
            action: ConfigAction::Get { key },
        } => {
            assert_eq!(key, "default");
        }
        _ => panic!("expected Config Get"),
    }
}

#[test]
fn parse_global_db_flag() {
    let cli = Cli::try_parse_from(["bond", "--db", "/tmp/test.db", "list"]).unwrap();
    assert_eq!(cli.db.unwrap().to_str().unwrap(), "/tmp/test.db");
    assert!(matches!(cli.command, Commands::List));
}
