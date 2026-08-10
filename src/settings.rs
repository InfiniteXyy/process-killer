use std::{env, fs, path::PathBuf};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Locale {
    Zh,
    En,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThemePreference {
    System,
    Light,
    Dark,
}

#[derive(Clone, Debug)]
pub struct Settings {
    pub locale: Locale,
    pub theme: ThemePreference,
    pub refresh_ms: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            locale: Locale::Zh,
            theme: ThemePreference::System,
            refresh_ms: 5_000,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let Ok(text) = fs::read_to_string(config_path()) else {
            return Self::default();
        };
        let mut settings = Self::default();
        for line in text.lines() {
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            match (key, value) {
                ("locale", "en") => settings.locale = Locale::En,
                ("locale", "zh") => settings.locale = Locale::Zh,
                ("theme", "light") => settings.theme = ThemePreference::Light,
                ("theme", "dark") => settings.theme = ThemePreference::Dark,
                ("theme", "system") => settings.theme = ThemePreference::System,
                ("refresh_ms", value) => {
                    if let Ok(value @ (1_000 | 5_000 | 10_000 | 20_000)) = value.parse() {
                        settings.refresh_ms = value;
                    }
                }
                _ => {}
            }
        }
        settings
    }

    pub fn save(&self) {
        let locale = match self.locale {
            Locale::Zh => "zh",
            Locale::En => "en",
        };
        let theme = match self.theme {
            ThemePreference::System => "system",
            ThemePreference::Light => "light",
            ThemePreference::Dark => "dark",
        };
        let path = config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(
            path,
            format!(
                "locale={locale}\ntheme={theme}\nrefresh_ms={}\n",
                self.refresh_ms
            ),
        );
    }
}

fn config_path() -> PathBuf {
    let base = env::var_os("APPDATA")
        .or_else(|| env::var_os("XDG_CONFIG_HOME"))
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(env::temp_dir);
    base.join("process-killer").join("settings.conf")
}
