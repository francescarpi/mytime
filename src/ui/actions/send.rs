use clap::{ArgMatches, Command};

use crate::core::config::Config;
use crate::core::utils::display::{error, success};
use crate::core::utils::grouper::group_tasks_for_the_integration;
use crate::db::traits::Db;
use crate::integrations::{get_integration, traits::Integration};
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct Send {}

impl Action for Send {
    const NAME: &'static str = "send";

    fn perform<'a, 'b>(config: &'a Config, db: &'b dyn Db, _sub_m: &ArgMatches) {
        let redmine = get_integration(&config);
        let mut total_tasks_sent = 0;
        let tasks = db.unreported_tasks();
        let tasks = group_tasks_for_the_integration(&tasks);

        for task in tasks {
            match redmine.report_task(&config, &task) {
                Ok(_) => {
                    total_tasks_sent += 1;

                    for id in task.ids_used {
                        success(format!(
                            "Task {}, external ID {}, sent successfully",
                            id, task.external_id
                        ));

                        db.report_task(&id).unwrap();
                    }
                }
                Err(e) => error(format!("Task {}. {}.", task.external_id, e)),
            }
        }

        println!("\n{total_tasks_sent} tasks sent");

        Show::new(db).today();
    }

    fn subcomand() -> Command {
        Command::new(Self::NAME).about("Send a unreported tasks to the configured integration")
    }
}
