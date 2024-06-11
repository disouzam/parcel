use std::path::PathBuf;
use std::sync::Arc;

use napi::Env;
use napi::JsObject;
use napi_derive::napi;
use parcel::rpc::nodejs::RpcHostNodejs;
use parcel::Parcel;
use parcel::ParcelOptions;

use crate::file_system::FileSystemNapi;

#[napi]
pub struct ParcelNapi {
  parcel: Arc<Parcel>,
}

#[napi]
impl ParcelNapi {
  #[napi(constructor)]
  pub fn new(env: Env, options: JsObject) -> napi::Result<Self> {
    // Debugging Instrumentation
    let _ = tracing_subscriber::fmt::try_init();
    let thread_id = std::thread::current().id();
    tracing::trace!(?thread_id, "parcel-napi initialize");

    // Wrap the JavaScript-supplied FileSystem
    let fs = FileSystemNapi::from_options(&env, &options)?;

    // Set up Nodejs plugin bindings
    let rpc_host_nodejs = RpcHostNodejs::new(&env, options.get_named_property("rpc")?)?;

    // Initialize Parcel
    let parcel = Parcel::new(ParcelOptions {
      fs,
      rpc: Some(Arc::new(rpc_host_nodejs)),
    });

    Ok(Self {
      parcel: Arc::new(parcel),
    })
  }

  // Temporary, for testing
  #[napi]
  pub async fn _testing_temp_fs_read_to_string(&self, path: String) -> napi::Result<String> {
    Ok(self.parcel.fs.read_to_string(&PathBuf::from(path))?)
  }

  #[napi]
  pub async fn _testing_temp_fs_is_file(&self, path: String) -> napi::Result<bool> {
    Ok(self.parcel.fs.is_file(&PathBuf::from(path)))
  }

  #[napi]
  pub async fn _testing_temp_fs_is_dir(&self, path: String) -> napi::Result<bool> {
    Ok(self.parcel.fs.is_dir(&PathBuf::from(path)))
  }

  #[napi]
  pub async fn _testing_rpc_ping(&self) -> napi::Result<()> {
    if self.parcel.rpc.as_ref().unwrap().ping().is_err() {
      return Err(napi::Error::from_reason("Failed to run"));
    }
    Ok(())
  }
}