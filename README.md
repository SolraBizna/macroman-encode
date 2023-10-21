# macroman-encode

## What

This crate provides an iterator that ingests a UTF-8 string and yields
MacRoman code points.

## Why

Because I don't like any word processor that's younger than I am.

I'm writing a word processor that's hyper-tailored to my needs and tastes.
One of the quirks this gives it is that it uses old Macintosh bitmap fonts
for display. Those fonts are for MacRoman, so I need the ability to convert
Unicode text into a sequence of MacRoman code points in order to know what
glyphs to draw.

## You're insane

Yeah

## What about…

- Composed vs decomposed: Both forms of supported characters are supported.
- ¤ vs €: Both characters are converted as $DB; which one is correct
  depends on whether your font predates Mac OS 8.5.
- Ω (capital omega) vs Ω (ohm sign): Both are converted as $BD. The
  question of which is correct only arises when converting *to* Unicode,
  not from it.
- The Apple symbol: Apple uses U+F8FF, a character in the Corporate Private
  Use Area, to represent its logo in text. We comply with this usage.
- Unsupported characters: If the crate encounters a Unicode code sequence
  for which it can't find a MacRoman-encodable prefix, it will yield a
  `None`, step by one code point, and try again.

## Legalese

MacRoman-Encode is copyright 2023, Solra Bizna, and licensed under either
of:

* Apache License, Version 2.0
([LICENSE-APACHE](LICENSE-APACHE) or
<http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the MacRoman-Encode crate by you, as defined
in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
