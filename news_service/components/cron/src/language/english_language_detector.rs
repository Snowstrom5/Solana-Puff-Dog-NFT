use lingua::Language::{English, French, German, Spanish, Swedish};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

pub struct EnglishLanguageDetector {
    detector: LanguageDetector,
}

impl EnglishLanguageDetector {
    pub fn new(detector: LanguageDetector) -> EnglishLanguageDetector {
        EnglishLanguageDetector { detector }
    }
    pub fn init() -> EnglishLanguageDetector {
        let languages = vec![English, French, German, Spanish, Swedish];
        let detector: LanguageDetector =
            LanguageDetectorBuilder::from_languages(&languages).build();
        EnglishLanguageDetector::new(detector)
    }

    pub fn is_english_language(&self, text: &str) -> bool {
        let detected_language: Option<Language> = self.detector.detect_language_of(text);
        detected_language == Some(English)
    }
}
