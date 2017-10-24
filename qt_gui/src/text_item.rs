/// C++ type: <span style='color: green;'>```QTextItem::RenderFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RenderFlag {
  /// C++ enum variant: <span style='color: green;'>```Dummy = -1```</span>
  Dummy = -1,
  /// C++ enum variant: <span style='color: green;'>```RightToLeft = 1```</span>
  RightToLeft = 1,
  /// C++ enum variant: <span style='color: green;'>```Overline = 16```</span>
  Overline = 16,
  /// C++ enum variant: <span style='color: green;'>```Underline = 32```</span>
  Underline = 32,
  /// C++ enum variant: <span style='color: green;'>```StrikeOut = 64```</span>
  StrikeOut = 64,
}

impl ::qt_core::flags::FlaggableEnum for RenderFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "RenderFlag"
  }
}

/// C++ type: <span style='color: green;'>```QTextItem```</span>
#[repr(C)]
pub struct TextItem([u8; ::type_sizes::QT_GUI_TEXT_ITEM_TEXT_ITEM]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextItem {
  unsafe fn new_uninitialized() -> TextItem {
    TextItem(::std::mem::uninitialized())
  }
}

impl TextItem {
  /// C++ method: <span style='color: green;'>```double QTextItem::ascent() const```</span>
  ///
  ///
  pub fn ascent(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextItem_ascent(self as *const ::text_item::TextItem) }
  }

  /// C++ method: <span style='color: green;'>```double QTextItem::descent() const```</span>
  ///
  ///
  pub fn descent(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextItem_descent(self as *const ::text_item::TextItem) }
  }

  /// C++ method: <span style='color: green;'>```QFont QTextItem::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::font::Font {
    {
      let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextItem_font_to_output(self as *const ::text_item::TextItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QTextItem::RenderFlag> QTextItem::renderFlags() const```</span>
  ///
  ///
  pub fn render_flags(&self) -> ::qt_core::flags::Flags<::text_item::RenderFlag> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextItem_renderFlags(self as *const ::text_item::TextItem) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```QString QTextItem::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextItem_text_to_output(self as *const ::text_item::TextItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QTextItem::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextItem_width(self as *const ::text_item::TextItem) }
  }
}

impl Drop for ::text_item::TextItem {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextItem::~QTextItem()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextItem_destructor(self as *mut ::text_item::TextItem) }
  }
}
