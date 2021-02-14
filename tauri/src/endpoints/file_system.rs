use super::allowlist_error;
use crate::{api::path::BaseDirectory, ApplicationDispatcherExt};

use serde::Deserialize;
use tauri_api::{dir, file, path::resolve_path};

use std::{fs, fs::File, io::Write, path::PathBuf};

/// The options for the directory functions on the file system API.
#[derive(Deserialize)]
pub struct DirOperationOptions {
  /// Whether the API should recursively perform the operation on the directory.
  #[serde(default)]
  pub recursive: bool,
  /// The base directory of the operation.
  /// The directory path of the BaseDirectory will be the prefix of the defined directory path.
  pub dir: Option<BaseDirectory>,
}

/// The options for the file functions on the file system API.
#[derive(Deserialize)]
pub struct FileOperationOptions {
  /// The base directory of the operation.
  /// The directory path of the BaseDirectory will be the prefix of the defined file path.
  pub dir: Option<BaseDirectory>,
}

/// The API descriptor.
#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  /// The read text file API.
  ReadTextFile {
    path: PathBuf,
    options: Option<FileOperationOptions>,
    callback: String,
    error: String,
  },
  /// The read binary file API.
  ReadBinaryFile {
    path: PathBuf,
    options: Option<FileOperationOptions>,
    callback: String,
    error: String,
  },
  /// The write file API.
  WriteFile {
    path: PathBuf,
    contents: String,
    options: Option<FileOperationOptions>,
    callback: String,
    error: String,
  },
  /// The write binary file API.
  WriteBinaryFile {
    path: PathBuf,
    contents: String,
    options: Option<FileOperationOptions>,
    callback: String,
    error: String,
  },
  /// The read dir API.
  ReadDir {
    path: PathBuf,
    options: Option<DirOperationOptions>,
    callback: String,
    error: String,
  },
  /// The copy file API.
  CopyFile {
    source: PathBuf,
    destination: PathBuf,
    options: Option<FileOperationOptions>,
    callback: String,
    error: String,
  },
  /// The create dir API.
  CreateDir {
    path: PathBuf,
    options: Option<DirOperationOptions>,
    callback: String,
    error: String,
  },
  /// The remove dir API.
  RemoveDir {
    path: PathBuf,
    options: Option<DirOperationOptions>,
    callback: String,
    error: String,
  },
  /// The remove file API.
  RemoveFile {
    path: PathBuf,
    options: Option<FileOperationOptions>,
    callback: String,
    error: String,
  },
  /// The rename file API.
  #[serde(rename_all = "camelCase")]
  RenameFile {
    old_path: PathBuf,
    new_path: PathBuf,
    options: Option<FileOperationOptions>,
    callback: String,
    error: String,
  },
  /// The resolve path API
  ResolvePath {
    path: String,
    directory: Option<BaseDirectory>,
    callback: String,
    error: String,
  },
}

impl Cmd {
  pub async fn run<D: ApplicationDispatcherExt + 'static>(
    self,
    webview_manager: &crate::WebviewManager<D>,
  ) {
    match self {
      Self::ReadTextFile {
        path,
        options,
        callback,
        error,
      } => {
        #[cfg(read_text_file)]
        read_text_file(webview_manager, path, options, callback, error).await;
        #[cfg(not(read_text_file))]
        allowlist_error(webview_manager, error, "readTextFile");
      }
      Self::ReadBinaryFile {
        path,
        options,
        callback,
        error,
      } => {
        #[cfg(read_binary_file)]
        read_binary_file(webview_manager, path, options, callback, error).await;
        #[cfg(not(read_binary_file))]
        allowlist_error(webview_manager, error, "readBinaryFile");
      }
      Self::WriteFile {
        path,
        contents,
        options,
        callback,
        error,
      } => {
        #[cfg(write_file)]
        write_file(webview_manager, path, contents, options, callback, error).await;
        #[cfg(not(write_file))]
        allowlist_error(webview_manager, error, "writeFile");
      }
      Self::WriteBinaryFile {
        path,
        contents,
        options,
        callback,
        error,
      } => {
        #[cfg(write_binary_file)]
        write_binary_file(webview_manager, path, contents, options, callback, error).await;
        #[cfg(not(write_binary_file))]
        allowlist_error(webview_manager, error, "writeBinaryFile");
      }
      Self::ReadDir {
        path,
        options,
        callback,
        error,
      } => {
        #[cfg(read_dir)]
        read_dir(webview_manager, path, options, callback, error).await;
        #[cfg(not(read_dir))]
        allowlist_error(webview_manager, error, "readDir");
      }
      Self::CopyFile {
        source,
        destination,
        options,
        callback,
        error,
      } => {
        #[cfg(copy_file)]
        copy_file(
          webview_manager,
          source,
          destination,
          options,
          callback,
          error,
        )
        .await;
        #[cfg(not(copy_file))]
        allowlist_error(webview_manager, error, "copyFile");
      }
      Self::CreateDir {
        path,
        options,
        callback,
        error,
      } => {
        #[cfg(create_dir)]
        create_dir(webview_manager, path, options, callback, error).await;
        #[cfg(not(create_dir))]
        allowlist_error(webview_manager, error, "createDir");
      }
      Self::RemoveDir {
        path,
        options,
        callback,
        error,
      } => {
        #[cfg(remove_dir)]
        remove_dir(webview_manager, path, options, callback, error).await;
        #[cfg(not(remove_dir))]
        allowlist_error(webview_manager, error, "removeDir");
      }
      Self::RemoveFile {
        path,
        options,
        callback,
        error,
      } => {
        #[cfg(remove_file)]
        remove_file(webview_manager, path, options, callback, error).await;
        #[cfg(not(remove_file))]
        allowlist_error(webview_manager, error, "removeFile");
      }
      Self::RenameFile {
        old_path,
        new_path,
        options,
        callback,
        error,
      } => {
        #[cfg(rename_file)]
        rename_file(
          webview_manager,
          old_path,
          new_path,
          options,
          callback,
          error,
        )
        .await;
        #[cfg(not(rename_file))]
        allowlist_error(webview_manager, error, "renameFile");
      }
      Self::ResolvePath {
        path,
        directory,
        callback,
        error,
      } => {
        #[cfg(path_api)]
        resolve_path_handler(webview_manager, path, directory, callback, error).await;
        #[cfg(not(path_api))]
        allowlist_error(webview_manager, error, "pathApi");
      }
    }
  }
}

/// Reads a directory.
#[cfg(read_dir)]
pub async fn read_dir<D: ApplicationDispatcherExt>(
  webview_manager: &crate::WebviewManager<D>,
  path: PathBuf,
  options: Option<DirOperationOptions>,
  callback: String,
  error: String,
) {
  crate::execute_promise(
    webview_manager,
    async move {
      let (recursive, dir) = if let Some(options_value) = options {
        (options_value.recursive, options_value.dir)
      } else {
        (false, None)
      };
      dir::read_dir(resolve_path(path, dir)?, recursive).map_err(crate::Error::FailedToExecuteApi)
    },
    callback,
    error,
  )
  .await;
}

/// Copies a file.
#[cfg(copy_file)]
pub async fn copy_file<D: ApplicationDispatcherExt>(
  webview_manager: &crate::WebviewManager<D>,
  source: PathBuf,
  destination: PathBuf,
  options: Option<FileOperationOptions>,
  callback: String,
  error: String,
) {
  crate::execute_promise(
    webview_manager,
    async move {
      let (src, dest) = match options.and_then(|o| o.dir) {
        Some(dir) => (
          resolve_path(source, Some(dir.clone()))?,
          resolve_path(destination, Some(dir))?,
        ),
        None => (source, destination),
      };
      fs::copy(src, dest)?;
      crate::Result::Ok(())
    },
    callback,
    error,
  )
  .await;
}

/// Creates a directory.
#[cfg(create_dir)]
pub async fn create_dir<D: ApplicationDispatcherExt>(
  webview_manager: &crate::WebviewManager<D>,
  path: PathBuf,
  options: Option<DirOperationOptions>,
  callback: String,
  error: String,
) {
  crate::execute_promise(
    webview_manager,
    async move {
      let (recursive, dir) = if let Some(options_value) = options {
        (options_value.recursive, options_value.dir)
      } else {
        (false, None)
      };
      let resolved_path = resolve_path(path, dir)?;
      if recursive {
        fs::create_dir_all(resolved_path)?;
      } else {
        fs::create_dir(resolved_path)?;
      }

      crate::Result::Ok(())
    },
    callback,
    error,
  )
  .await;
}

/// Removes a directory.
#[cfg(remove_dir)]
pub async fn remove_dir<D: ApplicationDispatcherExt>(
  webview_manager: &crate::WebviewManager<D>,
  path: PathBuf,
  options: Option<DirOperationOptions>,
  callback: String,
  error: String,
) {
  crate::execute_promise(
    webview_manager,
    async move {
      let (recursive, dir) = if let Some(options_value) = options {
        (options_value.recursive, options_value.dir)
      } else {
        (false, None)
      };
      let resolved_path = resolve_path(path, dir)?;
      if recursive {
        fs::remove_dir_all(resolved_path)?;
      } else {
        fs::remove_dir(resolved_path)?;
      }

      crate::Result::Ok(())
    },
    callback,
    error,
  )
  .await;
}

/// Removes a file
#[cfg(remove_file)]
pub async fn remove_file<D: ApplicationDispatcherExt>(
  webview_manager: &crate::WebviewManager<D>,
  path: PathBuf,
  options: Option<FileOperationOptions>,
  callback: String,
  error: String,
) {
  crate::execute_promise(
    webview_manager,
    async move {
      let resolved_path = resolve_path(path, options.and_then(|o| o.dir))?;
      fs::remove_file(resolved_path)?;
      crate::Result::Ok(())
    },
    callback,
    error,
  )
  .await;
}

/// Renames a file.
#[cfg(rename_file)]
pub async fn rename_file<D: ApplicationDispatcherExt>(
  webview_manager: &crate::WebviewManager<D>,
  old_path: PathBuf,
  new_path: PathBuf,
  options: Option<FileOperationOptions>,
  callback: String,
  error: String,
) {
  crate::execute_promise(
    webview_manager,
    async move {
      let (old, new) = match options.and_then(|o| o.dir) {
        Some(dir) => (
          resolve_path(old_path, Some(dir.clone()))?,
          resolve_path(new_path, Some(dir))?,
        ),
        None => (old_path, new_path),
      };
      fs::rename(old, new).map_err(crate::Error::Io)
    },
    callback,
    error,
  )
  .await;
}

/// Writes a text file.
#[cfg(write_file)]
pub async fn write_file<D: ApplicationDispatcherExt>(
  webview_manager: &crate::WebviewManager<D>,
  path: PathBuf,
  contents: String,
  options: Option<FileOperationOptions>,
  callback: String,
  error: String,
) {
  crate::execute_promise(
    webview_manager,
    async move {
      File::create(resolve_path(path, options.and_then(|o| o.dir))?)
        .map_err(crate::Error::Io)
        .and_then(|mut f| f.write_all(contents.as_bytes()).map_err(|err| err.into()))?;
      crate::Result::Ok(())
    },
    callback,
    error,
  )
  .await;
}

/// Writes a binary file.
#[cfg(write_binary_file)]
pub async fn write_binary_file<D: ApplicationDispatcherExt>(
  webview_manager: &crate::WebviewManager<D>,
  path: PathBuf,
  contents: String,
  options: Option<FileOperationOptions>,
  callback: String,
  error: String,
) {
  crate::execute_promise(
    webview_manager,
    async move {
      base64::decode(contents)
        .map_err(crate::Error::Base64Decode)
        .and_then(|c| {
          File::create(resolve_path(path, options.and_then(|o| o.dir))?)
            .map_err(|e| e.into())
            .and_then(|mut f| f.write_all(&c).map_err(|err| err.into()))
        })?;
      crate::Result::Ok(())
    },
    callback,
    error,
  )
  .await;
}

/// Reads a text file.
#[cfg(read_text_file)]
pub async fn read_text_file<D: ApplicationDispatcherExt>(
  webview_manager: &crate::WebviewManager<D>,
  path: PathBuf,
  options: Option<FileOperationOptions>,
  callback: String,
  error: String,
) {
  crate::execute_promise(
    webview_manager,
    async move {
      file::read_string(resolve_path(path, options.and_then(|o| o.dir))?)
        .map_err(crate::Error::FailedToExecuteApi)
    },
    callback,
    error,
  )
  .await;
}

/// Reads a binary file.
#[cfg(read_binary_file)]
pub async fn read_binary_file<D: ApplicationDispatcherExt>(
  webview_manager: &crate::WebviewManager<D>,
  path: PathBuf,
  options: Option<FileOperationOptions>,
  callback: String,
  error: String,
) {
  crate::execute_promise(
    webview_manager,
    async move {
      file::read_binary(resolve_path(path, options.and_then(|o| o.dir))?)
        .map_err(crate::Error::FailedToExecuteApi)
    },
    callback,
    error,
  )
  .await;
}

pub async fn resolve_path_handler<D: ApplicationDispatcherExt>(
  webview_manager: &crate::WebviewManager<D>,
  path: String,
  directory: Option<BaseDirectory>,
  callback: String,
  error: String,
) {
  crate::execute_promise(
    webview_manager,
    async move { resolve_path(path, directory).map_err(|e| e.into()) },
    callback,
    error,
  )
  .await
}

// test webview functionality.
#[cfg(test)]
mod test {
  // use super::*;
  // use web_view::*;

  // create a makeshift webview
  // fn create_test_webview() -> crate::Result<WebView<'static, ()>> {
  //   // basic html set into webview
  //   let content = r#"<html><head></head><body></body></html>"#;

  //   Ok(
  //     // use webview builder to create simple webview
  //     WebViewBuilder::new()
  //       .title("test")
  //       .size(800, 800)
  //       .resizable(true)
  //       .debug(true)
  //       .user_data(())
  //       .invoke_handler(|_wv, _arg| Ok(()))
  //       .content(Content::Html(content))
  //       .build()?,
  //   )
  // }

  /* #[test]
  #[cfg(not(any(target_os = "linux", target_os = "macos")))]
  // test the file_write functionality
  fn test_write_to_file() -> crate::Result<()> {
    // import read_to_string and write to be able to manipulate the file.
    use std::fs::{read_to_string, write};

    // create the webview
    let mut webview = create_test_webview()?;

    // setup the contents and the path.
    let contents = String::from(r#"Write to the Test file"#);
    let path = String::from("test/fixture/test.txt");

    // clear the file by writing nothing to it.
    write(&path, "")?;

    //call write file with the path and contents.
    write_file(
      &webview_manager,
      path.clone(),
      contents.clone(),
      String::from(""),
      String::from(""),
    );

    // sleep the main thread to wait for the promise to execute.
    std::thread::sleep(std::time::Duration::from_millis(200));

    // read from the file.
    let data = read_to_string(path)?;

    // check that the file contents is equal to the expected contents.
    assert_eq!(data, contents);

    Ok(())
  } */
}
