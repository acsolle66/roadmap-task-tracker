use std::env;

#[derive(PartialEq, Debug)]
pub enum Command {
    List,
    Add(String),
    Show(u8),
    Update((u8, String)),
    Delete(u8),
    Mark((u8, String)),
    Unknown,
}

impl Command {
    pub fn parse() -> Self {
        let arguments: Vec<String> = env::args().collect();
        Self::parse_from(&arguments)
    }
    fn parse_from(arguments: &Vec<String>) -> Self {
        // The first argument in arguments is the path of the executable (index: 0).

        // The second argument in arguments is the 'command' (index: 1).
        const COMMAND_INDEX: usize = 1;

        // Where 'task id' is used, it must be the third argument in arguments (index: 2).
        const TASK_ID_ARGUMENT_INDEX: usize = 2;

        // The user have to provide at least one command
        let argument_count: usize = 2;
        validate_argument_count(argument_count, &arguments);

        let command: &str = &arguments[COMMAND_INDEX];
        match command {
            "list" => {
                // No additional arguments used
                Command::List
            }
            "add" => {
                // User have to provide one command argument <task>
                let argument_count: usize = 3;
                validate_argument_count(argument_count, &arguments);

                const ADD_TASK_ARGUMENT_INDEX: usize = 2;
                let add_task: String = arguments[ADD_TASK_ARGUMENT_INDEX].clone();

                Command::Add(add_task)
            }
            "show" => {
                // User have to provide one command argument <task_id>
                let argument_count: usize = 3;
                validate_argument_count(argument_count, &arguments);

                let id: u8 = parse_id(&arguments[TASK_ID_ARGUMENT_INDEX]);

                Command::Show(id)
            }
            "update" => {
                // User have to provide two command arguments <task_id> <updated_task>
                let argument_count: usize = 4;
                validate_argument_count(argument_count, &arguments);

                let id: u8 = parse_id(&arguments[TASK_ID_ARGUMENT_INDEX]);

                const TASK_UPDATE_ARGUMENT_INDEX: usize = 3;
                let task_update: String = arguments[TASK_UPDATE_ARGUMENT_INDEX].clone();

                Command::Update((id, task_update))
            }
            "delete" => {
                // User have to provide one command argument <task_id>
                let argument_count: usize = 3;
                validate_argument_count(argument_count, &arguments);

                let id: u8 = parse_id(&arguments[TASK_ID_ARGUMENT_INDEX]);

                Command::Delete(id)
            }
            "mark" => {
                // User have to provide two command arguments <task_id> <state>
                let argument_count: usize = 4;
                validate_argument_count(argument_count, &arguments);

                let id: u8 = parse_id(&arguments[TASK_ID_ARGUMENT_INDEX]);

                const TASK_ARGUMENT_INDEX: usize = 3;
                let state: String = arguments[TASK_ARGUMENT_INDEX].to_owned();

                Command::Mark((id, state))
            }
            _ => Command::Unknown,
        }
    }
}

fn validate_argument_count(argument_count: usize, arguments: &Vec<String>) {
    const HELP_TEXT: &str = r"A simple CLI app for tracking tasks.
    Usage: task-tracker <command> [command-arguments]
    Commands:
    - list                                                      List all the tasks (e.g.: task-tracker add 'Buy 3 eggs.').
    - add <task>                                                Add one task (e.g.: task-tracker add 'Buy 3 eggs.').
    - show <task_id>                                            Show task (e.g.: task-tracker show 1).
    - update <task_id> <updated_task>                           Update task (e.g.: task-tracker update 1 'Buy 3 eggs and 1 milk.').
    - delete <task_id>                                          Delete task (e.g.: task-tracker delete 1).
    - mark <task_id> ['not-started', 'in-progress', 'done']     Mark task as 'not-started', 'in-progress' or 'done' (e.g.: task-tracker mark 1 done).
    ";

    if arguments.len() < argument_count {
        panic!("{HELP_TEXT}")
    }
}

fn parse_id(argument: &str) -> u8 {
    argument
        .parse()
        .expect("The 'id' must be a numerical value between 1-256.")
}

#[cfg(test)]
mod tests {
    use super::*;

    // validate_argument_count tests //
    #[test]
    fn test_validate_argument_count_valid() {
        let argument_count: usize = 2;

        let arguments: Vec<String> = vec!["one".to_string(), "two".to_string()];

        let result: () = validate_argument_count(argument_count, &arguments);
        assert_eq!(result, ());
    }

    #[test]
    #[should_panic]
    fn test_validate_argument_count_invalid() {
        let argument_count: usize = 3;

        let arguments: Vec<String> = vec!["one".to_string(), "two".to_string()];

        validate_argument_count(argument_count, &arguments);
    }

    // parse_id tests //
    #[test]
    fn test_parse_id_valid() {
        assert_eq!(parse_id("42"), 42);
    }
    #[test]
    #[should_panic]
    fn test_parse_id_invalid() {
        parse_id("invalid");
    }

    // Command::parse_from tests
    #[test]
    #[should_panic]
    fn test_command_parse_from_with_missing_command_should_panic() {
        let path_argument: String = "some/path".to_string();

        let arguments: Vec<String> = vec![path_argument];

        Command::parse_from(&arguments);
    }

    // Unknown
    #[test]
    fn test_command_parse_from_with_unknown_command_returns_unknown_variant() {
        let path_argument: String = "some/path".to_string();
        let command_argument: String = "invalid".to_string();

        let arguments: Vec<String> = vec![path_argument, command_argument];

        let command: Command = Command::parse_from(&arguments);
        assert_eq!(command, Command::Unknown)
    }

    // List
    #[test]
    fn test_command_parse_from_with_list_command_returns_list_variant() {
        let path_argument: String = "some/path".to_string();
        let command_argument: String = "list".to_string();

        let arguments: Vec<String> = vec![path_argument, command_argument];

        let command: Command = Command::parse_from(&arguments);
        assert_eq!(command, Command::List)
    }

    // Add
    #[test]
    fn test_command_parse_from_with_add_command_returns_add_variant() {
        let path_argument: String = "some/path".to_string();
        let command_argument: String = "add".to_string();
        let task_argument: String = "task".to_string();

        let arguments: Vec<String> = vec![path_argument, command_argument, task_argument.clone()];

        let command: Command = Command::parse_from(&arguments);
        assert_eq!(command, Command::Add(task_argument))
    }
    #[test]
    #[should_panic]
    fn test_command_parse_from_with_add_command_without_and_invalid_argument_count_should_panic() {
        let path_argument: String = "some/path".to_string();
        let command_argument: String = "add".to_string();

        let arguments: Vec<String> = vec![path_argument, command_argument];

        Command::parse_from(&arguments);
    }

    // Show
    #[test]
    fn test_command_parse_from_with_show_command_returns_show_variant() {
        let path_argument: String = "some/path".to_string();
        let command_argument: String = "show".to_string();
        let task_id_argument: String = "1".to_string();

        let arguments: Vec<String> =
            vec![path_argument, command_argument, task_id_argument.clone()];

        let command: Command = Command::parse_from(&arguments);
        assert_eq!(command, Command::Show(parse_id(&task_id_argument)))
    }
    #[test]
    #[should_panic]
    fn test_command_parse_from_with_show_command_and_invalid_argument_count_should_panic() {
        let path_argument: String = "some/path".to_string();
        let command_argument: String = "show".to_string();

        let arguments: Vec<String> = vec![path_argument, command_argument];

        Command::parse_from(&arguments);
    }

    // Update
    #[test]
    fn test_command_parse_from_with_update_command_returns_update_variant() {
        let path_argument: String = "some/path".to_string();
        let command_argument: String = "update".to_string();
        let task_id_argument: String = "1".to_string();
        let updated_task_argument: String = "updated task".to_string();

        let arguments: Vec<String> = vec![
            path_argument,
            command_argument,
            task_id_argument.clone(),
            updated_task_argument.clone(),
        ];

        let command: Command = Command::parse_from(&arguments);
        assert_eq!(
            command,
            Command::Update((parse_id(&task_id_argument), updated_task_argument))
        )
    }
    #[test]
    #[should_panic]
    fn test_command_parse_from_with_update_command_and_invalid_argument_count_should_panic() {
        let path_argument: String = "some/path".to_string();
        let command_argument: String = "update".to_string();
        let updated_task_argument: String = "updated task".to_string();

        let arguments: Vec<String> = vec![path_argument, command_argument, updated_task_argument];

        Command::parse_from(&arguments);
    }

    // Delete
    #[test]
    fn test_command_parse_from_with_delete_command_returns_delete_variant() {
        let path_argument: String = "some/path".to_string();
        let command_argument: String = "delete".to_string();
        let task_id_argument: String = "1".to_string();

        let arguments: Vec<String> =
            vec![path_argument, command_argument, task_id_argument.clone()];

        let command: Command = Command::parse_from(&arguments);
        assert_eq!(command, Command::Delete(parse_id(&task_id_argument)))
    }
    #[test]
    #[should_panic]
    fn test_command_parse_from_with_delete_command_and_invalid_argument_count_should_panic() {
        let path_argument: String = "some/path".to_string();
        let command_argument: String = "delete".to_string();

        let arguments: Vec<String> = vec![path_argument, command_argument];

        Command::parse_from(&arguments);
    }

    // Mark
    #[test]
    fn test_command_parse_from_with_mark_command_returns_mark_variant() {
        let path_argument: String = "some/path".to_string();
        let command_argument: String = "mark".to_string();
        let task_id_argument: String = "1".to_string();
        let state_argument: String = "done".to_string();

        let arguments: Vec<String> = vec![
            path_argument,
            command_argument,
            task_id_argument.clone(),
            state_argument.clone(),
        ];

        let command: Command = Command::parse_from(&arguments);
        assert_eq!(
            command,
            Command::Mark((parse_id(&task_id_argument), state_argument))
        )
    }
    #[test]
    #[should_panic]
    fn test_command_parse_from_with_mark_command_and_invalid_argument_count_should_panic() {
        let path_argument: String = "some/path".to_string();
        let command_argument: String = "mark".to_string();
        let state_argument: String = "done".to_string();

        let arguments: Vec<String> = vec![path_argument, command_argument, state_argument];

        Command::parse_from(&arguments);
    }
}
