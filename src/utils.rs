use wasm_bindgen::prelude::*;

/// 导出的js函数
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log_s(s: &str);
}

/// 打印到控制台
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log_s(&format_args!($($t)*).to_string()))
}
