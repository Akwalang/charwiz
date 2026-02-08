// src/lib.rs
#![forbid(unsafe_code)]

use core::fmt;

//
// -------------------- Compile-time flags (IMPORTANT) --------------------
// These are evaluated in THIS crate (zero_cost_logger), so macros can safely use them
// without `#[cfg(feature=...)]` inside macro expansions.
//

#[doc(hidden)]
pub const __ZCL_LOGGER_ENABLED: bool = cfg!(feature = "logger");

#[doc(hidden)]
pub const __ZCL_LOG_DEBUG_ENABLED: bool = cfg!(feature = "log-debug");
#[doc(hidden)]
pub const __ZCL_LOG_INFO_ENABLED: bool = cfg!(feature = "log-info");
#[doc(hidden)]
pub const __ZCL_LOG_WARN_ENABLED: bool = cfg!(feature = "log-warn");
#[doc(hidden)]
pub const __ZCL_LOG_ERROR_ENABLED: bool = cfg!(feature = "log-error");

#[doc(hidden)]
pub const __ZCL_COLOR_ENABLED: bool = cfg!(feature = "logger-color");
#[doc(hidden)]
pub const __ZCL_TS_ENABLED: bool = cfg!(feature = "logger-timestamp");
#[doc(hidden)]
pub const __ZCL_MARKUP_ENABLED: bool = cfg!(feature = "logger-markup");
#[doc(hidden)]
pub const __ZCL_ALIASES_ENABLED: bool = cfg!(feature = "logger-aliases");

//
// -------------------- Public macros (zero-cost when disabled) --------------------
//

// Compile-time aliases declaration.
// NOTE: This generates `mod __zcl_aliases` in the CURRENT crate.
// That means it affects the parser only if used inside `zero_cost_logger` itself
// (as in the default aliases at the bottom of this file).
#[macro_export]
macro_rules! aliases {
    (
        $(
            $name:literal => $tokens:literal
        ),* $(,)?
    ) => {
        // IMPORTANT: no #[cfg(feature=...)] here, because cfg in macros is checked
        // in the destination crate, not in zero_cost_logger.
        mod __zcl_aliases {
            #[inline(always)]
            pub fn resolve(name: &str) -> Option<&'static str> {
                match name {
                    $(
                        $name => Some($tokens),
                    )*
                    _ => None,
                }
            }
        }
    };
}

// Debug
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        // This constant lives in zero_cost_logger, so it is correct and warning-free.
        if $crate::__ZCL_LOG_DEBUG_ENABLED {
            // `log-debug` depends on `logger` in Cargo.toml, so internal exists when true.
            $crate::internal::emit($crate::internal::Level::Debug, format_args!($($arg)*));
        }
    }};
}

// Info
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        if $crate::__ZCL_LOG_INFO_ENABLED {
            $crate::internal::emit($crate::internal::Level::Info, format_args!($($arg)*));
        }
    }};
}

// Warn
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        if $crate::__ZCL_LOG_WARN_ENABLED {
            $crate::internal::emit($crate::internal::Level::Warn, format_args!($($arg)*));
        }
    }};
}

// Error
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        if $crate::__ZCL_LOG_ERROR_ENABLED {
            $crate::internal::emit($crate::internal::Level::Error, format_args!($($arg)*));
        }
    }};
}

// New line
#[macro_export]
macro_rules! new_line {
    () => {{
        if $crate::__ZCL_LOGGER_ENABLED {
            $crate::internal::emit_newline();
        }
    }};
}

//
// -------------------- Empty module to satisfy compiler when logger is disabled --------------------
//
#[cfg(not(feature = "logger"))]
pub mod internal {
    use core::fmt;

    #[derive(Clone, Copy)]
    pub enum Level {
        Debug,
        Info,
        Warn,
        Error,
    }

    // ---- zero-cost stubs (prod) ----

    #[inline(always)]
    pub fn emit(_level: Level, _args: fmt::Arguments) {
        // nothing
    }

    #[inline(always)]
    pub fn emit_newline() {
        // nothing
    }
}

//
// -------------------- Internal implementation (only with feature=logger) --------------------
//
#[cfg(feature = "logger")]
pub mod internal {
    use super::fmt;

    // -------- Level --------
    #[derive(Clone, Copy, Debug)]
    pub enum Level {
        Debug,
        Info,
        Warn,
        Error,
    }

    // -------- Output buffer (no heap) --------
    const BUF_CAP: usize = 2048;

    struct Buffer {
        buf: [u8; BUF_CAP],
        len: usize,
    }

    impl Buffer {
        #[inline(always)]
        fn new() -> Self {
            Self { buf: [0u8; BUF_CAP], len: 0 }
        }

        #[inline(always)]
        fn push_str(&mut self, s: &str) {
            let bytes = s.as_bytes();
            let available = self.buf.len().saturating_sub(self.len);
            if available == 0 {
                return;
            }
            let n = bytes.len().min(available);
            self.buf[self.len..self.len + n].copy_from_slice(&bytes[..n]);
            self.len += n;
        }

        #[inline(always)]
        fn push_byte(&mut self, b: u8) {
            if self.len < self.buf.len() {
                self.buf[self.len] = b;
                self.len += 1;
            }
        }

        #[inline(always)]
        fn flush(mut self) {
            use std::io::Write;
            self.push_byte(b'\n');
            let _ = std::io::stdout().write_all(&self.buf[..self.len]);
        }
    }

    // When markup is OFF we can stream fmt directly into our buffer without heap.
    #[cfg(not(feature = "logger-markup"))]
    struct FmtAdapter<'a>(&'a mut Buffer);

    #[cfg(not(feature = "logger-markup"))]
    impl fmt::Write for FmtAdapter<'_> {
        #[inline(always)]
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    // -------- Prefix: label + colors --------
    #[inline(always)]
    fn level_label(level: Level) -> &'static str {
        match level {
            Level::Debug => "DBG",
            Level::Info  => "LOG",
            Level::Warn  => "WRN",
            Level::Error => "ERR",
        }
    }

    #[cfg(feature = "logger-color")]
    const ANSI_RESET: &str = "\x1b[0m";

    // Label: colored BACKGROUND with WHITE text and colored spaces around
    #[cfg(feature = "logger-color")]
    #[inline(always)]
    fn write_level_label(out: &mut Buffer, level: Level) {
        let bg = match level {
            Level::Debug => "100", // bright black
            Level::Info  => "44",  // blue
            Level::Warn  => "43",  // yellow
            Level::Error => "41",  // red
        };

        out.push_str("\x1b[");
        out.push_str(bg);
        out.push_str(";37m ");
        out.push_str(level_label(level));
        out.push_str(" \x1b[0m ");
    }

    // Default colors for timestamp and message per level
    #[cfg(feature = "logger-color")]
    #[inline(always)]
    fn timestamp_default_fg(level: Level) -> &'static str {
        match level {
            Level::Debug => "90", // gray
            Level::Info  => "34", // blue
            Level::Warn  => "33", // yellow
            Level::Error => "31", // red
        }
    }

    #[cfg(feature = "logger-color")]
    #[inline(always)]
    fn message_default_fg(level: Level) -> &'static str {
        match level {
            Level::Debug => "90", // gray
            Level::Info  => "37", // white
            Level::Warn  => "33", // yellow
            Level::Error => "31", // red
        }
    }

    // -------- Timestamp (UTC) --------
    #[cfg(feature = "logger-timestamp")]
    fn days_to_ymd(days_since_epoch: i64) -> (i32, i32, i32) {
        let z   = days_since_epoch + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let y   = (yoe as i32) + (era as i32) * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100 + yoe / 400);
        let mp  = (5 * doy + 2) / 153;
        let d   = (doy - (153 * mp + 2) / 5 + 1) as i32;
        let m   = (mp + if mp < 10 { 3 } else { -9 }) as i32;
        let y   = y + if m <= 2 { 1 } else { 0 };

        (y, m, d)
    }

    #[cfg(feature = "logger-timestamp")]
    fn write_2(dst: &mut [u8], v: i32) {
        dst[0] = b'0' + ((v / 10) as u8);
        dst[1] = b'0' + ((v % 10) as u8);
    }

    #[cfg(feature = "logger-timestamp")]
    fn write_3(dst: &mut [u8], v: i32) {
        dst[0] = b'0' + ((v / 100) as u8);
        dst[1] = b'0' + (((v / 10) % 10) as u8);
        dst[2] = b'0' + ((v % 10) as u8);
    }

    #[cfg(feature = "logger-timestamp")]
    fn write_4(dst: &mut [u8], v: i32) {
        let v = v.max(0);

        dst[0] = b'0' + (((v / 1000) % 10) as u8);
        dst[1] = b'0' + (((v / 100) % 10) as u8);
        dst[2] = b'0' + (((v / 10) % 10) as u8);
        dst[3] = b'0' + ((v % 10) as u8);
    }

    #[cfg(feature = "logger-timestamp")]
    fn write_timestamp_utc(out: &mut Buffer) {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now();
        let dur = now.duration_since(UNIX_EPOCH).unwrap_or_default();
        let total_secs = dur.as_secs() as i64;
        let millis = dur.subsec_millis() as i32;

        let days = total_secs / 86_400;
        let sod  = (total_secs % 86_400) as i64;
        let hour = (sod / 3_600) as i32;
        let min  = ((sod % 3_600) / 60) as i32;
        let sec  = (sod % 60) as i32;

        let (y, m, d) = days_to_ymd(days);

        let mut buf = [0u8; 32];

        write_4(&mut buf[0..4], y);
        buf[4] = b'.';
        write_2(&mut buf[5..7], m);
        buf[7] = b'.';
        write_2(&mut buf[8..10], d);
        buf[10] = b' ';
        write_2(&mut buf[11..13], hour);
        buf[13] = b':';
        write_2(&mut buf[14..16], min);
        buf[16] = b':';
        write_2(&mut buf[17..19], sec);
        buf[19] = b'.';
        write_3(&mut buf[20..23], millis);

        let s = core::str::from_utf8(&buf[..23]).unwrap_or("0000.00.00 00:00:00.000");

        out.push_byte(b'[');
        out.push_str(s);
        out.push_str("] ");
    }

    // -------- Markup (<red,b,i>text</>) --------
    #[cfg(feature = "logger-markup")]
    #[inline(always)]
    fn color_name_to_fg_code(name: &str) -> Option<&'static str> {
        match name {
            "black"   => Some("30"),
            "red"     => Some("31"),
            "green"   => Some("32"),
            "orange"  => Some("33"),
            "yellow"  => Some("33"),
            "blue"    => Some("34"),
            "purple"  => Some("35"),
            "magenta" => Some("35"),
            "cyan"    => Some("36"),
            "white"   => Some("37"),
            "gray"    => Some("90"),
            _ => None,
        }
    }

    // Alias resolver:
    // This works because you declare `aliases!{...}` at the bottom of THIS crate,
    // so `crate::__zcl_aliases` exists in zero_cost_logger.
    #[cfg(feature = "logger-aliases")]
    #[inline(always)]
    fn resolve_alias(name: &str) -> Option<&'static str> {
        crate::__zcl_aliases::resolve(name)
    }

    #[cfg(feature = "logger-markup")]
    fn apply_markup_into(out: &mut Buffer, input: &str, default_fg: Option<&'static str>) {
        let bytes = input.as_bytes();
        let mut i = 0usize;

        while i < bytes.len() {
            if bytes[i] == b'<' {
                if let Some(gt_rel) = input[i..].find('>') {
                    let tag_inner = &input[i + 1..i + gt_rel];

                    if let Some(close_rel) = input[i + gt_rel + 1..].find("</>") {
                        let content_start = i + gt_rel + 1;
                        let content_end = content_start + close_rel;
                        let content = &input[content_start..content_end];

                        let token_source: &str = if cfg!(feature = "logger-aliases") {
                            // safe: function exists only when feature is on
                            #[cfg(feature = "logger-aliases")]
                            {
                                if let Some(t) = resolve_alias(tag_inner) { t } else { tag_inner }
                            }
                            #[cfg(not(feature = "logger-aliases"))]
                            {
                                tag_inner
                            }
                        } else {
                            tag_inner
                        };

                        let mut bold = false;
                        let mut italic = false;
                        let mut underline = false;
                        let mut dim = false;
                        let mut strike = false;
                        let mut reverse = false;
                        let mut fg: Option<&'static str> = None;

                        for token in token_source
                            .split(',')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                        {
                            let lower = token.to_ascii_lowercase();
                            match lower.as_str() {
                                "b" | "bold"      => bold = true,
                                "i" | "italic"    => italic = true,
                                "u" | "underline" => underline = true,
                                "d" | "dim"       => dim = true,
                                "s" | "strike"    => strike = true,
                                "r" | "reverse"   => reverse = true,
                                _ => {
                                    if fg.is_none() {
                                        if let Some(code) = color_name_to_fg_code(lower.as_str()) {
                                            fg = Some(code);
                                        }
                                    }
                                }
                            }
                        }

                        if bold || italic || underline || dim || strike || reverse || fg.is_some() {
                            out.push_str("\x1b[");
                            let mut first = true;

                            let push_code = |code: &str, out: &mut Buffer, first: &mut bool| {
                                if !*first { out.push_byte(b';'); } else { *first = false; }
                                out.push_str(code);
                            };

                            if bold      { push_code("1", out, &mut first); }
                            if italic    { push_code("3", out, &mut first); }
                            if underline { push_code("4", out, &mut first); }
                            if dim       { push_code("2", out, &mut first); }
                            if strike    { push_code("9", out, &mut first); }
                            if reverse   { push_code("7", out, &mut first); }
                            if let Some(c) = fg { push_code(c, out, &mut first); }

                            out.push_byte(b'm');
                            out.push_str(content);

                            if let Some(df) = default_fg {
                                out.push_str("\x1b[");
                                out.push_str(df);
                                out.push_str("m");
                            } else {
                                out.push_str("\x1b[0m");
                            }
                        } else {
                            out.push_str(content);
                        }

                        i = content_end + 3;
                        continue;
                    }
                }
            }

            let ch = input[i..].chars().next().unwrap();
            let mut tmp = [0u8; 4];

            let s = ch.encode_utf8(&mut tmp);

            out.push_str(s);
            i += ch.len_utf8();
        }
    }

    // -------- Message writing --------
    #[cfg(feature = "logger-markup")]
    fn write_message(out: &mut Buffer, level: Level, args: fmt::Arguments) {
        let mut s = std::string::String::new();
        let _ = fmt::write(&mut s, args);

        #[cfg(feature = "logger-color")]
        let df = Some(message_default_fg(level));

        #[cfg(not(feature = "logger-color"))]
        let df: Option<&'static str> = None;

        apply_markup_into(out, &s, df);
    }

    #[cfg(not(feature = "logger-markup"))]
    fn write_message(out: &mut Buffer, _level: Level, args: fmt::Arguments) {
        let _ = fmt::write(&mut FmtAdapter(out), args);
    }

    // -------- Emitters --------
    #[inline(always)]
    pub fn emit(level: Level, args: fmt::Arguments) {
        let mut out = Buffer::new();

        #[cfg(feature = "logger-color")]
        write_level_label(&mut out, level);

        #[cfg(not(feature = "logger-color"))]
        {
            out.push_str(level_label(level));
            out.push_str(" ");
        }

        #[cfg(feature = "logger-color")]
        {
            out.push_str("\x1b[");
            out.push_str(timestamp_default_fg(level));
            out.push_str("m");
        }

        #[cfg(feature = "logger-timestamp")]
        write_timestamp_utc(&mut out);

        #[cfg(feature = "logger-color")]
        {
            out.push_str("\x1b[");
            out.push_str(message_default_fg(level));
            out.push_str("m");
        }

        write_message(&mut out, level, args);

        #[cfg(feature = "logger-color")]
        out.push_str(ANSI_RESET);

        out.flush();
    }

    #[inline(always)]
    pub fn emit_newline() {
        Buffer::new().flush();
    }
}

//
// Default aliases for your style (compiled into this crate).
// These work when feature `logger-aliases` is enabled.
//
#[cfg(feature = "logger-aliases")]
aliases! {
    "$"  => "purple,i",
    "!"  => "yellow",
    "i!" => "yellow,i",
    "+"  => "green",
    "i+" => "green,i",
    "-"  => "red",
    "i-" => "red,i",
    "&"  => "cyan",
    "i&" => "cyan,i",
}
