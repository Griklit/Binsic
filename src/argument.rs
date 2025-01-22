use std::path::PathBuf;

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug, Clone)]
#[command(
    about = "二进制文件文本探查器，用于寻找二进制文件中可能包含的文本内容。", long_about = None
)]
#[command(version)]
pub struct CliArguments {
    #[arg(value_name = "PATH", help = "目标文件路径")]
    pub file: PathBuf,

    #[arg(short = 'r', long = "regex", value_name = "REGEX", help = "正则表达式")]
    pub re: Option<Regex>,

    #[arg(short = 'l', long, value_name = "LENGTH", default_value_t = 10, help = "最小有效长度")]
    pub min_length: usize,

    #[arg(
        short = 'e', long = "encoding",
        value_name = "ENCODING",
        value_parser = decoder_from_str,
        default_value = "utf-8",
        help = "编码方法"
    )]
    pub decoder: &'static encoding_rs::Encoding,

    #[arg(short = 'o', long, help = "输出文件路径")]
    pub output: Option<PathBuf>,
}

fn decoder_from_str(s: &str) -> Result<&'static encoding_rs::Encoding, String> {
    let s = s.to_ascii_lowercase().replace("_", "-");
    let r = match s.as_str() {
        "utf-8" | "utf8" => encoding_rs::UTF_8,
        "utf-16be" => encoding_rs::UTF_16BE,
        "utf-16le" => encoding_rs::UTF_16LE,
        "utf-16" => return Err("请指定字节序，utf-16be / utf-16le".to_string()),
        "gbk" => encoding_rs::GBK,
        "gb2312" => encoding_rs::GB18030,
        "gb18030" => encoding_rs::GB18030,
        "big5" => encoding_rs::BIG5,
        "euc-jp" => encoding_rs::EUC_JP,
        "shift-jis" => encoding_rs::SHIFT_JIS,
        "euc-kr" => encoding_rs::EUC_KR,
        "iso-2022-jp" => encoding_rs::ISO_2022_JP,
        "ibm866" => encoding_rs::IBM866,
        "macintosh" => encoding_rs::MACINTOSH,
        "replacement" => encoding_rs::REPLACEMENT,
        "koi8-r" => encoding_rs::KOI8_R,
        "koi8-u" => encoding_rs::KOI8_U,
        "x-mac-cyrillic" => encoding_rs::X_MAC_CYRILLIC,
        "iso-8859-1" | "iso-8859" => encoding_rs::WINDOWS_1252,
        "iso-8859-2" => encoding_rs::ISO_8859_2,
        "iso-8859-3" => encoding_rs::ISO_8859_3,
        "iso-8859-4" => encoding_rs::ISO_8859_4,
        "iso-8859-5" => encoding_rs::ISO_8859_5,
        "iso-8859-6" => encoding_rs::ISO_8859_6,
        "iso-8859-7" => encoding_rs::ISO_8859_7,
        "iso-8859-8" => encoding_rs::ISO_8859_8,
        "iso-8859-10" => encoding_rs::ISO_8859_10,
        "iso-8859-13" => encoding_rs::ISO_8859_13,
        "iso-8859-14" => encoding_rs::ISO_8859_14,
        "iso-8859-15" => encoding_rs::ISO_8859_15,
        "iso-8859-16" => encoding_rs::ISO_8859_16,
        "windows-1250" => encoding_rs::WINDOWS_1250,
        "windows-1251" => encoding_rs::WINDOWS_1251,
        "windows-1252" => encoding_rs::WINDOWS_1252,
        "windows-1253" => encoding_rs::WINDOWS_1253,
        "windows-1254" => encoding_rs::WINDOWS_1254,
        "windows-1255" => encoding_rs::WINDOWS_1255,
        "windows-1256" => encoding_rs::WINDOWS_1256,
        "windows-1257" => encoding_rs::WINDOWS_1257,
        "windows-1258" => encoding_rs::WINDOWS_1258,
        "windows-874" => encoding_rs::WINDOWS_874,
        _ => return Err(format!("未知编码：'{}'", s)),
    };
    Ok(r)
}