use clap::{ArgMatches, Command};

use crate::core::config::Config;
use crate::core::utils::display::{error, success};
use crate::db::traits::Db;
use crate::integrations::{redmine::Redmine, traits::Integration};
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct Send {}

impl Action for Send {
    fn perform<'a, 'b>(config: &'a Config, db: &'b dyn Db, _sub_m: &ArgMatches) {
        let mut total_tasks_sent = 0;
        let tasks = db.unreported_tasks();
        for task in tasks {
            match Redmine::report_task(&config, &task) {
                Ok(_) => {
                    total_tasks_sent += 1;
                    success(format!("Task with ID {} sent successfully", task.id));
                    db.report_task(&task.id).unwrap();
                },
                Err(_) => error(format!("Error sending task with ID {}", task.id)),
            }
        }

        println!("\n{total_tasks_sent} tasks sent");

        Show::new(db).today();
    }

    fn subcomand() -> Command {
        Command::new("send").about("Send unreported and finished tasks to the active integration")
    }
}
