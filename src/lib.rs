mod pages;
mod types;
mod api;
mod components;
mod route;
mod app;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
//使用wasm_bindgen定义wasm构建处开始的函数
#[wasm_bindgen(start)]
pub fn run_app() {
    // 使用App的new方法传入模块，挂载到body上
    App::<app::App>::new().mount_to_body();
}
