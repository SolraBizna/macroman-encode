#![cfg_attr(not(test), no_std)]

//! # What
//!
//! This crate provides an iterator that ingests a UTF-8 string and yields
//! MacRoman code points.
//!
//! # Why
//!
//! Because I don't like any word processor that's younger than I am.
//!
//! I'm writing a word processor that's hyper-tailored to my needs and tastes.
//! One of the quirks this gives it is that it uses old Macintosh bitmap fonts
//! for display. Those fonts are for MacRoman, so I need the ability to convert
//! Unicode text into a sequence of MacRoman code points in order to know what
//! glyphs to draw.
//!
//! # You're insane
//!
//! Yeah
//!
//! # What about…
//!
//! - Composed vs decomposed: Both forms of supported characters are supported.
//! - ¤ vs €: Both characters are converted as $DB; which one is correct
//!   depends on whether your font predates Mac OS 8.5.
//! - Ω (capital omega) vs Ω (ohm sign): Both are converted as $BD. The
//!   question of which is correct only arises when converting *to* Unicode,
//!   not from it.
//! - The Apple symbol: Apple uses U+F8FF, a character in the Corporate Private
//!   Use Area, to represent its logo in text. We comply with this usage.
//! - Unsupported characters: If the crate encounters a Unicode code sequence
//!   for which it can't find a MacRoman-encodable prefix, it will yield an
//!   `Err(codepoint)`, step by one code point, and try again.
//!
//! # Legalese
//!
//! MacRoman-Encode is copyright 2023, Solra Bizna, and licensed under either
//! of:
//!
//! * Apache License, Version 2.0
//! ([LICENSE-APACHE](LICENSE-APACHE) or
//! <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license
//! ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! Unless you explicitly state otherwise, any contribution intentionally
//! submitted for inclusion in the MacRoman-Encode crate by you, as defined
//! in the Apache-2.0 license, shall be dual licensed as above, without any
//! additional terms or conditions.

static KNOWN_SEQUENCES: &[(&str, u8)] = &[
    ("\u{0000}", 0),
    ("\u{0001}", 1),
    ("\u{0002}", 2),
    ("\u{0003}", 3),
    ("\u{0004}", 4),
    ("\u{0005}", 5),
    ("\u{0006}", 6),
    ("\u{0007}", 7),
    ("\u{0008}", 8),
    ("\u{0009}", 9),
    ("\u{000A}", 10),
    ("\u{000B}", 11),
    ("\u{000C}", 12),
    ("\u{000D}", 13),
    ("\u{000E}", 14),
    ("\u{000F}", 15),
    ("\u{0010}", 16),
    ("\u{0011}", 17),
    ("\u{0012}", 18),
    ("\u{0013}", 19),
    ("\u{0014}", 20),
    ("\u{0015}", 21),
    ("\u{0016}", 22),
    ("\u{0017}", 23),
    ("\u{0018}", 24),
    ("\u{0019}", 25),
    ("\u{001A}", 26),
    ("\u{001B}", 27),
    ("\u{001C}", 28),
    ("\u{001D}", 29),
    ("\u{001E}", 30),
    ("\u{001F}", 31),
    ("\u{0020}", 32),
    ("\u{0021}", 33),
    ("\u{0022}", 34),
    ("\u{0023}", 35),
    ("\u{0024}", 36),
    ("\u{0025}", 37),
    ("\u{0026}", 38),
    ("\u{0027}", 39),
    ("\u{0028}", 40),
    ("\u{0029}", 41),
    ("\u{002A}", 42),
    ("\u{002B}", 43),
    ("\u{002C}", 44),
    ("\u{002D}", 45),
    ("\u{002E}", 46),
    ("\u{002F}", 47),
    ("\u{0030}", 48),
    ("\u{0031}", 49),
    ("\u{0032}", 50),
    ("\u{0033}", 51),
    ("\u{0034}", 52),
    ("\u{0035}", 53),
    ("\u{0036}", 54),
    ("\u{0037}", 55),
    ("\u{0038}", 56),
    ("\u{0039}", 57),
    ("\u{003A}", 58),
    ("\u{003B}", 59),
    ("\u{003C}", 60),
    ("\u{003D}", 61),
    ("\u{003E}", 62),
    ("\u{003F}", 63),
    ("\u{0040}", 64),
    ("\u{0041}", 65),
    ("\u{0041}\u{0300}", 203),
    ("\u{0041}\u{0301}", 231),
    ("\u{0041}\u{0302}", 229),
    ("\u{0041}\u{0303}", 204),
    ("\u{0041}\u{0308}", 128),
    ("\u{0041}\u{030A}", 129),
    ("\u{0042}", 66),
    ("\u{0043}", 67),
    ("\u{0043}\u{0327}", 130),
    ("\u{0044}", 68),
    ("\u{0045}", 69),
    ("\u{0045}\u{0300}", 233),
    ("\u{0045}\u{0301}", 131),
    ("\u{0045}\u{0302}", 230),
    ("\u{0045}\u{0308}", 232),
    ("\u{0046}", 70),
    ("\u{0047}", 71),
    ("\u{0048}", 72),
    ("\u{0049}", 73),
    ("\u{0049}\u{0300}", 237),
    ("\u{0049}\u{0301}", 234),
    ("\u{0049}\u{0302}", 235),
    ("\u{0049}\u{0308}", 236),
    ("\u{004A}", 74),
    ("\u{004B}", 75),
    ("\u{004C}", 76),
    ("\u{004D}", 77),
    ("\u{004E}", 78),
    ("\u{004E}\u{0303}", 132),
    ("\u{004F}", 79),
    ("\u{004F}\u{0300}", 241),
    ("\u{004F}\u{0301}", 238),
    ("\u{004F}\u{0302}", 239),
    ("\u{004F}\u{0303}", 205),
    ("\u{004F}\u{0308}", 133),
    ("\u{0050}", 80),
    ("\u{0051}", 81),
    ("\u{0052}", 82),
    ("\u{0053}", 83),
    ("\u{0054}", 84),
    ("\u{0055}", 85),
    ("\u{0055}\u{0300}", 244),
    ("\u{0055}\u{0301}", 242),
    ("\u{0055}\u{0302}", 243),
    ("\u{0055}\u{0308}", 134),
    ("\u{0056}", 86),
    ("\u{0057}", 87),
    ("\u{0058}", 88),
    ("\u{0059}", 89),
    ("\u{0059}\u{0308}", 217),
    ("\u{005A}", 90),
    ("\u{005B}", 91),
    ("\u{005C}", 92),
    ("\u{005D}", 93),
    ("\u{005E}", 94),
    ("\u{005F}", 95),
    ("\u{0060}", 96),
    ("\u{0061}", 97),
    ("\u{0061}\u{0300}", 136),
    ("\u{0061}\u{0301}", 135),
    ("\u{0061}\u{0302}", 137),
    ("\u{0061}\u{0303}", 139),
    ("\u{0061}\u{0308}", 138),
    ("\u{0061}\u{030A}", 140),
    ("\u{0062}", 98),
    ("\u{0063}", 99),
    ("\u{0063}\u{0327}", 141),
    ("\u{0064}", 100),
    ("\u{0065}", 101),
    ("\u{0065}\u{0300}", 143),
    ("\u{0065}\u{0301}", 142),
    ("\u{0065}\u{0302}", 144),
    ("\u{0065}\u{0308}", 145),
    ("\u{0066}", 102),
    ("\u{0067}", 103),
    ("\u{0068}", 104),
    ("\u{0069}", 105),
    ("\u{0069}\u{0300}", 147),
    ("\u{0069}\u{0301}", 146),
    ("\u{0069}\u{0302}", 148),
    ("\u{0069}\u{0308}", 149),
    ("\u{006A}", 106),
    ("\u{006B}", 107),
    ("\u{006C}", 108),
    ("\u{006D}", 109),
    ("\u{006E}", 110),
    ("\u{006E}\u{0303}", 150),
    ("\u{006F}", 111),
    ("\u{006F}\u{0300}", 152),
    ("\u{006F}\u{0301}", 151),
    ("\u{006F}\u{0302}", 153),
    ("\u{006F}\u{0303}", 155),
    ("\u{006F}\u{0308}", 154),
    ("\u{0070}", 112),
    ("\u{0071}", 113),
    ("\u{0072}", 114),
    ("\u{0073}", 115),
    ("\u{0074}", 116),
    ("\u{0075}", 117),
    ("\u{0075}\u{0300}", 157),
    ("\u{0075}\u{0301}", 156),
    ("\u{0075}\u{0302}", 158),
    ("\u{0075}\u{0308}", 159),
    ("\u{0076}", 118),
    ("\u{0077}", 119),
    ("\u{0078}", 120),
    ("\u{0079}", 121),
    ("\u{0079}\u{0308}", 216),
    ("\u{007A}", 122),
    ("\u{007B}", 123),
    ("\u{007C}", 124),
    ("\u{007D}", 125),
    ("\u{007E}", 126),
    ("\u{007F}", 127),
    ("\u{00A0}", 202),
    ("\u{00A1}", 193),
    ("\u{00A2}", 162),
    ("\u{00A3}", 163),
    ("\u{00A4}", 219),
    ("\u{00A5}", 180),
    ("\u{00A7}", 164),
    ("\u{00A8}", 172),
    ("\u{00A9}", 169),
    ("\u{00AA}", 187),
    ("\u{00AB}", 199),
    ("\u{00AC}", 194),
    ("\u{00AE}", 168),
    ("\u{00AF}", 248),
    ("\u{00B0}", 161),
    ("\u{00B1}", 177),
    ("\u{00B4}", 171),
    ("\u{00B5}", 181),
    ("\u{00B6}", 166),
    ("\u{00B7}", 225),
    ("\u{00B8}", 252),
    ("\u{00BA}", 188),
    ("\u{00BB}", 200),
    ("\u{00BF}", 192),
    ("\u{00C0}", 203),
    ("\u{00C1}", 231),
    ("\u{00C2}", 229),
    ("\u{00C3}", 204),
    ("\u{00C4}", 128),
    ("\u{00C5}", 129),
    ("\u{00C6}", 174),
    ("\u{00C7}", 130),
    ("\u{00C8}", 233),
    ("\u{00C9}", 131),
    ("\u{00CA}", 230),
    ("\u{00CB}", 232),
    ("\u{00CC}", 237),
    ("\u{00CD}", 234),
    ("\u{00CE}", 235),
    ("\u{00CF}", 236),
    ("\u{00D1}", 132),
    ("\u{00D2}", 241),
    ("\u{00D3}", 238),
    ("\u{00D4}", 239),
    ("\u{00D5}", 205),
    ("\u{00D6}", 133),
    ("\u{00D8}", 175),
    ("\u{00D9}", 244),
    ("\u{00DA}", 242),
    ("\u{00DB}", 243),
    ("\u{00DC}", 134),
    ("\u{00DF}", 167),
    ("\u{00E0}", 136),
    ("\u{00E1}", 135),
    ("\u{00E2}", 137),
    ("\u{00E3}", 139),
    ("\u{00E4}", 138),
    ("\u{00E5}", 140),
    ("\u{00E6}", 190),
    ("\u{00E7}", 141),
    ("\u{00E8}", 143),
    ("\u{00E9}", 142),
    ("\u{00EA}", 144),
    ("\u{00EB}", 145),
    ("\u{00EC}", 147),
    ("\u{00ED}", 146),
    ("\u{00EE}", 148),
    ("\u{00EF}", 149),
    ("\u{00F1}", 150),
    ("\u{00F2}", 152),
    ("\u{00F3}", 151),
    ("\u{00F4}", 153),
    ("\u{00F5}", 155),
    ("\u{00F6}", 154),
    ("\u{00F7}", 214),
    ("\u{00F8}", 191),
    ("\u{00F9}", 157),
    ("\u{00FA}", 156),
    ("\u{00FB}", 158),
    ("\u{00FC}", 159),
    ("\u{00FF}", 216),
    ("\u{0131}", 245),
    ("\u{0152}", 206),
    ("\u{0153}", 207),
    ("\u{0178}", 217),
    ("\u{0192}", 196),
    ("\u{02C6}", 246),
    ("\u{02C7}", 255),
    ("\u{02D8}", 249),
    ("\u{02D9}", 250),
    ("\u{02DA}", 251),
    ("\u{02DB}", 254),
    ("\u{02DC}", 247),
    ("\u{02DD}", 253),
    ("\u{03A9}", 189),
    ("\u{03C0}", 185),
    ("\u{2013}", 208),
    ("\u{2014}", 209),
    ("\u{2018}", 212),
    ("\u{2019}", 213),
    ("\u{201A}", 226),
    ("\u{201C}", 210),
    ("\u{201D}", 211),
    ("\u{201E}", 227),
    ("\u{2020}", 160),
    ("\u{2021}", 224),
    ("\u{2022}", 165),
    ("\u{2026}", 201),
    ("\u{2030}", 228),
    ("\u{2039}", 220),
    ("\u{203A}", 221),
    ("\u{2044}", 218),
    ("\u{20AC}", 219),
    ("\u{2122}", 170),
    ("\u{2126}", 189),
    ("\u{2202}", 182),
    ("\u{2206}", 198),
    ("\u{220F}", 184),
    ("\u{2211}", 183),
    ("\u{221A}", 195),
    ("\u{221E}", 176),
    ("\u{222B}", 186),
    ("\u{2248}", 197),
    ("\u{2260}", 173),
    ("\u{2264}", 178),
    ("\u{2265}", 179),
    ("\u{25CA}", 215),
    ("\u{F8FF}", 240),
    ("\u{FB01}", 222),
    ("\u{FB02}", 223),
];

struct MacRomanEncoder<'a> {
    pos: usize,
    rem: &'a str,
}

impl Iterator for MacRomanEncoder<'_> {
    type Item = (usize, usize, Result<u8, char>);
    fn next(&mut self) -> Option<(usize, usize, Result<u8, char>)> {
        if self.rem.is_empty() {
            None
        } else {
            let pos = self.pos;
            let best = match KNOWN_SEQUENCES
                .binary_search_by(|(prefix, _)| prefix.cmp(&self.rem))
            {
                Ok(x) => x,
                Err(x) => x.saturating_sub(1),
            };
            if best < KNOWN_SEQUENCES.len() {
                let (sequence, code) = KNOWN_SEQUENCES[best];
                if let Some(rest) = self.rem.strip_prefix(sequence) {
                    self.rem = rest;
                    self.pos += sequence.len();
                    return Some((pos, sequence.len(), Ok(code)));
                }
            }
            let codepoint = self.rem.chars().next().unwrap();
            let len = self
                .rem
                .char_indices()
                .nth(1)
                .map(|(i, _)| i)
                .unwrap_or(self.rem.len());
            self.rem = &self.rem[len..];
            self.pos += len;
            Some((pos, len, Err(codepoint)))
        }
    }
}

pub fn encode(
    input: &str,
) -> impl '_ + Iterator<Item = (usize, usize, Result<u8, char>)> {
    MacRomanEncoder { pos: 0, rem: input }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn quebecois_glass() {
        const SRC: &str = "J'peux manger d'la vitre, ça m'fa pas mal.";
        const DST: &[u8] = b"J'peux manger d'la vitre, \x8Da m'fa pas mal.";
        assert_eq!(
            encode(SRC)
                .map(|(_pos, _len, c)| c)
                .collect::<Result<Vec<u8>, char>>()
                .unwrap(),
            DST
        )
    }
    #[test]
    fn norse_glass() {
        const SRC: &str = "Ek get etið gler án þess að verða sár.";
        const DST: &[u8] = b"Ek get eti@ gler \x87n @ess a@ ver@a s\x87r.";
        assert_eq!(
            encode(SRC)
                .map(|(_pos, _len, c)| c)
                .map(|x| x.unwrap_or(b'@'))
                .collect::<Vec<u8>>(),
            DST
        )
    }
}
