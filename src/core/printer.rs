use std::cmp::min;
const MAX_WIDTH: usize = 120;
const MAX_HEIGHT: usize = 10;

/// スタイル付きの文字列を生成するマクロ
#[macro_export]
macro_rules! styled {
    ($($arg:tt)*) => {
        ::console::style(format!($($arg)*))
    };
}

/// 出力を省略して表示する関数
///
/// # 引数
///
/// * `output` - 出力文字列
///
/// # 戻り値
///
/// 省略された出力文字列
pub(crate) fn abbr(output: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    for line in output.split('\n') {
        if line.len() < MAX_WIDTH {
            lines.push(line.to_string());
        } else {
            lines.push(line[..min(MAX_WIDTH, line.len())].to_string() + "...");
        }

        if lines.len() == MAX_HEIGHT {
            lines.push("...".to_string());
            break;
        }
    }
    lines.join("\n")
}
