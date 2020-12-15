#[cfg(test)]
mod test_language_identifier {
    use icu::locid::LanguageIdentifier;

    fn parse(string: &'static str) -> LanguageIdentifier {
        string
            .parse()
            .expect("Failed to parse language identifier.")
    }

    #[test]
    fn simple() {
        let language_id = parse("zh");
        assert_eq!(language_id.language, "zh");
        assert_eq!(language_id.script, None);
        assert_eq!(language_id.region, None);
        assert_eq!(language_id.variants.into_raw(), None);
    }

    #[test]
    fn with_script_region() {
        let language_id = parse("zh-Hans-CN");
        assert_eq!(language_id.language, "zh");
        assert_eq!(language_id.script.unwrap(), "Hans");
        assert_eq!(language_id.region.unwrap(), "CN");
        assert_eq!(language_id.variants.into_raw(), None);
    }

    #[test]
    fn with_variant() {
        let language_id = parse("de-CH-1996");
        assert_eq!(language_id.language, "de");
        assert_eq!(language_id.script, None);
        assert_eq!(language_id.region.unwrap(), "CH");
        let variants = language_id.variants.into_raw().expect("Expected variants");
        assert_eq!(variants.len(), 1);
        assert_eq!(variants[0], "1996");
    }
}
