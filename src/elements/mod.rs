pub mod color;
pub mod status_element;

use crate::world::board::Tile;
use crate::world::{board::Board, ZZTPoint};
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

pub type Glyph = u8;
type GlyphFunc = fn(board: &Board, tile: &Tile, point: ZZTPoint, tick: usize) -> Glyph;

pub struct Element {
    pub element_type: ElementType,
    pub glyph: Glyph,
    pub color: Color,
    pub glyph_func: Option<GlyphFunc>,
}

pub const ELEMENTS: [Element; 54] = [
    Element {
        element_type: ElementType::Empty,
        glyph: 0x20,
        color: Color::new(0x0F),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::BoardEdge,
        glyph: 0x00,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Messenger,
        glyph: 0x20,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Monitor,
        glyph: 0x20,
        color: Color::new(0x07),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Player,
        glyph: 0x02,
        color: Color::new(0x1F),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Ammo,
        glyph: 0x84,
        color: Color::new(0x03),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Torch,
        glyph: 0x9D,
        color: Color::new(0x06),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Gem,
        glyph: 0x04,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Key,
        glyph: 0x0C,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Door,
        glyph: 0x0A,
        color: Color::new(0xFE),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Scroll,
        glyph: 0xE8,
        color: Color::new(0x0F),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Passage,
        glyph: 0xF0,
        color: Color::new(0xFE),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Duplicator,
        glyph: 0xFA,
        color: Color::new(0x0F),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Bomb,
        glyph: 0x0B,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Energizer,
        glyph: 0x7F,
        color: Color::new(0x05),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Star,
        glyph: 0x53,
        color: Color::new(0x0F),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Clockwise,
        glyph: 0x2F,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Counter,
        glyph: 0x5C,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Bullet,
        glyph: 0xF8,
        color: Color::new(0x0F),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Water,
        glyph: 0xB0,
        color: Color::new(0xF9),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Forest,
        glyph: 0xB0,
        color: Color::new(0x20),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Solid,
        glyph: 0xDB,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Normal,
        glyph: 0xB2,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Breakable,
        glyph: 0xB1,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Boulder,
        glyph: 0xFE,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::SliderNS,
        glyph: 0x12,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::SliderEW,
        glyph: 0x1D,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Fake,
        glyph: 0xB2,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Invisible,
        glyph: 0xB0,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::BlinkWall,
        glyph: 0xCE,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Transporter,
        glyph: 0xC5,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Line,
        glyph: 0xCE,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Ricochet,
        glyph: 0x2A,
        color: Color::new(0x0A),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::BlinkRayHorizontal,
        glyph: 0xCD,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Bear,
        glyph: 0x99,
        color: Color::new(0x06),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Ruffian,
        glyph: 0x05,
        color: Color::new(0x0D),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Object,
        glyph: 0x02,
        color: Color::new(0xFF),
        glyph_func: Some(object_glyph),
    },
    Element {
        element_type: ElementType::Slime,
        glyph: 0x2A,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Shark,
        glyph: 0x5E,
        color: Color::new(0x07),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::SpinningGun,
        glyph: 0x18,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Pusher,
        glyph: 0x10,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Lion,
        glyph: 0xEA,
        color: Color::new(0x0C),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Tiger,
        glyph: 0xE3,
        color: Color::new(0x0B),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::BlinkRayVertical,
        glyph: 0xBA,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Head,
        glyph: 0xE9,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Segment,
        glyph: 0x4F,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::Invalid,
        glyph: 0x20,
        color: Color::new(0xFF),
        glyph_func: None,
    },
    Element {
        element_type: ElementType::TextBlue,
        glyph: 0x20,
        color: Color::new(0x1F),
        glyph_func: Some(text_glyph),
    },
    Element {
        element_type: ElementType::TextGreen,
        glyph: 0x20,
        color: Color::new(0x2F),
        glyph_func: Some(text_glyph),
    },
    Element {
        element_type: ElementType::TextCyan,
        glyph: 0x20,
        color: Color::new(0x3F),
        glyph_func: Some(text_glyph),
    },
    Element {
        element_type: ElementType::TextRed,
        glyph: 0x20,
        color: Color::new(0x4F),
        glyph_func: Some(text_glyph),
    },
    Element {
        element_type: ElementType::TextPurple,
        glyph: 0x20,
        color: Color::new(0x5F),
        glyph_func: Some(text_glyph),
    },
    Element {
        element_type: ElementType::TextBrown,
        glyph: 0x20,
        color: Color::new(0x6F),
        glyph_func: Some(text_glyph),
    },
    Element {
        element_type: ElementType::TextBlack,
        glyph: 0x20,
        color: Color::new(0x0F),
        glyph_func: Some(text_glyph),
    },
];

fn text_glyph(_board: &Board, tile: &Tile, _point: ZZTPoint, _tick: usize) -> Glyph {
    tile.color
}

fn object_glyph(board: &Board, _tile: &Tile, point: ZZTPoint, _tick: usize) -> Glyph {
    board.status_at(point).p1
}