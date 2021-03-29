pub mod color;

use color::Color;

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum ElementType {
    Empty = 0,
    BoardEdge,
    Messenger,
    Monitor,
    Player,
    Ammo,
    Torch,
    Gem,
    Key,
    Door,
    Scroll,
    Passage,
    Duplicator,
    Bomb,
    Energizer,
    Star,
    Clockwise,
    Counter,
    Bullet,
    Water,
    Forest,
    Solid,
    Normal,
    Breakable,
    Boulder,
    SliderNS,
    SliderEW,
    Fake,
    Invisible,
    BlinkWall,
    Transporter,
    Line,
    Ricochet,
    BlinkRayHorizontal,
    Bear,
    Ruffian,
    Object,
    Slime,
    Shark,
    SpinningGun,
    Pusher,
    Lion,
    Tiger,
    BlinkRayVertical,
    Head,
    Segment,
    Invalid,
    TextBlue,
    TextGreen,
    TextCyan,
    TextRed,
    TextPurple,
    TextBrown,
    TextBlack,
}

#[derive(Debug)]
pub struct Element {
    pub element_type: ElementType,
    pub character: u8,
    pub color: Color,
}

pub const ELEMENTS: Vec<Element> = vec![
    Element {
        element_type: ElementType::Empty,
        character: 0x20,
        color: Color::new(0x70),
    },
    Element {
        element_type: ElementType::BoardEdge,
        character: 0x20,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Messenger,
        character: 0x20,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Monitor,
        character: 0x20,
        color: Color::new(0x07),
    },
    Element {
        element_type: ElementType::Player,
        character: 0x02,
        color: Color::new(0x1F),
    },
    Element {
        element_type: ElementType::Ammo,
        character: 0x84,
        color: Color::new(0x03),
    },
    Element {
        element_type: ElementType::Torch,
        character: 0x9D,
        color: Color::new(0x06),
    },
    Element {
        element_type: ElementType::Gem,
        character: 0x04,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Key,
        character: 0x0C,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Door,
        character: 0x0A,
        color: Color::new(0xFE),
    },
    Element {
        element_type: ElementType::Scroll,
        character: 0xE8,
        color: Color::new(0x0F),
    },
    Element {
        element_type: ElementType::Passage,
        character: 0xF0,
        color: Color::new(0xFE),
    },
    Element {
        element_type: ElementType::Duplicator,
        character: 0xFA,
        color: Color::new(0x0F),
    },
    Element {
        element_type: ElementType::Bomb,
        character: 0x0B,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Energizer,
        character: 0x7F,
        color: Color::new(0x05),
    },
    Element {
        element_type: ElementType::Star,
        character: 0x53,
        color: Color::new(0x0F),
    },
    Element {
        element_type: ElementType::Clockwise,
        character: 0x2F,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Counter,
        character: 0x5C,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Bullet,
        character: 0xF8,
        color: Color::new(0x0F),
    },
    Element {
        element_type: ElementType::Water,
        character: 0xB0,
        color: Color::new(0xF9),
    },
    Element {
        element_type: ElementType::Forest,
        character: 0xB0,
        color: Color::new(0x20),
    },
    Element {
        element_type: ElementType::Solid,
        character: 0xDB,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Normal,
        character: 0xB2,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Breakable,
        character: 0xB1,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Boulder,
        character: 0xFE,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::SliderNS,
        character: 0x12,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::SliderEW,
        character: 0x1D,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Fake,
        character: 0xB2,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Invisible,
        character: 0xB0,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::BlinkWall,
        character: 0xCE,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Transporter,
        character: 0xC5,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Line,
        character: 0xCE,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Ricochet,
        character: 0x2A,
        color: Color::new(0x0A),
    },
    Element {
        element_type: ElementType::BlinkRayHorizontal,
        character: 0xCD,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Bear,
        character: 0x99,
        color: Color::new(0x06),
    },
    Element {
        element_type: ElementType::Ruffian,
        character: 0x05,
        color: Color::new(0x0D),
    },
    Element {
        element_type: ElementType::Object,
        character: 0x02,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Slime,
        character: 0x2A,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Shark,
        character: 0x5E,
        color: Color::new(0x07),
    },
    Element {
        element_type: ElementType::SpinningGun,
        character: 0x18,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Pusher,
        character: 0x10,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Lion,
        character: 0xEA,
        color: Color::new(0x0C),
    },
    Element {
        element_type: ElementType::Tiger,
        character: 0xE3,
        color: Color::new(0x0B),
    },
    Element {
        element_type: ElementType::BlinkRayVertical,
        character: 0xBA,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Head,
        character: 0xE9,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Segment,
        character: 0x4F,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::Invalid,
        character: 0x20,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::TextBlue,
        character: 0x20,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::TextGreen,
        character: 0x20,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::TextCyan,
        character: 0x20,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::TextRed,
        character: 0x20,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::TextPurple,
        character: 0x20,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::TextBrown,
        character: 0x20,
        color: Color::new(0xFF),
    },
    Element {
        element_type: ElementType::TextBlack,
        character: 0x20,
        color: Color::new(0xFF),
    },
];
