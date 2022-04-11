#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Alive,
    Dead,
    Green,
}

#[derive(Clone, Copy)]
pub struct Cellule {
    pub state: State,
    pub pstate: State,
}

impl Cellule {
    pub fn new_dead() -> Self {
        Self { state: State::Dead, pstate: State::Dead }
    }

    pub fn set_alive(&mut self) {
        self.state = State::Alive;
    }

    pub fn set_green(&mut self) {
        self.state = State::Green;
    }

    pub fn set_dead(&mut self) {
        self.state = State::Dead;
    }

    pub fn is_alive(self) -> bool {
        self.state == State::Alive
    }

    pub fn is_dead(self) -> bool {
        self.state == State::Dead
    }

    pub fn set_palive(&mut self) {
        self.pstate = State::Alive;
    }

    pub fn set_pgreen(&mut self) {
        self.pstate = State::Green;
    }

    pub fn set_pdead(&mut self) {
        self.pstate = State::Dead;
    }

    pub fn is_palive(self) -> bool {
        self.pstate == State::Alive
    }

    pub fn is_pdead(self) -> bool {
        self.pstate == State::Dead
    }

    pub fn toggle(&mut self, letter:String, player: u8) {
        if(letter == "T"){
            self.set_alive();
        }
        else{
            self.set_green();
        }
        if(player == 1){
            self.set_palive();
        }
        else{
            self.set_pgreen();
        }
    }

}