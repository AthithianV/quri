use anyhow::anyhow;
use gpui::*;
use rust_embed::RustEmbed;
use std::borrow::Cow;

use gpui::SharedString;
use gpui_component::IconNamed;

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "icons/**/*.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("could not find asset at path \"{path}\""))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

#[derive(Clone, Copy)]
pub enum AppIcon {
    Database,
    Ai,
    Save,
    History,
    Code,
    Refresh,
}

impl IconNamed for AppIcon {
    fn path(self) -> SharedString {
        match self {
            Self::Database => "icons/database.svg",
            Self::Ai => "icons/ai.svg",
            Self::Save => "icons/save.svg",
            Self::History => "icons/history.svg",
            Self::Code => "icons/code.svg",
            Self::Refresh => "icons/refresh.svg",
        }
        .into()
    }
}

pub struct CombinedAssets<A, B>(pub A, pub B);

impl<A, B> AssetSource for CombinedAssets<A, B>
where
    A: AssetSource,
    B: AssetSource,
{
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        // Try the first source; if it returns None or an error, try the second
        match self.0.load(path) {
            Ok(Some(asset)) => Ok(Some(asset)),
            _ => self.1.load(path),
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut list = self.0.list(path).unwrap_or_default();
        if let Ok(extra) = self.1.list(path) {
            list.extend(extra);
        }
        Ok(list)
    }
}
