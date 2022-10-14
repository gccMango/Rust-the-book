trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("hello~ I'm Pilot");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Go fly~");
    }
}

impl Human {
    fn fly(&self) {
        println!("*Swing the arms*");
    }
}

fn main() {
    let person = Human;
    person.fly();

    //explicity trait name :: associated function
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
