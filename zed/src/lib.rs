use std::{collections::HashMap, env};
use zed_extension_api::{
    self as zed, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "@zeroheight/mcp-server";
const SERVER_PATH: &str = "node_modules/@zeroheight/mcp-server/dist/stdio.js";
const ZEROHEIGHT_ACCESS_TOKEN: &str = "ZEROHEIGHT_ACCESS_TOKEN";
const ZEROHEIGHT_CLIENT_ID: &str = "ZEROHEIGHT_CLIENT_ID";
const ZEROHEIGHT_MCP_CLIENT: &str = "ZEROHEIGHT_MCP_CLIENT";
const MCP_CLIENT: &str = "zed-mcp-plugin";
const INSTALLATION_INSTRUCTIONS: &str = "The zeroheight MCP server is installed automatically from npm. Configure them in Zed's context server settings when you can't use OAuth";
const SETTINGS_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "ZEROHEIGHT_ACCESS_TOKEN": {
      "type": "string",
      "description": "Optional zeroheight access token."
    },
    "ZEROHEIGHT_CLIENT_ID": {
      "type": "string",
      "description": "Optional zeroheight OAuth client ID."
    }
  },
  "additionalProperties": false
}"#;
const DEFAULT_SETTINGS: &str = r#"{
  "ZEROHEIGHT_ACCESS_TOKEN": "",
  "ZEROHEIGHT_CLIENT_ID": ""
}"#;

struct ZeroheightModelContextExtension;

impl zed::Extension for ZeroheightModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        install_server_package()?;

        let settings =
            zed::settings::ContextServerSettings::for_project(context_server_id.as_ref(), project)?;

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![server_script_path()?],
            env: env_vars_from_settings(settings)?,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        Ok(Some(ContextServerConfiguration {
            installation_instructions: INSTALLATION_INSTRUCTIONS.to_string(),
            settings_schema: SETTINGS_SCHEMA.to_string(),
            default_settings: DEFAULT_SETTINGS.to_string(),
        }))
    }
}

fn install_server_package() -> Result<()> {
    let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
    let installed_version = zed::npm_package_installed_version(PACKAGE_NAME)?;

    if installed_version.as_deref() != Some(latest_version.as_str()) {
        zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
    }

    Ok(())
}

fn server_script_path() -> Result<String> {
    Ok(env::current_dir()
        .map_err(|err| format!("failed to get extension directory: {err}"))?
        .join(SERVER_PATH)
        .to_string_lossy()
        .to_string())
}

fn env_vars_from_settings(settings: zed::settings::ContextServerSettings) -> Result<zed::EnvVars> {
    let mut env = settings
        .command
        .and_then(|command| command.env)
        .unwrap_or_default();

    remove_blank_zeroheight_env_vars(&mut env);

    if let Some(settings) = settings.settings.as_ref() {
        let settings = settings
            .as_object()
            .ok_or("zeroheight context server settings must be a JSON object")?;
        merge_setting_env_var(&mut env, settings, ZEROHEIGHT_ACCESS_TOKEN)?;
        merge_setting_env_var(&mut env, settings, ZEROHEIGHT_CLIENT_ID)?;
    }

    env.insert(ZEROHEIGHT_MCP_CLIENT.to_string(), MCP_CLIENT.to_string());

    Ok(env.into_iter().collect())
}

fn remove_blank_zeroheight_env_vars(env: &mut HashMap<String, String>) {
    env.retain(|name, value| {
        !matches!(
            name.as_str(),
            ZEROHEIGHT_ACCESS_TOKEN | ZEROHEIGHT_CLIENT_ID
        ) || !value.trim().is_empty()
    });
}

fn merge_setting_env_var(
    env: &mut HashMap<String, String>,
    settings: &zed::serde_json::Map<String, zed::serde_json::Value>,
    key: &str,
) -> Result<()> {
    match settings.get(key) {
        None | Some(zed::serde_json::Value::Null) => {}
        Some(zed::serde_json::Value::String(value)) => {
            if !value.trim().is_empty() {
                env.insert(key.to_string(), value.clone());
            }
        }
        Some(_) => return Err(format!("zeroheight setting `{key}` must be a string")),
    }

    Ok(())
}

zed::register_extension!(ZeroheightModelContextExtension);

#[cfg(test)]
mod tests {
    use super::*;
    use zed::serde_json::{from_value, json, Value};

    fn env_for(settings: Value) -> Result<HashMap<String, String>> {
        let settings = from_value(settings).expect("valid context server settings");
        Ok(env_vars_from_settings(settings)?.into_iter().collect())
    }

    #[test]
    fn settings_override_command_env() {
        let env = env_for(json!({
            "command": { "env": { "ZEROHEIGHT_ACCESS_TOKEN": "from-env" } },
            "settings": { "ZEROHEIGHT_ACCESS_TOKEN": "from-settings" }
        }))
        .unwrap();

        assert_eq!(env["ZEROHEIGHT_ACCESS_TOKEN"], "from-settings");
        assert_eq!(env["ZEROHEIGHT_MCP_CLIENT"], "zed-mcp-plugin");
    }

    #[test]
    fn blank_credentials_are_never_passed_to_the_server() {
        let env = env_for(json!({
            "command": { "env": {
                "ZEROHEIGHT_ACCESS_TOKEN": "  ",
                "ZEROHEIGHT_CLIENT_ID": "client-from-env"
            } },
            "settings": { "ZEROHEIGHT_ACCESS_TOKEN": "", "ZEROHEIGHT_CLIENT_ID": "" }
        }))
        .unwrap();

        assert!(!env.contains_key("ZEROHEIGHT_ACCESS_TOKEN"));
        assert_eq!(env["ZEROHEIGHT_CLIENT_ID"], "client-from-env");
    }

    #[test]
    fn non_string_setting_is_rejected() {
        let err = env_for(json!({ "settings": { "ZEROHEIGHT_ACCESS_TOKEN": 123 } })).unwrap_err();

        assert!(err.contains("ZEROHEIGHT_ACCESS_TOKEN"), "{err}");
    }
}
