#![allow(dead_code)]

trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button: {}", self.label);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a select box with options: {:?}", self.options);
    }
}

fn main() {
    let submit_button = Box::new(Button {
        label: "Submit".to_string(),
        width: 100,
        height: 50,
    });

    let select_box = Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
        ],
    });

    let screen = Screen {
        components: vec![submit_button, select_box],
    };

    screen.run();
}
