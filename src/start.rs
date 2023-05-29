use crate::config::Config;

pub struct Start {}

impl Start {
    pub fn task(config: Config, desc: String) {
        config
            .conn
            .execute(
                "INSERT INTO tasks (desc, start_at) VALUES (?1, ?2)",
                [desc.clone(), config.now.to_rfc3339()],
            )
            .unwrap();

        println!("✅ Task \"{}\" added!", desc);
    }
}
