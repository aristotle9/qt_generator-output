/// C++ type: <span style='color: green;'>```QMessageBox::ButtonRole```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ButtonRole {
  /// C++ enum variant: <span style='color: green;'>```InvalidRole = -1```</span>
  InvalidRole = -1,
  /// C++ enum variant: <span style='color: green;'>```AcceptRole = 0```</span>
  AcceptRole = 0,
  /// C++ enum variant: <span style='color: green;'>```RejectRole = 1```</span>
  RejectRole = 1,
  /// C++ enum variant: <span style='color: green;'>```DestructiveRole = 2```</span>
  DestructiveRole = 2,
  /// C++ enum variant: <span style='color: green;'>```ActionRole = 3```</span>
  ActionRole = 3,
  /// C++ enum variant: <span style='color: green;'>```HelpRole = 4```</span>
  HelpRole = 4,
  /// C++ enum variant: <span style='color: green;'>```YesRole = 5```</span>
  YesRole = 5,
  /// C++ enum variant: <span style='color: green;'>```NoRole = 6```</span>
  NoRole = 6,
  /// C++ enum variant: <span style='color: green;'>```ResetRole = 7```</span>
  ResetRole = 7,
  /// C++ enum variant: <span style='color: green;'>```ApplyRole = 8```</span>
  ApplyRole = 8,
  /// C++ enum variant: <span style='color: green;'>```NRoles = 9```</span>
  NRoles = 9,
}

/// C++ type: <span style='color: green;'>```QMessageBox::Icon```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Icon {
  /// C++ enum variant: <span style='color: green;'>```NoIcon = 0```</span>
  NoIcon = 0,
  /// C++ enum variant: <span style='color: green;'>```Information = 1```</span>
  Information = 1,
  /// C++ enum variant: <span style='color: green;'>```Warning = 2```</span>
  Warning = 2,
  /// C++ enum variant: <span style='color: green;'>```Critical = 3```</span>
  Critical = 3,
  /// C++ enum variant: <span style='color: green;'>```Question = 4```</span>
  Question = 4,
}

/// C++ type: <span style='color: green;'>```QMessageBox```</span>
#[repr(C)]
pub struct MessageBox(u8);

impl MessageBox {
  /// C++ method: <span style='color: green;'>```static void QMessageBox::about(QWidget* parent, const QString& title, const QString& text)```</span>
  ///
  ///
  pub unsafe fn about(parent: *mut ::widget::Widget,
                      title: &::qt_core::string::String,
                      text: &::qt_core::string::String) {
    ::ffi::qt_widgets_c_QMessageBox_about(parent,
                                          title as *const ::qt_core::string::String,
                                          text as *const ::qt_core::string::String)
  }

  /// C++ method: <span style='color: green;'>```QMessageBox::aboutQt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn about_qt(*mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QMessageBox::aboutQt(QWidget* parent)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn about_qt((*mut ::widget::Widget, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QMessageBox::aboutQt(QWidget* parent, const QString& title = ?)```</span>
  ///
  ///
  pub unsafe fn about_qt<Args>(args: Args) -> ()
    where Args: overloading::MessageBoxAboutQtArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMessageBox::addButton```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_button(&mut self, ::message_box::StandardButton) -> *mut ::push_button::PushButton```<br>
  /// C++ method: <span style='color: green;'>```QPushButton* QMessageBox::addButton(QMessageBox::StandardButton button)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_button(&mut self, (&::qt_core::string::String, ::message_box::ButtonRole)) -> *mut ::push_button::PushButton```<br>
  /// C++ method: <span style='color: green;'>```QPushButton* QMessageBox::addButton(const QString& text, QMessageBox::ButtonRole role)```</span>
  ///
  ///
  pub fn add_button<'largs, Args>(&'largs mut self, args: Args) -> *mut ::push_button::PushButton
    where Args: overloading::MessageBoxAddButtonArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QMessageBox::addButton(QAbstractButton* button, QMessageBox::ButtonRole role)```</span>
  ///
  ///
  pub unsafe fn add_button_unsafe(&mut self,
                                  button: *mut ::abstract_button::AbstractButton,
                                  role: ::message_box::ButtonRole) {
    ::ffi::qt_widgets_c_QMessageBox_addButton_button_role(self as *mut ::message_box::MessageBox, button, role)
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* QMessageBox::button(QMessageBox::StandardButton which) const```</span>
  ///
  ///
  pub fn button(&self, which: ::message_box::StandardButton) -> *mut ::abstract_button::AbstractButton {
    unsafe { ::ffi::qt_widgets_c_QMessageBox_button(self as *const ::message_box::MessageBox, which) }
  }

  /// C++ method: <span style='color: green;'>```QMessageBox::ButtonRole QMessageBox::buttonRole(QAbstractButton* button) const```</span>
  ///
  ///
  pub unsafe fn button_role(&self, button: *mut ::abstract_button::AbstractButton) -> ::message_box::ButtonRole {
    ::ffi::qt_widgets_c_QMessageBox_buttonRole(self as *const ::message_box::MessageBox, button)
  }

  /// C++ method: <span style='color: green;'>```QString QMessageBox::buttonText(int button) const```</span>
  ///
  ///
  pub fn button_text(&self, button: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMessageBox_buttonText_to_output(self as *const ::message_box::MessageBox,
                                                             button,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*> QMessageBox::buttons() const```</span>
  ///
  ///
  pub fn buttons(&self) -> ::list::ListAbstractButtonMutPtr {
    {
      let mut object: ::list::ListAbstractButtonMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMessageBox_buttons_to_output(self as *const ::message_box::MessageBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCheckBox* QMessageBox::checkBox() const```</span>
  ///
  ///
  pub fn check_box(&self) -> *mut ::check_box::CheckBox {
    unsafe { ::ffi::qt_widgets_c_QMessageBox_checkBox(self as *const ::message_box::MessageBox) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* QMessageBox::clickedButton() const```</span>
  ///
  ///
  pub fn clicked_button(&self) -> *mut ::abstract_button::AbstractButton {
    unsafe { ::ffi::qt_widgets_c_QMessageBox_clickedButton(self as *const ::message_box::MessageBox) }
  }

  /// C++ method: <span style='color: green;'>```QMessageBox::critical```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn critical((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String)) -> ::message_box::StandardButton```<br>
  /// C++ method: <span style='color: green;'>```static QMessageBox::StandardButton QMessageBox::critical(QWidget* parent, const QString& title, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn critical((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::qt_core::flags::Flags<::message_box::StandardButton>)) -> ::message_box::StandardButton```<br>
  /// C++ method: <span style='color: green;'>```static QMessageBox::StandardButton QMessageBox::critical(QWidget* parent, const QString& title, const QString& text, QFlags<QMessageBox::StandardButton> buttons = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn critical((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::qt_core::flags::Flags<::message_box::StandardButton>, ::message_box::StandardButton)) -> ::message_box::StandardButton```<br>
  /// C++ method: <span style='color: green;'>```static QMessageBox::StandardButton QMessageBox::critical(QWidget* parent, const QString& title, const QString& text, QFlags<QMessageBox::StandardButton> buttons = ?, QMessageBox::StandardButton defaultButton = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn critical((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::message_box::StandardButton, ::message_box::StandardButton)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::critical(QWidget* parent, const QString& title, const QString& text, QMessageBox::StandardButton button0, QMessageBox::StandardButton button1)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn critical((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::critical(QWidget* parent, const QString& title, const QString& text, const QString& button0Text)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn critical((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::critical(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn critical((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::critical(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?, const QString& button2Text = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn critical((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::critical(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?, const QString& button2Text = ?, int defaultButtonNumber = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn critical((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::critical(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?, const QString& button2Text = ?, int defaultButtonNumber = ?, int escapeButtonNumber = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn critical((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::critical(QWidget* parent, const QString& title, const QString& text, int button0, int button1)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn critical((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::critical(QWidget* parent, const QString& title, const QString& text, int button0, int button1, int button2 = ?)```</span>
  ///
  ///
  pub unsafe fn critical<Args>(args: Args) -> Args::ReturnType
    where Args: overloading::MessageBoxCriticalArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPushButton* QMessageBox::defaultButton() const```</span>
  ///
  ///
  pub fn default_button(&self) -> *mut ::push_button::PushButton {
    unsafe { ::ffi::qt_widgets_c_QMessageBox_defaultButton(self as *const ::message_box::MessageBox) }
  }

  /// C++ method: <span style='color: green;'>```QString QMessageBox::detailedText() const```</span>
  ///
  ///
  pub fn detailed_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMessageBox_detailedText_to_output(self as *const ::message_box::MessageBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* QMessageBox::escapeButton() const```</span>
  ///
  ///
  pub fn escape_button(&self) -> *mut ::abstract_button::AbstractButton {
    unsafe { ::ffi::qt_widgets_c_QMessageBox_escapeButton(self as *const ::message_box::MessageBox) }
  }

  /// C++ method: <span style='color: green;'>```QMessageBox::Icon QMessageBox::icon() const```</span>
  ///
  ///
  pub fn icon(&self) -> ::message_box::Icon {
    unsafe { ::ffi::qt_widgets_c_QMessageBox_icon(self as *const ::message_box::MessageBox) }
  }

  /// C++ method: <span style='color: green;'>```QPixmap QMessageBox::iconPixmap() const```</span>
  ///
  ///
  pub fn icon_pixmap(&self) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMessageBox_iconPixmap_as_ptr(self as *const ::message_box::MessageBox) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QMessageBox::information```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn information((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String)) -> ::message_box::StandardButton```<br>
  /// C++ method: <span style='color: green;'>```static QMessageBox::StandardButton QMessageBox::information(QWidget* parent, const QString& title, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn information((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::qt_core::flags::Flags<::message_box::StandardButton>)) -> ::message_box::StandardButton```<br>
  /// C++ method: <span style='color: green;'>```static QMessageBox::StandardButton QMessageBox::information(QWidget* parent, const QString& title, const QString& text, QFlags<QMessageBox::StandardButton> buttons = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn information((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::qt_core::flags::Flags<::message_box::StandardButton>, ::message_box::StandardButton)) -> ::message_box::StandardButton```<br>
  /// C++ method: <span style='color: green;'>```static QMessageBox::StandardButton QMessageBox::information(QWidget* parent, const QString& title, const QString& text, QFlags<QMessageBox::StandardButton> buttons = ?, QMessageBox::StandardButton defaultButton = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn information((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::message_box::StandardButton)) -> ::message_box::StandardButton```<br>
  /// C++ method: <span style='color: green;'>```static QMessageBox::StandardButton QMessageBox::information(QWidget* parent, const QString& title, const QString& text, QMessageBox::StandardButton button0)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn information((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::message_box::StandardButton, ::message_box::StandardButton)) -> ::message_box::StandardButton```<br>
  /// C++ method: <span style='color: green;'>```static QMessageBox::StandardButton QMessageBox::information(QWidget* parent, const QString& title, const QString& text, QMessageBox::StandardButton button0, QMessageBox::StandardButton button1 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn information((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::information(QWidget* parent, const QString& title, const QString& text, const QString& button0Text)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn information((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::information(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn information((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::information(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?, const QString& button2Text = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn information((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::information(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?, const QString& button2Text = ?, int defaultButtonNumber = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn information((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::information(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?, const QString& button2Text = ?, int defaultButtonNumber = ?, int escapeButtonNumber = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn information((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::information(QWidget* parent, const QString& title, const QString& text, int button0)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn information((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::information(QWidget* parent, const QString& title, const QString& text, int button0, int button1 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn information((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::information(QWidget* parent, const QString& title, const QString& text, int button0, int button1 = ?, int button2 = ?)```</span>
  ///
  ///
  pub unsafe fn information<Args>(args: Args) -> Args::ReturnType
    where Args: overloading::MessageBoxInformationArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString QMessageBox::informativeText() const```</span>
  ///
  ///
  pub fn informative_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMessageBox_informativeText_to_output(self as *const ::message_box::MessageBox,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QMessageBox::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QMessageBox_metaObject(self as *const ::message_box::MessageBox) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMessageBox::QMessageBox()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::message_box::MessageBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMessageBox_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMessageBox::QMessageBox(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::message_box::MessageBox> {
    let ffi_result = ::ffi::qt_widgets_c_QMessageBox_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::open(QObject* receiver, const char* member)```</span>
  ///
  ///
  pub unsafe fn open(&mut self, receiver: *mut ::qt_core::object::Object, member: *const ::libc::c_char) {
    ::ffi::qt_widgets_c_QMessageBox_open(self as *mut ::message_box::MessageBox, receiver, member)
  }

  /// C++ method: <span style='color: green;'>```virtual int QMessageBox::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QMessageBox_qt_metacall(self as *mut ::message_box::MessageBox,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QMessageBox::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QMessageBox_qt_metacast(self as *mut ::message_box::MessageBox, arg1)
  }

  /// C++ method: <span style='color: green;'>```QMessageBox::question```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn question((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String)) -> ::message_box::StandardButton```<br>
  /// C++ method: <span style='color: green;'>```static QMessageBox::StandardButton QMessageBox::question(QWidget* parent, const QString& title, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn question((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::qt_core::flags::Flags<::message_box::StandardButton>)) -> ::message_box::StandardButton```<br>
  /// C++ method: <span style='color: green;'>```static QMessageBox::StandardButton QMessageBox::question(QWidget* parent, const QString& title, const QString& text, QFlags<QMessageBox::StandardButton> buttons = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn question((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::qt_core::flags::Flags<::message_box::StandardButton>, ::message_box::StandardButton)) -> ::message_box::StandardButton```<br>
  /// C++ method: <span style='color: green;'>```static QMessageBox::StandardButton QMessageBox::question(QWidget* parent, const QString& title, const QString& text, QFlags<QMessageBox::StandardButton> buttons = ?, QMessageBox::StandardButton defaultButton = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn question((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::message_box::StandardButton, ::message_box::StandardButton)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::question(QWidget* parent, const QString& title, const QString& text, QMessageBox::StandardButton button0, QMessageBox::StandardButton button1)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn question((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::question(QWidget* parent, const QString& title, const QString& text, const QString& button0Text)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn question((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::question(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn question((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::question(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?, const QString& button2Text = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn question((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::question(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?, const QString& button2Text = ?, int defaultButtonNumber = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn question((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::question(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?, const QString& button2Text = ?, int defaultButtonNumber = ?, int escapeButtonNumber = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn question((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::question(QWidget* parent, const QString& title, const QString& text, int button0)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn question((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::question(QWidget* parent, const QString& title, const QString& text, int button0, int button1 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn question((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::question(QWidget* parent, const QString& title, const QString& text, int button0, int button1 = ?, int button2 = ?)```</span>
  ///
  ///
  pub unsafe fn question<Args>(args: Args) -> Args::ReturnType
    where Args: overloading::MessageBoxQuestionArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QMessageBox::removeButton(QAbstractButton* button)```</span>
  ///
  ///
  pub unsafe fn remove_button(&mut self, button: *mut ::abstract_button::AbstractButton) {
    ::ffi::qt_widgets_c_QMessageBox_removeButton(self as *mut ::message_box::MessageBox, button)
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setButtonText(int button, const QString& text)```</span>
  ///
  ///
  pub fn set_button_text(&mut self, button: ::libc::c_int, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QMessageBox_setButtonText(self as *mut ::message_box::MessageBox,
                                                    button,
                                                    text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setCheckBox(QCheckBox* cb)```</span>
  ///
  ///
  pub unsafe fn set_check_box(&mut self, cb: *mut ::check_box::CheckBox) {
    ::ffi::qt_widgets_c_QMessageBox_setCheckBox(self as *mut ::message_box::MessageBox, cb)
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setDefaultButton(QMessageBox::StandardButton button)```</span>
  ///
  ///
  pub fn set_default_button(&mut self, button: ::message_box::StandardButton) {
    unsafe { ::ffi::qt_widgets_c_QMessageBox_setDefaultButton_QMessageBox_StandardButton(self as *mut ::message_box::MessageBox, button) }
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setDefaultButton(QPushButton* button)```</span>
  ///
  ///
  pub unsafe fn set_default_button_unsafe(&mut self, button: *mut ::push_button::PushButton) {
    ::ffi::qt_widgets_c_QMessageBox_setDefaultButton_QPushButton(self as *mut ::message_box::MessageBox, button)
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setDetailedText(const QString& text)```</span>
  ///
  ///
  pub fn set_detailed_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QMessageBox_setDetailedText(self as *mut ::message_box::MessageBox,
                                                      text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setEscapeButton(QMessageBox::StandardButton button)```</span>
  ///
  ///
  pub fn set_escape_button(&mut self, button: ::message_box::StandardButton) {
    unsafe {
      ::ffi::qt_widgets_c_QMessageBox_setEscapeButton_QMessageBox_StandardButton(self as *mut ::message_box::MessageBox, button)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setEscapeButton(QAbstractButton* button)```</span>
  ///
  ///
  pub unsafe fn set_escape_button_unsafe(&mut self, button: *mut ::abstract_button::AbstractButton) {
    ::ffi::qt_widgets_c_QMessageBox_setEscapeButton_QAbstractButton(self as *mut ::message_box::MessageBox, button)
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setIcon(QMessageBox::Icon arg1)```</span>
  ///
  ///
  pub fn set_icon(&mut self, arg1: ::message_box::Icon) {
    unsafe { ::ffi::qt_widgets_c_QMessageBox_setIcon(self as *mut ::message_box::MessageBox, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setIconPixmap(const QPixmap& pixmap)```</span>
  ///
  ///
  pub fn set_icon_pixmap(&mut self, pixmap: &::qt_gui::pixmap::Pixmap) {
    unsafe {
      ::ffi::qt_widgets_c_QMessageBox_setIconPixmap(self as *mut ::message_box::MessageBox,
                                                    pixmap as *const ::qt_gui::pixmap::Pixmap)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setInformativeText(const QString& text)```</span>
  ///
  ///
  pub fn set_informative_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QMessageBox_setInformativeText(self as *mut ::message_box::MessageBox,
                                                         text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setStandardButtons(QFlags<QMessageBox::StandardButton> buttons)```</span>
  ///
  ///
  pub fn set_standard_buttons(&mut self, buttons: ::qt_core::flags::Flags<::message_box::StandardButton>) {
    unsafe {
      ::ffi::qt_widgets_c_QMessageBox_setStandardButtons(self as *mut ::message_box::MessageBox,
                                                         buttons.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setText(const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QMessageBox_setText(self as *mut ::message_box::MessageBox,
                                              text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setTextFormat(Qt::TextFormat format)```</span>
  ///
  ///
  pub fn set_text_format(&mut self, format: &::qt_core::qt::TextFormat) {
    unsafe {
      ::ffi::qt_widgets_c_QMessageBox_setTextFormat(self as *mut ::message_box::MessageBox,
                                                    format as *const ::qt_core::qt::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setWindowModality(Qt::WindowModality windowModality)```</span>
  ///
  ///
  pub fn set_window_modality(&mut self, window_modality: &::qt_core::qt::WindowModality) {
    unsafe {
      ::ffi::qt_widgets_c_QMessageBox_setWindowModality(self as *mut ::message_box::MessageBox,
                                                        window_modality as *const ::qt_core::qt::WindowModality)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMessageBox::setWindowTitle(const QString& title)```</span>
  ///
  ///
  pub fn set_window_title(&mut self, title: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QMessageBox_setWindowTitle(self as *mut ::message_box::MessageBox,
                                                     title as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QMessageBox::StandardButton QMessageBox::standardButton(QAbstractButton* button) const```</span>
  ///
  ///
  pub unsafe fn standard_button(&self,
                                button: *mut ::abstract_button::AbstractButton)
                                -> ::message_box::StandardButton {
    ::ffi::qt_widgets_c_QMessageBox_standardButton(self as *const ::message_box::MessageBox, button)
  }

  /// C++ method: <span style='color: green;'>```QFlags<QMessageBox::StandardButton> QMessageBox::standardButtons() const```</span>
  ///
  ///
  pub fn standard_buttons(&self) -> ::qt_core::flags::Flags<::message_box::StandardButton> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMessageBox_standardButtons(self as *const ::message_box::MessageBox) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```static QPixmap QMessageBox::standardIcon(QMessageBox::Icon icon)```</span>
  ///
  ///
  pub fn standard_icon(icon: ::message_box::Icon) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMessageBox_standardIcon_as_ptr(icon) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QString QMessageBox::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMessageBox_text_to_output(self as *const ::message_box::MessageBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QMessageBox::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMessageBox_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QMessageBox::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMessageBox_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMessageBox::warning```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn warning((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String)) -> ::message_box::StandardButton```<br>
  /// C++ method: <span style='color: green;'>```static QMessageBox::StandardButton QMessageBox::warning(QWidget* parent, const QString& title, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn warning((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::qt_core::flags::Flags<::message_box::StandardButton>)) -> ::message_box::StandardButton```<br>
  /// C++ method: <span style='color: green;'>```static QMessageBox::StandardButton QMessageBox::warning(QWidget* parent, const QString& title, const QString& text, QFlags<QMessageBox::StandardButton> buttons = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn warning((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::qt_core::flags::Flags<::message_box::StandardButton>, ::message_box::StandardButton)) -> ::message_box::StandardButton```<br>
  /// C++ method: <span style='color: green;'>```static QMessageBox::StandardButton QMessageBox::warning(QWidget* parent, const QString& title, const QString& text, QFlags<QMessageBox::StandardButton> buttons = ?, QMessageBox::StandardButton defaultButton = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn warning((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::message_box::StandardButton, ::message_box::StandardButton)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::warning(QWidget* parent, const QString& title, const QString& text, QMessageBox::StandardButton button0, QMessageBox::StandardButton button1)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn warning((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::warning(QWidget* parent, const QString& title, const QString& text, const QString& button0Text)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn warning((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::warning(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn warning((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::warning(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?, const QString& button2Text = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn warning((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::warning(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?, const QString& button2Text = ?, int defaultButtonNumber = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn warning((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::warning(QWidget* parent, const QString& title, const QString& text, const QString& button0Text, const QString& button1Text = ?, const QString& button2Text = ?, int defaultButtonNumber = ?, int escapeButtonNumber = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn warning((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::warning(QWidget* parent, const QString& title, const QString& text, int button0, int button1)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn warning((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QMessageBox::warning(QWidget* parent, const QString& title, const QString& text, int button0, int button1, int button2 = ?)```</span>
  ///
  ///
  pub unsafe fn warning<Args>(args: Args) -> Args::ReturnType
    where Args: overloading::MessageBoxWarningArgs
  {
    args.exec()
  }
}

impl ::cpp_utils::CppDeletable for ::message_box::MessageBox {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QMessageBox_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `MessageBox`.
  pub struct Signals<'a>(&'a ::message_box::MessageBox);
  /// Represents a built-in Qt signal `QMessageBox::buttonClicked`.
  ///
  /// An object of this type can be created from `MessageBox` with `object.signals().button_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MessageBox` object.
  pub struct ButtonClicked<'a>(&'a ::message_box::MessageBox);
  impl<'a> ::qt_core::connection::Receiver for ButtonClicked<'a> {
    type Arguments = (*mut ::abstract_button::AbstractButton,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2buttonClicked(QAbstractButton*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ButtonClicked<'a> {}
  /// Represents a built-in Qt signal `QMessageBox::rejected`.
  ///
  /// An object of this type can be created from `MessageBox` with `object.signals().rejected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MessageBox` object.
  pub struct Rejected<'a>(&'a ::message_box::MessageBox);
  impl<'a> ::qt_core::connection::Receiver for Rejected<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rejected()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Rejected<'a> {}
  /// Represents a built-in Qt signal `QMessageBox::accepted`.
  ///
  /// An object of this type can be created from `MessageBox` with `object.signals().accepted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MessageBox` object.
  pub struct Accepted<'a>(&'a ::message_box::MessageBox);
  impl<'a> ::qt_core::connection::Receiver for Accepted<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2accepted()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Accepted<'a> {}
  /// Represents a built-in Qt signal `QMessageBox::finished`.
  ///
  /// An object of this type can be created from `MessageBox` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MessageBox` object.
  pub struct Finished<'a>(&'a ::message_box::MessageBox);
  impl<'a> ::qt_core::connection::Receiver for Finished<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2finished(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Finished<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QMessageBox::buttonClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn button_clicked(&self) -> ButtonClicked {
      ButtonClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMessageBox::rejected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rejected(&self) -> Rejected {
      Rejected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMessageBox::accepted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accepted(&self) -> Accepted {
      Accepted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMessageBox::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `MessageBox`.
  pub struct Slots<'a>(&'a ::message_box::MessageBox);
  /// Represents a built-in Qt slot `QMessageBox::reject`.
  ///
  /// An object of this type can be created from `MessageBox` with `object.slots().reject()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MessageBox` object.
  pub struct Reject<'a>(&'a ::message_box::MessageBox);
  impl<'a> ::qt_core::connection::Receiver for Reject<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reject()\0"
    }
  }
  /// Represents a built-in Qt slot `QMessageBox::accept`.
  ///
  /// An object of this type can be created from `MessageBox` with `object.slots().accept()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MessageBox` object.
  pub struct Accept<'a>(&'a ::message_box::MessageBox);
  impl<'a> ::qt_core::connection::Receiver for Accept<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1accept()\0"
    }
  }
  /// Represents a built-in Qt slot `QMessageBox::open`.
  ///
  /// An object of this type can be created from `MessageBox` with `object.slots().open()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MessageBox` object.
  pub struct Open<'a>(&'a ::message_box::MessageBox);
  impl<'a> ::qt_core::connection::Receiver for Open<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1open()\0"
    }
  }
  /// Represents a built-in Qt slot `QMessageBox::exec`.
  ///
  /// An object of this type can be created from `MessageBox` with `object.slots().exec()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MessageBox` object.
  pub struct Exec<'a>(&'a ::message_box::MessageBox);
  impl<'a> ::qt_core::connection::Receiver for Exec<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1exec()\0"
    }
  }
  /// Represents a built-in Qt slot `QMessageBox::done`.
  ///
  /// An object of this type can be created from `MessageBox` with `object.slots().done()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MessageBox` object.
  pub struct Done<'a>(&'a ::message_box::MessageBox);
  impl<'a> ::qt_core::connection::Receiver for Done<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1done(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QMessageBox::showExtension`.
  ///
  /// An object of this type can be created from `MessageBox` with `object.slots().show_extension()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MessageBox` object.
  pub struct ShowExtension<'a>(&'a ::message_box::MessageBox);
  impl<'a> ::qt_core::connection::Receiver for ShowExtension<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showExtension(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QMessageBox::reject`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reject(&self) -> Reject {
      Reject(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMessageBox::accept`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accept(&self) -> Accept {
      Accept(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMessageBox::open`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn open(&self) -> Open {
      Open(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMessageBox::exec`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exec(&self) -> Exec {
      Exec(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMessageBox::done`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn done(&self) -> Done {
      Done(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMessageBox::showExtension`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_extension(&self) -> ShowExtension {
      ShowExtension(self.0)
    }
  }
  impl ::message_box::MessageBox {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QMessageBox::StandardButton```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StandardButton {
  /// C++ enum variant: <span style='color: green;'>```ButtonMask = -769```</span>
  ButtonMask = -769,
  /// C++ enum variant: <span style='color: green;'>```NoButton = 0```</span>
  NoButton = 0,
  /// C++ enum variant: <span style='color: green;'>```Default = 256```</span>
  Default = 256,
  /// C++ enum variant: <span style='color: green;'>```Escape = 512```</span>
  Escape = 512,
  /// C++ enum variant: <span style='color: green;'>```FlagMask = 768```</span>
  FlagMask = 768,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Ok = 1024```</span>
  /// - <span style='color: green;'>```FirstButton = 1024```</span>
  ///
  Ok = 1024,
  /// C++ enum variant: <span style='color: green;'>```Save = 2048```</span>
  Save = 2048,
  /// C++ enum variant: <span style='color: green;'>```SaveAll = 4096```</span>
  SaveAll = 4096,
  /// C++ enum variant: <span style='color: green;'>```Open = 8192```</span>
  Open = 8192,
  /// C++ enum variant: <span style='color: green;'>```Yes = 16384```</span>
  Yes = 16384,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```YesToAll = 32768```</span>
  /// - <span style='color: green;'>```YesAll = 32768```</span>
  ///
  YesToAll = 32768,
  /// C++ enum variant: <span style='color: green;'>```No = 65536```</span>
  No = 65536,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```NoToAll = 131072```</span>
  /// - <span style='color: green;'>```NoAll = 131072```</span>
  ///
  NoToAll = 131072,
  /// C++ enum variant: <span style='color: green;'>```Abort = 262144```</span>
  Abort = 262144,
  /// C++ enum variant: <span style='color: green;'>```Retry = 524288```</span>
  Retry = 524288,
  /// C++ enum variant: <span style='color: green;'>```Ignore = 1048576```</span>
  Ignore = 1048576,
  /// C++ enum variant: <span style='color: green;'>```Close = 2097152```</span>
  Close = 2097152,
  /// C++ enum variant: <span style='color: green;'>```Cancel = 4194304```</span>
  Cancel = 4194304,
  /// C++ enum variant: <span style='color: green;'>```Discard = 8388608```</span>
  Discard = 8388608,
  /// C++ enum variant: <span style='color: green;'>```Help = 16777216```</span>
  Help = 16777216,
  /// C++ enum variant: <span style='color: green;'>```Apply = 33554432```</span>
  Apply = 33554432,
  /// C++ enum variant: <span style='color: green;'>```Reset = 67108864```</span>
  Reset = 67108864,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```RestoreDefaults = 134217728```</span>
  /// - <span style='color: green;'>```LastButton = 134217728```</span>
  ///
  RestoreDefaults = 134217728,
}

impl ::qt_core::flags::FlaggableEnum for StandardButton {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "StandardButton"
  }
}

impl ::cpp_utils::DynamicCast<::message_box::MessageBox> for ::dialog::Dialog {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::message_box::MessageBox> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMessageBox_G_dynamic_cast_QMessageBox_ptr_QDialog(self as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::message_box::MessageBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMessageBox_G_dynamic_cast_QMessageBox_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::message_box::MessageBox> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::message_box::MessageBox> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMessageBox_G_dynamic_cast_QMessageBox_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::message_box::MessageBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMessageBox_G_dynamic_cast_QMessageBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::message_box::MessageBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QObject_ptr(self as *mut ::message_box::MessageBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QObject_ptr(self as *const ::message_box::MessageBox as *mut ::message_box::MessageBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::message_box::MessageBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QPaintDevice_ptr(self as *mut ::message_box::MessageBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QPaintDevice_ptr(self as *const ::message_box::MessageBox as *mut ::message_box::MessageBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::dialog::Dialog> for ::message_box::MessageBox {
  fn static_cast_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QDialog_ptr(self as *mut ::message_box::MessageBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QDialog_ptr(self as *const ::message_box::MessageBox as *mut ::message_box::MessageBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::message_box::MessageBox {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QWidget_ptr(self as *mut ::message_box::MessageBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QWidget_ptr(self as *const ::message_box::MessageBox as *mut ::message_box::MessageBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::message_box::MessageBox> for ::dialog::Dialog {
  unsafe fn static_cast_mut(&mut self) -> &mut ::message_box::MessageBox {
    let ffi_result =
      ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QDialog(self as *mut ::dialog::Dialog);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::message_box::MessageBox {
    let ffi_result = ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::message_box::MessageBox> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::message_box::MessageBox {
    let ffi_result =
      ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::message_box::MessageBox {
    let ffi_result = ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::message_box::MessageBox> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::message_box::MessageBox {
    let ffi_result = ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::message_box::MessageBox {
    let ffi_result = ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::message_box::MessageBox> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::message_box::MessageBox {
    let ffi_result =
      ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::message_box::MessageBox {
    let ffi_result = ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::message_box::MessageBox {
  type Target = ::dialog::Dialog;
  fn deref(&self) -> &::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QDialog_ptr(self as *const ::message_box::MessageBox as *mut ::message_box::MessageBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::message_box::MessageBox {
  fn deref_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMessageBox_G_static_cast_QDialog_ptr(self as *mut ::message_box::MessageBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MessageBox::about_qt](../struct.MessageBox.html#method.about_qt) method.
  pub trait MessageBoxAboutQtArgs {
    unsafe fn exec(self) -> ();
  }
  impl MessageBoxAboutQtArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> () {
      let parent = self;
      ::ffi::qt_widgets_c_QMessageBox_aboutQt_parent(parent)
    }
  }
  impl<'a> MessageBoxAboutQtArgs for (*mut ::widget::Widget, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> () {
      let parent = self.0;
      let title = self.1;
      ::ffi::qt_widgets_c_QMessageBox_aboutQt_parent_title(parent, title as *const ::qt_core::string::String)
    }
  }
  /// This trait represents a set of arguments accepted by [MessageBox::add_button](../struct.MessageBox.html#method.add_button) method.
  pub trait MessageBoxAddButtonArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::message_box::MessageBox) -> *mut ::push_button::PushButton;
  }
  impl<'largs> MessageBoxAddButtonArgs<'largs> for ::message_box::StandardButton {
    fn exec(self, original_self: &'largs mut ::message_box::MessageBox) -> *mut ::push_button::PushButton {
      let button = self;
      unsafe {
        ::ffi::qt_widgets_c_QMessageBox_addButton_button(original_self as *mut ::message_box::MessageBox, button)
      }
    }
  }
  impl<'largs> MessageBoxAddButtonArgs<'largs> for (&'largs ::qt_core::string::String, ::message_box::ButtonRole) {
    fn exec(self, original_self: &'largs mut ::message_box::MessageBox) -> *mut ::push_button::PushButton {
      let text = self.0;
      let role = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QMessageBox_addButton_text_role(original_self as *mut ::message_box::MessageBox,
                                                            text as *const ::qt_core::string::String,
                                                            role)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MessageBox::critical](../struct.MessageBox.html#method.critical) method.
  pub trait MessageBoxCriticalArgs {
    type ReturnType;
    unsafe fn exec(self) -> Self::ReturnType;
  }
  impl<'a> MessageBoxCriticalArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::string::String) {
    type ReturnType = ::message_box::StandardButton;
    unsafe fn exec(self) -> ::message_box::StandardButton {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      ::ffi::qt_widgets_c_QMessageBox_critical_QWidget_QString_QString(parent,
                                                                       title as *const ::qt_core::string::String,
                                                                       text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxCriticalArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           ::qt_core::flags::Flags<::message_box::StandardButton>) {
    type ReturnType = ::message_box::StandardButton;
    unsafe fn exec(self) -> ::message_box::StandardButton {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let buttons = self.3;
      ::ffi::qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QFlags_QMessageBox_StandardButton(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, buttons.to_int() as ::libc::c_uint)
    }
  }
  impl<'a> MessageBoxCriticalArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           ::qt_core::flags::Flags<::message_box::StandardButton>,
                                           ::message_box::StandardButton) {
    type ReturnType = ::message_box::StandardButton;
    unsafe fn exec(self) -> ::message_box::StandardButton {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let buttons = self.3;
      let default_button = self.4;
      ::ffi::qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QFlags_QMessageBox_StandardButton_QMessageBox_StandardButton(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, buttons.to_int() as ::libc::c_uint, default_button)
    }
  }
  impl<'a> MessageBoxCriticalArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           ::message_box::StandardButton,
                                           ::message_box::StandardButton) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      let button1 = self.4;
      ::ffi::qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QMessageBox_StandardButton_QMessageBox_StandardButton(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0, button1)
    }
  }
  impl<'a> MessageBoxCriticalArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      ::ffi::qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxCriticalArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      ::ffi::qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString_QString(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxCriticalArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      let button2_text = self.5;
      ::ffi::qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString_QString_QString(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String, button2_text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxCriticalArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      let button2_text = self.5;
      let default_button_number = self.6;
      ::ffi::qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString_QString_QString_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String, button2_text as *const ::qt_core::string::String, default_button_number)
    }
  }
  impl<'a> MessageBoxCriticalArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           ::libc::c_int,
                                           ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      let button2_text = self.5;
      let default_button_number = self.6;
      let escape_button_number = self.7;
      ::ffi::qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString_QString_QString_int_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String, button2_text as *const ::qt_core::string::String, default_button_number, escape_button_number)
    }
  }
  impl<'a> MessageBoxCriticalArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           ::libc::c_int,
                                           ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      let button1 = self.4;
      ::ffi::qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_int_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0, button1)
    }
  }
  impl<'a> MessageBoxCriticalArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           ::libc::c_int,
                                           ::libc::c_int,
                                           ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      let button1 = self.4;
      let button2 = self.5;
      ::ffi::qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_int_int_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0, button1, button2)
    }
  }
  /// This trait represents a set of arguments accepted by [MessageBox::information](../struct.MessageBox.html#method.information) method.
  pub trait MessageBoxInformationArgs {
    type ReturnType;
    unsafe fn exec(self) -> Self::ReturnType;
  }
  impl<'a> MessageBoxInformationArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::string::String) {
    type ReturnType = ::message_box::StandardButton;
    unsafe fn exec(self) -> ::message_box::StandardButton {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      ::ffi::qt_widgets_c_QMessageBox_information_QWidget_QString_QString(parent,
                                                                          title as *const ::qt_core::string::String,
                                                                          text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxInformationArgs
    for (*mut ::widget::Widget,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              ::qt_core::flags::Flags<::message_box::StandardButton>) {
    type ReturnType = ::message_box::StandardButton;
    unsafe fn exec(self) -> ::message_box::StandardButton {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let buttons = self.3;
      ::ffi::qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QFlags_QMessageBox_StandardButton(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, buttons.to_int() as ::libc::c_uint)
    }
  }
  impl<'a> MessageBoxInformationArgs
    for (*mut ::widget::Widget,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              ::qt_core::flags::Flags<::message_box::StandardButton>,
                                              ::message_box::StandardButton) {
    type ReturnType = ::message_box::StandardButton;
    unsafe fn exec(self) -> ::message_box::StandardButton {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let buttons = self.3;
      let default_button = self.4;
      ::ffi::qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QFlags_QMessageBox_StandardButton_QMessageBox_StandardButton(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, buttons.to_int() as ::libc::c_uint, default_button)
    }
  }
  impl<'a> MessageBoxInformationArgs
    for (*mut ::widget::Widget,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              ::message_box::StandardButton) {
    type ReturnType = ::message_box::StandardButton;
    unsafe fn exec(self) -> ::message_box::StandardButton {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      ::ffi::qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QMessageBox_StandardButton(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0)
    }
  }
  impl<'a> MessageBoxInformationArgs
    for (*mut ::widget::Widget,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              ::message_box::StandardButton,
                                              ::message_box::StandardButton) {
    type ReturnType = ::message_box::StandardButton;
    unsafe fn exec(self) -> ::message_box::StandardButton {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      let button1 = self.4;
      ::ffi::qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QMessageBox_StandardButton_QMessageBox_StandardButton(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0, button1)
    }
  }
  impl<'a> MessageBoxInformationArgs
    for (*mut ::widget::Widget,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      ::ffi::qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxInformationArgs
    for (*mut ::widget::Widget,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      ::ffi::qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString_QString(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxInformationArgs
    for (*mut ::widget::Widget,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      let button2_text = self.5;
      ::ffi::qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString_QString_QString(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String, button2_text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxInformationArgs
    for (*mut ::widget::Widget,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      let button2_text = self.5;
      let default_button_number = self.6;
      ::ffi::qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString_QString_QString_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String, button2_text as *const ::qt_core::string::String, default_button_number)
    }
  }
  impl<'a> MessageBoxInformationArgs
    for (*mut ::widget::Widget,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              ::libc::c_int,
                                              ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      let button2_text = self.5;
      let default_button_number = self.6;
      let escape_button_number = self.7;
      ::ffi::qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString_QString_QString_int_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String, button2_text as *const ::qt_core::string::String, default_button_number, escape_button_number)
    }
  }
  impl<'a> MessageBoxInformationArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::string::String, ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      ::ffi::qt_widgets_c_QMessageBox_information_QWidget_QString_QString_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0)
    }
  }
  impl<'a> MessageBoxInformationArgs
    for (*mut ::widget::Widget,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              ::libc::c_int,
                                              ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      let button1 = self.4;
      ::ffi::qt_widgets_c_QMessageBox_information_QWidget_QString_QString_int_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0, button1)
    }
  }
  impl<'a> MessageBoxInformationArgs
    for (*mut ::widget::Widget,
                                              &'a ::qt_core::string::String,
                                              &'a ::qt_core::string::String,
                                              ::libc::c_int,
                                              ::libc::c_int,
                                              ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      let button1 = self.4;
      let button2 = self.5;
      ::ffi::qt_widgets_c_QMessageBox_information_QWidget_QString_QString_int_int_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0, button1, button2)
    }
  }
  /// This trait represents a set of arguments accepted by [MessageBox::question](../struct.MessageBox.html#method.question) method.
  pub trait MessageBoxQuestionArgs {
    type ReturnType;
    unsafe fn exec(self) -> Self::ReturnType;
  }
  impl<'a> MessageBoxQuestionArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::string::String) {
    type ReturnType = ::message_box::StandardButton;
    unsafe fn exec(self) -> ::message_box::StandardButton {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      ::ffi::qt_widgets_c_QMessageBox_question_QWidget_QString_QString(parent,
                                                                       title as *const ::qt_core::string::String,
                                                                       text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxQuestionArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           ::qt_core::flags::Flags<::message_box::StandardButton>) {
    type ReturnType = ::message_box::StandardButton;
    unsafe fn exec(self) -> ::message_box::StandardButton {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let buttons = self.3;
      ::ffi::qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QFlags_QMessageBox_StandardButton(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, buttons.to_int() as ::libc::c_uint)
    }
  }
  impl<'a> MessageBoxQuestionArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           ::qt_core::flags::Flags<::message_box::StandardButton>,
                                           ::message_box::StandardButton) {
    type ReturnType = ::message_box::StandardButton;
    unsafe fn exec(self) -> ::message_box::StandardButton {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let buttons = self.3;
      let default_button = self.4;
      ::ffi::qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QFlags_QMessageBox_StandardButton_QMessageBox_StandardButton(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, buttons.to_int() as ::libc::c_uint, default_button)
    }
  }
  impl<'a> MessageBoxQuestionArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           ::message_box::StandardButton,
                                           ::message_box::StandardButton) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      let button1 = self.4;
      ::ffi::qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QMessageBox_StandardButton_QMessageBox_StandardButton(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0, button1)
    }
  }
  impl<'a> MessageBoxQuestionArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      ::ffi::qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxQuestionArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      ::ffi::qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString_QString(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxQuestionArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      let button2_text = self.5;
      ::ffi::qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString_QString_QString(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String, button2_text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxQuestionArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      let button2_text = self.5;
      let default_button_number = self.6;
      ::ffi::qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString_QString_QString_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String, button2_text as *const ::qt_core::string::String, default_button_number)
    }
  }
  impl<'a> MessageBoxQuestionArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           ::libc::c_int,
                                           ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      let button2_text = self.5;
      let default_button_number = self.6;
      let escape_button_number = self.7;
      ::ffi::qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString_QString_QString_int_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String, button2_text as *const ::qt_core::string::String, default_button_number, escape_button_number)
    }
  }
  impl<'a> MessageBoxQuestionArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::string::String, ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      ::ffi::qt_widgets_c_QMessageBox_question_QWidget_QString_QString_int(parent,
                                                                           title as *const ::qt_core::string::String,
                                                                           text as *const ::qt_core::string::String,
                                                                           button0)
    }
  }
  impl<'a> MessageBoxQuestionArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           ::libc::c_int,
                                           ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      let button1 = self.4;
      ::ffi::qt_widgets_c_QMessageBox_question_QWidget_QString_QString_int_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0, button1)
    }
  }
  impl<'a> MessageBoxQuestionArgs
    for (*mut ::widget::Widget,
                                           &'a ::qt_core::string::String,
                                           &'a ::qt_core::string::String,
                                           ::libc::c_int,
                                           ::libc::c_int,
                                           ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      let button1 = self.4;
      let button2 = self.5;
      ::ffi::qt_widgets_c_QMessageBox_question_QWidget_QString_QString_int_int_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0, button1, button2)
    }
  }
  /// This trait represents a set of arguments accepted by [MessageBox::warning](../struct.MessageBox.html#method.warning) method.
  pub trait MessageBoxWarningArgs {
    type ReturnType;
    unsafe fn exec(self) -> Self::ReturnType;
  }
  impl<'a> MessageBoxWarningArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::string::String) {
    type ReturnType = ::message_box::StandardButton;
    unsafe fn exec(self) -> ::message_box::StandardButton {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      ::ffi::qt_widgets_c_QMessageBox_warning_QWidget_QString_QString(parent,
                                                                      title as *const ::qt_core::string::String,
                                                                      text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxWarningArgs
    for (*mut ::widget::Widget,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          ::qt_core::flags::Flags<::message_box::StandardButton>) {
    type ReturnType = ::message_box::StandardButton;
    unsafe fn exec(self) -> ::message_box::StandardButton {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let buttons = self.3;
      ::ffi::qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QFlags_QMessageBox_StandardButton(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, buttons.to_int() as ::libc::c_uint)
    }
  }
  impl<'a> MessageBoxWarningArgs
    for (*mut ::widget::Widget,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          ::qt_core::flags::Flags<::message_box::StandardButton>,
                                          ::message_box::StandardButton) {
    type ReturnType = ::message_box::StandardButton;
    unsafe fn exec(self) -> ::message_box::StandardButton {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let buttons = self.3;
      let default_button = self.4;
      ::ffi::qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QFlags_QMessageBox_StandardButton_QMessageBox_StandardButton(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, buttons.to_int() as ::libc::c_uint, default_button)
    }
  }
  impl<'a> MessageBoxWarningArgs
    for (*mut ::widget::Widget,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          ::message_box::StandardButton,
                                          ::message_box::StandardButton) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      let button1 = self.4;
      ::ffi::qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QMessageBox_StandardButton_QMessageBox_StandardButton(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0, button1)
    }
  }
  impl<'a> MessageBoxWarningArgs
    for (*mut ::widget::Widget,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      ::ffi::qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxWarningArgs
    for (*mut ::widget::Widget,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      ::ffi::qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString_QString(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxWarningArgs
    for (*mut ::widget::Widget,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      let button2_text = self.5;
      ::ffi::qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString_QString_QString(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String, button2_text as *const ::qt_core::string::String)
    }
  }
  impl<'a> MessageBoxWarningArgs
    for (*mut ::widget::Widget,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      let button2_text = self.5;
      let default_button_number = self.6;
      ::ffi::qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString_QString_QString_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String, button2_text as *const ::qt_core::string::String, default_button_number)
    }
  }
  impl<'a> MessageBoxWarningArgs
    for (*mut ::widget::Widget,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          ::libc::c_int,
                                          ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0_text = self.3;
      let button1_text = self.4;
      let button2_text = self.5;
      let default_button_number = self.6;
      let escape_button_number = self.7;
      ::ffi::qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString_QString_QString_int_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0_text as *const ::qt_core::string::String, button1_text as *const ::qt_core::string::String, button2_text as *const ::qt_core::string::String, default_button_number, escape_button_number)
    }
  }
  impl<'a> MessageBoxWarningArgs
    for (*mut ::widget::Widget,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          ::libc::c_int,
                                          ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      let button1 = self.4;
      ::ffi::qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_int_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0, button1)
    }
  }
  impl<'a> MessageBoxWarningArgs
    for (*mut ::widget::Widget,
                                          &'a ::qt_core::string::String,
                                          &'a ::qt_core::string::String,
                                          ::libc::c_int,
                                          ::libc::c_int,
                                          ::libc::c_int) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self) -> ::libc::c_int {
      let parent = self.0;
      let title = self.1;
      let text = self.2;
      let button0 = self.3;
      let button1 = self.4;
      let button2 = self.5;
      ::ffi::qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_int_int_int(parent, title as *const ::qt_core::string::String, text as *const ::qt_core::string::String, button0, button1, button2)
    }
  }
}
