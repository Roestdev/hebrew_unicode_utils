//use hebrew_unicode_script::meteg_layout::HebrewMetegJoinerCharacters;
use crate::meteg_layout::HebrewMetegJoinerCharacters;
use hebrew_unicode_script::HebrewUnicodeScript;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)] // Display
pub enum IsPresent {
    Yes,
    #[default]
    No,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)] // Display
pub struct HebrewCharacterTypes {
    pub accent: IsPresent,
    pub consonant: IsPresent, // wil be derived
    pub consonant_normal: IsPresent,
    pub consonant_final: IsPresent,
    pub consonant_alternative: IsPresent,
    pub consonant_wide: IsPresent,
    pub consonant_with_vowel: IsPresent,
    pub ligature: IsPresent,
    pub ligature_yiddish: IsPresent,
    pub mark: IsPresent,
    pub point: IsPresent, // wil be derived
    pub point_reading_sign: IsPresent,
    pub point_semi_vowel: IsPresent,
    pub point_vowel: IsPresent,
    pub punctuation: IsPresent,
    pub yod_triangle: IsPresent,
    pub meteg_zero_width_non_joiner: IsPresent,
    pub meteg_zero_width_joiner: IsPresent,
    pub meteg_combining_grapheme_joiner: IsPresent,
    pub meteg_ascii_space: IsPresent,
    pub whitespace: IsPresent,
    pub non_hebrew: IsPresent,
}

impl HebrewCharacterTypes {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)] // Display
pub struct HebrewCharacterTypeCount {
    pub accent_cnt: u32,
    pub consonant_cnt: u32,
    pub consonant_normal_cnt: u32,
    pub consonant_final_cnt: u32,
    pub consonant_alternative_cnt: u32,
    pub consonant_wide_cnt: u32,
    pub consonant_with_vowel_cnt: u32,
    pub ligature_cnt: u32,
    pub ligature_yiddish_cnt: u32,
    pub mark_cnt: u32,
    pub point_cnt: u32,
    pub point_reading_sign_cnt: u32,
    pub point_semi_vowel_cnt: u32,
    pub point_vowel_cnt: u32,
    pub punctuation_cnt: u32,
    pub yod_triangle_cnt: u32,
    pub meteg_zero_width_non_joiner_cnt: u32,
    pub meteg_zero_width_joiner_cnt: u32,
    pub meteg_combining_grapheme_joiner_cnt: u32,
    pub meteg_ascii_space_cnt: u32,
    pub whitespace_cnt: u32,
    pub non_hebrew_cnt: u32,
}

impl HebrewCharacterTypeCount {
    fn new() -> Self {
        Default::default()
    }
}

/// Get Hebrew character types for a given string
///
/// # Examples
/// ```
/// use hebrew_unicode_utils::hebrew_character_types;
/// use hebrew_unicode_utils::IsPresent;
/// 
/// let test_string = "";
/// let results = hebrew_character_types(test_string);
/// println!("{:?}",results);
/// assert_eq!(results.accent, IsPresent::No);
///
/// ```
pub fn hebrew_character_types(text: &str) -> HebrewCharacterTypes {
    let mut char_type = HebrewCharacterTypes::new();
    for c in text.chars() {
        match c {
            c if c.is_hbr_accent() => char_type.accent = IsPresent::Yes,
            c if c.is_hbr_consonant_normal() => char_type.consonant_normal = IsPresent::Yes,
            c if c.is_hbr_consonant_final() => char_type.consonant_final = IsPresent::Yes,
            c if c.is_apf_alternative() => char_type.consonant_alternative = IsPresent::Yes,
            c if c.is_apf_consonant_wide() => char_type.consonant_wide = IsPresent::Yes,
            c if c.is_apf_consonant_with_vowel() => char_type.consonant_with_vowel = IsPresent::Yes,
            c if c.is_apf_ligature() => char_type.ligature = IsPresent::Yes,
            c if c.is_hbr_ligature_yiddish() => char_type.ligature_yiddish = IsPresent::Yes,
            c if c.is_hbr_mark() => char_type.mark = IsPresent::Yes,
            c if c.is_hbr_point_vowel() => char_type.point_vowel = IsPresent::Yes,
            c if c.is_hbr_point_semi_vowel() => char_type.point_semi_vowel = IsPresent::Yes,
            c if c.is_hbr_point_reading_sign() => char_type.point_reading_sign = IsPresent::Yes,
            c if c.is_hbr_punctuation() => char_type.punctuation = IsPresent::Yes,
            c if c.is_hbr_yod_triangle() => char_type.yod_triangle = IsPresent::Yes,

            c if c.is_zero_width_non_joiner() => {
                char_type.meteg_zero_width_non_joiner = IsPresent::Yes
            }
            c if c.is_zero_width_joiner() => {
                char_type.meteg_zero_width_joiner = IsPresent::Yes
            }
            c if c.is_combining_grapheme_joiner() => {
                char_type.meteg_combining_grapheme_joiner = IsPresent::Yes
            }
            //c if c.is_ascii_space() => char_type.meteg_ascii_space = IsPresent::Yes,
            c if c.is_whitespace() => char_type.whitespace = IsPresent::Yes,
            _ => char_type.non_hebrew = IsPresent::Yes,
        }
    }

    // derive type CONSONANT from their sub-type
    if char_type.consonant_normal == IsPresent::Yes
        || char_type.consonant_final == IsPresent::Yes
        || char_type.consonant_alternative == IsPresent::Yes
        || char_type.consonant_wide == IsPresent::Yes
    {
        char_type.consonant = IsPresent::Yes
    }

    // derive type POINT from their sub-types
    if char_type.point_vowel == IsPresent::Yes
        || char_type.point_semi_vowel == IsPresent::Yes
        || char_type.point_reading_sign == IsPresent::Yes
    {
        char_type.point = IsPresent::Yes
    }

    // return the struct
    char_type
}

/// Get Hebrew character types for a given string
///
/// # Examples
/// ```
/// use hebrew_unicode_utils::hebrew_character_types;
/// use hebrew_unicode_utils::IsPresent;
///
/// let test_string = "";
/// let results = hebrew_character_types(test_string);
/// assert_eq!(results.accent, IsPresent::No);
///
/// ```
pub fn hebrew_character_type_cnt(text: &str) -> HebrewCharacterTypeCount {
    let mut char_type_cnt = HebrewCharacterTypeCount::new();
    for c in text.chars() {
        match c {
            c if c.is_hbr_accent() => char_type_cnt.accent_cnt += 1,
            c if c.is_hbr_consonant() => char_type_cnt.consonant_cnt += 1,
            c if c.is_hbr_consonant_normal() => char_type_cnt.consonant_normal_cnt += 1,
            c if c.is_hbr_consonant_final() => char_type_cnt.consonant_final_cnt += 1,
            c if c.is_apf_alternative() => char_type_cnt.consonant_alternative_cnt += 1,
            c if c.is_apf_consonant_wide() => char_type_cnt.consonant_wide_cnt += 1,
            c if c.is_apf_consonant_with_vowel() => char_type_cnt.consonant_with_vowel_cnt += 1,
            c if c.is_apf_ligature() => char_type_cnt.ligature_cnt += 1,
            c if c.is_hbr_ligature_yiddish() => char_type_cnt.ligature_yiddish_cnt += 1,
            c if c.is_hbr_mark() => char_type_cnt.mark_cnt += 1,
            c if c.is_hbr_point() => char_type_cnt.point_cnt += 1,
            c if c.is_hbr_point_vowel() => char_type_cnt.point_vowel_cnt += 1,
            c if c.is_hbr_point_semi_vowel() => char_type_cnt.point_semi_vowel_cnt += 1,
            c if c.is_hbr_point_reading_sign() => char_type_cnt.point_reading_sign_cnt += 1,
            c if c.is_hbr_punctuation() => char_type_cnt.punctuation_cnt += 1,
            c if c.is_hbr_yod_triangle() => char_type_cnt.yod_triangle_cnt += 1,
            c if c.is_whitespace() => char_type_cnt.whitespace_cnt += 1,
            _ => char_type_cnt.non_hebrew_cnt += 1,
        }
    }

    // return the struct
    char_type_cnt
}

/// Get the frequency hebrew characters of the given string
///
/// # Examples
/// ```
/// use hebrew_unicode_utils::hebrew_character_frequency;
/// use hebrew_unicode_utils::IsPresent;
///
/// //let test_string = "abc abc";
/// let test_string = "ב x הב";
/// let results = hebrew_character_frequency(test_string);
/// //println!("hebrew_character_frequency\n{:#?}", results);
/// assert!(!results.contains_key(&" ".to_string()));
/// assert!(!results.contains_key(&"x".to_string()));
/// assert!(results.contains_key(&"ב".to_string()));
/// assert_eq!(results.get(&"ב".to_string()), Some(&2));  
/// assert_eq!(results.get(&"ה".to_string()), Some(&1));
/// ```    
pub fn hebrew_character_frequency(s: &str) -> HashMap<String, usize> {
    let mut char_frequency: HashMap<String, usize> = HashMap::new();

    for c in s.chars() {
        if c.is_script_hbr() {
            *char_frequency.entry(c.to_string()).or_insert(0) += 1;
        }
    }
    char_frequency
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn hbr_character_types_no_hebrew_chars() {
        let test_string = "no hebrew characters";
        let results = hebrew_character_types(test_string);
        assert_eq!(results.accent, IsPresent::No);
        assert_eq!(results.consonant, IsPresent::No);
        assert_eq!(results.consonant_normal, IsPresent::No);
        assert_eq!(results.consonant_final, IsPresent::No);
        assert_eq!(results.ligature, IsPresent::No);
        assert_eq!(results.mark, IsPresent::No);
        assert_eq!(results.point, IsPresent::No);
        assert_eq!(results.point_vowel, IsPresent::No);
        assert_eq!(results.point_semi_vowel, IsPresent::No);
        assert_eq!(results.point_reading_sign, IsPresent::No);
        assert_eq!(results.punctuation, IsPresent::No);
        assert_eq!(results.yod_triangle, IsPresent::No);
        assert_eq!(results.whitespace, IsPresent::Yes);
        assert_eq!(results.non_hebrew, IsPresent::Yes);
    }

    #[test]
    fn hebrew_character_types_empty_string() {
        let test_string = "";
        let results: HebrewCharacterTypes = hebrew_character_types(test_string);
        assert_eq!(results.accent, IsPresent::No);
        assert_eq!(results.consonant, IsPresent::No);
        assert_eq!(results.consonant_normal, IsPresent::No);
        assert_eq!(results.consonant_final, IsPresent::No);
        assert_eq!(results.ligature, IsPresent::No);
        assert_eq!(results.mark, IsPresent::No);
        assert_eq!(results.point, IsPresent::No);
        assert_eq!(results.point_vowel, IsPresent::No);
        assert_eq!(results.point_semi_vowel, IsPresent::No);
        assert_eq!(results.point_reading_sign, IsPresent::No);
        assert_eq!(results.punctuation, IsPresent::No);
        assert_eq!(results.yod_triangle, IsPresent::No);
        assert_eq!(results.whitespace, IsPresent::No);
        assert_eq!(results.non_hebrew, IsPresent::No);
    }
    #[test]
    fn hbr_character_types_hebrew_text() {
        let input_string = "בְּרֵאשִׁית בָּרָא אֱלֹהִים אֵת הַשָּׁמַיִם וְאֵת הָאָרֶץ׃";
        let results = hebrew_character_types(input_string);
        println!("test_hbr_character_types_no_3:\n{:?}", results);
        assert_eq!(results.accent, IsPresent::No);
        assert_eq!(results.consonant, IsPresent::Yes);
        assert_eq!(results.consonant_normal, IsPresent::Yes);
        assert_eq!(results.consonant_final, IsPresent::Yes);
        assert_eq!(results.ligature, IsPresent::No);
        assert_eq!(results.mark, IsPresent::No);
        assert_eq!(results.point, IsPresent::Yes);
        assert_eq!(results.point_vowel, IsPresent::Yes);
        assert_eq!(results.point_semi_vowel, IsPresent::Yes);
        assert_eq!(results.point_reading_sign, IsPresent::Yes);
        assert_eq!(results.punctuation, IsPresent::Yes);
        assert_eq!(results.yod_triangle, IsPresent::No);
        assert_eq!(results.whitespace, IsPresent::Yes);
        assert_eq!(results.non_hebrew, IsPresent::No);
    }

    #[test]
    fn hebrew_character_frequency_no_hebrew_chars() {
        let test_string = "no hebrew characters";
        let freq_map = hebrew_character_frequency(test_string);
        assert!(freq_map.is_empty());
    }

    #[test]
    fn hebrew_character_frequency_mixed_hebrew_and_ascii() {
        let test_string = "Xבהב";
        let freq_map = hebrew_character_frequency(test_string);
        assert_eq!(freq_map.get(&"X".to_string()), None);
        assert_eq!(freq_map.get(&"ב".to_string()), Some(&2));
        assert_eq!(freq_map.get(&"ה".to_string()), Some(&1));
    }
}
