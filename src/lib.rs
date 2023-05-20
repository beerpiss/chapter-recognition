#![no_std]

extern crate alloc;
extern crate lua_patterns;

use alloc::{string::ToString, vec};
use lua_patterns::LuaPattern;

#[cfg(test)]
mod tests;

fn get_chapter_number_from_match(s: (&str, &str)) -> Option<f32> {
    let chapter = s.0.parse::<f32>().unwrap();

    if s.1.is_empty() {
        return Some(chapter);
    }

    let subchapter = if s.1.contains("extra") {
        0.99
    } else if s.1.contains("omake") {
        0.98
    } else if s.1.contains("special") {
        0.97
    } else if let Some(chr) = s.1.chars().next() {
        if let Ok(num) = s.1.parse::<f32>() {
            num / (10_u32.pow(s.1.len() as u32) as f32)
        } else {
            match chr {
                'a'..='h' => (chr as u8 as f32 - 96.0) / 10.0,
                // ASCII 65 to 74
                'A'..='H' => (chr as u8 as f32 - 64.0) / 10.0,
                _ => 0.0,
            }
        }
    } else {
        0.0
    };

    Some(chapter + subchapter)
}

pub fn parse_volume_number<T: AsRef<str>>(title: T, chapter: T) -> f32 {
    let patterns = vec![
        LuaPattern::new("%f[%w]volume%A?(%d+)"),
        LuaPattern::new("%f[%w]season%A?(%d+)"),
        LuaPattern::new("%f[%w]vol.?%A?(%d+)"),
        LuaPattern::new("%f[%w][vs]%A?(%d+)"),
    ];

    let mut name = chapter.as_ref().to_lowercase();
    name = name.replace(&title.as_ref().to_lowercase(), "");

    for mut pattern in patterns {
        if let Some(s) = pattern.match_maybe(&name) {
            if let Ok(num) = s.parse::<f32>() {
                return num;
            }
        }
    }

    -1.0
}

pub fn parse_chapter_number<T: AsRef<str>>(title: T, chapter: T) -> f32 {
    let mut number_pattern = LuaPattern::new("(%d+)%.?(%w*)");
    let mut basic_pattern = LuaPattern::new("ch.%s-(%d+)%.?(%w*)");

    let unwanted_patterns = vec![
        LuaPattern::new("%f[%a][vs]%A?%d+"),
        LuaPattern::new("%f[%a]ver%A?%d+"),
        LuaPattern::new("%f[%a]vol%A?%d+"),
        LuaPattern::new("%f[%a]version%A?%d+"),
        LuaPattern::new("%f[%a]volume%A?%d+"),
        LuaPattern::new("%f[%a]season%A?%d+"),
    ];

    let replacements = vec![
        (" special", "special"),
        (" omake", "omake"),
        (" extra", "extra"),
    ];

    let full_chapter_name = chapter.as_ref().to_lowercase();
    let mut name = full_chapter_name.replace(&title.as_ref().to_lowercase(), "");

    name = name.trim().replace([',', '-'], ".");

    for mut pattern in unwanted_patterns {
        name = pattern.gsub(&name, "").trim().to_string();
    }

    for replacement in replacements {
        name = name.replace(replacement.0, replacement.1);
    }

    for pattern in vec![&mut basic_pattern, &mut number_pattern] {
        if let Some(s) = pattern.match_maybe_2(&name) {
            if let Some(num) = get_chapter_number_from_match(s) {
                return num;
            }
        }
    }

    -1.0
}
