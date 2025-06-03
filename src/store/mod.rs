pub mod json_store;

use crate::model;

pub trait TaskStore {
    fn get_tasks(&self, state_filter: String) -> Vec<model::Task>;
    fn get_task(&self, id: u8) -> Option<&model::Task>;
    fn add_task(&mut self, task: String) -> u8;
    fn set_state(&mut self, id: u8, state: String) -> bool;
    fn update_task(&mut self, id: u8, updated_task: String) -> bool;
    fn remove_task(&mut self, id: u8) -> bool;
}
