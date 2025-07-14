#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

#[allow(dead_code)]
pub fn process_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Process",
        Language::IT => "Processo",
        _ => "Process",
    }
}
