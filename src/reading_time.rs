/// Rust port of ngryman's reading-time package for Node.JS
/// Go check it out: https://github.com/ngryman/reading-time

fn is_cjk(c: char) -> bool {
    match c as u32 {
        // Hiragana (Katakana not included on purpose).
        0x3040 ..= 0x309f |
        // CJK Unified ideographs
        0x4e00 ..= 0x9fff |
        // Hangul
        0xac00 ..= 0xd7a3 |
        // CJK extensions
        0x20000 ..= 0x2ebe0 => true,

        _ => false
    }
}

fn is_ansi_word_bound(c: char) -> bool {
    match c {
        ' ' | '\n' | '\r' | '\t' => true,
        _ => false
    }
}

fn is_punctuation(c: char) -> bool {
    match c as u32 {
        0x21   ..= 0x2f |
        0x3a   ..= 0x40 |
        0x5b   ..= 0x60 |
        0x7b   ..= 0x7e |
        // CJK Symbols and Punctuation
        0x3000 ..= 0x303f |
        // Full-width ASCII punctuation variants
        0xff00 ..= 0xffef => true,

        _ => false
    }
}

pub fn count_words(text: String) -> u32 {
    let chars: Vec<_> = text.chars().collect();

    let mut words = 0;
    let mut start = 0;
    let mut end = text.len() - 1 ;

    while is_ansi_word_bound(chars[start]) { start += 1; }
    while is_ansi_word_bound(chars[end]) { end += 1; }

    let normalized_text: Vec<_> = format!("{}\n", text).chars().collect();

    let mut i = start;
    while i <= end {
        let current = normalized_text[i];
        let next = normalized_text[i+1];

        if is_cjk(current) || (!is_ansi_word_bound(current) && (is_ansi_word_bound(next) || is_cjk(next))) {
            words += 1;
        }

        if is_cjk(current) {
            while i <= end && (is_punctuation(next) || is_ansi_word_bound(next)) {
                i += 1;
            }
        }

        i += 1;
    }
    words
}

const WPM: u32 = 200;
pub fn reading_time_from_words(words: u32) -> u32 {
    words / WPM
}
