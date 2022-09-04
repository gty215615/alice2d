use alice_core::math::Vector2f;

use super::{KeyCode, Modifiers, MouseButton};


#[derive(Clone, Debug, PartialEq)]
pub enum Event {
   /// The integration detected a "copy" event (e.g. Cmd+C).
   Copy,
   /// The integration detected a "cut" event (e.g. Cmd+X).
   Cut,
   /// The integration detected a "paste" event (e.g. Cmd+V).
   Paste(String),
   /// Text input, e.g. via keyboard.
   ///
   /// When the user presses enter/return, do not send a [`Text`](Event::Text) (just [`Key::Enter`]).
   Text(String),
   /// A key was pressed or released.
   Key {
       key: KeyCode,
       /// Was it pressed or released?
       pressed: bool,
       /// The state of the modifier keys at the time of the event.
       modifiers: Modifiers,
   },

   /// The mouse or touch moved to a new place.
   PointerMoved(Vector2f),

   /// A mouse button was pressed or released (or a touch started or stopped).
   PointerButton {
       /// Where is the pointer?
       pos: Vector2f,
       /// What mouse button? For touches, use [`PointerButton::Primary`].
       button: MouseButton,
       /// Was it the button/touch pressed this frame, or released?
       pressed: bool,
       /// The state of the modifier keys at the time of the event.
       modifiers: Modifiers,
   },
   /// The mouse left the screen, or the last/primary touch input disappeared.
   ///
   /// This means there is no longer a cursor on the screen for hovering etc.
   ///
   /// On touch-up first send `PointerButton{pressed: false, …}` followed by `PointerLeft`.
   PointerGone,

   /// How many points (logical pixels) the user scrolled.
   ///
   /// The direction of the vector indicates how to move the _content_ that is being viewed.
   /// So if you get positive values, the content being viewed should move to the right and down,
   /// revealing new things to the left and up.
   ///
   /// A positive X-value indicates the content is being moved right,
   /// as when swiping right on a touch-screen or track-pad with natural scrolling.
   ///
   /// A positive Y-value indicates the content is being moved down,
   /// as when swiping down on a touch-screen or track-pad with natural scrolling.
   ///
   /// Shift-scroll should result in horizontal scrolling (it is up to the integrations to do this).
   Scroll(Vector2f),

   /// Zoom scale factor this frame (e.g. from ctrl-scroll or pinch gesture).
   /// * `zoom = 1`: no change.
   /// * `zoom < 1`: pinch together
   /// * `zoom > 1`: pinch spread
   Zoom(f32),

   /// IME composition start.
   CompositionStart,
   /// A new IME candidate is being suggested.
   CompositionUpdate(String),
   /// IME composition ended with this final result.
   CompositionEnd(String),


}