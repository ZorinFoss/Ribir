use super::CommonEvent;
use crate::impl_common_event_deref;
mod from_mouse;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointerId(usize);

/// The pointer is a hardware-agnostic device that can target a specific set of
/// screen coordinates.
///
/// Having a single event model for pointers can simplify creating Web sites and
/// applications and provide a good user experience regardless of the user's
/// hardware. However, for scenarios when device-specific handling is desired,
/// pointer events defines a pointerType property to inspect the device type
/// which produced the event. Reference: <https://developer.mozilla.org/en-US/docs/Web/API/Pointer_events#term_pointer_event>
#[derive(Debug)]
pub struct PointerEvent {
  /// A unique identifier for the pointer causing the event.
  pub id: PointerId,
  /// The width (magnitude on the X axis), in pixels, of the contact geometry of
  /// the pointer.
  pub width: f32,
  /// the height (magnitude on the Y axis), in pixels, of the contact geometry
  /// of the pointer.
  pub height: f32,
  /// the normalized pressure of the pointer input in the range of 0 to 1, where
  /// 0 and 1 represent the minimum and maximum pressure the hardware is capable
  /// of detecting, respectively. tangentialPressure
  /// The normalized tangential pressure of the pointer input (also known as
  /// barrel pressure or cylinder stress) in the range -1 to 1, where 0 is the
  /// neutral position of the control.
  pub pressure: f32,
  /// The plane angle (in degrees, in the range of -90 to 90) between the Y–Z
  /// plane and the plane containing both the pointer (e.g. pen stylus) axis and
  /// the Y axis.
  pub tilt_x: f32,
  /// The plane angle (in degrees, in the range of -90 to 90) between the X–Z
  /// plane and the plane containing both the pointer (e.g. pen stylus) axis and
  /// the X axis.
  pub tilt_y: f32,
  /// The clockwise rotation of the pointer (e.g. pen stylus) around its major
  /// axis in degrees, with a value in the range 0 to 359.
  pub twist: f32,
  ///  Indicates the device type that caused the event (mouse, pen, touch, etc.)
  pub point_type: PointerType,
  /// Indicates if the pointer represents the primary pointer of this pointer
  /// type.
  pub is_primary: bool,

  pub common: CommonEvent,
}

bitflags! {
  #[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
  pub struct MouseButtons: u8 {
    /// Primary button (usually the left button)
    const PRIMARY = 0b0000_0001;
    /// Secondary button (usually the right button)
    const SECONDARY = 0b0000_0010;
    /// Auxiliary button (usually the mouse wheel button or middle button)
    const AUXILIARY = 0b0000_0100;
    /// 4th button (typically the "Browser Back" button)
    const FOURTH = 0b0000_1000;
    /// 5th button (typically the "Browser Forward" button)
    const FIFTH = 0b0001_0000;
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PointerType {
  /// The event was generated by a mouse device.
  Mouse,
  /// The event was generated by a pen or stylus device.
  Pen,
  /// The event was generated by a touch, such as a finger.
  Touch,
}

impl_common_event_deref!(PointerEvent);
#[cfg(test)]
mod tests {

  use crate::{prelude::*, reset_test_env, test_helper::*};

  fn tap_on(wnd: &Window, x: f32, y: f32) {
    wnd.process_cursor_move(Point::new(x, y));

    wnd.process_mouse_press(Box::new(DummyDeviceId), MouseButtons::PRIMARY);
    wnd.process_mouse_release(Box::new(DummyDeviceId), MouseButtons::PRIMARY);
  }

  #[test]
  fn tap_focus() {
    reset_test_env!();

    let (tap, w_tap) = split_value(0);
    let (focused, w_focused) = split_value(false);

    let w = fn_widget! {
      let mut host = @MockMulti {};
      watch!(*$read(host.is_focused()))
        .subscribe(move |v| *$write(w_focused) = v);

      @(host) {
        @MockBox {
          size: Size::new(50., 50.,),
          on_tap: move |_| *$write(w_tap) += 1,
        }
        @MockBox {
          size: Size::new(50., 50.,),
          on_tap: move |_| *$write(w_tap) += 1,
          on_key_down: move |_| println!("dummy code"),
        }
      }
    };
    let wnd = TestWindow::new_with_size(w, Size::new(100., 100.));
    wnd.draw_frame();

    tap_on(&wnd, 25., 25.);
    wnd.draw_frame();
    assert_eq!(*tap.read(), 1);
    assert!(!*focused.read());

    tap_on(&wnd, 75., 25.);
    wnd.draw_frame();
    assert_eq!(*tap.read(), 2);
    assert!(*focused.read());
  }
}
