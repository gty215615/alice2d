use std::{rc::Rc, cell::{RefCell, RefMut}, ops::Deref};

use alice_core::{color::Color, math::Vector2f};

use crate::{paint::{display::Layout, shape::Shape, mesh::Mesh, path::Path}, window::winit_window::WinitWindow, input::{input_state::InputContext, input_system::{InputSystem, self}}, geometry::rect::Rect};

use super::{painter::{Painter, draw_rect, draw_row_text, draw_icon}, layer::{PaintJob, Layer}, response::{Response, self}, theme::Theme, store::Store, utils::id_type_map::IdTypeMap, id::Id};



#[derive(Clone)]
pub struct Context(Rc<RefCell<ContextInstance>>);

impl Deref for Context {
    type Target = Rc<RefCell<ContextInstance>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Context {
    pub fn new(window:&WinitWindow , input:Rc<RefCell<InputSystem>>) -> Self {
        Self(Rc::new(RefCell::new(ContextInstance::new(window,input))))
    }




    pub(super) fn interact(&mut self , id:Id, rect:Rect) -> Response {
        let hovered = self.rect_contains_points(rect);

        self.interact_with_hovered(id,rect, hovered)
    }

    fn interact_with_hovered(&mut self , id:Id, rect:Rect , hovered:bool) -> Response {
        let hovered = hovered; // TODO enable

        let mut response = Response {
            ctx:self.clone(),
            hovered,
            id,
            clicked: Default::default(),
            double_clicked: Default::default(),
            triple_clicked: Default::default(),
            is_pointer_button_down:false,
            interact_pointer_pos: None,
            changed: Default::default(),
            
        };

        self.want_focus(id);

        let ctx_instance = &mut *self.0.borrow_mut();
        let store = &mut ctx_instance.store;
        let input = &mut ctx_instance.input.borrow();
        response.is_pointer_button_down =  store.interaction.click_id == Some(id);
        
     
        for event in &input.ctx.pointer.pointer_events {
            match event {
                crate::input::input_state::PointerEvent::Pressed(_)=>{
                    if hovered {
                        if store.interaction.click_id.is_none() {
                          
                            store.interaction.click_id = Some(id);
                            response.is_pointer_button_down = true;
                            
                        } 
                    }
                }

                crate::input::input_state::PointerEvent::Released(click)=>{
                    if hovered {
                        if let Some(click) = click {
                            let clicked = hovered && response.is_pointer_button_down;
                            response.clicked[click.button as usize] = clicked;
                            response.double_clicked[click.button as usize] = clicked && click.is_double();
                            response.triple_clicked[click.button as usize] = clicked && click.is_triple();

                        }
                    }
                }
                _ => {}
            }
        }

        response
    }

    fn want_focus(&mut self , id:Id){
        self.0.borrow_mut().store.want_focus(id);
    }

    fn rect_contains_points(&self , rect:Rect) -> bool {
        let point_pos = self.0.borrow().input.borrow().ctx.pointer.latest_pos();

        if let Some(pos) = point_pos {
            rect.container(pos)
        }else{
            false
        }
    }
}
// Rc<RefCell<Context>>
pub struct ContextInstance {
    pub layout:Layout,
    pub painter:Painter,
    pub input:Rc<RefCell<InputSystem>>,
    pub theme:Theme ,
    pub store:Store

}

impl ContextInstance {
    pub fn new( window:&WinitWindow  , input:Rc<RefCell<InputSystem>>) -> Self {
        let size = window.get_size();

        let layout = Layout::new(Vector2f::ZERO ,size.x as f32, size.y as f32);

        Self { layout, painter:Painter::new() , input , theme:Theme::default() , store:Store::default()  }
    }

    pub fn begin_frame(&mut self) {
        self.layout.clear();
        let input_ctx = &self.input.borrow().ctx;
        let raw_input = &self.input.borrow().raw_input;
        self.store.begin_frame(input_ctx, raw_input)

    }


    pub fn end_frame(&mut self ) -> Vec<Mesh> {

        self.painter.layers.sort_by(|a,b| a.z_index.cmp(&b.z_index) );

        let mut meshs = Vec::with_capacity(self.painter.layers.len());
        let mut path = Path::new();
        for layer in self.painter.layers.iter_mut() {
            let commands = layer.drain();
            let mut mesh = Mesh::default();
            for shape in commands {
                match shape {
                    Shape::Rect(shape) => {
                        draw_rect(&mut path,1.0, shape, &mut mesh);
                    }

                    Shape::Text(shape)=>{
                        draw_row_text(&mut self.painter.font,&shape.text,shape.pos,&mut mesh);
                    }

                    Shape::Icon(shape)=>{
                        draw_icon(&mut self.painter.font,&shape.path,shape.pos,&mut mesh);
                    }


                    _ => {}
                }
            }

            meshs.push(mesh)

        }

        meshs

    }

    pub fn data(&mut self) -> &mut IdTypeMap {
        &mut self.store.data
    }

  
 
}