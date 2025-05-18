use yew::prelude::*; //yew封装了视图组件，使用App方法将模块进行运行
pub struct Home {
    state:State,
    link:ComponentLink<Self>,
}
pub enum Msg{
    AddToCart(i32),
}
impl Component for Home {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {//create方法当home模块被创建时使用,传入属性和link
        let products:Vec<Product> = vec![
            Product{
                id:1,
                name: "Apple".to_string(),
                description: "一个苹果".to_string(),
                image: "/products/apple.png".to_string(),
                price: 3.65,
            },Product{
                id:2,
                name: "Banana".to_string(),
                description: "一个香蕉".to_string(),
                image: "/products/banana.png".to_string(),
                price: 9.43,
            },
        ];
        let cart_products = vec![];
        Self{
            state:State{
                products,
                cart_products,
            },link
        }
    }
    fn update(&mut self, message: Self::Message) -> ShouldRender {//更新时会传入消息，匹配消息的id属性，更新数量或者添加
        println!("更新！！");//这里将会被转换为单页web应用的输出格式（js代码）
        match message {
            Msg::AddToCart(product_id) => {
                let product = self.state.products.iter().find(|p:&&Product| p.id == product_id).unwrap();
                let cart_product = self.state.cart_products.iter_mut().find(|cp:&&mut CartProduct| cp.product.id==product_id);
                if let Some(cp) = cart_product{
                    cp.quantity += 1;
                }else {
                    self.state.cart_products.push(CartProduct{
                        product:product.clone(),
                        quantity:1,
                    })
                }
                true
            }
        }
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
    fn view(&self) -> Html {//html宏相当于jsx语法
        let products:Vec<Html> = self.state.products.iter().map(|product: &Product| {
            let _product_id = product.id;
            html!{
                <div>
                    <img src={&product.image} />
                    <div>{&product.name}</div>
                    <div>{"$"}{&product.price}</div>
                    <button onclick=self.link.callback(move |_| Msg::AddToCart(_product_id))>{"Add To Cart"}</button>
                </div>
            }
        }).collect();
        let cart_value = self.state.cart_products.iter().fold(0.0,|acc,cp| acc+(cp.quantity as f64 * cp.product.price));
        html!{
            <div>
                <span>{format!("购物车值：{:.2}",cart_value)}</span>
                <span>{products}</span>
            </div>
        }
    }
}
#[derive(Clone)]
struct Product{ //定义产品结构体
    id:i32,
    name:String,
    description:String,
    image:String,
    price:f64,
}
struct State{//状态仓库
    products:Vec<Product>,
    cart_products: Vec<CartProduct>,
}
struct CartProduct{
    product:Product,
    quantity:i32,
}