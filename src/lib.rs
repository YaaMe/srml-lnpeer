extern crate futures;
extern crate exit_future;
extern crate ln_primitives;
extern crate sr_primitives;
extern crate substrate_service;
extern crate ln_manager;

pub use ln_manager::LnManager;

use std::mem;
use std::sync::Arc;
use std::marker::PhantomData;

use futures::future;
use futures::future::Future;
use futures::future::FutureExt;
use futures::channel::mpsc;
// use futures::sync::mpsc;
use exit_future::Exit;

use ln_manager::ln_bridge::settings::Settings;
use ln_manager::executor::Larva;

use sr_primitives::traits::{self, ProvideRuntimeApi};
pub use ln_primitives::LnApi;
// use substrate_service::{SpawnTaskHandle, Executor};

pub type Executor = tokio::runtime::TaskExecutor;

#[derive(Clone)]
struct Drone {
    // spawn_task_handle: SpawnTaskHandle,
    executor: Executor,
    // exit: Exit
}
impl Drone {
  // fn new(spawn_task_handle: SpawnTaskHandle, exit: Exit) -> Self {
  //   Self { spawn_task_handle, exit }
    // }
    fn new(executor: Executor) -> Self {
        Self { executor }
    }
}
// impl Larva for Drone {
//   fn spawn_task(
//     &self,
//     // task: impl Future<Item = (), Error = ()> + Send + 'static
//       //) -> Result<(), futures::future::ExecuteError<Box<dyn Future<Item = (), Error = ()> + Send>>> {
//   ) -> Result<(), futures::task::SpawnError> {
//       self.executor.spawn(task.map(|_| ()));
//       Ok(())
//     // self.spawn_task_handle.execute(Box::new(task.select(exit).then(|_| Ok(()))))
//   }
// }
impl Larva for Drone {
    fn spawn_task(
        &self,
        task: impl Future<Output = Result<(), ()>> + Send + 'static,
    ) -> Result<(), futures::task::SpawnError> {
        self.executor.spawn(task.map(|_| ()));
        Ok(())
    }
}

pub struct LnBridge<C, Block> {
  client: Arc<C>,
  ln_manager: Arc<LnManager<Drone>>,
  _block: PhantomData<Block>,
}

impl<C, Block> LnBridge<C, Block> where
  Block: traits::Block,
  C: ProvideRuntimeApi,
  C::Api: LnApi<Block>,
{
  pub fn new(
    client: Arc<C>,
    // spawn_task_handle: impl Executor<Box<dyn Future<Item = (), Error = ()> + Send>> + Sync + Send + Clone + 'static,
    // exit: Exit
  ) -> Self {
      let settings = Settings::new(&String::from("./Settings.toml")).unwrap();
      let runtime = tokio::runtime::Runtime::new().unwrap();
      let executor = runtime.executor();
      let drone = Drone::new(executor);
    // let client = service.client();
    // let drone = Drone::new(spawn_task_handle, exit);
      let runtime_api = client.runtime_api();
      let ln_manager = runtime.block_on(LnManager::new(settings, drone)).unwrap();
      // let ln_manager = Arc::new(LnManager::new(settings, drone));
      let ln_manager = Arc::new(ln_manager);
    Self {
      client,
      ln_manager,
      _block: PhantomData
    }
  }
}

// impl<C, Block> LnBridge<C, Block> where
//   Block: traits::Block,
//   C: ProvideRuntimeApi,
//   C::Api: LnApi<Block>,
// {
//   pub fn on_linked(&self) {
//     // let n = self.client.info().best_number;
//     let runtime_api = self.client.runtime_api();
//   }
// }
