# Hebrew_Unicode_Utils
## Table of contents <a name="toc"></a>
- [Hebrew\_Unicode\_Utils](#hebrew_unicode_utils)
  - [Table of contents ](#table-of-contents-)
  - [Introduction](#introduction)
  - [Description ](#description-)
    - [Notes](#notes)
  - [Examples](#examples)
    - [Removing](#removing)
    - [Showing](#showing)
    - [Statistics](#statistics)
  - [Safety ](#safety-)
  - [Panics ](#panics-)
  - [Errors ](#errors-)
  - [License ](#license-)
  - [Contribution ](#contribution-)

## Introduction

**This current readme gives a rough overview of the functionality as I currently envision it and functions as a model for implentation!**

I know the target audience for this crate is small, but perhaps there are others who are interested and have ideas and/or wishes they would like to see applied to this crate.   
Please let me know, so that we can discuss whether your ideas are suitable and feasible for this crate. 

Note: This section will be updated or deleted over time.

## Description <a name="description"></a>

This crate (*hebrew_unicode_utils*) is a library written in Rust and can be used for editing strings which contains Hebrew characters. It is built on top of the low-level crate *hebrew_unicode_script*.

Functionality of this crate will only focus on the [Unicode Block Hebrew](https://www.unicode.org/charts/PDF/U0590.pdf).

The types of functionality of this library can be captured in the following four categories:

1. **Removing**

   *This is about removing a certain set of Hebrew character types from a string.*


2. **Showing**
   
   *This category is all about showing a particular type of Hebrew characters, for example, only vowel characters.*

   *The idea behind this is that this could help people (who want to learn Hebrew) to distinguish the different characters.*

   *Note:*  
   *Consonants will always be shown in combination with e.g. vowel characters, otherwise the readability will decrease. For example, if there are multiple vowels in one sentence, then if there are no consonants  shown, then all vowels will be displayed on top of each other. Which would make the sentence unreadable.*


4. **Statistics**
   
   *This category contains functionality that gives the user information about the particular statistics of a text string.*

   *For example, what Hebrew character types are in my text string?*

### Notes

- Vowels are sometimes called *Hebrew Points*
- Accents are sometimes called *Hebrew Cantilationmarks*
- Sof Pasuq (סוֹף פָּסוּק) U+05C3  => **׃׃**
- Maqaf (מַקָּף) U+05be => **־**

For an overview of released versions see [releases](https://github.com/Roestdev/hebrew_unicode_utils/releases).

[^ TOC](#toc)

## Examples

### Removing

```rust
use hebrew_unicode_utils::remove_hbr_ligature_yiddish;
    
let test_str = "XװױײZ";
let test_str_filtered = remove_hbr_ligature_yiddish(test_str);

assert_eq!(test_str_filtered.as_ref(),"XZ");
```

```rust   
use hebrew_unicode_utils::remove_hbr_accent;

let test_str = "בְּרֵאשִׁ֖ית";
let test_str_filtered = remove_hbr_accent(test_str);

assert_eq!(test_str_filtered.as_ref(), "בְּרֵאשִׁית");
```
### Showing

```rust   
use hebrew_unicode_utils::show_hbr_mark;

let input_str = "Q מִצְרָ֑יְמָה ה֯";
let input_str_showed = show_hbr_mark(input_str);
        
assert_eq!(input_str_showed.as_ref(), "Q מצרימה ה֯");
```

```rust   
use hebrew_unicode_utils::show_hbr_point_semi_vowel;

let input_str = "ֲדְ נָפֶשׁ גֱכֳע";
let input_str_showed = show_hbr_point_semi_vowel(input_str);

assert_eq!(input_str_showed.as_ref(), "ֲדְ נפש גֱכֳע");
    
```

### Statistics

```rust   
use hebrew_unicode_utils::get_hbr_character_frequency;
    
let input_string = "Xבהב";
let freq_map = get_hbr_character_frequency(input_string);
assert_eq!(freq_map.contains_key("X"), false);
assert_eq!(freq_map.get(&"ב".to_string()), Some(&2));  
assert_eq!(freq_map.get(&"ה".to_string()), Some(&1));
```

```rust   
use hebrew_unicode_utils::get_hbr_character_types;
    
let input_string = "Xבהב";
let type_struct = get_hbr_character_types(input_string);
assert_eq!(type_struct.accent, false);
assert_eq!(type_struct.consonant, true);
assert_eq!(type_struct.non_hebrew, true);
```

## Safety <a name="safety"></a>

All functions are written in safe Rust.


[^ TOC](#toc)

## Panics <a name="panics"></a>

Todo

[^ TOC](#toc)

## Errors <a name="errors"></a>

ToDo

[^ TOC](#toc)

## License <a name="license"></a>

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

[^ TOC](#toc)

## Contribution <a name="contribution"></a>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[^ TOC](#toc)

This crate has been inspired by [niqqud](https://crates.io/crates/niqqud)