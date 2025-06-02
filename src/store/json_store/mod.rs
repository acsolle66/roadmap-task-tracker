use super::TaskStore;

use crate::model;

use json;
use std::env;
use std::fs;
use std::path;

#[derive(Debug)]
pub struct JsonStore {
    store: Vec<model::Task>,
    last_id: u8,
}

impl TaskStore for JsonStore {
    fn get_tasks(&self) -> &Vec<model::Task> {
        return &self.store;
    }

    fn get_task(&self, id: u8) -> Option<&model::Task> {
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
        let task: model::Task = model::Task::new(id, task, model::TaskState::NotStarted);
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
        let file_path: path::PathBuf = JsonStore::get_file_path();
        if JsonStore::file_exists(&file_path) {
            let source: String = fs::read_to_string(file_path).unwrap();
            let json_object: json::JsonValue = json::parse(&source).unwrap();
            Self::convert_from_json_object(json_object)
        } else {
            let store: Vec<model::Task> = vec![];
            let last_id: u8 = 0;
            return JsonStore { store, last_id };
        }
    }

    fn get_mut_task(&mut self, id: u8) -> Option<&mut model::Task> {
        for task in self.store.iter_mut() {
            if id == task.get_id() {
                return Some(task);
            }
        }
        return None;
    }
    fn convert_from_json_object(json_object: json::JsonValue) -> JsonStore {
        let mut store: Vec<model::Task> = vec![];
        for member in json_object.members() {
            let id: u8 = member["id"].as_u8().unwrap();
            let task: String = member["task"].as_str().unwrap().to_owned();
            let state: model::TaskState =
                model::TaskState::parse(member["state"].as_str().unwrap().to_owned());
            let task: model::Task = model::Task::new(id, task, state);
            store.push(task);
        }
        let length: usize = store.len();
        let last_id: u8 = store[length - 1].get_id();
        return JsonStore { store, last_id };
    }

    pub fn save(&self) {
        let file_path: path::PathBuf = JsonStore::get_file_path();
        let contents: String = self.convert_to_json_object().to_string();
        fs::write(file_path, contents).unwrap()
    }

    fn convert_to_json_object(&self) -> json::JsonValue {
        let mut json_store: json::JsonValue = json::array![];
        for member in &self.store {
            let id: u8 = member.get_id();
            let task: String = member.get_task().to_owned();
            let state: String = match member.get_state() {
                &model::TaskState::Done => "done".to_owned(),
                &model::TaskState::InProgress => "in-progress".to_owned(),
                &model::TaskState::NotStarted => "not-started".to_owned(),
            };
            let task: json::JsonValue = json::object! {id:id, task:task, state:state};
            json_store.push(task).unwrap();
        }
        return json_store;
    }

    fn get_file_path() -> path::PathBuf {
        let home: path::PathBuf = match env::home_dir() {
            Some(home_path) => home_path,
            None => panic!("Can not read the home path"),
        };
        let path: &path::Path = path::Path::new("tasks.json");
        home.join(path)
    }

    fn file_exists(file_path: &path::PathBuf) -> bool {
        file_path.exists()
    }
}
