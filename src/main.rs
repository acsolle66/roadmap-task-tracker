mod cli;
mod task;

fn main() {
    let command = cli::Command::parse();
    match command {
        cli::Command::List => println!("List command called"),
        cli::Command::Add(task) => println!("Add command called: {task}"),
        cli::Command::Show(id) => println!("Show command called: {id}"),
        cli::Command::Update((id, update)) => println!("Update command called {id},{update}"),
        cli::Command::Delete(id) => println!("Delete command called {id}"),
        cli::Command::Mark((id, state)) => match state {
            task::State::Done => println!("Mark command called {id}, done"),
            task::State::NotStarted => println!("Mark command called {id}, not-started"),
            task::State::InProgress => println!("Mark command called {id}, in-progress"),
        },
        cli::Command::Unknown => println!("Unknown command called"),
    }
}
