use iced::widget::svg::Handle;
use iced::widget::Svg;
use iced::{Length, Renderer};
use serde::{Deserialize, Serialize};

use crate::countries::flags_pictures::{
    BR, CN, DE, ES, FI, FLAGS_WIDTH_SMALL, FR, GB, GR, IT, JP, KR, PL, PT, RO, RU, SE, TR, UA, UZ,
};
use crate::StyleType;

/// This enum defines the available languages.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize, Hash)]
pub enum Language {
    /// English (default language).
    EN,
    /// Italian.
    IT,
    /// French.
    FR,
    /// Spanish.
    ES,
    /// Polish.
    PL,
    /// German,
    DE,
    /// Ukrainian
    UK,
    /// Simplified Chinese
    ZH,
    /// Romanian
    RO,
    /// Korean
    KO,
    /// European Portuguese
    PT,
    /// Turkish
    TR,
    /// Russian
    RU,
    /// Greek
    EL,
    // /// Persian
    // FA,
    /// Swedish
    SV,
    /// Finnish
    FI,
    /// Japanese
    JA,
    /// Uzbek
    UZ,
    /// Brazilian Portuguese
    PT_BR,
}

impl Default for Language {
    fn default() -> Self {
        Self::EN
    }
}

impl Language {
    pub(crate) const ROW1: [Language; 1] = [Language::EN];
    pub(crate) const ROW2: [Language; 3] = [Language::DE, Language::EL, Language::ES];
    pub(crate) const ROW3: [Language; 3] = [Language::FI, Language::FR, Language::IT];
    pub(crate) const ROW4: [Language; 3] = [Language::JA, Language::KO, Language::PL];
    pub(crate) const ROW5: [Language; 3] = [Language::PT, Language::PT_BR, Language::RO];
    pub(crate) const ROW6: [Language; 3] = [Language::RU, Language::SV, Language::TR];
    pub(crate) const ROW7: [Language; 3] = [Language::UK, Language::UZ, Language::ZH];

    pub fn get_radio_label(&self) -> &str {
        match self {
            Language::EN => "English",
            Language::IT => "Italiano",
            Language::FR => "Français",
            Language::ES => "Español",
            Language::PL => "Polski",
            Language::DE => "Deutsch",
            Language::UK => "Українська",
            Language::ZH => "简体中文",
            Language::RO => "Română",
            Language::KO => "한국어",
            Language::TR => "Türkçe",
            Language::RU => "Русский",
            Language::PT | Language::PT_BR => "Português",
            Language::EL => "Ελληνικά",
            // Language::FA => "فارسی",
            Language::SV => "Svenska",
            Language::FI => "Suomi",
            Language::JA => "日本語",
            Language::UZ => "O'zbekcha",
        }
    }

    pub fn get_flag(self) -> Svg<Renderer<StyleType>> {
        Svg::new(Handle::from_memory(Vec::from(match self {
            Language::ZH => CN,
            Language::DE => DE,
            Language::ES => ES,
            Language::FR => FR,
            Language::EN => GB,
            Language::IT => IT,
            Language::KO => KR,
            Language::PL => PL,
            Language::PT => PT,
            Language::RO => RO,
            Language::RU => RU,
            Language::TR => TR,
            Language::UK => UA,
            Language::EL => GR,
            // Language::FA => IR,
            Language::SV => SE,
            Language::FI => FI,
            Language::JA => JP,
            Language::UZ => UZ,
            Language::PT_BR => BR,
        })))
        .width(Length::Fixed(FLAGS_WIDTH_SMALL))
    }
}
