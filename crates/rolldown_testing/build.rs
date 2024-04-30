// cSpell:disable
use jsonschema::is_valid;
use rolldown_testing_config::TestConfig;
use schemars::schema_for;
use serde_json::to_string_pretty;
use std::fs;
use std::path::PathBuf;

fn main() {
  // If the definition of `TestConfig` changes, this build script will automatically re-run due to we rely on `rolldown_testing_config` in `Cargo.toml` already.
  // So we only add `build.rs` as the dependency to prevent unnecessary re-runs for every `cargo build`
  println!("cargo:rerun-if-changed=build.rs");
  std::env::set_current_dir("../").unwrap();
  // Add config files as dependencies to re-run this build script when they change.
  let config_paths = glob::glob("rolldown/tests/**/_config.json")
    .expect("Failed to scan config files")
    .map(Result::unwrap)
    .collect::<Vec<_>>();
  for path in &config_paths {
    println!("cargo:rerun-if-changed={path:?}");
  }

  let schema = schema_for!(TestConfig);
  let schema_str = to_string_pretty(&schema).expect("Should be valid Schema");
  let schema_value = serde_json::from_str(&schema_str).expect("Should be valid JSON");
  // Validate all the config files by the schema
  for path in config_paths {
    let config_str =
      fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read config file: {e:?}"));
    let config_value = serde_json::from_str(&config_str).expect("Failed to parse test config file");
    assert!(is_valid(&schema_value, &config_value), "Invalid config: {path:?}");
  }

  let scheme_path =
    PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").expect("Should have CARGO_MANIFEST_DIR"))
      .join("_test.scheme.json");
  fs::write(scheme_path, schema_str).expect("Failed to write schema");
}
