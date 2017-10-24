/// C++ type: <span style='color: green;'>```QToolTip```</span>
#[repr(C)]
pub struct ToolTip(u8);

impl ToolTip {
  /// C++ method: <span style='color: green;'>```static QFont QToolTip::font()```</span>
  ///
  ///
  pub fn font() -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QToolTip_font_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static void QToolTip::hideText()```</span>
  ///
  ///
  pub fn hide_text() {
    unsafe { ::ffi::qt_widgets_c_QToolTip_hideText() }
  }

  /// C++ method: <span style='color: green;'>```static bool QToolTip::isVisible()```</span>
  ///
  ///
  pub fn is_visible() -> bool {
    unsafe { ::ffi::qt_widgets_c_QToolTip_isVisible() }
  }

  /// C++ method: <span style='color: green;'>```static QPalette QToolTip::palette()```</span>
  ///
  ///
  pub fn palette() -> ::qt_gui::palette::Palette {
    {
      let mut object: ::qt_gui::palette::Palette =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QToolTip_palette_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static void QToolTip::setFont(const QFont& arg1)```</span>
  ///
  ///
  pub fn set_font(arg1: &::qt_gui::font::Font) {
    unsafe { ::ffi::qt_widgets_c_QToolTip_setFont(arg1 as *const ::qt_gui::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```static void QToolTip::setPalette(const QPalette& arg1)```</span>
  ///
  ///
  pub fn set_palette(arg1: &::qt_gui::palette::Palette) {
    unsafe { ::ffi::qt_widgets_c_QToolTip_setPalette(arg1 as *const ::qt_gui::palette::Palette) }
  }

  /// C++ method: <span style='color: green;'>```static void QToolTip::showText(const QPoint& pos, const QString& text)```</span>
  ///
  ///
  pub fn show_text(pos: &::qt_core::point::Point, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QToolTip_showText_pos_text(pos as *const ::qt_core::point::Point,
                                                     text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QToolTip::showText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn show_text_unsafe((&::qt_core::point::Point, &::qt_core::string::String, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QToolTip::showText(const QPoint& pos, const QString& text, QWidget* w = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn show_text_unsafe((&::qt_core::point::Point, &::qt_core::string::String, *mut ::widget::Widget, &::qt_core::rect::Rect)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QToolTip::showText(const QPoint& pos, const QString& text, QWidget* w, const QRect& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn show_text_unsafe((&::qt_core::point::Point, &::qt_core::string::String, *mut ::widget::Widget, &::qt_core::rect::Rect, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QToolTip::showText(const QPoint& pos, const QString& text, QWidget* w, const QRect& rect, int msecShowTime)```</span>
  ///
  ///
  pub unsafe fn show_text_unsafe<Args>(args: Args) -> ()
    where Args: overloading::ToolTipShowTextUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QString QToolTip::text()```</span>
  ///
  ///
  pub fn text() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QToolTip_text_to_output(&mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::tool_tip::ToolTip {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QToolTip_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ToolTip::show_text_unsafe](../struct.ToolTip.html#method.show_text_unsafe) method.
  pub trait ToolTipShowTextUnsafeArgs {
    unsafe fn exec(self) -> ();
  }
  impl<'a> ToolTipShowTextUnsafeArgs
    for (&'a ::qt_core::point::Point, &'a ::qt_core::string::String, *mut ::widget::Widget) {
    unsafe fn exec(self) -> () {
      let pos = self.0;
      let text = self.1;
      let w = self.2;
      ::ffi::qt_widgets_c_QToolTip_showText_pos_text_w(pos as *const ::qt_core::point::Point,
                                                       text as *const ::qt_core::string::String,
                                                       w)
    }
  }
  impl<'a> ToolTipShowTextUnsafeArgs
    for (&'a ::qt_core::point::Point, &'a ::qt_core::string::String, *mut ::widget::Widget, &'a ::qt_core::rect::Rect) {
    unsafe fn exec(self) -> () {
      let pos = self.0;
      let text = self.1;
      let w = self.2;
      let rect = self.3;
      ::ffi::qt_widgets_c_QToolTip_showText_pos_text_w_rect(pos as *const ::qt_core::point::Point,
                                                            text as *const ::qt_core::string::String,
                                                            w,
                                                            rect as *const ::qt_core::rect::Rect)
    }
  }
  impl<'a> ToolTipShowTextUnsafeArgs
    for (&'a ::qt_core::point::Point,
                                              &'a ::qt_core::string::String,
                                              *mut ::widget::Widget,
                                              &'a ::qt_core::rect::Rect,
                                              ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let pos = self.0;
      let text = self.1;
      let w = self.2;
      let rect = self.3;
      let msec_show_time = self.4;
      ::ffi::qt_widgets_c_QToolTip_showText_pos_text_w_rect_msecShowTime(pos as *const ::qt_core::point::Point,
                                                                         text as *const ::qt_core::string::String,
                                                                         w,
                                                                         rect as *const ::qt_core::rect::Rect,
                                                                         msec_show_time)
    }
  }
}
