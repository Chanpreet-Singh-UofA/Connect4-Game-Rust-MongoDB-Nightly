use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Home
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            () => true,
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="w3-container" id="services" style="margin-top:75px;margin-bottom:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Welcome"}</b></h5>
                //<hr style="width:50px;border:5px solid black" class="w3-round"/>
                <hr style="height:2px;border-width:0;color:black;background-color:black"/>
                <hr style="height:2px;border-width:0;color:gray;background-color:gray"/>
                <p>
                    {"This application contains the following two board games, both in human Vs. human and human Vs. Computer versions."}
                </p>
                <ul>
                    <li>{"Connect 4"}</li>
                    <li>{"TOOT-OTTO"}</li>
                </ul>
                <p>{"Select the game of your choice from one of the drop down menus on the top navigation bar, and start playing. Enjoy!"}</p>
            </div>
        }
    }
}