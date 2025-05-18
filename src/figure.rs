type FigureState = [&'static str; 12];

pub struct Figure;

impl Figure {
    const STATES: [FigureState; 7] = [
        Self::BASE,
        Self::HEAD,
        Self::NECK,
        Self::LEFT_ARM,
        Self::RIGHT_ARM,
        Self::LEFT_LEG,
        Self::RIGHT_LEG,
    ];

    pub fn get_state(strikes: usize) -> String {
        Self::STATES[strikes].join("\n")
    }

    const BASE: FigureState = [
        "================== ",
        "|                | ",
        "|                | ",
        "|                | ",
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "================== ",
    ];

    const HEAD: FigureState = [
        "================== ",
        "|                | ",
        "|                | ",
        "|                | ",
        "|                O ",
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "================== ",
    ];

    const NECK: FigureState = [
        "================== ",
        "|                | ",
        "|                | ",
        "|                | ",
        "|                O ",
        "|                | ",
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "================== ",
    ];

    const LEFT_ARM: FigureState = [
        "================== ",
        "|                | ",
        "|                | ",
        "|                | ",
        "|                O ",
        "|               /| ",
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "================== ",
    ];

    const RIGHT_ARM: FigureState = [
        "================== ",
        "|                | ",
        "|                | ",
        "|                | ",
        "|                O ",
      r#"|               /|\"#,
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "================== ",
    ];

    const LEFT_LEG: FigureState = [
        "================== ",
        "|                | ",
        "|                | ",
        "|                | ",
        "|                O ",
      r#"|               /|\"#,
        "|               /  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "================== ",
    ];

    const RIGHT_LEG: FigureState = [
        "================== ",
        "|                | ",
        "|                | ",
        "|                | ",
        "|                O ",
      r#"|               /|\"#,
      r#"|               / \"#,
        "|                  ",
        "|                  ",
        "|                  ",
        "|                  ",
        "================== ",
    ];
}
