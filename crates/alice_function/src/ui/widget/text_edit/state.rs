use crate::ui::{context::Context, id::Id};

use super::cursor::Cursor;

#[derive(Debug,Clone)]
pub struct EditState {
    cursor:Option<Cursor>
}

impl EditState {
    pub fn load(ctx:&Context , id:Id) -> Option<Self> {
        ctx.borrow_mut().data().get_persisted(id)
    }

    pub fn save(self , ctx:&Context,id:Id) {
        ctx.borrow_mut().data().insert_persisted(id, self)
    }

    pub fn set_cursor( &mut self , cursor:Option<Cursor> ){
        self.cursor = cursor;
    }

    pub fn cursor_range(&self) -> Option<Cursor> {
        self.cursor.or_else(||{
            self.cursor.map(|cursor| Cursor::new(cursor.index) )
        })
    }

}