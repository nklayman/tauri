use super::{common, osx_bundle};
use crate::Settings;

use anyhow::Context;

use std::{
  env,
  fs::{self, write},
  path::PathBuf,
  process::{Command, Stdio},
};

/// Bundles the project.
/// Returns a vector of PathBuf that shows where the DMG was created.
pub fn bundle_project(settings: &Settings) -> crate::Result<Vec<PathBuf>> {
  // generate the .app bundle
  osx_bundle::bundle_project(settings)?;

  // get the target path
  let output_path = settings.project_out_directory().join("bundle/dmg");
  let package_base_name = format!(
    "{}_{}_{}",
    settings.main_binary_name(),
    settings.version_string(),
    match settings.binary_arch() {
      "x86_64" => "x64",
      other => other,
    }
  );
  let dmg_name = format!("{}.dmg", &package_base_name);
  let dmg_path = output_path.join(&dmg_name);

  let bundle_name = &format!("{}.app", &package_base_name);
  let bundle_dir = settings.project_out_directory().join("bundle/osx");
  let bundle_path = bundle_dir.join(&bundle_name.clone());

  let support_directory_path = output_path.join("support");
  if output_path.exists() {
    fs::remove_dir_all(&output_path)
      .with_context(|| format!("Failed to remove old {}", dmg_name))?;
  }
  fs::create_dir_all(&support_directory_path).with_context(|| {
    format!(
      "Failed to create output directory at {:?}",
      support_directory_path
    )
  })?;

  // create paths for script
  let bundle_script_path = output_path.join("bundle_dmg.sh");
  let license_script_path = support_directory_path.join("dmg-license.py");

  common::print_bundling(format!("{:?}", &dmg_path).as_str())?;

  // write the scripts
  write(
    &bundle_script_path,
    include_str!("templates/dmg/bundle_dmg"),
  )?;
  write(
    support_directory_path.join("template.applescript"),
    include_str!("templates/dmg/template.applescript"),
  )?;
  write(
    &license_script_path,
    include_str!("templates/dmg/dmg-license.py"),
  )?;

  // chmod script for execution
  Command::new("chmod")
    .arg("777")
    .arg(&bundle_script_path)
    .arg(&license_script_path)
    .current_dir(output_path)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .output()
    .expect("Failed to chmod script");

  let mut args = vec![
    "--volname",
    &package_base_name,
    "--volicon",
    "../../../../icons/icon.icns",
    "--icon",
    &bundle_name,
    "180",
    "170",
    "--app-drop-link",
    "480",
    "170",
    "--window-size",
    "660",
    "400",
    "--hide-extension",
    &bundle_name,
  ];

  if let Some(license_path) = settings.osx_license() {
    args.push("--eula");
    args.push(license_path);
  }

  // Issue #592 - Building MacOS dmg files on CI
  // https://github.com/tauri-apps/tauri/issues/592
  if let Some(value) = env::var_os("CI") {
    if value == "true" {
      args.push("--skip-jenkins");
    }
  }

  // execute the bundle script
  let mut cmd = Command::new(&bundle_script_path);
  cmd
    .current_dir(bundle_dir.clone())
    .args(args)
    .args(vec![dmg_name.as_str(), bundle_name.as_str()]);

  common::print_info("running bundle_dmg.sh")?;
  common::execute_with_verbosity(&mut cmd, &settings).map_err(|_| {
    crate::Error::ShellScriptError(format!(
      "error running bundle_dmg.sh{}",
      if settings.is_verbose() {
        ""
      } else {
        ", try running with --verbose to see command output"
      }
    ))
  })?;

  fs::rename(bundle_dir.join(dmg_name), dmg_path.clone())?;
  Ok(vec![bundle_path, dmg_path])
}
