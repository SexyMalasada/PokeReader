pub type FontLetterLine = u8;
pub type FontLetter = [FontLetterLine; 8];

// Thanks to NTR
const FONT: [FontLetter; 95] = [
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Char 032 ( )
    [0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x18, 0x00], // Char 033 (!)
    [0x6C, 0x6C, 0x6C, 0x00, 0x00, 0x00, 0x00, 0x00], // Char 034 (")
    [0x6C, 0x6C, 0xFE, 0x6C, 0xFE, 0x6C, 0x6C, 0x00], // Char 035 (#)
    [0x18, 0x7E, 0xC0, 0x7C, 0x06, 0xFC, 0x18, 0x00], // Char 036 ($)
    [0x00, 0xC6, 0xCC, 0x18, 0x30, 0x66, 0xC6, 0x00], // Char 037 (%)
    [0x38, 0x6C, 0x38, 0x76, 0xDC, 0xCC, 0x76, 0x00], // Char 038 (&)
    [0x30, 0x30, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00], // Char 039 (')
    [0x0C, 0x18, 0x30, 0x30, 0x30, 0x18, 0x0C, 0x00], // Char 040 (()
    [0x30, 0x18, 0x0C, 0x0C, 0x0C, 0x18, 0x30, 0x00], // Char 041 ())
    [0x00, 0x66, 0x3C, 0xFF, 0x3C, 0x66, 0x00, 0x00], // Char 042 (*)
    [0x00, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x00, 0x00], // Char 043 (+)
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x30], // Char 044 (,)
    [0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00], // Char 045 (-)
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00], // Char 046 (.)
    [0x06, 0x0C, 0x18, 0x30, 0x60, 0xC0, 0x80, 0x00], // Char 047 (/)
    [0x7C, 0xCE, 0xDE, 0xF6, 0xE6, 0xC6, 0x7C, 0x00], // Char 048 (0)
    [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00], // Char 049 (1)
    [0x7C, 0xC6, 0x06, 0x7C, 0xC0, 0xC0, 0xFE, 0x00], // Char 050 (2)
    [0xFC, 0x06, 0x06, 0x3C, 0x06, 0x06, 0xFC, 0x00], // Char 051 (3)
    [0x0C, 0xCC, 0xCC, 0xCC, 0xFE, 0x0C, 0x0C, 0x00], // Char 052 (4)
    [0xFE, 0xC0, 0xFC, 0x06, 0x06, 0xC6, 0x7C, 0x00], // Char 053 (5)
    [0x7C, 0xC0, 0xC0, 0xFC, 0xC6, 0xC6, 0x7C, 0x00], // Char 054 (6)
    [0xFE, 0x06, 0x06, 0x0C, 0x18, 0x30, 0x30, 0x00], // Char 055 (7)
    [0x7C, 0xC6, 0xC6, 0x7C, 0xC6, 0xC6, 0x7C, 0x00], // Char 056 (8)
    [0x7C, 0xC6, 0xC6, 0x7E, 0x06, 0x06, 0x7C, 0x00], // Char 057 (9)
    [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x00], // Char 058 (:)
    [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x30], // Char 059 (;)
    [0x0C, 0x18, 0x30, 0x60, 0x30, 0x18, 0x0C, 0x00], // Char 060 (<)
    [0x00, 0x00, 0x7E, 0x00, 0x7E, 0x00, 0x00, 0x00], // Char 061 (=)
    [0x30, 0x18, 0x0C, 0x06, 0x0C, 0x18, 0x30, 0x00], // Char 062 (>)
    [0x3C, 0x66, 0x0C, 0x18, 0x18, 0x00, 0x18, 0x00], // Char 063 (?)
    [0x7C, 0xC6, 0xDE, 0xDE, 0xDE, 0xC0, 0x7E, 0x00], // Char 064 (@)
    [0x38, 0x6C, 0xC6, 0xC6, 0xFE, 0xC6, 0xC6, 0x00], // Char 065 (A)
    [0xFC, 0xC6, 0xC6, 0xFC, 0xC6, 0xC6, 0xFC, 0x00], // Char 066 (B)
    [0x7C, 0xC6, 0xC0, 0xC0, 0xC0, 0xC6, 0x7C, 0x00], // Char 067 (C)
    [0xF8, 0xCC, 0xC6, 0xC6, 0xC6, 0xCC, 0xF8, 0x00], // Char 068 (D)
    [0xFE, 0xC0, 0xC0, 0xF8, 0xC0, 0xC0, 0xFE, 0x00], // Char 069 (E)
    [0xFE, 0xC0, 0xC0, 0xF8, 0xC0, 0xC0, 0xC0, 0x00], // Char 070 (F)
    [0x7C, 0xC6, 0xC0, 0xC0, 0xCE, 0xC6, 0x7C, 0x00], // Char 071 (G)
    [0xC6, 0xC6, 0xC6, 0xFE, 0xC6, 0xC6, 0xC6, 0x00], // Char 072 (H)
    [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00], // Char 073 (I)
    [0x06, 0x06, 0x06, 0x06, 0x06, 0xC6, 0x7C, 0x00], // Char 074 (J)
    [0xC6, 0xCC, 0xD8, 0xF0, 0xD8, 0xCC, 0xC6, 0x00], // Char 075 (K)
    [0xC0, 0xC0, 0xC0, 0xC0, 0xC0, 0xC0, 0xFE, 0x00], // Char 076 (L)
    [0xC6, 0xEE, 0xFE, 0xFE, 0xD6, 0xC6, 0xC6, 0x00], // Char 077 (M)
    [0xC6, 0xE6, 0xF6, 0xDE, 0xCE, 0xC6, 0xC6, 0x00], // Char 078 (N)
    [0x7C, 0xC6, 0xC6, 0xC6, 0xC6, 0xC6, 0x7C, 0x00], // Char 079 (O)
    [0xFC, 0xC6, 0xC6, 0xFC, 0xC0, 0xC0, 0xC0, 0x00], // Char 080 (P)
    [0x7C, 0xC6, 0xC6, 0xC6, 0xD6, 0xDE, 0x7C, 0x06], // Char 081 (Q)
    [0xFC, 0xC6, 0xC6, 0xFC, 0xD8, 0xCC, 0xC6, 0x00], // Char 082 (R)
    [0x7C, 0xC6, 0xC0, 0x7C, 0x06, 0xC6, 0x7C, 0x00], // Char 083 (S)
    [0xFF, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00], // Char 084 (T)
    [0xC6, 0xC6, 0xC6, 0xC6, 0xC6, 0xC6, 0xFE, 0x00], // Char 085 (U)
    [0xC6, 0xC6, 0xC6, 0xC6, 0xC6, 0x7C, 0x38, 0x00], // Char 086 (V)
    [0xC6, 0xC6, 0xC6, 0xC6, 0xD6, 0xFE, 0x6C, 0x00], // Char 087 (W)
    [0xC6, 0xC6, 0x6C, 0x38, 0x6C, 0xC6, 0xC6, 0x00], // Char 088 (X)
    [0xC6, 0xC6, 0xC6, 0x7C, 0x18, 0x30, 0xE0, 0x00], // Char 089 (Y)
    [0xFE, 0x06, 0x0C, 0x18, 0x30, 0x60, 0xFE, 0x00], // Char 090 (Z)
    [0x3C, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3C, 0x00], // Char 091 ([)
    [0xC0, 0x60, 0x30, 0x18, 0x0C, 0x06, 0x02, 0x00], // Char 092 (\)
    [0x3C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x3C, 0x00], // Char 093 (])
    [0x10, 0x38, 0x6C, 0xC6, 0x00, 0x00, 0x00, 0x00], // Char 094 (^)
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF], // Char 095 (_)
    [0x18, 0x18, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00], // Char 096 (`)
    [0x00, 0x00, 0x7C, 0x06, 0x7E, 0xC6, 0x7E, 0x00], // Char 097 (a)
    [0xC0, 0xC0, 0xC0, 0xFC, 0xC6, 0xC6, 0xFC, 0x00], // Char 098 (b)
    [0x00, 0x00, 0x7C, 0xC6, 0xC0, 0xC6, 0x7C, 0x00], // Char 099 (c)
    [0x06, 0x06, 0x06, 0x7E, 0xC6, 0xC6, 0x7E, 0x00], // Char 100 (d)
    [0x00, 0x00, 0x7C, 0xC6, 0xFE, 0xC0, 0x7C, 0x00], // Char 101 (e)
    [0x1C, 0x36, 0x30, 0x78, 0x30, 0x30, 0x78, 0x00], // Char 102 (f)
    [0x00, 0x00, 0x7E, 0xC6, 0xC6, 0x7E, 0x06, 0xFC], // Char 103 (g)
    [0xC0, 0xC0, 0xFC, 0xC6, 0xC6, 0xC6, 0xC6, 0x00], // Char 104 (h)
    [0x18, 0x00, 0x38, 0x18, 0x18, 0x18, 0x3C, 0x00], // Char 105 (i)
    [0x06, 0x00, 0x06, 0x06, 0x06, 0x06, 0xC6, 0x7C], // Char 106 (j)
    [0xC0, 0xC0, 0xCC, 0xD8, 0xF8, 0xCC, 0xC6, 0x00], // Char 107 (k)
    [0x38, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00], // Char 108 (l)
    [0x00, 0x00, 0xCC, 0xFE, 0xFE, 0xD6, 0xD6, 0x00], // Char 109 (m)
    [0x00, 0x00, 0xFC, 0xC6, 0xC6, 0xC6, 0xC6, 0x00], // Char 110 (n)
    [0x00, 0x00, 0x7C, 0xC6, 0xC6, 0xC6, 0x7C, 0x00], // Char 111 (o)
    [0x00, 0x00, 0xFC, 0xC6, 0xC6, 0xFC, 0xC0, 0xC0], // Char 112 (p)
    [0x00, 0x00, 0x7E, 0xC6, 0xC6, 0x7E, 0x06, 0x06], // Char 113 (q)
    [0x00, 0x00, 0xFC, 0xC6, 0xC0, 0xC0, 0xC0, 0x00], // Char 114 (r)
    [0x00, 0x00, 0x7E, 0xC0, 0x7C, 0x06, 0xFC, 0x00], // Char 115 (s)
    [0x18, 0x18, 0x7E, 0x18, 0x18, 0x18, 0x0E, 0x00], // Char 116 (t)
    [0x00, 0x00, 0xC6, 0xC6, 0xC6, 0xC6, 0x7E, 0x00], // Char 117 (u)
    [0x00, 0x00, 0xC6, 0xC6, 0xC6, 0x7C, 0x38, 0x00], // Char 118 (v)
    [0x00, 0x00, 0xC6, 0xC6, 0xD6, 0xFE, 0x6C, 0x00], // Char 119 (w)
    [0x00, 0x00, 0xC6, 0x6C, 0x38, 0x6C, 0xC6, 0x00], // Char 120 (x)
    [0x00, 0x00, 0xC6, 0xC6, 0xC6, 0x7E, 0x06, 0xFC], // Char 121 (y)
    [0x00, 0x00, 0xFE, 0x0C, 0x38, 0x60, 0xFE, 0x00], // Char 122 (z)
    [0x0E, 0x18, 0x18, 0x70, 0x18, 0x18, 0x0E, 0x00], // Char 123 ({)
    [0x18, 0x18, 0x18, 0x00, 0x18, 0x18, 0x18, 0x00], // Char 124 (|)
    [0x70, 0x18, 0x18, 0x0E, 0x18, 0x18, 0x70, 0x00], // Char 125 (})
    [0x76, 0xDC, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Char 126 (~)
];

pub fn convert_letter_to_font(letter: char) -> FontLetter {
    let letter = letter as usize;
    let safe_letter = if !(32..=127).contains(&letter) {
        63
    } else {
        letter
    };

    FONT[(safe_letter - 32)]
}
