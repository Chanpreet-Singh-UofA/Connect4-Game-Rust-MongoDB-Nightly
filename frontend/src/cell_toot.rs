#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Alive,
    Dead,
    Green,
}

#[derive(Clone, Copy)]
pub struct Cellule {
    pub state: State,
}

impl Cellule {
    pub fn new_dead() -> Self {
        Self { state: State::Dead }
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

    pub fn toggle(&mut self) {
        if self.is_dead() {
            self.set_alive()
        } else if self.is_alive() {
            self.set_green()
        }
        else{
            self.set_dead()
        }
    }

}