use json;
use std::vec::Vec;

#[derive(PartialEq, Debug)]
pub enum TaskState {
    Done,
    NotStarted,
    InProgress,
}
impl TaskState {
    pub fn parse(state: String) -> TaskState {
        if state == String::from("not-started") {
            TaskState::NotStarted
        } else if state == String::from("in-progress") {
            TaskState::InProgress
        } else if state == String::from("done") {
            TaskState::Done
        } else {
            panic!("The state parameter must be one of ['not-started', 'in-progress', 'done'].")
        }
    }
}

impl From<&TaskState> for String {
    fn from(value: &TaskState) -> Self {
        match value {
            TaskState::NotStarted => String::from("not-started"),
            TaskState::InProgress => String::from("in-progress"),
            TaskState::Done => String::from("done"),
        }
    }
}

#[derive(Debug)]
pub struct Task {
    id: u8,
    task: String,
    state: TaskState,
}
impl Task {
    pub fn new(id: u8, task: String, state: TaskState) -> Self {
        Task { id, task, state }
    }
    pub fn get_id(&self) -> u8 {
        self.id
    }
    pub fn get_task(&self) -> &String {
        &self.task
    }

    pub fn get_state(&self) -> &TaskState {
        &self.state
    }

    pub fn set_state(&mut self, state: String) {
        self.state = TaskState::parse(state);
    }

    pub fn set_task(&mut self, task: String) {
        self.task = task;
    }
}

impl From<&Task> for json::JsonValue {
    fn from(task_model: &Task) -> json::JsonValue {
        json::object! {
            "id"  => task_model.get_id(),
            "task" => task_model.get_task().to_owned(),
            "state" =>String::from(task_model.get_state())
        }
    }
}
