use super::{Context, Module, RootModuleConfig};

use crate::configs::status::{DisplayMode, StatusConfig};
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

    let mut module = context.new_module("exit_code");
    let config = StatusConfig::try_load(module.config);

    let error = exit_code != "0";
    let pipeline_error = pipestatus.iter().any(|&code| code != "0");
    let mismatch = exit_code != *pipestatus.last()?;

    let display_mode = config.display_mode;
    let show = match display_mode {
        DisplayMode::Always => true,
        DisplayMode::Error => error,
        DisplayMode::PipelineError => pipeline_error,
        DisplayMode::ErrorOrMismatch => pipeline_error || mismatch,
    };
    if !show {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => {
                    if error {
                        Some(Ok(config.error_style.to_owned()))
                    } else {
                        Some(Ok(config.success_style.to_owned()))
                    }
                }
                _ => None,
            })
            .map(|variable| match variable {
                "status" => Some(Ok(exit_code.to_owned())),
                "status_if_error" => Some(Ok(if error {
                    exit_code.to_owned()
                } else {
                    "".to_owned()
                })),
                "pipeline_status" => Some(Ok(pipestatus.join(config.pipeline_separator))),
                "pipeline_status_if_error" => Some(Ok(if error || pipeline_error {
                    pipestatus.join(config.pipeline_separator)
                } else {
                    "".to_owned()
                })),
                "status_symbol" => {
                    if error {
                        Some(Ok(config.error_symbol.to_owned()))
                    } else {
                        Some(Ok(config.success_symbol.to_owned()))
                    }
                }
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `status`:\n{}", error);
            return None;
        }
    });

    module.get_prefix().set_value("");
    module.get_suffix().set_value("");
    Some(module)
}
