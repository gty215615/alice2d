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