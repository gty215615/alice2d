use std::ops::Deref;



#[derive(Debug,Default,Clone, Copy , PartialEq)]
pub struct Color((f32,f32,f32,f32));

impl Color {
    pub const WHITE:Color = Color((1.0,1.0,1.0,1.0));
    pub const BLACK:Color = Color((0.0,0.0,0.0,1.0));
    pub const RED:  Color = Color((1.0,0.0,0.0,1.0));
    pub const GREEN:Color = Color((0.0,1.0,0.0,1.0));
    pub const BLUE: Color = Color((0.0,0.0,1.0,1.0));
    pub const TRANSPARENT:Color = Color((0.0,0.0,0.0,0.0));

    pub fn from_rgba(r:f32,g:f32,b:f32,a:f32) -> Self {
        Color((r,g,b,a))
    }

    pub fn from_rgb(r:f32,g:f32,b:f32) -> Self {
        Color((r,g,b,1.0))
    }

    pub fn from_f32_arr(f32_set:[f32;3]) -> Self {
        Color((f32_set[0],f32_set[1],f32_set[2],1.0))
    }

    pub fn is_transparent(&self) -> bool {
        self.0.3 == 0.0
    }
    
}
unsafe impl bytemuck::Pod for Color {}
unsafe impl bytemuck::Zeroable for Color {}


impl From<Rgba> for Color {
    fn from(c: Rgba) -> Self {
        Self((
            linear_f32_from_gamma_u8(c.0.0),
            linear_f32_from_gamma_u8(c.0.1),
            linear_f32_from_gamma_u8(c.0.2),
            linear_f32_from_linear_u8(c.0.3)
        ))
    }
}

impl Deref for Color {
    type Target = (f32,f32,f32,f32);

    fn deref(&self) -> &Self::Target {
        &self.0
    }

  
}


impl std::ops::Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Color((self.0.0 * rhs, self.0.1 * rhs, self.0.2 * rhs, self.0.3 * rhs))
    }
}


#[allow(clippy::derive_hash_xor_eq)]
impl std::hash::Hash for Color {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        super::f32_hash(state, self.0.0);
        super::f32_hash(state, self.0.1);
        super::f32_hash(state, self.0.2);
        super::f32_hash(state, self.0.3);
    }
}


#[derive(Debug,Default,Clone, Copy , PartialEq)]
pub struct Rgba(pub (u8,u8,u8,u8));

impl Rgba {
    pub const TRANSPARENT:Rgba = Rgba((0,0,0,0));
    pub const WHITE:Rgba = Rgba((255,255,255,255));
    pub const MIDDLE_SPRING_GREEN:Rgba = Rgba(( 113 , 247 , 159 , 255 ));
    pub const MIDDLE_TURQUOISE:Rgba = Rgba(( 61 , 214 , 208 , 255 ));
    pub const KEPPEL:Rgba = Rgba(( 21 , 176 , 151 , 255 ));
    pub const CAFE_NOIR:Rgba = Rgba(( 81 , 60 , 44 , 255 ));
    pub const BISTRE:Rgba = Rgba(( 40 , 25 , 14 , 255 ));
    pub const BLACK:Rgba = Rgba(( 0 , 0 , 0 , 255 ));
    
}

impl From<Color> for Rgba {
    fn from(c: Color) -> Self {
        Self((
            gamma_u8_from_linear_f32(c.0.0),
            gamma_u8_from_linear_f32(c.0.1),
            gamma_u8_from_linear_f32(c.0.2),
            linear_u8_from_linear_f32(c.0.3)
        ))
    }
}

/// gamma [0, 255] -> linear [0, 1].
pub fn linear_f32_from_gamma_u8(s: u8) -> f32 {
    if s <= 10 {
        s as f32 / 3294.6
    } else {
        ((s as f32 + 14.025) / 269.025).powf(2.4)
    }
}

/// linear [0, 255] -> linear [0, 1].
/// Useful for alpha-channel.
#[inline(always)]
pub fn linear_f32_from_linear_u8(a: u8) -> f32 {
    a as f32 / 255.0
}

/// linear [0, 1] -> gamma [0, 255] (clamped).
/// Values outside this range will be clamped to the range.
pub fn gamma_u8_from_linear_f32(l: f32) -> u8 {
    if l <= 0.0 {
        0
    } else if l <= 0.0031308 {
        fast_round(3294.6 * l)
    } else if l <= 1.0 {
        fast_round(269.025 * l.powf(1.0 / 2.4) - 14.025)
    } else {
        255
    }
}

/// linear [0, 1] -> linear [0, 255] (clamped).
/// Useful for alpha-channel.
#[inline(always)]
pub fn linear_u8_from_linear_f32(a: f32) -> u8 {
    fast_round(a * 255.0)
}

fn fast_round(r: f32) -> u8 {
    (r + 0.5).floor() as _ // rust does a saturating cast since 1.45
}


/// gamma [0, 1] -> linear [0, 1] (not clamped).
/// Works for numbers outside this range (e.g. negative numbers).
pub fn linear_from_gamma(gamma: f32) -> f32 {
    if gamma < 0.0 {
        -linear_from_gamma(-gamma)
    } else if gamma <= 0.04045 {
        gamma / 12.92
    } else {
        ((gamma + 0.055) / 1.055).powf(2.4)
    }
}

/// linear [0, 1] -> gamma [0, 1] (not clamped).
/// Works for numbers outside this range (e.g. negative numbers).
pub fn gamma_from_linear(linear: f32) -> f32 {
    if linear < 0.0 {
        -gamma_from_linear(-linear)
    } else if linear <= 0.0031308 {
        12.92 * linear
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    }
}
