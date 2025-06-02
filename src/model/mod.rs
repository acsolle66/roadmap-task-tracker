use json::JsonValue;
use json::object;

#[derive(PartialEq, Debug)]
pub enum TaskState {
    NotStarted,
    InProgress,
    Done,
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

impl TryFrom<String> for TaskState {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == String::from("not-started") {
            Ok(TaskState::NotStarted)
        } else if value == String::from("in-progress") {
            Ok(TaskState::InProgress)
        } else if value == String::from("done") {
            Ok(TaskState::Done)
        } else {
            Err("Unknown task state parameter.".to_owned())
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
        self.state = TaskState::try_from(state).unwrap();
    }

    pub fn set_task(&mut self, task: String) {
        self.task = task;
    }
}

impl From<&Task> for JsonValue {
    fn from(task_model: &Task) -> JsonValue {
        object! {
            "id"  => task_model.get_id(),
            "task" => task_model.get_task().to_owned(),
            "state" =>String::from(task_model.get_state())
        }
    }
}

impl TryFrom<&JsonValue> for Task {
    type Error = String;
    fn try_from(json_object: &JsonValue) -> Result<Self, Self::Error> {
        let id: u8 = json_object["id"].as_u8().ok_or("Can not parse task id")?;
        let task: String = json_object["task"].to_string();
        let state: TaskState = TaskState::try_from(json_object["state"].to_string())?;
        Ok(Task::new(id, task, state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TaskState tests //
    // Parse TaskState from String not-started
    #[test]
    fn test_parse_task_state_not_started_from_string() {
        assert_eq!(
            TaskState::NotStarted,
            TaskState::try_from("not-started".to_string()).unwrap()
        );
    }

    // Parse TaskState from String in-pogress
    #[test]
    fn test_parse_task_state_in_progress_from_string() {
        assert_eq!(
            TaskState::InProgress,
            TaskState::try_from("in-progress".to_string()).unwrap()
        );
    }

    // Parse TaskState from String done
    #[test]
    fn test_parse_task_state_done_from_string() {
        assert_eq!(
            TaskState::Done,
            TaskState::try_from("done".to_string()).unwrap()
        );
    }

    // Parse TaskState from invalid String
    #[test]
    #[should_panic]
    fn test_parse_task_state_from_invalid_string() {
        TaskState::try_from("invalid".to_string()).unwrap();
    }

    // Parse String from TaskState not-started
    #[test]
    fn test_parse_string_not_started_from_task_state() {
        assert_eq!(
            "not-started".to_string(),
            String::from(&TaskState::NotStarted)
        );
    }

    // Parse String from TaskState in-progress
    #[test]
    fn test_parse_string_in_progress_from_task_state() {
        assert_eq!(
            "in-progress".to_string(),
            String::from(&TaskState::InProgress)
        );
    }

    // Parse String from TaskState done
    #[test]
    fn test_parse_string_done_from_task_state() {
        assert_eq!("done".to_string(), String::from(&TaskState::Done));
    }
}
