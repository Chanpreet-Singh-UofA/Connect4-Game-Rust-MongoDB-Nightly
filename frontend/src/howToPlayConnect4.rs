use yew::prelude::*;

pub struct HowToConnect4;

impl Component for HowToConnect4 {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        HowToConnect4
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            () => true,
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"How to Play Connect 4"}</b></h5>
                <hr style="width:50px;border:5px solid black" class="w3-round"/>
                <p>
                    {"Connect Four is a two-player connection game in which the players take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs."}
                </p>
                <br/>
                <div><h5>{"To play Connect 4 follow the following steps:"}</h5></div>
                <ul>
                    <li>{"When Starting a new game, player 1 is red and player 2 is green"}</li>
                    <li>{"When playing against the computer you are always player1 and thus red"}</li>
                    <li>{"Click on the desired column on the game board to place your disc"}</li>
                    <li>{"Try to connect 4 of your colored discs either horizontally or vertically or diagonally"}</li>

                </ul>
                <br/>
                <p>
                    {"For More information on Connect 4 click "}<a href="https://en.wikipedia.org/wiki/Connect_Four">{"here"}</a>
                </p>
            </div>
        }
    }
}