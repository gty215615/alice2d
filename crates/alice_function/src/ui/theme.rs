use alice_core::color::Rgba;



pub struct Theme {
    pub primary_color:          Rgba,
    pub primary_color_hover:    Rgba,
    pub primary_color_active:   Rgba,
    pub primary_color_outline:  Rgba,

    pub success_color:          Rgba,
    pub success_color_hover:    Rgba,
    pub success_color_active:   Rgba,
    pub success_color_outline:  Rgba,
    pub success_color_bg:       Rgba,
    pub success_color_border:   Rgba,


    pub error_color:            Rgba,
    pub error_color_hover:      Rgba,
    pub error_color_active:     Rgba,
    pub error_color_outline:    Rgba,
    pub error_color_bg:         Rgba,
    pub error_color_border:     Rgba,

    pub warning_color:          Rgba,
    pub warning_color_hover:    Rgba,
    pub warning_color_active:   Rgba,
    pub warning_color_outline:  Rgba,
    pub warning_color_bg:       Rgba,
    pub warning_color_border:   Rgba,

    pub info_color:             Rgba,
    pub info_color_bg:          Rgba,
    pub info_color_border:      Rgba,

    pub default_color:          Rgba,
    pub danger_color:           Rgba,
    pub disabled_bg_color:      Rgba,
    pub disabled_border_color:  Rgba,
}

impl Default for Theme {
    fn default() -> Self {
        Self { 
            primary_color:              Rgba((24,144,255,255)), 
            primary_color_hover:        Rgba((64,169,255,255)), 
            primary_color_active:       Rgba(( 9,109,217,255)), 
            primary_color_outline:      Rgba((24,144,255,51 )), 
            
            success_color:              Rgba((82,196, 26,255)), 
            success_color_hover:        Rgba((115,209,61,255)), 
            success_color_active:       Rgba((56,158,13 ,255)), 
            success_color_outline:      Rgba((82,196,26 ,51 )), 
            success_color_bg:           Rgba((246,255,237,255)), 
            success_color_border:       Rgba((183,235,143,255)), 
            
            error_color: Rgba((24,144,255,255)), 
            error_color_hover: Rgba((24,144,255,255)), 
            error_color_active: Rgba((24,144,255,255)),
            error_color_outline: Rgba((24,144,255,255)), 
            error_color_bg: Rgba((24,144,255,255)), 
            error_color_border: Rgba((24,144,255,255)), 
            
            warning_color: Rgba((24,144,255,255)), 
            warning_color_hover: Rgba((24,144,255,255)), 
            warning_color_active: Rgba((24,144,255,255)), 
            warning_color_outline: Rgba((24,144,255,255)), 
            warning_color_bg: Rgba((24,144,255,255)), 
            warning_color_border: Rgba((24,144,255,255)), 
            
            info_color: Rgba((24,144,255,255)), 
            info_color_bg: Rgba((24,144,255,255)), 
            info_color_border: Rgba((24,144,255,255)), 
            
            default_color: Rgba((24,144,255,255)), 
            danger_color: Rgba((24,144,255,255)), 
            disabled_bg_color: Rgba((24,144,255,255)), 
            disabled_border_color: Rgba((24,144,255,255)) }
    }
}