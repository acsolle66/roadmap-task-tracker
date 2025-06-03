use crate::model;
use crate::store;

pub struct TaskService<'a> {
    store: &'a mut dyn store::TaskStore,
}

impl<'a> TaskService<'a> {
    pub fn new(store: &'a mut dyn store::TaskStore) -> Self {
        TaskService { store }
    }

    pub fn list(self, state_filter: String) {
        let tasks: Vec<model::Task> = self.store.get_tasks(state_filter);
        for task in tasks {
            let task_id: u8 = task.get_id();
            println!("### {task_id} ###");
            let task_state: &model::TaskState = task.get_state();
            println!("State: {task_state:?}");
            let task_content: &String = task.get_task();
            println!("{task_content}");
            println!()
        }
    }

    pub fn add(self, task: String) {
        let id: u8 = self.store.add_task(task);
        println!("Task added with id #{id}.");
    }

    pub fn show(self, id: u8) {
        let task: Option<&model::Task> = self.store.get_task(id);
        match task {
            Some(task) => {
                let task_id: u8 = task.get_id();
                println!("### {task_id} ###");
                let task_state: &model::TaskState = task.get_state();
                println!("State: {task_state:?}");
                let task_content: &String = task.get_task();
                println!("{task_content}");
                println!()
            }
            None => println!("No task found with id {id}"),
        }
    }

    pub fn update(self, id: u8, updated_task: String) {
        match self.store.update_task(id, updated_task.clone()) {
            true => println!("Successfully updated task #{id}"),
            false => println!("Can not update task #{id}"),
        };
    }

    pub fn delete(self, id: u8) {
        match self.store.remove_task(id) {
            true => println!("Successfully deleted task #{id}"),
            false => println!("Can not delete task #{id}"),
        };
    }

    pub fn mark(self, id: u8, state: String) {
        match self.store.set_state(id, state) {
            true => println!("Successfully marked task #{id}"),
            false => println!("Can not mark task #{id}"),
        };
    }
}
