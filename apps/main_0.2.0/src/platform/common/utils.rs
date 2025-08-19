pub fn normalize_language_name(lang: String) -> String {
  match lang.as_str() {
    "ru" => "ru-RU".to_string(),
    "en" => "en-US".to_string(),
    "fr" => "fr-FR".to_string(),
    "de" => "de-DE".to_string(),
    "es" => "es-ES".to_string(),
    "it" => "it-IT".to_string(),
    "zh" => "zh-CN".to_string(),
    "ja" => "ja-JP".to_string(),
    "ko" => "ko-KR".to_string(),
    _ => lang,
  }
}
