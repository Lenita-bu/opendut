use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use strum::{VariantNames, EnumVariantNames, EnumString, Display};
use crate::core::metadata::cargo_netbird_versions;
use crate::core::util::consume_output;




#[derive(Debug, PartialEq, EnumString, EnumVariantNames, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum EnvVars {
    Puser,
    Pgroup,
    Puid,
    Pgid,
    DockerUser,
    DockerGid,
    OpendutRepoRoot,
    NetbirdSignalVersion,
    NetbirdManagementVersion,
    NetbirdDashboardVersion,
}

pub trait ProjectRootDir {
    fn project_dir() -> String;
    fn project_path_buf() -> PathBuf;
}
impl ProjectRootDir for PathBuf {
    fn project_dir() -> String {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            .output();
        consume_output(output).expect("Failed to determine git project root directory")
    }

    fn project_path_buf() -> PathBuf {
        PathBuf::from(PathBuf::project_dir())
    }
}


fn get_docker_group_id() -> String {
    let docker_getent_group = consume_output(Command::new("getent").arg("group").arg("docker").output()).expect("Failed to get docker group.");
    let docker_group_id = docker_getent_group.split(":").nth(2).expect("Failed to get docker group id").to_string();
    docker_group_id
}

pub(crate) fn check_dot_env_variables() {
    let user_name = consume_output(Command::new("id").arg("--user").arg("--name").output()).expect("Failed to get user name");
    let user_id = consume_output(Command::new("id").arg("--user").output()).expect("Failed to get user id");
    let group_name = match consume_output(Command::new("id").arg("--group").arg("--name").output()) {
        Ok(group_name) => { group_name }
        Err(error) => {
            println!("Failed to get group name: {:?}. Using 'general' as fallback name for your group.", error);
            "general".to_string()
        }
    };
    let group_id = consume_output(Command::new("id").arg("--group").output()).expect("Failed to get group id");
    let docker_gid = get_docker_group_id();
    let git_repo_root = PathBuf::project_dir();

    let mut missing_env_vars = "".to_owned();
    let mut error_messages = "".to_owned();

    let netbird = cargo_netbird_versions();
    let env_map = HashMap::from([
        (EnvVars::Puser.to_string(), user_name.clone()),
        (EnvVars::Puid.to_string(), user_id.clone()),
        (EnvVars::Pgroup.to_string(), group_name.clone()),
        (EnvVars::Pgid.to_string(), group_id.clone()),
        (EnvVars::DockerUser.to_string(), format!("{}:{}", user_id, group_id)),
        (EnvVars::DockerGid.to_string(), docker_gid.clone()),
        (EnvVars::OpendutRepoRoot.to_string(), git_repo_root.clone()),
        (EnvVars::NetbirdManagementVersion.to_string(), netbird.netbird_management_version),
        (EnvVars::NetbirdSignalVersion.to_string(), netbird.netbird_signal_version),
        (EnvVars::NetbirdDashboardVersion.to_string(), netbird.netbird_dashboard_version),
    ]);

    for (env_key, env_value) in &env_map {
        match env::var(env_key) {
            Ok(value) => {
                // check if all environment variables are set correctly
                if value != *env_value {
                    let wrong_key_value = format!("Env variable is set as '{}={}'. Expected: '{}={}'\n", env_key, value, env_key, env_value);
                    println!("{}", wrong_key_value);
                    error_messages.push_str(&wrong_key_value);
                }
            }
            Err(_) => {
                // check if all required environment variables are set
                let missing_key_value = format!("{}={}\n", env_key, env_value);
                missing_env_vars.push_str(&missing_key_value);
            }
        };
    }

    if !missing_env_vars.is_empty() {
        println!("Missing environment variables in file '.env': \n{}", missing_env_vars);
    }
    if missing_env_vars.len() > 0 || error_messages.len() > 0 {
        panic!("There are errors in the environment variables in file '.env': \n{}", error_messages);
    }

    assert_eq!(["PUSER", "PGROUP", "PUID", "PGID", "DOCKER_USER", "DOCKER_GID", "OPENDUT_REPO_ROOT", "NETBIRD_SIGNAL_VERSION", "NETBIRD_MANAGEMENT_VERSION", "NETBIRD_DASHBOARD_VERSION"], EnvVars::VARIANTS);
}


pub(crate) fn boolean_env_var(name: &str) -> bool {
    env::var(name).unwrap_or("false".to_string()) == "true".to_string()
}