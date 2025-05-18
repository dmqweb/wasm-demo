mod pages;
mod types;
mod api;
mod components;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::pages::Home;
use crate::components::ProductCard;
//使用wasm_bindgen定义wasm构建处开始的函数
#[wasm_bindgen(start)]
pub fn run_app() {
    // 使用App的new方法传入模块，挂载到body上
    App::<Home>::new().mount_to_body();
}
