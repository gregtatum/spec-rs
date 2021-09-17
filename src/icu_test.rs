#[cfg(test)]
mod test_language_identifier {
    use icu::locid::macros::langid;
    use icu::locid::{LanguageIdentifier, Locale};

    #[test]
    fn simple() {
        let language_id = "zh".parse::<LanguageIdentifier>().unwrap();
        assert_eq!(language_id.language, "zh");
        assert_eq!(language_id.script, None);
        assert_eq!(language_id.region, None);
        assert_eq!(language_id.variants.into_raw(), None);
    }

    #[test]
    fn with_script_and_region() {
        let language_id = "zh-Hans-CN".parse::<LanguageIdentifier>().unwrap();
        assert_eq!(language_id.language, "zh");
        assert_eq!(language_id.script.unwrap(), "Hans");
        assert_eq!(language_id.region.unwrap(), "CN");
        assert_eq!(language_id.variants.into_raw(), None);
    }

    #[test]
    fn with_variant() {
        let language_id = "de-CH-1996".parse::<LanguageIdentifier>().unwrap();
        assert_eq!(language_id.language, "de");
        assert_eq!(language_id.script, None);
        assert_eq!(language_id.region.unwrap(), "CH");
        let variants = language_id.variants.into_raw().expect("Expected variants");
        assert_eq!(variants.len(), 1);
        assert_eq!(variants[0], "1996");
    }

    #[test]
    fn canonicalization() {
        // It represents language identifiers in the canonicalized form.
        let language_id = "ES".parse::<LanguageIdentifier>().unwrap();
        assert_eq!(language_id.language, "es");
    }

    #[test]
    fn with_macro() {
        let language_id = langid!("zh-Hans-CN");
        assert_eq!(language_id.language, "zh");
        assert_eq!(language_id.script.unwrap(), "Hans");
        assert_eq!(language_id.region.unwrap(), "CN");
    }

    #[test]
    fn mutating_locales() {
        let mut loc: Locale = "en-US".parse().expect("Parsing failed.");
        assert_eq!(loc.language, "en");
        assert_eq!(loc.to_string(), "en-US");
        loc.language = "zh".parse().expect("Parsing zh failed.");
        assert_eq!(loc.to_string(), "zh-US");
    }

    // This will be a compiler error due to the procedural macro failing.

    // #[test]
    // fn bad_macros_will_panic() {
    //     use icu::locid::macros::langid;
    //     let language_id = langid!("asdf");
    // }
}

#[cfg(test)]
mod test_providers {
    extern crate test;
    use self::test::{black_box, Bencher};
    use std::path::PathBuf;

    use icu::datetime::{date::MockDateTime, options, DateTimeFormat, DateTimeFormatOptions};
    use icu::locid::macros::langid;

    // The data must be generated first from:
    //
    // > cd ~/dev/icu4x/components/provider_fs
    // > cargo run --features export-bin -- --cldr-tag 37.0.0 --out ~/me/spec-rs/data/icu --all-keys
    //
    // This generates the proper manifest.json, which at this time doesn't appear
    // to be documented for manual generation or consumption.
    use icu_provider_fs::FsDataProvider;

    fn get_provider() -> FsDataProvider {
        FsDataProvider::try_new(PathBuf::from("data/icu"))
            .expect("Unable to find a provider at that directory.")
    }

    #[test]
    fn test_initializing_an_fs_provider() {
        let _provider = get_provider();
    }

    #[test]
    fn test_parsing_mock_date() {
        let date = "2020-10-14T13:21:50"
            .parse::<MockDateTime>()
            .expect("Failed to parse a date time.");

        assert_eq!(usize::from(date.year), 2020, "Parses the year");

        // Month and day start at 0.
        assert_eq!(usize::from(date.month), 9, "Parses the month");
        assert_eq!(usize::from(date.day), 13, "Parses the day");

        // Time starts at 1.
        assert_eq!(usize::from(date.hour), 13, "Parses the hour");
        assert_eq!(usize::from(date.minute), 21, "Parses the minute");
        assert_eq!(usize::from(date.second), 50, "Parses the second");
    }

    #[test]
    fn test_format_data_time_defaults() {
        let lid = langid!("en");

        let date = "2020-10-14T13:21:50"
            .parse::<MockDateTime>()
            .expect("Failed to parse a date time.");

        let formatter =
            DateTimeFormat::try_new(lid, &get_provider(), &DateTimeFormatOptions::default())
                .expect("Failed to create a DateTimeFormat");

        let formatted_date = formatter.format(&date);

        assert_eq!(
            formatted_date.to_string(),
            "October 14, 2020 at 1:21:50 PM z"
        );
    }

    #[should_panic(expected = "not implemented")]
    #[test]
    fn test_format_data_time_components() {
        let lid = langid!("en");

        let options = DateTimeFormatOptions::from(options::components::Bag {
            year: Some(options::components::Numeric::Numeric),
            month: Some(options::components::Month::Long),
            day: Some(options::components::Numeric::Numeric),

            hour: Some(options::components::Numeric::TwoDigit),
            minute: Some(options::components::Numeric::TwoDigit),

            preferences: None,

            ..Default::default()
        });

        // The components are not implemented yet.
        let _formatter = DateTimeFormat::try_new(lid, &get_provider(), &options)
            .expect("Failed to create a DateTimeFormat");
    }

    #[test]
    fn test_format_data_time_style() {
        use icu::datetime::options::style;

        let lid = langid!("en");

        let date = "2020-10-14T13:21:50"
            .parse::<MockDateTime>()
            .expect("Failed to parse a date time.");

        let options = DateTimeFormatOptions::Style(style::Bag {
            date: Some(style::Date::Medium),
            time: Some(style::Time::Short),
            ..Default::default()
        });

        let formatter = DateTimeFormat::try_new(lid, &get_provider(), &options)
            .expect("Failed to create a DateTimeFormat");

        let formatted_date = formatter.format(&date);

        assert_eq!(formatted_date.to_string(), "Oct 14, 2020, 1:21 PM");
    }

    #[bench]
    fn bench_date_time(bencher: &mut Bencher) {
        bencher.iter(|| {
            use icu::datetime::options::style;

            let lid = langid!("en");

            let date = "2020-10-14T13:21:50"
                .parse::<MockDateTime>()
                .expect("Failed to parse a date time.");

            let options = DateTimeFormatOptions::Style(style::Bag {
                date: Some(style::Date::Medium),
                time: Some(style::Time::Short),
                ..Default::default()
            });

            let formatter = DateTimeFormat::try_new(lid, &get_provider(), &options)
                .expect("Failed to create a DateTimeFormat");

            black_box(formatter.format(&date));
        });
    }
}
