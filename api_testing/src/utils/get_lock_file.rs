use std::env;
use tokio::fs;

#[derive(Clone, Debug)]
pub struct LockFile {
    pub name: String,
    pub pid: String,
    pub port: String,
    pub password: String,
    pub protocol: String,
}

pub async fn get_lock_file() -> LockFile {
    let mut file: Option<LockFile> = None;

    while file.is_none() {
        let f = fs::read_to_string(format!(
            "{}\\Riot Games\\Riot Client\\Config\\lockfile",
            env::var("LocalAppData").unwrap()
        ))
        .await;
        if let Ok(f) = f {
            let lock_vec = f.split(":").collect::<Vec<&str>>();

            file = Some(LockFile {
                name: lock_vec[0].to_string(),
                pid: lock_vec[1].to_string(),
                port: lock_vec[2].to_string(),
                password: lock_vec[3].to_string(),
                protocol: lock_vec[4].to_string(),
            })
        } else {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    file.unwrap()
}
