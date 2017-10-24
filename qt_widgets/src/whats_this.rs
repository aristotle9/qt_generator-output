/// C++ type: <span style='color: green;'>```QWhatsThis```</span>
#[repr(C)]
pub struct WhatsThis(u8);

impl WhatsThis {
  /// C++ method: <span style='color: green;'>```static QAction* QWhatsThis::createAction()```</span>
  ///
  ///
  pub fn create_action() -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QWhatsThis_createAction_no_args() }
  }

  /// C++ method: <span style='color: green;'>```static QAction* QWhatsThis::createAction(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn create_action_unsafe(parent: *mut ::qt_core::object::Object) -> *mut ::action::Action {
    ::ffi::qt_widgets_c_QWhatsThis_createAction_parent(parent)
  }

  /// C++ method: <span style='color: green;'>```static void QWhatsThis::enterWhatsThisMode()```</span>
  ///
  ///
  pub fn enter_whats_this_mode() {
    unsafe { ::ffi::qt_widgets_c_QWhatsThis_enterWhatsThisMode() }
  }

  /// C++ method: <span style='color: green;'>```static void QWhatsThis::hideText()```</span>
  ///
  ///
  pub fn hide_text() {
    unsafe { ::ffi::qt_widgets_c_QWhatsThis_hideText() }
  }

  /// C++ method: <span style='color: green;'>```static bool QWhatsThis::inWhatsThisMode()```</span>
  ///
  ///
  pub fn in_whats_this_mode() -> bool {
    unsafe { ::ffi::qt_widgets_c_QWhatsThis_inWhatsThisMode() }
  }

  /// C++ method: <span style='color: green;'>```static void QWhatsThis::leaveWhatsThisMode()```</span>
  ///
  ///
  pub fn leave_whats_this_mode() {
    unsafe { ::ffi::qt_widgets_c_QWhatsThis_leaveWhatsThisMode() }
  }

  /// C++ method: <span style='color: green;'>```static void QWhatsThis::showText(const QPoint& pos, const QString& text)```</span>
  ///
  ///
  pub fn show_text(pos: &::qt_core::point::Point, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWhatsThis_showText_pos_text(pos as *const ::qt_core::point::Point,
                                                       text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static void QWhatsThis::showText(const QPoint& pos, const QString& text, QWidget* w = ?)```</span>
  ///
  ///
  pub unsafe fn show_text_unsafe(pos: &::qt_core::point::Point,
                                 text: &::qt_core::string::String,
                                 w: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QWhatsThis_showText_pos_text_w(pos as *const ::qt_core::point::Point,
                                                       text as *const ::qt_core::string::String,
                                                       w)
  }
}

impl ::cpp_utils::CppDeletable for ::whats_this::WhatsThis {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QWhatsThis_delete
  }
}
