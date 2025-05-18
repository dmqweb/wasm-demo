use serde::{Deserialize, Serialize};//使用json解析的语法特性
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Product { //Product结构体
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub price: f64,
}
#[derive(Clone, Debug)]
pub struct CartProduct { //购物车产品结构体
    pub product: Product,
    pub quantity: i32,
}