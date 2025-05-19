use crate::types::Product;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get_products(callback: FetchCallback<Vec<Product>>) -> FetchTask {
    let req = Request::get("/products/products.json")
        .body(Nothing)
        .expect("获取json失败");

    FetchService::fetch(req, callback).expect("fetch失败")
}

pub fn get_product(id: i32, callback: FetchCallback<Product>) -> FetchTask {
    let req = Request::get(format!("/products/{}.json", id))
        .body(Nothing)
        .expect("get_product req失败");
    FetchService::fetch(req, callback).expect("fetch时失败")
}
