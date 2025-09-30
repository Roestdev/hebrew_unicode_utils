pub trait HebrewMetegJoinerCharacters {
    //fn is_ascii_space(&self) -> bool;
    fn is_zero_width_non_joiner(&self) -> bool;
    fn is_zero_width_joiner(&self) -> bool;
    fn is_combining_grapheme_joiner(&self) -> bool;
}

impl HebrewMetegJoinerCharacters for char {
    // source:: collections_api.rs
    // module:: Unicode Script Hebrew

    // fn is_ascii_space(&self) -> bool {
    //     is_ascii_space(*self)
    // }

    fn is_zero_width_joiner(&self) -> bool {
        is_zero_width_joiner(*self)
    }

    fn is_zero_width_non_joiner(&self) -> bool {
        is_zero_width_non_joiner(*self)
    }

    fn is_combining_grapheme_joiner(&self) -> bool {
        is_combining_grapheme_joiner(*self)
    }
}

// additional characters that can be present in Hebrew

/// Checks if the given character is a: ZWNJ (zero width non-joiner)
///
/// # Example
/// ```
/// use hebrew_unicode_utils::is_zero_width_non_joiner;
///
/// let char = '\u{200C}';
/// assert!(is_zero_width_non_joiner(char));
/// 
/// let char = '\u{200D}';
/// assert!(!is_zero_width_non_joiner(char));
/// 
/// 
/// ```
pub fn is_zero_width_non_joiner(c: char) -> bool {
    // U+200C
    matches!(c, '\u{200C}')
}

/// Checks if the given character is a: ZWJ (zero width joiner)
///
/// # Example
/// ```
/// use hebrew_unicode_utils::is_zero_width_joiner;
///
/// let char = '\u{200D}';
/// assert!(is_zero_width_joiner(char));
/// 
/// let char = '\u{034F}';
/// assert!(!is_zero_width_joiner(char));
/// 
/// 
/// ```
pub fn is_zero_width_joiner(c: char) -> bool {
    // U+200D
    matches!(c, '\u{200D}')
}

/// Checks if the given character is a: CGJ (combining grapheme joiner)
///
/// # Example
/// ```
/// use hebrew_unicode_utils::is_combining_grapheme_joiner;
///
/// let consonant = '\u{034F}';
/// assert!(is_combining_grapheme_joiner(consonant));
/// 
/// let consonant = '\u{200C}';
/// assert!(!is_combining_grapheme_joiner(consonant));
/// 
/// 
/// 
/// ```
pub fn is_combining_grapheme_joiner(c: char) -> bool {
    // U+034F
    matches!(c, '\u{034F}')
}
