use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct StatusConfig<'a> {
    pub format: &'a str,
    pub pipeline_separator: &'a str,
    pub display_mode: DisplayMode,
    pub success_symbol: &'a str,
    pub error_symbol: &'a str,
    pub success_style: &'a str,
    pub error_style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for StatusConfig<'a> {
    fn new() -> Self {
        StatusConfig {
            format: "[$status_symbol$pipeline_status_if_error]($style) ",
            pipeline_separator: "|",
            display_mode: DisplayMode::ErrorOrMismatch,
            success_symbol: "✔",
            error_symbol: "✖",
            success_style: "",
            error_style: "red bold",
            disabled: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DisplayMode {
    Always,
    Error,
    PipelineError,
    ErrorOrMismatch,
}

impl<'a> ModuleConfig<'a> for DisplayMode {
    fn from_config(config: &'a toml::Value) -> Option<Self> {
        dbg!(config);
        match config.as_str()? {
            "always" => Some(DisplayMode::Always),
            "error" => Some(DisplayMode::Error),
            "pipeline error" => Some(DisplayMode::PipelineError),
            "error or mismatch" => Some(DisplayMode::ErrorOrMismatch),
            _ => None,
        }
    }
}
