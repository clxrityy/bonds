use bonds_cli::args::{Cli, Commands, ConfigAction, MetadataAction};
use clap::Parser;

// Helper function to extract the command from the parsed CLI arguments for testing purposes.
fn cmd(cli: Cli) -> Commands {
    cli.command.expect("expected subcommand")
}

#[test]
fn parse_add_with_target() {
    let cli = Cli::try_parse_from(["bond", "add", "/src", "/tgt"]).unwrap();

    match cmd(cli) {
        Commands::Add {
            source,
            target,
            name: _,
            flags: _,
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
    match cmd(cli) {
        Commands::Add {
            source,
            target,
            flags: _,
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
    assert!(matches!(cmd(cli), Commands::List));
}

#[test]
fn parse_info() {
    let cli = Cli::try_parse_from(["bond", "info", "abc123"]).unwrap();
    match cmd(cli) {
        Commands::Info { id } => assert_eq!(id, "abc123"),
        _ => panic!("expected Info"),
    }
}

#[test]
fn parse_remove_with_target_flag() {
    let cli = Cli::try_parse_from(["bond", "remove", "abc123", "--with-target"]).unwrap();
    match cmd(cli) {
        Commands::Remove {
            id,
            with_target,
            flags: _,
        } => {
            assert_eq!(id, "abc123");
            assert!(with_target);
        }
        _ => panic!("expected Remove"),
    }
}

#[test]
fn parse_update_target_only() {
    let cli = Cli::try_parse_from(["bond", "update", "abc123", "--target", "/new"]).unwrap();
    match cmd(cli) {
        Commands::Update {
            id,
            source,
            target,
            name,
            flags: _,
        } => {
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
    match cmd(cli) {
        Commands::Update {
            id,
            source,
            target,
            name,
            flags: _,
        } => {
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
    match cmd(cli) {
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
    match cmd(cli) {
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
    assert_eq!(cli.db.as_ref().unwrap().to_str().unwrap(), "/tmp/test.db");
    assert!(matches!(cmd(cli), Commands::List));
}

#[test]
fn parse_add_rejects_contents_flag() {
    let err = Cli::try_parse_from(["bond", "add", "/src", "/tgt", "--contents"]);
    assert!(err.is_err());
}

#[test]
fn parse_no_subcommand_is_allowed() {
    let cli = Cli::try_parse_from(["bond"]).unwrap();
    assert!(cli.command.is_none());
}

#[test]
fn parse_add_with_name() {
    let cli = Cli::try_parse_from(["bond", "add", "/src", "--name", "foo"]).unwrap();
    match cmd(cli) {
        Commands::Add { name, .. } => assert_eq!(name.unwrap(), "foo"),
        _ => panic!("expected Add"),
    }
}

#[test]
fn parse_update_with_name() {
    let cli = Cli::try_parse_from(["bond", "update", "abc", "--name", "bar"]).unwrap();
    match cmd(cli) {
        Commands::Update { id, name, .. } => {
            assert_eq!(id, "abc");
            assert_eq!(name.unwrap(), "bar");
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn parse_migrate_with_dest() {
    let cli = Cli::try_parse_from(["bond", "migrate", "foo", "/new/dir"]).unwrap();
    match cmd(cli) {
        Commands::Migrate { id, dest, flags: _ } => {
            assert_eq!(id, "foo");
            assert_eq!(dest.unwrap().to_str().unwrap(), "/new/dir");
        }
        _ => panic!("expected Migrate"),
    }
}

#[test]
fn parse_migrate_without_dest() {
    let cli = Cli::try_parse_from(["bond", "migrate", "foo"]).unwrap();
    match cmd(cli) {
        Commands::Migrate { id, dest, flags: _ } => {
            assert_eq!(id, "foo");
            assert!(dest.is_none());
        }
        _ => panic!("expected Migrate"),
    }
}

#[test]
fn parse_metadata_get_all() {
    let cli = Cli::try_parse_from(["bond", "metadata", "get", "foo"]).unwrap();
    match cmd(cli) {
        Commands::Metadata {
            action: MetadataAction::Get { id, key },
        } => {
            assert_eq!(id, "foo");
            assert!(key.is_none());
        }
        _ => panic!("expected Metadata Get"),
    }
}

#[test]
fn parse_metadata_get_key() {
    let cli = Cli::try_parse_from(["bond", "metadata", "get", "foo", "owner"]).unwrap();
    match cmd(cli) {
        Commands::Metadata {
            action: MetadataAction::Get { id, key },
        } => {
            assert_eq!(id, "foo");
            assert_eq!(key.unwrap(), "owner");
        }
        _ => panic!("expected Metadata Get"),
    }
}

#[test]
fn parse_metadata_set() {
    let cli = Cli::try_parse_from(["bond", "metadata", "set", "foo", "owner", "mj"]).unwrap();
    match cmd(cli) {
        Commands::Metadata {
            action: MetadataAction::Set { id, key, value },
        } => {
            assert_eq!(id, "foo");
            assert_eq!(key, "owner");
            assert_eq!(value, "mj");
        }
        _ => panic!("expected Metadata Set"),
    }
}

#[test]
fn parse_metadata_remove() {
    let cli = Cli::try_parse_from(["bond", "metadata", "remove", "foo", "owner"]).unwrap();
    match cmd(cli) {
        Commands::Metadata {
            action: MetadataAction::Remove { id, key },
        } => {
            assert_eq!(id, "foo");
            assert_eq!(key, "owner");
        }
        _ => panic!("expected Metadata Remove"),
    }
}
