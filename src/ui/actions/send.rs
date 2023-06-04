use clap::{ArgMatches, Command};

use crate::core::config::Config;
use crate::core::utils::display::{error, success};
use crate::db::traits::Db;
use crate::integrations::{get_integration, traits::Integration};
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct Send {}

impl Action for Send {
    fn perform<'a, 'b>(config: &'a Config, db: &'b dyn Db, _sub_m: &ArgMatches) {
        let redmine = get_integration(&config);
        let mut total_tasks_sent = 0;
        let tasks = db.unreported_tasks();
        for task in tasks {
            match redmine.report_task(&config, &task) {
                Ok(_) => {
                    total_tasks_sent += 1;
                    success(format!(
                        "Task {}, external ID {}, sent successfully",
                        task.id,
                        task.external_id.unwrap()
                    ));
                    db.report_task(&task.id).unwrap();
                }
                Err(e) => error(format!("Task {}. {}.", task.id, e)),
            }
        }

        println!("\n{total_tasks_sent} tasks sent");

        Show::new(db).today();
    }

    fn subcomand() -> Command {
        Command::new("send").about("Send unreported and finished tasks to the active integration")
    }
}
