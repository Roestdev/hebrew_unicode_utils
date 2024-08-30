// use hebrew_unicode_script::HebrewUnicodeScript;
// use std::borrow::Cow;

pub mod unicode_block_hebrew {
    use hebrew_unicode_script::HebrewUnicodeScript;
    use std::borrow::Cow;

    /// Removes all characters belonging to the unicode block 'Hebrew' from the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::remove_hbr_block;
    ///
    /// let test_str = "abcוַֽיְחִי־שֵׁ֗םאַֽחֲרֵי֙הוֹלִיד֣וֹאֶת־אַרְפַּכְשָׁ֔דחֲמֵ֥שׁמֵא֖וֹתשָׁנָ֑הוַdיּ֥וֹלֶדבָּנִ֖יםeוּבָfנֽוֹת׃";
    /// let test_str_filtered = remove_hbr_block(test_str);
    /// assert_eq!(test_str_filtered.as_ref(),"abcdef");
    /// ```
    pub fn remove_hbr_block(string: &str) -> Cow<'_, str> {
        string.chars().filter(|&c| !c.is_hbr_block()).collect()
    }
    /// Removes all Hebrew acccents from the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::remove_hbr_accent;
    ///
    /// let test_str = "בְּרֵאשִׁ֖ית";
    /// let test_str_filtered = remove_hbr_accent(test_str);
    /// assert_eq!(test_str_filtered.as_ref(), "בְּרֵאשִׁית");
    /// ```
    pub fn remove_hbr_accent(string: &str) -> Cow<'_, str> {
        string.chars().filter(|&c| !c.is_hbr_accent()).collect()
    }

    /// Removes all Hebrew marks from the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::remove_hbr_mark;
    ///
    /// let test_str = "ה֯";
    /// let test_str_filtered = remove_hbr_mark(test_str);
    /// assert_eq!(test_str_filtered.as_ref(),"ה");
    /// ```
    pub fn remove_hbr_mark(string: &str) -> Cow<'_, str> {
        string.chars().filter(|&c| !c.is_hbr_mark()).collect()
    }
    /// Removes all Hebrew points from the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::remove_hbr_point;
    ///
    /// let test_str = "בָ";
    /// let test_str_filtered = remove_hbr_point(test_str);
    /// assert_eq!(test_str_filtered.as_ref(),"ב");
    /// ```
    pub fn remove_hbr_point(string: &str) -> Cow<'_, str> {
        string.chars().filter(|&c| !c.is_hbr_point()).collect()
    }

    /// Removes all Hebrew point vowels from the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::remove_hbr_point_vowel;
    ///
    /// let test_str = "֟כַ כֳ כּ כ";
    /// let test_str_filtered = remove_hbr_point_vowel(test_str);
    /// assert_eq!(test_str_filtered.as_ref(),"֟כ כֳ כּ כ");
    /// ```
    pub fn remove_hbr_point_vowel(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_point_vowel())
            .collect()
    }
    /// Removes all Hebrew point semi-vowels from the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::remove_hbr_point_semi_vowel;
    ///
    /// let test_str = "ֲדְגֱכֳע";
    /// let test_str_filtered = remove_hbr_point_semi_vowel(test_str);
    /// assert_eq!(test_str_filtered.as_ref(),"דגכע");
    /// ```
    pub fn remove_hbr_point_semi_vowel(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_point_semi_vowel())
            .collect()
    }
    /// Removes all Hebrew point reading signs from the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::remove_hbr_point_reading_sign;
    ///
    /// let test_str = "םּ";
    /// let test_str_filtered = remove_hbr_point_reading_sign(test_str);
    /// assert_eq!(test_str_filtered.as_ref(),"ם");
    /// ```
    pub fn remove_hbr_point_reading_sign(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_point_reading_sign())
            .collect()
    }
    /// Removes all Hebrew punctuations from the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::remove_hbr_punctuation;
    ///
    /// //let test_str = "וַֽיְחִי־שֵׁ֗םאַֽחֲרֵי֙הוֹלִיד֣וֹאֶת־אַרְפַּכְשָׁ֔דחֲמֵ֥שׁמֵא֖וֹתשָׁנָ֑הוַיּ֥וֹלֶדבָּנִ֖יםוּבָנֽוֹת׃";
    /// let test_str = "וַֽיְהִי־בֹ֖קֶר";
    /// let test_str_filtered = remove_hbr_punctuation(test_str);
    /// assert_eq!(test_str_filtered.as_ref(),"וַֽיְהִיבֹ֖קֶר");
    /// ```
    pub fn remove_hbr_punctuation(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_punctuation())
            .collect()
    }
    /// Removes all Hebrew letters (final and normal) from the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::remove_hbr_consonant;
    ///
    /// let test_str = "AאBףC";
    /// let test_str_filtered = remove_hbr_consonant(test_str);
    /// assert_eq!(test_str_filtered.as_ref(),"ABC");
    /// ```
    pub fn remove_hbr_consonant(string: &str) -> Cow<'_, str> {
        string.chars().filter(|&c| !c.is_hbr_consonant()).collect()
    }
    /// Removes all Hebrew normal letters from the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::remove_hbr_consonant_normal;
    ///
    /// let test_str = "AאBףC";
    /// let test_str_filtered = remove_hbr_consonant_normal(test_str);
    /// assert_eq!(test_str_filtered.as_ref(),"ABףC");
    /// ```
    pub fn remove_hbr_consonant_normal(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_consonant_normal())
            .collect()
    }
    /// Removes all Hebrew final letters from the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::remove_hbr_consonant_final;
    ///
    /// let test_str = "AאBףC";
    /// let test_str_filtered = remove_hbr_consonant_final(test_str);
    /// assert_eq!(test_str_filtered.as_ref(),"AאBC");
    /// ```
    pub fn remove_hbr_consonant_final(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_consonant_final())
            .collect()
    }
    /// Removes all Hebrew yod triangles from the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::remove_hbr_yod_triangle;
    ///
    /// let test_str = format!("A{}Z",'\u{05EF}');
    /// let test_str_filtered = remove_hbr_yod_triangle(&test_str);
    /// assert_eq!(test_str_filtered.as_ref(),"AZ");
    /// ```
    pub fn remove_hbr_yod_triangle(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_yod_triangle())
            .collect()
    }
    /// Removes all Yiddish ligatures from the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::remove_hbr_ligature_yiddish;
    ///
    /// let test_str = "XװױײZ";
    /// let test_str_filtered = remove_hbr_ligature_yiddish(test_str);
    /// assert_eq!(test_str_filtered.as_ref(),"XZ");
    /// ```
    pub fn remove_hbr_ligature_yiddish(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_ligature_yiddish())
            .collect()
    }
}
#[cfg(test)]
mod unit_test {
    use crate::*;
    // #[test]
    // fn rem_hbr_block() {
    //     let test_str = "וַֽיְחִי־שֵׁ֗םאַֽחֲרֵי֙הוֹלִיד֣וֹת׃";
    //     let test_str_filtered = remove_hbr_block(test_str);
    //     assert_eq!(test_str_filtered.as_ref(), "");
    // }

    #[test]
    fn rem_hbr_accent() {
        let test_str = "בְּרֵאשִׁ֖ית";
        let test_str_filtered = remove_hbr_accent(test_str);
        assert_eq!(test_str_filtered.as_ref(), "בְּרֵאשִׁית");
    }

    #[test]
    pub fn rem_hbr_mark() {
        let test_str = "ה֯";
        let test_str_filtered = remove_hbr_mark(test_str);
        assert_eq!(test_str_filtered.as_ref(), "ה");
    }

    #[test]
    pub fn rem_hbr_point() {
        let test_str = "בָ";
        let test_str_filtered = remove_hbr_point(test_str);
        assert_eq!(test_str_filtered.as_ref(), "ב");
    }

    #[test]
    pub fn rem_hbr_point_vowel() {
        let test_str = "ִל";
        let test_str_filtered = remove_hbr_point_vowel(test_str);
        assert_eq!(test_str_filtered.as_ref(), "ל");
    }

    #[test]
    pub fn rem_hbr_point_semi_vowel() {
        let test_str = "ֲדְגֱכֳע";
        let test_str_filtered = remove_hbr_point_semi_vowel(test_str);
        assert_eq!(test_str_filtered.as_ref(), "דגכע");
    }

    #[test]
    pub fn rem_hbr_point_reading_sign() {
        let test_str = "םּ";
        let test_str_filtered = remove_hbr_point_reading_sign(test_str);
        assert_eq!(test_str_filtered.as_ref(), "ם");
    }

    #[test]
    pub fn rem_hbr_punctuation() {
        let test_str = "וַֽיְהִי־בֹ֖קֶר";
        let test_str_filtered = remove_hbr_punctuation(test_str);
        assert_eq!(test_str_filtered.as_ref(), "וַֽיְהִיבֹ֖קֶר");
    }

    #[test]
    pub fn rem_hbr_consonant() {
        let test_str = "AאBףC";
        let test_str_filtered = remove_hbr_consonant(test_str);
        assert_eq!(test_str_filtered.as_ref(), "ABC");
    }

    #[test]
    pub fn rem_hbr_consonant_normal() {
        let test_str = "AאBףC";
        let test_str_filtered = remove_hbr_consonant_normal(test_str);
        assert_eq!(test_str_filtered.as_ref(), "ABףC");
    }

    #[test]
    pub fn rem_hbr_consonant_final() {
        let test_str = "AאBףC";
        let test_str_filtered = remove_hbr_consonant_final(test_str);
        assert_eq!(test_str_filtered.as_ref(), "AאBC");
    }

    #[test]
    pub fn rem_hbr_yod_triangle() {
        let test_str = format!("A{}Z", '\u{05EF}');
        let test_str_filtered = remove_hbr_yod_triangle(&test_str);
        assert_eq!(test_str_filtered.as_ref(), "AZ");
    }

    #[test]
    pub fn rem_hbr_ligature_yiddish() {
        let test_str = "XװױײZ";
        let test_str_filtered = remove_hbr_ligature_yiddish(test_str);
        assert_eq!(test_str_filtered.as_ref(), "XZ");
    }
}
