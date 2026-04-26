//! # OpenCrab Branding Constants
//!
//! Single source of truth for all brand-specific, compile-time constants used
//! across the workspace.  To rebrand the entire project, edit **only this
//! file** — every other crate pulls its values from here.
//!
//! ## Naming Convention
//!
//! Constants are grouped by category.  Each constant has a doc-comment
//! explaining where it is consumed so that future maintainers can grep for
//! dependents easily.

// ---------------------------------------------------------------------------
// 1. Product name (human-readable)
// ---------------------------------------------------------------------------

/// Human-readable product name shown in UI, logs, and error messages.
pub const PRODUCT_NAME: &str = "OpenCrab";

// ---------------------------------------------------------------------------
// 2. Home directory
// ---------------------------------------------------------------------------

/// Name of the dot-directory under `$HOME` that stores user-level
/// configuration, auth tokens, themes, skills cache, and SQLite databases.
///
/// Resolved by `codex-utils-home-dir::find_codex_home()`.
pub const HOME_DIR_NAME: &str = ".opencrab";

/// Environment variable that, when set, overrides the default home directory.
///
/// Consumed by `codex-utils-home-dir::find_codex_home()`.
pub const HOME_ENV_VAR: &str = "OPENCRAB_HOME";

// ---------------------------------------------------------------------------
// 3. SQLite storage
// ---------------------------------------------------------------------------

/// Environment variable that overrides the SQLite database directory.
/// Falls back to the home directory when unset.
///
/// Consumed by `codex-state`.
pub const SQLITE_HOME_ENV_VAR: &str = "OPENCRAB_SQLITE_HOME";

// ---------------------------------------------------------------------------
// 4. Project-level config directory
// ---------------------------------------------------------------------------

/// Name of the per-project configuration directory (lives at repo root).
/// Contains project-scoped `config.toml`, instructions, etc.
///
/// Consumed by `codex-core::config_loader` and sandbox modules.
pub const PROJECT_CONFIG_DIR_NAME: &str = ".opencrab";

// ---------------------------------------------------------------------------
// 5. macOS Managed Preferences (MDM)
// ---------------------------------------------------------------------------

/// Application identifier used for macOS managed preferences (MDM / profiles).
///
/// Consumed by `codex-core::config_loader::macos`.
pub const MACOS_MDM_DOMAIN: &str = "com.opencrab.opencrab";

// ---------------------------------------------------------------------------
// 6. App-server environment variables
// ---------------------------------------------------------------------------

/// Env var pointing to a managed config TOML path (debug / test hook).
pub const APP_SERVER_MANAGED_CONFIG_PATH_ENV: &str = "OPENCRAB_APP_SERVER_MANAGED_CONFIG_PATH";

/// Env var that disables loading managed config entirely (debug / test hook).
pub const APP_SERVER_DISABLE_MANAGED_CONFIG_ENV: &str = "OPENCRAB_APP_SERVER_DISABLE_MANAGED_CONFIG";

// ---------------------------------------------------------------------------
// 7. API key environment variable
// ---------------------------------------------------------------------------

/// Environment variable for the API key.
///
/// Consumed by `codex-login::auth`.
pub const API_KEY_ENV_VAR: &str = "OPENCRAB_API_KEY";

// ---------------------------------------------------------------------------
// 8. Keyring
// ---------------------------------------------------------------------------

/// Service name used when storing/retrieving auth credentials from the OS
/// keyring (macOS Keychain, Windows Credential Manager, etc.).
///
/// Consumed by `codex-login::auth::storage`.
pub const KEYRING_SERVICE_NAME: &str = "OpenCrab Auth";
