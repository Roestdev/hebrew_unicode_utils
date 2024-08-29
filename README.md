# Hebrew_Unicode_Utils
## Table of contents <a name="toc"></a>
- [Hebrew\_Unicode\_Utils](#hebrew_unicode_utils)
  - [Table of contents ](#table-of-contents-)
  - [Introduction](#introduction)
  - [Description ](#description-)
    - [Notes](#notes)
  - [Safety ](#safety-)
  - [Panics ](#panics-)
  - [Errors ](#errors-)
  - [License ](#license-)
  - [Contribution ](#contribution-)

## Introduction

**This current readme gives a rough overview of the functionality as I currently envision it and functions as a model for implentation!**

I know the target audience for this crate is small, but perhaps there are others who are interested and have ideas and/or wishes they would like to see applied to this crate.   
Please let me know so that we can discuss whether your ideas are suitable and feasible for this crate. 

Note: This section will be updated or deleted over time.

## Description <a name="description"></a>

This crate (*hebrew_unicode_utils*) is a library written in Rust and can be used for editing strings which contains Hebrew characters. It is built on top of the low-level crate *hebrew_unicode_script*.

Functionality of this crate will only focus on the [Unicode Block Hebrew](https://www.unicode.org/charts/PDF/U0590.pdf).

The types of functionality of this library can be captured in the following four categories:

1. **Removing**

   *This is about removing a certain set of Hebrew character types from a string.*
<br><br>
2. **Showing**
   
   *This category is all about showing a particular type of Hebrew characters, for example, only vowel characters.*

   *The idea behind this is that this can help people who want to learn Hebrew learn to distinguish the different characters.*

   **Note:**  
   Consonants will always be shown in combination with e.g. vowel characters.
<br><br>
3. **Current practices**
   
   *On the Internet, certain ways have already been established for displaying Hebrew text.* 
   *The idea is to include functionality that is already used by various websites.*

   *One example is the [BlueLetterBible](https://www.blueletterbible.org/wlc/gen/1/1/s_1001), where it is possible to show  cantilation marks and or vowel points on top of the consonants.* 

   The implementation usually consists of the following components:
   - **Consonants**: Show only consonants with the Maqqef's and Sof-Pasuq's.   
   -  **Vowels:** Show *consonants* + all *vowels*.   
   -  **Accents**: Show *Vowels* + all *accents.*
<br><br>
4. **Statistics**
   
   This category contains functionality that gives the user information about the particular statistics of a text string.

   For example, what Hebrew character types are in my text string? 

### Notes

- Vowels are sometimes called **Hebrew Points**
- Accents are sometimes called **Hebrew Cantilationmarks**

<br>

For an overview of released versions see [releases](https://github.com/Roestdev/hebrew_unicode_utils/releases).

[^ TOC](#toc)

## Safety <a name="safety"></a>

All functions are written in safe Rust.


[^ TOC](#toc)

## Panics <a name="panics"></a>

Not that I am aware of.

[^ TOC](#toc)

## Errors <a name="errors"></a>

All (trait)functions return either true *or* false.

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