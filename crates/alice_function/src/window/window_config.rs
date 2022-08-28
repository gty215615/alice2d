

pub struct MainWindowConfig {
  pub  title:String,
  pub  width:u32,
  pub  height:u32,
  pub  is_maximized:bool,
  pub  is_fullscreen:bool,

}


impl MainWindowConfig {
    pub fn new(title:String,width:u32,height:u32) -> Self {
        Self {
            title,
            width,
            height,
            is_maximized:false,
            is_fullscreen:false,
        }
    }
}







