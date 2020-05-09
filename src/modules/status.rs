use super::{Context, Module, RootModuleConfig};

use crate::configs::status::StatusConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the status of the last command
///
/// Will display the status only if it is not 0
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let exit_code_default = std::string::String::from("0");
    let exit_code = context
        .properties
        .get("status_code")
        .unwrap_or(&exit_code_default);
    let pipestatus: Vec<&str> = match context.properties.get("pipestatus") {
        Some(val) => val.split_ascii_whitespace().collect(),
        // fallback if --pipestatus is not provided
        None => vec![&exit_code],
    };

    // kind of a hack to not show `<previous status code>` when a user sends ^C to clear the line
    if exit_code == "130" && *pipestatus.last()? != "130" {
        return None;
    }

    let error = exit_code != "0";
    let pipeline_error = pipestatus.iter().any(|&code| code != "0");

    if error || pipeline_error {
        let mut module = context.new_module("exit_code");
        let config = StatusConfig::try_load(module.config);

        let formatter = if let Ok(formatter) = StringFormatter::new(config.format) {
            formatter.map(|variable| match variable {
                "status" => Some(exit_code.to_owned()),
                "pipeline_status" => Some(pipestatus.join(config.pipeline_separator)),
                _ => None,
            })
        } else {
            log::warn!("Error parsing format string in `status.format`");
            return None;
        };
        module.set_segments(formatter.parse(None));
        module.get_prefix().set_value("");
        module.get_suffix().set_value("");
        Some(module)
    } else {
        None
    }
}
