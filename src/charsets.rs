pub const BLOCK: &[&str] = &[" ", "░", "▒", "▓", "█"];
pub const CHINESE: &[&str] = &[
    "\u{3000}", "一", "二", "十", "人", "丁", "口", "王", "日", "木", "金", "華", "爱", "黑", "墨",
    "龍", "龘",
];
pub const DEFAULT: &[&str] = &[
    " ", ".", "`", "^", "\"", "\\", ",", ":", ";", "I", "l", "!", "i", ">", "<", "~", "+", "_",
    "-", "?", "]", "[", "}", "{", "1", ")", "(", "|", "\\", "/", "t", "f", "j", "r", "x", "n",
    "u", "v", "c", "z", "X", "Y", "U", "J", "C", "L", "Q", "0", "O", "Z", "m", "w", "q", "p",
    "d", "b", "k", "h", "a", "o", "*", "#", "M", "W", "&", "8", "%", "B", "$", "@",
];
pub const EMOJI: &[&str] = &[
    "\u{3000}", "\u{3000}", "。", "，", "🧔", "👶", "🗣", "👥", "👤", "👀", "👁", "🦴", "🦷", "🫁",
    "🫀", "🧠", "👃", "🦻", "👂", "👅", "🦀", "👿", "🦀", "👄", "🤳", "💅", "🖖", "👆", "🙏", "🤝",
    "🦿", "🦾", "💪", "🤏", "👌", "🤘", "🤞", "👊", "🤚", "🤛", "🙌", "😾", "😿", "🙀", "😺", "👾",
    "👽", "👻", "💀", "👺", "🦀", "👹", "🤡", "💤", "😴", "🥸", "🥳", "🥶", "🥵", "🤮", "🤢", "🤕",
    "😭", "😓", "😯", "😰", "😨", "😱", "😮", "😩", "😫", "🙁", "😔", "😡", "🤬", "😠", "🙄", "😐",
    "😶", "🧐", "😛", "🤗", "🤐", "🤑", "😝", "🤩", "😋", "😊", "😉", "🤣", "😅", "😆",
];
pub const RUSSIAN: &[&str] = &[
    " ", " ", "Я", "Ю", "Э", "Ь", "Ы", "Ъ", "Щ", "Ш", "Ч", "Ц", "Х", "Ф", "У", "Т", "С", "P", "П",
    "О", "Н", "М", "Л", "К", "Й", "И", "З", "Ж", "Ё", "Е", "Д", "Г", "В", "Б", "А",
];
pub const SLIGHT: &[&str] = &[
    " ", " ", ".", "`", "\"", "\\", ":", "I", "!", ">", "~", "_", "?", "[", "{", "|", ")", "(",
    "\\", "\\\\", "/", "Y", "L", "p", "d", "a", "*", "W", "8", "%", "@", "$",
];

pub fn from_str(s: &str, charsets: &mut Vec<&str>) {
    match s {
        "block" => {
            charsets.extend_from_slice(BLOCK);
        },
        "chinese" => {
            charsets.extend_from_slice(CHINESE);
        },
        "default" => {
            charsets.extend_from_slice(DEFAULT);
        },
        "emoji" => {
            charsets.extend_from_slice(EMOJI);
        },
        "russian" => {
            charsets.extend_from_slice(RUSSIAN);
        },
        "slight" => {
            charsets.extend_from_slice(SLIGHT);
        },
                _ => {
            // Create a vector of &str slices from the individual characters of the input string
            for ch in s.chars() {
                let leaked = Box::leak(ch.to_string().into_boxed_str());
                charsets.push(&*leaked);
            }
        }
    }
}
