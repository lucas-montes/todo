use crate::save_file::TasksFile;
use crate::tasks::handle_function;
use crate::utils::{Day, Priority};
use serde::{Deserialize, Serialize};
use structopt::clap::AppSettings::{ColoredHelp, DeriveDisplayOrder, VersionlessSubcommands};
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize, StructOpt)]
#[structopt(
    name = "ToDo CLI",
    about = "This is the CLI for ToDo tasks. You can perform CRUD operations",
    global_settings(&[ColoredHelp,DeriveDisplayOrder,VersionlessSubcommands])

    )]
pub enum Cli {
    #[structopt(
        name = "create",
        help = "Create a new task. The only required field is the title.",
        about = "Create a new task. The only required field is the title."
    )]
    Create {
        #[structopt(short = "t", long)]
        title: String,
        #[structopt(short = "c", long = "comment")]
        description: Option<String>,
        #[structopt(short = "m", long = "due_date")]
        due_date: Option<String>,
        #[structopt(short = "s", long = "start")]
        start: Option<String>,
        #[structopt(short = "e", long = "end")]
        end: Option<String>,
        #[structopt(short = "p", long = "priority")]
        priority: Option<Priority>,
        #[structopt(short = "o", long = "one_off")]
        one_off: Option<bool>,
        #[structopt(short = "f", long = "finished")]
        done: Option<bool>,
        #[structopt(short = "l", long = "list_days")]
        days: Option<Vec<Day>>,
    },

    #[structopt(
        name = "read",
        help = "Get all the tasks. You might pass an Id or a Title to filter.",
        about = "Get all the tasks. You might pass an Id or a Title to filter."
    )]
    Read {
        #[structopt(short = "i", long)]
        id: Option<i16>,
        #[structopt(short = "t", long)]
        title: Option<String>,
    },

    #[structopt(
        name = "update",
        help = "Update a given task. To do so you will need to provide an ID.",
        about = "Update a given task. To do so you will need to provide an ID."
    )]
    Update {
        #[structopt(short = "i", long)]
        id: i16,
        #[structopt(short = "t", long)]
        title: Option<String>,
        #[structopt(short = "c", long = "comment")]
        description: Option<String>,
        #[structopt(short = "m", long = "due_date")]
        due_date: Option<String>,
        #[structopt(short = "s", long = "start")]
        start: Option<String>,
        #[structopt(short = "e", long = "end")]
        end: Option<String>,
        #[structopt(short = "p", long = "priority")]
        priority: Option<Priority>,
        #[structopt(short = "o", long = "one_off")]
        one_off: Option<bool>,
        #[structopt(short = "f", long = "finished")]
        done: Option<bool>,
        #[structopt(short = "l", long = "list_days")]
        days: Option<Vec<Day>>,
    },
    #[structopt(
        name = "delete",
        help = "Delete a given task. To find it send the Id or the Title.",
        about = "Delete a given task. To find it send the Id or the Title."
    )]
    Delete {
        #[structopt(short = "i", long)]
        id: Option<i16>,
        #[structopt(short = "t", long = "title")]
        title: Option<String>,
    },
    #[structopt(
        name = "daemon",
        help = "Handle the daemon functions",
        about = "Will run the fuctions available to the daemon"
    )]
    Daemon {
        #[structopt(short = "f", long = "function", hidden = true)]
        function: String,
    },
}
pub fn apply_cli_operation(cli: Cli) -> i16 {
    let mut todos_file = TasksFile::get_or_create();
    match cli {
        Cli::Daemon { function } => handle_function(&function),
        Cli::Create {
            title,
            description,
            due_date,
            start,
            end,
            priority,
            one_off,
            done,
            days,
        } => todos_file.add(
            title,
            description,
            due_date,
            start,
            end,
            priority,
            one_off,
            done,
            days,
        ),
        Cli::Read { id, title } => todos_file.read(id, title),
        Cli::Update {
            id,
            title,
            description,
            due_date,
            start,
            end,
            priority,
            one_off,
            done,
            days,
        } => todos_file.update(
            id,
            title,
            description,
            due_date,
            start,
            end,
            priority,
            one_off,
            done,
            days,
        ),
        Cli::Delete { id, title } => todos_file.delete(id, title),
    }
}
