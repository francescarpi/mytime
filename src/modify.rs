use crate::config::Config;

pub struct Modify {}

impl Modify {
    pub fn task(config: &Config, id: i64, desc: String) {
        match config.conn.execute("UPDATE tasks SET desc = ?1 WHERE id = ?2", [desc, id.to_string()]) {
            Ok(_) => {
                println!("\n✅ Task updated!\n");
            },
            Err(_) => {
                println!("\n‼️ There is not any task with this ID!\n");
            }
        }
    }

}
