use crate::Version;
use crate::applications::ApplicationName;
use crate::context::RuntimeContext;
use crate::error::{Result, UserError};
use crate::logging::Event;
use fd_lock::RwLock;
use std::fs::File;

/// runs the given function while holding a lock on the app folder
pub fn with_lock<T>(app_name: &ApplicationName, version: &Version, ctx: &RuntimeContext, f: impl FnOnce() -> Result<T>) -> Result<T> {
  // acquire the lock
  let app_folder = ctx.yard.create_app_folder(app_name, version)?;
  let lock_path = app_folder.join(".run-that-app-lock");
  let lock_file = File::create(&lock_path).map_err(|err| UserError::CannotCreateFile {
    filename: lock_path.clone(),
    err: err.to_string(),
  })?;
  (ctx.log)(Event::LockAcquireBegin { app: app_name });
  let mut lock = RwLock::new(lock_file);
  let guard = lock.write().map_err(|err| UserError::LockCannotAcquire {
    filename: lock_path,
    err: err.to_string(),
  })?;
  (ctx.log)(Event::LockAcquireSuccess);

  let result = f();

  // release the lock
  (ctx.log)(Event::LockRelease { app: app_name });
  drop(guard);

  // Note: don't delete the lockfile
  // because that would allow another process to create a new file
  // and acquire a lock on that one.

  result
}
