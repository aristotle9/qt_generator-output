/// C++ type: <span style='color: green;'>```QWizardPage```</span>
#[repr(C)]
pub struct WizardPage(u8);

impl WizardPage {
  /// C++ method: <span style='color: green;'>```QString QWizardPage::buttonText(QWizard::WizardButton which) const```</span>
  ///
  ///
  pub fn button_text(&self, which: &::wizard::WizardButton) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWizardPage_buttonText_to_output(self as *const ::wizard_page::WizardPage,
                                                             which as *const ::wizard::WizardButton,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QWizardPage::cleanupPage()```</span>
  ///
  ///
  pub fn cleanup_page(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWizardPage_cleanupPage(self as *mut ::wizard_page::WizardPage) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QWizardPage::initializePage()```</span>
  ///
  ///
  pub fn initialize_page(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWizardPage_initializePage(self as *mut ::wizard_page::WizardPage) }
  }

  /// C++ method: <span style='color: green;'>```bool QWizardPage::isCommitPage() const```</span>
  ///
  ///
  pub fn is_commit_page(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWizardPage_isCommitPage(self as *const ::wizard_page::WizardPage) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QWizardPage::isComplete() const```</span>
  ///
  ///
  pub fn is_complete(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWizardPage_isComplete(self as *const ::wizard_page::WizardPage) }
  }

  /// C++ method: <span style='color: green;'>```bool QWizardPage::isFinalPage() const```</span>
  ///
  ///
  pub fn is_final_page(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWizardPage_isFinalPage(self as *const ::wizard_page::WizardPage) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QWizardPage::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QWizardPage_metaObject(self as *const ::wizard_page::WizardPage) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QWizardPage::QWizardPage()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::wizard_page::WizardPage> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizardPage_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QWizardPage::QWizardPage(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::wizard_page::WizardPage> {
    let ffi_result = ::ffi::qt_widgets_c_QWizardPage_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QWizardPage::nextId() const```</span>
  ///
  ///
  pub fn next_id(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWizardPage_nextId(self as *const ::wizard_page::WizardPage) }
  }

  /// C++ method: <span style='color: green;'>```QPixmap QWizardPage::pixmap(QWizard::WizardPixmap which) const```</span>
  ///
  ///
  pub fn pixmap(&self, which: &::wizard::WizardPixmap) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QWizardPage_pixmap_as_ptr(self as *const ::wizard_page::WizardPage,
                                                    which as *const ::wizard::WizardPixmap)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QWizardPage::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QWizardPage_qt_metacall(self as *mut ::wizard_page::WizardPage,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QWizardPage::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QWizardPage_qt_metacast(self as *mut ::wizard_page::WizardPage, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QWizardPage::setButtonText(QWizard::WizardButton which, const QString& text)```</span>
  ///
  ///
  pub fn set_button_text(&mut self, which: &::wizard::WizardButton, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWizardPage_setButtonText(self as *mut ::wizard_page::WizardPage,
                                                    which as *const ::wizard::WizardButton,
                                                    text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWizardPage::setCommitPage(bool commitPage)```</span>
  ///
  ///
  pub fn set_commit_page(&mut self, commit_page: bool) {
    unsafe { ::ffi::qt_widgets_c_QWizardPage_setCommitPage(self as *mut ::wizard_page::WizardPage, commit_page) }
  }

  /// C++ method: <span style='color: green;'>```void QWizardPage::setFinalPage(bool finalPage)```</span>
  ///
  ///
  pub fn set_final_page(&mut self, final_page: bool) {
    unsafe { ::ffi::qt_widgets_c_QWizardPage_setFinalPage(self as *mut ::wizard_page::WizardPage, final_page) }
  }

  /// C++ method: <span style='color: green;'>```void QWizardPage::setPixmap(QWizard::WizardPixmap which, const QPixmap& pixmap)```</span>
  ///
  ///
  pub fn set_pixmap(&mut self, which: &::wizard::WizardPixmap, pixmap: &::qt_gui::pixmap::Pixmap) {
    unsafe {
      ::ffi::qt_widgets_c_QWizardPage_setPixmap(self as *mut ::wizard_page::WizardPage,
                                                which as *const ::wizard::WizardPixmap,
                                                pixmap as *const ::qt_gui::pixmap::Pixmap)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWizardPage::setSubTitle(const QString& subTitle)```</span>
  ///
  ///
  pub fn set_sub_title(&mut self, sub_title: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWizardPage_setSubTitle(self as *mut ::wizard_page::WizardPage,
                                                  sub_title as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWizardPage::setTitle(const QString& title)```</span>
  ///
  ///
  pub fn set_title(&mut self, title: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWizardPage_setTitle(self as *mut ::wizard_page::WizardPage,
                                               title as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QWizardPage::subTitle() const```</span>
  ///
  ///
  pub fn sub_title(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWizardPage_subTitle_to_output(self as *const ::wizard_page::WizardPage, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QWizardPage::title() const```</span>
  ///
  ///
  pub fn title(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWizardPage_title_to_output(self as *const ::wizard_page::WizardPage, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QWizardPage::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QWizardPage_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QWizardPage::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QWizardPage_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QWizardPage::validatePage()```</span>
  ///
  ///
  pub fn validate_page(&mut self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWizardPage_validatePage(self as *mut ::wizard_page::WizardPage) }
  }
}

impl ::cpp_utils::CppDeletable for ::wizard_page::WizardPage {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QWizardPage_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `WizardPage`.
  pub struct Signals<'a>(&'a ::wizard_page::WizardPage);
  /// Represents a built-in Qt signal `QWizardPage::windowTitleChanged`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct WindowTitleChanged<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for WindowTitleChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowTitleChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowTitleChanged<'a> {}
  /// Represents a built-in Qt signal `QWizardPage::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct WindowIconTextChanged<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for WindowIconTextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowIconTextChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowIconTextChanged<'a> {}
  /// Represents a built-in Qt signal `QWizardPage::windowIconChanged`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct WindowIconChanged<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for WindowIconChanged<'a> {
    type Arguments = (&'static ::qt_gui::icon::Icon,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowIconChanged(const QIcon&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowIconChanged<'a> {}
  /// Represents a built-in Qt signal `QWizardPage::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for CustomContextMenuRequested<'a> {
    type Arguments = (&'static ::qt_core::point::Point,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2customContextMenuRequested(const QPoint&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CustomContextMenuRequested<'a> {}
  /// Represents a built-in Qt signal `QWizardPage::completeChanged`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.signals().complete_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct CompleteChanged<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for CompleteChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2completeChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CompleteChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QWizardPage::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWizardPage::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWizardPage::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWizardPage::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWizardPage::completeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn complete_changed(&self) -> CompleteChanged {
      CompleteChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `WizardPage`.
  pub struct Slots<'a>(&'a ::wizard_page::WizardPage);
  /// Represents a built-in Qt slot `QWizardPage::setEnabled`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct SetEnabled<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::hide`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct Hide<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::showNormal`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct ShowNormal<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::setDisabled`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct SetDisabled<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::show`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct Show<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::setWindowTitle`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct SetWindowTitle<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::showMaximized`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct ShowMaximized<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::showMinimized`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct ShowMinimized<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::repaint`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct Repaint<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::showFullScreen`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct ShowFullScreen<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::setVisible`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct SetVisible<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::setStyleSheet`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct SetStyleSheet<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::close`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct Close<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::raise`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct Raise<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::update`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct Update<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::setHidden`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct SetHidden<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::setWindowModified`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct SetWindowModified<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::setFocus`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct SetFocus<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::updateMicroFocus`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct UpdateMicroFocus<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizardPage::lower`.
  ///
  /// An object of this type can be created from `WizardPage` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `WizardPage` object.
  pub struct Lower<'a>(&'a ::wizard_page::WizardPage);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QWizardPage::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizardPage::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
  }
  impl ::wizard_page::WizardPage {
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

impl ::cpp_utils::DynamicCast<::wizard_page::WizardPage> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::wizard_page::WizardPage> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QWizardPage_G_dynamic_cast_QWizardPage_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::wizard_page::WizardPage> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizardPage_G_dynamic_cast_QWizardPage_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::wizard_page::WizardPage {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QWizardPage_G_static_cast_QObject_ptr(self as *mut ::wizard_page::WizardPage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizardPage_G_static_cast_QObject_ptr(self as *const ::wizard_page::WizardPage as *mut ::wizard_page::WizardPage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::wizard_page::WizardPage {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QWizardPage_G_static_cast_QPaintDevice_ptr(self as *mut ::wizard_page::WizardPage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizardPage_G_static_cast_QPaintDevice_ptr(self as *const ::wizard_page::WizardPage as *mut ::wizard_page::WizardPage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::wizard_page::WizardPage {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QWizardPage_G_static_cast_QWidget_ptr(self as *mut ::wizard_page::WizardPage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizardPage_G_static_cast_QWidget_ptr(self as *const ::wizard_page::WizardPage as *mut ::wizard_page::WizardPage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::wizard_page::WizardPage> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::wizard_page::WizardPage {
    let ffi_result =
      ::ffi::qt_widgets_c_QWizardPage_G_static_cast_QWizardPage_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::wizard_page::WizardPage {
    let ffi_result = ::ffi::qt_widgets_c_QWizardPage_G_static_cast_QWizardPage_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::wizard_page::WizardPage> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::wizard_page::WizardPage {
    let ffi_result = ::ffi::qt_widgets_c_QWizardPage_G_static_cast_QWizardPage_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::wizard_page::WizardPage {
    let ffi_result = ::ffi::qt_widgets_c_QWizardPage_G_static_cast_QWizardPage_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::wizard_page::WizardPage> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::wizard_page::WizardPage {
    let ffi_result =
      ::ffi::qt_widgets_c_QWizardPage_G_static_cast_QWizardPage_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::wizard_page::WizardPage {
    let ffi_result = ::ffi::qt_widgets_c_QWizardPage_G_static_cast_QWizardPage_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::wizard_page::WizardPage {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizardPage_G_static_cast_QWidget_ptr(self as *const ::wizard_page::WizardPage as *mut ::wizard_page::WizardPage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::wizard_page::WizardPage {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QWizardPage_G_static_cast_QWidget_ptr(self as *mut ::wizard_page::WizardPage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
