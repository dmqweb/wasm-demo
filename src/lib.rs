use wasm_bindgen::prelude::*;
use yew::prelude::*;
struct Hello {} //Hello组件
impl Component for Hello {
    //为Hello实现Component组件trait
    type Message = (); //消息类型
    type Properties = (); //属性类型
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
    fn view(&self) -> Html {
        html! { <span>{"Hello World!"}</span> }
    }
}
#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Hello>::new().mount_to_body();
}
