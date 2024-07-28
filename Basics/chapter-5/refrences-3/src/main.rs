struct Anime {
    name: &'static str,
    pass: bool,
}

fn main() {
    let aria = Anime {
        name: "Aria: The animation",
        pass: true,
    };

    assert_eq!(aria.name, "Aria: The animation");
    assert_eq!(aria.pass, true);
}
