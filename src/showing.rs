pub mod unicode_block_hebrew {
    use hebrew_unicode_script::HebrewUnicodeScript;
    use std::borrow::Cow;

    /// Shows all Hebrew acccents that are found in the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::show_hbr_accent;
    ///
    /// let input_str = "לַ֭מְנַצֵּחַ";
    /// let input_str_showed = show_hbr_accent(input_str);
    /// assert_eq!(input_str_showed.as_ref(), "ל֭מנצח");
    /// ```
    pub fn show_hbr_accent(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_block() || c.is_hbr_consonant() || c.is_hbr_accent())
            .collect()
    }

    /// Shows all Hebrew marks that are found in the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::show_hbr_mark;
    ///
    /// let input_str = "כַ a כ֯ כׄ כׅ";
    /// let input_str_showed = show_hbr_mark(input_str);
    /// assert_eq!(input_str_showed.as_ref(),"כ a כ֯ כׄ כׅ");
    /// ```
    pub fn show_hbr_mark(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_block() || c.is_hbr_consonant() || c.is_hbr_mark())
            .collect()
    }
    /// Shows all Hebrew points that are found in the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::show_hbr_point;
    ///
    /// let input_str = "חֲ֝קַרְתַּ֗נִי";
    /// let input_str_showed = show_hbr_point(input_str);
    /// assert_eq!(input_str_showed.as_ref(),"חֲקַרְתַּנִי");
    /// ```
    pub fn show_hbr_point(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_block() || c.is_hbr_consonant() || c.is_hbr_point())
            .collect()
    }

    /// Shows all Hebrew point vowels that are found in the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::show_hbr_point_vowel;
    ///
    /// let input_str = "חֲ֝קַרְתַּ֗נִי";
    /// let input_str_showed = show_hbr_point_vowel(input_str);
    /// assert_eq!(input_str_showed.as_ref(),"חקַרתַנִי");
    /// ```
    pub fn show_hbr_point_vowel(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_block() || c.is_hbr_consonant() || c.is_hbr_point_vowel())
            .collect()
    }
    /// Shows all Hebrew point semi-vowels that are found in the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::show_hbr_point_semi_vowel;
    ///
    /// let input_str = "abc כֳ כֲ כֱ כְ";
    /// let input_str_showed = show_hbr_point_semi_vowel(input_str);
    /// assert_eq!(input_str_showed.as_ref(),"abc כֳ כֲ כֱ כְ");
    /// ```
    pub fn show_hbr_point_semi_vowel(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_block() || c.is_hbr_consonant() || c.is_hbr_point_semi_vowel())
            .collect()
    }

    /// Shows all Hebrew point reading signs that are found in the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::show_hbr_point_reading_sign;
    ///
    /// let input_str = "אִם־אֶסַּק";
    /// let input_str_showed = show_hbr_point_reading_sign(input_str);
    /// assert_eq!(input_str_showed.as_ref(),"אםאסּק");
    /// ```
    pub fn show_hbr_point_reading_sign(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_block() || c.is_hbr_consonant() || c.is_hbr_point_reading_sign())
            .collect()
    }
    /// Shows all Hebrew punctuations that are found in the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::show_hbr_punctuation;
    ///
    /// let input_str = "וַֽיְהִי־בֹ֖קֶר";
    /// let input_str_showed = show_hbr_punctuation(input_str);
    /// assert_eq!(input_str_showed.as_ref(),"ויהי־בקר");
    /// ```
    pub fn show_hbr_punctuation(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_block() || c.is_hbr_consonant() || c.is_hbr_punctuation())
            .collect()
    }
    /// Shows all Hebrew letters (final and normal) that are found in the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::show_hbr_consonant;
    ///
    /// let input_str = "A ֱאB ֟ףC";
    /// let input_str_showed = show_hbr_consonant(input_str);
    /// assert_eq!(input_str_showed.as_ref(),"A אB ףC");
    /// ```
    pub fn show_hbr_consonant(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_block() || c.is_hbr_consonant())
            .collect()
    }
    /// Shows all Hebrew normal letters that are found in the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::show_hbr_consonant_normal;
    ///
    /// let input_str = "AאB ףC";
    /// let input_str_showed = show_hbr_consonant_normal(input_str);
    /// assert_eq!(input_str_showed.as_ref(),"AאB C");
    /// ```
    pub fn show_hbr_consonant_normal(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_block() || c.is_hbr_consonant_normal())
            .collect()
    }
    /// Shows all Hebrew final letters that are found in the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::show_hbr_consonant_final;
    ///
    /// let input_str = "A אB ףC";
    /// let input_str_showed = show_hbr_consonant_final(input_str);
    /// assert_eq!(input_str_showed.as_ref(),"A B ףC");
    /// ```
    pub fn show_hbr_consonant_final(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_block() || c.is_hbr_consonant_final())
            .collect()
    }
    /// Shows all Hebrew yod triangles that are found in the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::show_hbr_yod_triangle;
    ///
    /// let input_str = format!("A{}Z לּ",'\u{05EF}');
    /// let input_str_showed = show_hbr_yod_triangle(&input_str);
    /// let expected_result = format!("A{}Z ל",'\u{05EF}');
    /// assert_eq!(input_str_showed.as_ref(),expected_result);
    /// ```
    pub fn show_hbr_yod_triangle(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_block() || c.is_hbr_consonant() || c.is_hbr_yod_triangle())
            .collect()
    }
    /// Shows all Yiddish ligatures that are found in the given string.
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_utils::show_hbr_ligature_yiddish;
    ///
    /// let input_str = "X װױײ Z כֱ";
    /// let input_str_showed = show_hbr_ligature_yiddish(input_str);
    /// assert_eq!(input_str_showed.as_ref(),"X װױײ Z כ");
    /// ```
    pub fn show_hbr_ligature_yiddish(string: &str) -> Cow<'_, str> {
        string
            .chars()
            .filter(|&c| !c.is_hbr_block() || c.is_hbr_consonant() || c.is_hbr_ligature_yiddish())
            .collect()
    }
}
#[cfg(test)]
mod unit_test {
    use crate::*;
    // #[test]
    // fn test_showing_hbr_block() {
    //     let input_str = "וַֽיְחִי־שֵׁ֗םאַֽחֲרֵי֙הוֹלִיד֣וֹת׃";
    //     let input_str_showed = show_hbr_block(input_str);
    //     assert_eq!(input_str_showed.as_ref(), "");
    // }

    #[test]
    fn test_showing_hbr_accent() {
        let input_str = "בְּרֵאשִׁ֖ית";
        let input_str_showed = show_hbr_accent(input_str);
        assert_eq!(input_str_showed.as_ref(), "בראש֖ית");
    }

    #[test]
    pub fn test_showing_hbr_mark() {
        let input_str = "Q מִצְרָ֑יְמָה ה֯";
        let input_str_showed = show_hbr_mark(input_str);
        assert_eq!(input_str_showed.as_ref(), "Q מצרימה ה֯");
    }

    #[test]
    pub fn test_showing_hbr_point() {
        let input_str = "שִׁבְעִ֣ים";
        let input_str_showed = show_hbr_point(input_str);
        assert_eq!(input_str_showed.as_ref(), "שִׁבְעִים");
    }

    #[test]
    pub fn test_showing_hbr_point_vowel() {
        let input_str = "שִׁבְעִ֣יםֱ";
        let input_str_showed = show_hbr_point_vowel(input_str);
        assert_eq!(input_str_showed.as_ref(), "שִבעִים");
    }

    #[test]
    pub fn test_showing_hbr_point_semi_vowel() {
        let input_str = "ֲדְ נָפֶשׁ גֱכֳע";
        let input_str_showed = show_hbr_point_semi_vowel(input_str);
        assert_eq!(input_str_showed.as_ref(), "ֲדְ נפש גֱכֳע");
    }

    #[test]
    pub fn test_showing_hbr_point_reading_sign() {
        let input_str = "נִתְחַכְּמָה";
        let input_str_showed = show_hbr_point_reading_sign(input_str);
        assert_eq!(input_str_showed.as_ref(), "נתחכּמה");
    }

    #[test]
    pub fn test_showing_hbr_punctuation() {
        let input_str = "וַֽיְהִי־בֹ֖קֶר";
        let input_str_showed = show_hbr_punctuation(input_str);
        assert_eq!(input_str_showed.as_ref(), "ויהי־בקר");
    }

    #[test]
    pub fn test_showing_hbr_consonant() {
        let input_str = "A ֱאB ֵףC";
        let input_str_showed = show_hbr_consonant(input_str);
        assert_eq!(input_str_showed.as_ref(), "A אB ףC");
    }

    #[test]
    pub fn test_showing_hbr_consonant_normal() {
        let input_str = "A ֱאB ָףC";
        let input_str_showed = show_hbr_consonant_normal(input_str);
        assert_eq!(input_str_showed.as_ref(), "A אB C");
    }

    #[test]
    pub fn test_showing_hbr_consonant_final() {
        let input_str = "A ֱאB ָףC";
        let input_str_showed = show_hbr_consonant_final(input_str);
        assert_eq!(input_str_showed.as_ref(), "A B ףC");
    }

    #[test]
    pub fn test_showing_hbr_yod_triangle() {
        let input_str = format!("A הָבָה {} Z", '\u{05EF}');
        let input_str_showed = show_hbr_yod_triangle(&input_str);
        let expected_output = format!("A הבה {} Z", '\u{05EF}');
        assert_eq!(input_str_showed.as_ref(), expected_output);
    }

    #[test]
    pub fn test_showing_hbr_ligature_yiddish() {
        let input_str = "X נִֽתְחַכְּמָ֖ה װױײ Z";
        let input_str_showed = show_hbr_ligature_yiddish(input_str);
        assert_eq!(input_str_showed.as_ref(), "X נתחכמה װױײ Z");
    }
}
