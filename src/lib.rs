mod utils;

use crate::utils::log_s;
use wasm_bindgen::prelude::*;

/// 在全局使用 wee_alloc 作为内存分配器
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// 设置一个全局的panic hook，用于在panic时输出错误信息
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// 初始化函数，启动时执行
#[wasm_bindgen(start)]
pub fn init() {
    // 设置panic hook
    set_panic_hook();
    // 打印到控制台
    console_log!("YNYG Rust WebAssembly");
}
