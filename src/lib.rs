//! A ZeroClaw WIT tool plugin: `sol-guard`.
//!
//! Assesses risk for Solana token mints by fetching and parsing on-chain
//! account data (mint/freeze authorities, Token-2022 extensions). The pure
//! risk-assessment core lives in [`guard`] with no wasm dependency, so it
//! compiles and tests on the host with a plain `cargo test`; the wasm component
//! reuses the exact same logic through this shim.
//!
//! Build:  rustup target add wasm32-wasip2
//!         cargo build --target wasm32-wasip2 --release

pub mod guard;

#[cfg(target_family = "wasm")]
mod component {
    wit_bindgen::generate!({
        path: "../../wit/v0",
        world: "tool-plugin",
        features: ["plugins-wit-v0"],
    });

    use std::collections::HashMap;

    use crate::guard::{assess, RiskConfig, RpcClient};
    use exports::zeroclaw::plugin::plugin_info::Guest as PluginInfo;
    use exports::zeroclaw::plugin::tool::{Guest as Tool, ToolResult};
    use zeroclaw::plugin::logging::{
        log_record, LogLevel, PluginAction, PluginEvent, PluginOutcome,
    };

    struct SolGuard;

    const PLUGIN_NAME: &str = "sol-guard";
    const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
    const TOOL_NAME: &str = "sol-guard";

    struct WakiRpcClient;

    impl RpcClient for WakiRpcClient {
        fn get_account_info(&self, rpc_url: &str, mint: &str) -> Result<serde_json::Value, String> {
            let body = serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getAccountInfo",
                "params": [
                    mint,
                    { "encoding": "jsonParsed" }
                ]
            });
            waki::Client::new()
                .post(rpc_url)
                .json(&body)
                .send()
                .map_err(|e| e.to_string())?
                .json::<serde_json::Value>()
                .map_err(|e| e.to_string())
        }
    }

    #[derive(serde::Deserialize)]
    struct ExecuteArgs {
        mint: String,
        #[serde(rename = "__config", default)]
        config: HashMap<String, String>,
    }

    impl PluginInfo for SolGuard {
        fn plugin_name() -> String {
            PLUGIN_NAME.to_string()
        }
        fn plugin_version() -> String {
            PLUGIN_VERSION.to_string()
        }
    }

    impl Tool for SolGuard {
        fn name() -> String {
            TOOL_NAME.to_string()
        }

        fn description() -> String {
            "Assess risk for a Solana token mint by analysing on-chain \
             mint/freeze authorities and Token-2022 extensions. \
             Returns a GREEN/AMBER/RED verdict with human-readable reasons."
                .to_string()
        }

        fn parameters_schema() -> String {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "mint": {
                        "type": "string",
                        "description": "The base58 mint address to assess."
                    }
                },
                "required": ["mint"]
            })
            .to_string()
        }

        fn execute(args: String) -> Result<ToolResult, String> {
            let parsed: ExecuteArgs = match serde_json::from_str(&args) {
                Ok(a) => a,
                Err(e) => {
                    emit(
                        PluginAction::Fail,
                        PluginOutcome::Failure,
                        "invalid arguments",
                        None,
                    );
                    return Ok(ToolResult {
                        success: false,
                        output: String::new(),
                        error: Some(format!("invalid arguments: {e}")),
                    });
                }
            };

            let cfg = RiskConfig::from_section(&parsed.config);
            let client = WakiRpcClient;

            match assess(&parsed.mint, &cfg, &client) {
                Ok(report) => {
                    let summary = report.to_summary();
                    let n = report.findings.len();
                    emit(
                        PluginAction::Complete,
                        PluginOutcome::Success,
                        "assessed mint",
                        Some(n),
                    );
                    Ok(ToolResult {
                        success: true,
                        output: summary,
                        error: None,
                    })
                }
                Err(e) => {
                    emit(
                        PluginAction::Complete,
                        PluginOutcome::Failure,
                        "assessment failed",
                        None,
                    );
                    Ok(ToolResult {
                        success: false,
                        output: String::new(),
                        error: Some(scrub(&e)),
                    })
                }
            }
        }
    }

    /// Map internal errors to a message safe to hand back to the LLM/user —
    /// no transport internals, no RPC URL, no config values.
    fn scrub(e: &crate::guard::RiskError) -> String {
        use crate::guard::RiskError;
        match e {
            RiskError::InvalidMint(_) => "that doesn't look like a valid mint address".to_string(),
            RiskError::Rpc(_) => "couldn't reach the configured RPC endpoint".to_string(),
            RiskError::UnexpectedResponse(_) => "rpc returned an unexpected response".to_string(),
        }
    }

    fn emit(
        action: PluginAction,
        outcome: PluginOutcome,
        message: &str,
        findings: Option<usize>,
    ) {
        let attrs = findings.map(|n| format!("{{\"findings\":{n}}}"));
        log_record(
            LogLevel::Info,
            &PluginEvent {
                function_name: "sol_guard::tool::execute".to_string(),
                action,
                outcome: Some(outcome),
                duration_ms: None,
                attrs,
                message: message.to_string(),
            },
        );
    }

    export!(SolGuard);
}
