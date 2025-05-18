mod pages;
mod types;
mod api;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::pages::Home;
//使用wasm_bindgen定义wasm构建处开始的函数
#[wasm_bindgen(start)]
pub fn run_app() {
    // 使用App的new方法传入模块，挂载到body上
    App::<Home>::new().mount_to_body();
}
struct Product{
    name:String,
    description:String,
    image:String,
    price:f64,
}
struct State{//创建一个新的结构体State，包含products字段来保存来自服务器的产品
    products:Vec<Product>
}