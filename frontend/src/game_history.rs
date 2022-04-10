use yew::{classes, html, Component, Context, Html, NodeRef};
pub struct game_history {
    hello:String,
}
pub enum Msg {}

impl Component for game_history {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self{
            hello: "hello".to_string(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ &self.hello }</h1>
            </div>
        }
    }
}