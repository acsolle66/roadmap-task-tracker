use super::TaskStore;

use crate::model::{Task, TaskState};

use json::JsonValue;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct JsonStore {
    store: Vec<Task>,
    last_id: u8,
}

impl TaskStore for JsonStore {
    fn get_tasks(&self) -> &Vec<Task> {
        return &self.store;
    }

    fn get_task(&self, id: u8) -> Option<&Task> {
        for task in &self.store {
            let task_id: u8 = task.get_id();
            if task_id == id {
                return Some(&task);
            }
        }
        return None;
    }

    fn add_task(&mut self, task: String) -> u8 {
        let id: u8 = self.last_id + 1;
        let task: Task = Task::new(id, task, TaskState::NotStarted);
        self.store.push(task);
        self.last_id += 1;
        self.save();
        return id;
    }

    fn set_state(&mut self, id: u8, state: String) -> bool {
        let task = self.get_mut_task(id);
        match task {
            Some(task) => {
                task.set_state(state.clone());
                self.save();
                true
            }
            None => false,
        }
    }
    fn update_task(&mut self, id: u8, updated_task: String) -> bool {
        let task = self.get_mut_task(id);
        match task {
            Some(task) => {
                task.set_task(updated_task.clone());
                self.save();
                true
            }
            None => false,
        }
    }
    fn remove_task(&mut self, id: u8) -> bool {
        for (index, task) in self.store.iter_mut().enumerate() {
            if id == task.get_id() {
                self.store.remove(index);
                self.save();
                return true;
            }
        }
        return false;
    }
}

impl JsonStore {
    pub fn load() -> Self {
        let file_path: PathBuf = JsonStore::get_file_path();
        if JsonStore::file_exists(&file_path) {
            let source: String = fs::read_to_string(file_path).unwrap();
            let json_object: JsonValue = json::parse(&source).unwrap();
            Self::from_json_array(json_object)
        } else {
            let store: Vec<Task> = vec![];
            let last_id: u8 = 0;
            return JsonStore { store, last_id };
        }
    }

    pub fn save(&self) {
        let file_path: PathBuf = JsonStore::get_file_path();
        let contents: String = self.to_json_array().to_string();
        fs::write(file_path, contents).unwrap()
    }

    fn get_mut_task(&mut self, id: u8) -> Option<&mut Task> {
        for task in self.store.iter_mut() {
            if id == task.get_id() {
                return Some(task);
            }
        }
        return None;
    }

    fn from_json_array(json_array: JsonValue) -> JsonStore {
        let mut store: Vec<Task> = vec![];
        for json_value in json_array.members() {
            let task: Task = Task::try_from(json_value).unwrap();
            store.push(task);
        }
        let length: usize = store.len();
        let last_id: u8 = store[length - 1].get_id();
        return JsonStore { store, last_id };
    }

    fn to_json_array(&self) -> JsonValue {
        let mut json_array: JsonValue = json::array![];
        for task_object in &self.store {
            let task: JsonValue = JsonValue::from(task_object);
            json_array.push(task).unwrap();
        }
        return json_array;
    }

    fn get_file_path() -> PathBuf {
        let home: PathBuf = match env::home_dir() {
            Some(home_path) => home_path,
            None => panic!("Can not read the home path"),
        };
        let path: &Path = Path::new("tasks.json");
        home.join(path)
    }

    fn file_exists(file_path: &PathBuf) -> bool {
        file_path.exists()
    }
}
