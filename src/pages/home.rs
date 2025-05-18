use yew::prelude::*; //yew封装了视图组件，使用App方法将模块进行运行
pub struct Home {state:State}
impl Component for Home {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {//create方法当home模块被创建时使用
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
        Self{
            state:State{
                products
            }
        }
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
    fn view(&self) -> Html {//html宏相当于jsx语法
        let products:Vec<Html> = self.state.products.iter().map(|product: &Product| {
            html!{
                <div>
                    <img src={&product.image} />
                    <div>{&product.name}</div>
                    <div>{"$"}{&product.price}</div>
                </div>
            }
        }).collect();
        html!(<span>{products}</span>)
    }
}
struct Product{ //定义产品结构体
    id:i32,
    name:String,
    description:String,
    image:String,
    price:f64,
}
struct State{//状态仓库
    products:Vec<Product>
}