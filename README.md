# Hebrew_Unicode_Utils

![Crates.io License](https://img.shields.io/crates/l/hebrew_unicode_utils)
![Crates.io Version](https://img.shields.io/crates/v/hebrew_unicode_utils)
![docs.rs](https://img.shields.io/docsrs/hebrew_unicode_utils)
[![Build & Test](https://github.com/Roestdev/hebrew_unicode_utils/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/Roestdev/hebrew_unicode_utils/actions/workflows/build_and_test.yml)
[![Clippy Analyze](https://github.com/Roestdev/hebrew_unicode_utils/actions/workflows/clippy_analyze.yml/badge.svg)](https://github.com/Roestdev/hebrew_unicode_utils/actions/workflows/clippy_analyze.yml)

## Project Status 

**This project is currently in the development mode.**

`Current Version:`   
The latest version is v0.4.3. 

`Stability:`   
The crate is currently in development mode and has **not** undergone thorough testing, making it unsuitable for production use at this time. However, it is expected to evolve in the future.

Despite its experimental nature, all released versions available on GitHub are guaranteed to pass the following Cargo sub-commands without any issues:
``` txt
cargo fmt
cargo check
cargo clippy
cargo build
cargo doc
cargo test
```

`Updates:`   
While the project is not actively seeking new features or major enhancements, critical bug fixes and security updates will be addressed as needed. Users are encouraged to report any issues they encounter.

## Description

This crate (*hebrew_unicode_utils*) is a library written in Rust and is built on top of the low-level crate `hebrew_unicode_script`. This crate will focus on the *Unicode Hebrew Block* and the *Hebrew accent systems*.

The functionality of this library can be broadly categorized into the following types:

 - **Removing**

   This is about removing a certain set of Hebrew character types from a string.  

 - **Showing**

   This category is all about showing a particular type of Hebrew character.

   The idea behind this is that this could help people (who want to learn Hebrew) to distinguish the different characters.

   *Note:*  
   Consonants will always be shown in combination with e.g. vowel characters, otherwise the readability will decrease. For example, if there are multiple vowels in one sentence, then if there are no consonants  shown, then all vowels will be displayed on top of each other. Which would make the sentence unreadable.  

 - **Statistics**

   This category contains functionality that gives the user information about the particular statistics of a text string.

   For example an answer on the following question: "What Hebrew character types are in my text string?" 

 - **Meteg Layout**
   
   There are some characters that are used inside the Hebrew text for correct rendering in documents and websites when displaying text.

   This category contains the functionality to determine if a character is a meteg layout character or not. 

   Supported characters are: ZWNJ (zero width non-joiner), ZWJ (zero width joiner) and CGJ (combining grapheme joiner)

 - **Accents**
   
   This category facilitates the identification and validation of Hebrew accents as used in the Tanach. Both a check on individual accents and membership of collections are possible.

## Accent system in the Tanach

- based on genre
- disjunt versus conju
- hiarchy groups
- sometine consists of two unicode code-points
- some accents

## Examples

### Removing characters

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
### Showing characters

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
use hebrew_unicode_utils::hebrew_character_frequency;
    
let input_string = "Xבהב";
let freq_map = hebrew_character_frequency(input_string);
assert_eq!(freq_map.get(&"X".to_string()), None);
assert_eq!(freq_map.get(&"ב".to_string()), Some(&2));  
assert_eq!(freq_map.get(&"ה".to_string()), Some(&1));
```

```rust   
use hebrew_unicode_utils::hebrew_character_types;
use hebrew_unicode_utils::IsPresent;
    
let input_string = "Xבהב";
let type_struct = hebrew_character_types(input_string);
assert_eq!(type_struct.accent, IsPresent::No);
assert_eq!(type_struct.consonant, IsPresent::Yes);
assert_eq!(type_struct.non_hebrew, IsPresent::Yes);
```

### TODO Meteg layout characters

```rust   
use hebrew_unicode_utils::hebrew_character_types;
use hebrew_unicode_utils::IsPresent;
    
let input_string = "Xבהב";
let type_struct = hebrew_character_types(input_string);
assert_eq!(type_struct.accent, IsPresent::No);
assert_eq!(type_struct.consonant, IsPresent::Yes);
assert_eq!(type_struct.non_hebrew, IsPresent::Yes);
```

###  TODO Accents - 21 books


```rust   
use hebrew_unicode_utils::hebrew_character_types;
use hebrew_unicode_utils::IsPresent;
    
let input_string = "Xבהב";
let type_struct = hebrew_character_types(input_string);
assert_eq!(type_struct.accent, IsPresent::No);
assert_eq!(type_struct.consonant, IsPresent::Yes);
assert_eq!(type_struct.non_hebrew, IsPresent::Yes);
```
###  TODO Accents - 3 books


```rust   
use hebrew_unicode_utils::hebrew_character_types;
use hebrew_unicode_utils::IsPresent;
    
let input_string = "Xבהב";
let type_struct = hebrew_character_types(input_string);
assert_eq!(type_struct.accent, IsPresent::No);
assert_eq!(type_struct.consonant, IsPresent::Yes);
assert_eq!(type_struct.non_hebrew, IsPresent::Yes);
```

## Release

For an overview of released versions see [releases](https://github.com/Roestdev/hebrew_unicode_utils/releases).   


## Install

For installation see the [hebrew_unicode_utils](https://crates.io/crates/hebrew_unicode_utils) page at crates.io.

## Safety

All functions are written in safe Rust.

## Panics

No panics for so far I know of.

## Errors

All functions return either a *Cow*, a *Struct* or a *HashMap*.

## License

The `hebrew_unicode_utils` library is distributed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or
   <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

## References

 - [Unicode Block Hebrew - chapter 9.1](https://www.unicode.org/charts/PDF/U0590.pdf) (meteg)
  
 - [Basics of HEBREW ACCENTS](https://zondervanacademic.com/products/basics-of-hebrew-accents), written by Mark D. Futato, Sr.

 - [Hebrew Cantillation Marks And Their Encoding](https://mechon-mamre.org/c/hr/index.htm) by Helmut Richter
  
 - [Gesenius Hebrew Grammar - §15. The Accents.](https://en.wikisource.org/wiki/Gesenius%27_Hebrew_Grammar/15._The_Accents)


## Notes
 - Vowels are sometimes called *Hebrew Points*
 - Accents are sometimes called *Hebrew Cantilationmarks*
 
This crate has been inspired by [niqqud](https://crates.io/crates/niqqud)