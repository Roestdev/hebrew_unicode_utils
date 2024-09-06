pub mod unicode_block_hebrew {
    use hebrew_unicode_script::HebrewUnicodeScript;
    use std::collections::HashMap;

    #[derive(Debug, Default)]
    pub struct HebrewCharacterTypes {
        pub accent: bool,
        pub consonant: bool,
        pub consonant_normal: bool,
        pub consonant_final: bool,
        pub ligature_yiddish: bool,
        pub mark: bool,
        pub point: bool,
        pub point_vowel: bool,
        pub point_semi_vowel: bool,
        pub point_reading_sign: bool,
        pub punctuation: bool,
        pub yod_triangle: bool,
        pub whitespace: bool,
        pub non_hebrew: bool,
    }

    impl HebrewCharacterTypes {
        fn new() -> Self {
            Default::default()
        }
    }
    /// Get Hebrew character types for a given string
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::get_hbr_character_types;
    ///
    /// let test_string = "";
    /// let struct_result = get_hbr_character_types(test_string);
    /// assert_eq!(struct_result.accent, false);
    ///
    /// ```
    pub fn get_hbr_character_types(text: &str) -> HebrewCharacterTypes {
        let mut found_types = HebrewCharacterTypes::new();
        for c in text.chars() {
            match c {
                c if c.is_hbr_accent() => found_types.accent = true,
                //c if c.is_hbr_consonant() => found_types.consonant = true,
                c if c.is_hbr_consonant_normal() => found_types.consonant_normal = true,
                c if c.is_hbr_consonant_final() => found_types.consonant_final = true,
                c if c.is_hbr_ligature_yiddish() => found_types.ligature_yiddish = true,
                c if c.is_hbr_mark() => found_types.mark = true,
                //c if c.is_hbr_point() => found_types.point = true,
                c if c.is_hbr_point_vowel() => found_types.point_vowel = true,
                c if c.is_hbr_point_semi_vowel() => found_types.point_semi_vowel = true,
                c if c.is_hbr_point_reading_sign() => found_types.point_reading_sign = true,
                c if c.is_hbr_punctuation() => found_types.punctuation = true,
                c if c.is_hbr_yod_triangle() => found_types.yod_triangle = true,
                c if c.is_whitespace() => found_types.whitespace = true,
                _ => found_types.non_hebrew = true,
            }
        }
        // derive type CONSONANT from their sub-types
        found_types.consonant = found_types.consonant_normal || found_types.consonant_final;
        // derive type POINT from their sub-types
        found_types.point = found_types.point_vowel
            || found_types.point_semi_vowel
            || found_types.point_reading_sign;
        // return the struct
        found_types
    }

    /// Get the frequency hebrew characters of the given string
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::get_hbr_character_frequency;
    ///
    /// //let test_string = "abc abc";
    /// let test_string = "בהב";
    /// let struct_result = get_hbr_character_frequency(test_string);
    /// //println!("get_hbr_character_frequency\n{:#?}", struct_result);
    /// assert_eq!(struct_result.contains_key("ב"), true);
    /// assert_eq!(struct_result.get(&"ב".to_string()), Some(&2));  
    /// assert_eq!(struct_result.get(&"ה".to_string()), Some(&1));
    /// ```    
    pub fn get_hbr_character_frequency(s: &str) -> HashMap<String, usize> {
        let mut char_frequency: HashMap<String, usize> = HashMap::new();

        for c in s.chars() {
            if c.is_hbr_block() {
                *char_frequency.entry(c.to_string()).or_insert(0) += 1;
            }
        }
        char_frequency
    }
}

#[cfg(test)]
mod unit_test {
    use super::unicode_block_hebrew::*;

    #[test]
    fn hbr_character_types_no_hebrew_chars() {
        let test_string = "no hebrew characters";
        let struct_result = get_hbr_character_types(test_string);
        assert_eq!(struct_result.accent, false);
        assert_eq!(struct_result.consonant, false);
        assert_eq!(struct_result.consonant_normal, false);
        assert_eq!(struct_result.consonant_final, false);
        assert_eq!(struct_result.ligature_yiddish, false);
        assert_eq!(struct_result.mark, false);
        assert_eq!(struct_result.point, false);
        assert_eq!(struct_result.point_vowel, false);
        assert_eq!(struct_result.point_semi_vowel, false);
        assert_eq!(struct_result.point_reading_sign, false);
        assert_eq!(struct_result.punctuation, false);
        assert_eq!(struct_result.yod_triangle, false);
        assert_eq!(struct_result.whitespace, true);
        assert_eq!(struct_result.non_hebrew, true);
    }

    #[test]
    fn get_hbr_character_types_empty_string() {
        let test_string = "";
        let struct_result: HebrewCharacterTypes = get_hbr_character_types(test_string);
        assert_eq!(struct_result.accent, false);
        assert_eq!(struct_result.consonant, false);
        assert_eq!(struct_result.consonant_normal, false);
        assert_eq!(struct_result.consonant_final, false);
        assert_eq!(struct_result.ligature_yiddish, false);
        assert_eq!(struct_result.mark, false);
        assert_eq!(struct_result.point, false);
        assert_eq!(struct_result.point_vowel, false);
        assert_eq!(struct_result.point_semi_vowel, false);
        assert_eq!(struct_result.point_reading_sign, false);
        assert_eq!(struct_result.punctuation, false);
        assert_eq!(struct_result.yod_triangle, false);
        assert_eq!(struct_result.whitespace, false);
        assert_eq!(struct_result.non_hebrew, false);
    }
    #[test]
    fn hbr_character_types_hebrew_text() {
        let input_string = "בְּרֵאשִׁית בָּרָא אֱלֹהִים אֵת הַשָּׁמַיִם וְאֵת הָאָרֶץ׃";
        let struct_result = get_hbr_character_types(input_string);
        println!("test_hbr_character_types_no_3:\n{:?}", struct_result);
        assert_eq!(struct_result.accent, false);
        assert_eq!(struct_result.consonant, true);
        assert_eq!(struct_result.consonant_normal, true);
        assert_eq!(struct_result.consonant_final, true);
        assert_eq!(struct_result.ligature_yiddish, false);
        assert_eq!(struct_result.mark, false);
        assert_eq!(struct_result.point, true);
        assert_eq!(struct_result.point_vowel, true);
        assert_eq!(struct_result.point_semi_vowel, true);
        assert_eq!(struct_result.point_reading_sign, true);
        assert_eq!(struct_result.punctuation, true);
        assert_eq!(struct_result.yod_triangle, false);
        assert_eq!(struct_result.whitespace, true);
        assert_eq!(struct_result.non_hebrew, false);
    }

    #[test]
    fn get_hbr_character_frequency_no_hebrew_chars() {
        let test_string = "no hebrew characters";
        let freq_map = get_hbr_character_frequency(test_string);
        assert!(freq_map.is_empty());
    }

    #[test]
    fn get_hbr_character_frequency_hebrew_and_ascii() {
        let test_string = "Xבהב";
        let freq_map = get_hbr_character_frequency(test_string);
        assert_eq!(freq_map.contains_key("X"), false);
        assert_eq!(freq_map.get(&"ב".to_string()), Some(&2));
        assert_eq!(freq_map.get(&"ה".to_string()), Some(&1));
    }
}
