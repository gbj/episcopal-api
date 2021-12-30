use language::Language;

pub fn locale_to_language(locale: &str) -> Language {
    match locale {
        "en-US" => Language::En,
        "es-ES" => Language::Es,
        "fr-FR" => Language::Fr,
        _ => Language::En,
    }
}
