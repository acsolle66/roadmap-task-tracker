mod cli;
mod model;
mod service;
mod store;

use crate::store::json_store::JsonStore;

fn main() {
    let mut store: JsonStore = JsonStore::load();
    let service = service::TaskService::new(&mut store);
    match cli::Command::parse() {
        cli::Command::List => service.list(),
        cli::Command::Add(task) => service.add(task),
        cli::Command::Show(task_id) => service.show(task_id),
        cli::Command::Update((task_id, updated_task)) => service.update(task_id, updated_task),
        cli::Command::Delete(task_id) => service.delete(task_id),
        cli::Command::Mark((task_id, state)) => service.mark(task_id, state),
        cli::Command::Unknown => println!("Unknown command called"),
    }
}
