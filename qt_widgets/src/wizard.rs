/// C++ type: <span style='color: green;'>```QWizard```</span>
#[repr(C)]
pub struct Wizard(u8);

impl Wizard {
  /// C++ method: <span style='color: green;'>```int QWizard::addPage(QWizardPage* page)```</span>
  ///
  ///
  pub unsafe fn add_page(&mut self, page: *mut ::wizard_page::WizardPage) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QWizard_addPage(self as *mut ::wizard::Wizard, page)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWizard::back()```</span>
  ///
  ///
  pub fn back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWizard_back(self as *mut ::wizard::Wizard) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* QWizard::button(QWizard::WizardButton which) const```</span>
  ///
  ///
  pub fn button(&self, which: ::wizard::WizardButton) -> *mut ::abstract_button::AbstractButton {
    unsafe { ::ffi::qt_widgets_c_QWizard_button(self as *const ::wizard::Wizard, which) }
  }

  /// C++ method: <span style='color: green;'>```QString QWizard::buttonText(QWizard::WizardButton which) const```</span>
  ///
  ///
  pub fn button_text(&self, which: ::wizard::WizardButton) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWizard_buttonText_to_output(self as *const ::wizard::Wizard, which, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QWizard::currentId() const```</span>
  ///
  ///
  pub fn current_id(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWizard_currentId(self as *const ::wizard::Wizard) }
  }

  /// C++ method: <span style='color: green;'>```QWizardPage* QWizard::currentPage() const```</span>
  ///
  ///
  pub fn current_page(&self) -> *mut ::wizard_page::WizardPage {
    unsafe { ::ffi::qt_widgets_c_QWizard_currentPage(self as *const ::wizard::Wizard) }
  }

  /// C++ method: <span style='color: green;'>```QVariant QWizard::field(const QString& name) const```</span>
  ///
  ///
  pub fn field(&self, name: &::qt_core::string::String) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWizard_field_to_output(self as *const ::wizard::Wizard,
                                                    name as *const ::qt_core::string::String,
                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QWizard::hasVisitedPage(int id) const```</span>
  ///
  ///
  pub fn has_visited_page(&self, id: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWizard_hasVisitedPage(self as *const ::wizard::Wizard, id) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QWizard::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QWizard_metaObject(self as *const ::wizard::Wizard) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWizard::next()```</span>
  ///
  ///
  pub fn next(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWizard_next(self as *mut ::wizard::Wizard) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QWizard::nextId() const```</span>
  ///
  ///
  pub fn next_id(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWizard_nextId(self as *const ::wizard::Wizard) }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QWizard::WizardOption> QWizard::options() const```</span>
  ///
  ///
  pub fn options(&self) -> ::qt_core::flags::Flags<::wizard::WizardOption> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizard_options(self as *const ::wizard::Wizard) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```QWizardPage* QWizard::page(int id) const```</span>
  ///
  ///
  pub fn page(&self, id: ::libc::c_int) -> *mut ::wizard_page::WizardPage {
    unsafe { ::ffi::qt_widgets_c_QWizard_page(self as *const ::wizard::Wizard, id) }
  }

  /// C++ method: <span style='color: green;'>```QList<int> QWizard::pageIds() const```</span>
  ///
  ///
  pub fn page_ids(&self) -> ::qt_core::list::ListCInt {
    {
      let mut object: ::qt_core::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWizard_pageIds_to_output(self as *const ::wizard::Wizard, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPixmap QWizard::pixmap(QWizard::WizardPixmap which) const```</span>
  ///
  ///
  pub fn pixmap(&self, which: ::wizard::WizardPixmap) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizard_pixmap_as_ptr(self as *const ::wizard::Wizard, which) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QWizard::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QWizard_qt_metacall(self as *mut ::wizard::Wizard,
                                            arg1 as *const ::qt_core::meta_object::Call,
                                            arg2,
                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QWizard::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QWizard_qt_metacast(self as *mut ::wizard::Wizard, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QWizard::removePage(int id)```</span>
  ///
  ///
  pub fn remove_page(&mut self, id: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QWizard_removePage(self as *mut ::wizard::Wizard, id) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWizard::restart()```</span>
  ///
  ///
  pub fn restart(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWizard_restart(self as *mut ::wizard::Wizard) }
  }

  /// C++ method: <span style='color: green;'>```void QWizard::setButton(QWizard::WizardButton which, QAbstractButton* button)```</span>
  ///
  ///
  pub unsafe fn set_button(&mut self, which: ::wizard::WizardButton, button: *mut ::abstract_button::AbstractButton) {
    ::ffi::qt_widgets_c_QWizard_setButton(self as *mut ::wizard::Wizard, which, button)
  }

  /// C++ method: <span style='color: green;'>```void QWizard::setButtonLayout(const QList<QWizard::WizardButton>& layout)```</span>
  ///
  ///
  pub fn set_button_layout(&mut self, layout: &::list::ListWizardWizardButton) {
    unsafe {
      ::ffi::qt_widgets_c_QWizard_setButtonLayout(self as *mut ::wizard::Wizard,
                                                  layout as *const ::list::ListWizardWizardButton)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWizard::setButtonText(QWizard::WizardButton which, const QString& text)```</span>
  ///
  ///
  pub fn set_button_text(&mut self, which: ::wizard::WizardButton, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWizard_setButtonText(self as *mut ::wizard::Wizard,
                                                which,
                                                text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWizard::setDefaultProperty(const char* className, const char* property, const char* changedSignal)```</span>
  ///
  ///
  pub unsafe fn set_default_property(&mut self,
                                     class_name: *const ::libc::c_char,
                                     property: *const ::libc::c_char,
                                     changed_signal: *const ::libc::c_char) {
    ::ffi::qt_widgets_c_QWizard_setDefaultProperty(self as *mut ::wizard::Wizard,
                                                   class_name,
                                                   property,
                                                   changed_signal)
  }

  /// C++ method: <span style='color: green;'>```void QWizard::setField(const QString& name, const QVariant& value)```</span>
  ///
  ///
  pub fn set_field(&mut self, name: &::qt_core::string::String, value: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_widgets_c_QWizard_setField(self as *mut ::wizard::Wizard,
                                           name as *const ::qt_core::string::String,
                                           value as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```QWizard::setOption```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_option(&mut self, ::wizard::WizardOption) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWizard::setOption(QWizard::WizardOption option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_option(&mut self, (::wizard::WizardOption, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWizard::setOption(QWizard::WizardOption option, bool on = ?)```</span>
  ///
  ///
  pub fn set_option<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WizardSetOptionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWizard::setOptions(QFlags<QWizard::WizardOption> options)```</span>
  ///
  ///
  pub fn set_options(&mut self, options: ::qt_core::flags::Flags<::wizard::WizardOption>) {
    unsafe {
      ::ffi::qt_widgets_c_QWizard_setOptions(self as *mut ::wizard::Wizard,
                                             options.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWizard::setPage(int id, QWizardPage* page)```</span>
  ///
  ///
  pub unsafe fn set_page(&mut self, id: ::libc::c_int, page: *mut ::wizard_page::WizardPage) {
    ::ffi::qt_widgets_c_QWizard_setPage(self as *mut ::wizard::Wizard, id, page)
  }

  /// C++ method: <span style='color: green;'>```void QWizard::setPixmap(QWizard::WizardPixmap which, const QPixmap& pixmap)```</span>
  ///
  ///
  pub fn set_pixmap(&mut self, which: ::wizard::WizardPixmap, pixmap: &::qt_gui::pixmap::Pixmap) {
    unsafe {
      ::ffi::qt_widgets_c_QWizard_setPixmap(self as *mut ::wizard::Wizard,
                                            which,
                                            pixmap as *const ::qt_gui::pixmap::Pixmap)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWizard::setSideWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_side_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QWizard_setSideWidget(self as *mut ::wizard::Wizard, widget)
  }

  /// C++ method: <span style='color: green;'>```void QWizard::setStartId(int id)```</span>
  ///
  ///
  pub fn set_start_id(&mut self, id: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QWizard_setStartId(self as *mut ::wizard::Wizard, id) }
  }

  /// C++ method: <span style='color: green;'>```void QWizard::setSubTitleFormat(Qt::TextFormat format)```</span>
  ///
  ///
  pub fn set_sub_title_format(&mut self, format: &::qt_core::qt::TextFormat) {
    unsafe {
      ::ffi::qt_widgets_c_QWizard_setSubTitleFormat(self as *mut ::wizard::Wizard,
                                                    format as *const ::qt_core::qt::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWizard::setTitleFormat(Qt::TextFormat format)```</span>
  ///
  ///
  pub fn set_title_format(&mut self, format: &::qt_core::qt::TextFormat) {
    unsafe {
      ::ffi::qt_widgets_c_QWizard_setTitleFormat(self as *mut ::wizard::Wizard,
                                                 format as *const ::qt_core::qt::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QWizard::setVisible(bool visible)```</span>
  ///
  ///
  pub fn set_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QWizard_setVisible(self as *mut ::wizard::Wizard, visible) }
  }

  /// C++ method: <span style='color: green;'>```void QWizard::setWizardStyle(QWizard::WizardStyle style)```</span>
  ///
  ///
  pub fn set_wizard_style(&mut self, style: ::wizard::WizardStyle) {
    unsafe { ::ffi::qt_widgets_c_QWizard_setWizardStyle(self as *mut ::wizard::Wizard, style) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QWizard::sideWidget() const```</span>
  ///
  ///
  pub fn side_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QWizard_sideWidget(self as *const ::wizard::Wizard) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QWizard::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWizard_sizeHint_to_output(self as *const ::wizard::Wizard, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QWizard::startId() const```</span>
  ///
  ///
  pub fn start_id(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWizard_startId(self as *const ::wizard::Wizard) }
  }

  /// C++ method: <span style='color: green;'>```bool QWizard::testOption(QWizard::WizardOption option) const```</span>
  ///
  ///
  pub fn test_option(&self, option: ::wizard::WizardOption) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWizard_testOption(self as *const ::wizard::Wizard, option) }
  }

  /// C++ method: <span style='color: green;'>```static QString QWizard::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QWizard_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QWizard::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QWizard_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QWizard::validateCurrentPage()```</span>
  ///
  ///
  pub fn validate_current_page(&mut self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWizard_validateCurrentPage(self as *mut ::wizard::Wizard) }
  }

  /// C++ method: <span style='color: green;'>```QList<int> QWizard::visitedPages() const```</span>
  ///
  ///
  pub fn visited_pages(&self) -> ::qt_core::list::ListCInt {
    {
      let mut object: ::qt_core::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWizard_visitedPages_to_output(self as *const ::wizard::Wizard, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWizard::WizardStyle QWizard::wizardStyle() const```</span>
  ///
  ///
  pub fn wizard_style(&self) -> ::wizard::WizardStyle {
    unsafe { ::ffi::qt_widgets_c_QWizard_wizardStyle(self as *const ::wizard::Wizard) }
  }
}

impl ::cpp_utils::CppDeletable for ::wizard::Wizard {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QWizard_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Wizard`.
  pub struct Signals<'a>(&'a ::wizard::Wizard);
  /// Represents a built-in Qt signal `QWizard::customButtonClicked`.
  ///
  /// An object of this type can be created from `Wizard` with `object.signals().custom_button_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct CustomButtonClicked<'a>(&'a ::wizard::Wizard);
  impl<'a> ::qt_core::connection::Receiver for CustomButtonClicked<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2customButtonClicked(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CustomButtonClicked<'a> {}
  /// Represents a built-in Qt signal `QWizard::accepted`.
  ///
  /// An object of this type can be created from `Wizard` with `object.signals().accepted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct Accepted<'a>(&'a ::wizard::Wizard);
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
  /// Represents a built-in Qt signal `QWizard::pageRemoved`.
  ///
  /// An object of this type can be created from `Wizard` with `object.signals().page_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct PageRemoved<'a>(&'a ::wizard::Wizard);
  impl<'a> ::qt_core::connection::Receiver for PageRemoved<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2pageRemoved(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PageRemoved<'a> {}
  /// Represents a built-in Qt signal `QWizard::helpRequested`.
  ///
  /// An object of this type can be created from `Wizard` with `object.signals().help_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct HelpRequested<'a>(&'a ::wizard::Wizard);
  impl<'a> ::qt_core::connection::Receiver for HelpRequested<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2helpRequested()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HelpRequested<'a> {}
  /// Represents a built-in Qt signal `QWizard::pageAdded`.
  ///
  /// An object of this type can be created from `Wizard` with `object.signals().page_added()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct PageAdded<'a>(&'a ::wizard::Wizard);
  impl<'a> ::qt_core::connection::Receiver for PageAdded<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2pageAdded(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PageAdded<'a> {}
  /// Represents a built-in Qt signal `QWizard::currentIdChanged`.
  ///
  /// An object of this type can be created from `Wizard` with `object.signals().current_id_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct CurrentIdChanged<'a>(&'a ::wizard::Wizard);
  impl<'a> ::qt_core::connection::Receiver for CurrentIdChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentIdChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentIdChanged<'a> {}
  /// Represents a built-in Qt signal `QWizard::rejected`.
  ///
  /// An object of this type can be created from `Wizard` with `object.signals().rejected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct Rejected<'a>(&'a ::wizard::Wizard);
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
  /// Represents a built-in Qt signal `QWizard::finished`.
  ///
  /// An object of this type can be created from `Wizard` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct Finished<'a>(&'a ::wizard::Wizard);
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
    /// Returns an object representing a built-in Qt signal `QWizard::customButtonClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_button_clicked(&self) -> CustomButtonClicked {
      CustomButtonClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWizard::accepted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accepted(&self) -> Accepted {
      Accepted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWizard::pageRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn page_removed(&self) -> PageRemoved {
      PageRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWizard::helpRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn help_requested(&self) -> HelpRequested {
      HelpRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWizard::pageAdded`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn page_added(&self) -> PageAdded {
      PageAdded(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWizard::currentIdChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_id_changed(&self) -> CurrentIdChanged {
      CurrentIdChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWizard::rejected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rejected(&self) -> Rejected {
      Rejected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWizard::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Wizard`.
  pub struct Slots<'a>(&'a ::wizard::Wizard);
  /// Represents a built-in Qt slot `QWizard::open`.
  ///
  /// An object of this type can be created from `Wizard` with `object.slots().open()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct Open<'a>(&'a ::wizard::Wizard);
  impl<'a> ::qt_core::connection::Receiver for Open<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1open()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizard::back`.
  ///
  /// An object of this type can be created from `Wizard` with `object.slots().back()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct Back<'a>(&'a ::wizard::Wizard);
  impl<'a> ::qt_core::connection::Receiver for Back<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1back()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizard::showExtension`.
  ///
  /// An object of this type can be created from `Wizard` with `object.slots().show_extension()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct ShowExtension<'a>(&'a ::wizard::Wizard);
  impl<'a> ::qt_core::connection::Receiver for ShowExtension<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showExtension(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWizard::exec`.
  ///
  /// An object of this type can be created from `Wizard` with `object.slots().exec()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct Exec<'a>(&'a ::wizard::Wizard);
  impl<'a> ::qt_core::connection::Receiver for Exec<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1exec()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizard::restart`.
  ///
  /// An object of this type can be created from `Wizard` with `object.slots().restart()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct Restart<'a>(&'a ::wizard::Wizard);
  impl<'a> ::qt_core::connection::Receiver for Restart<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1restart()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizard::reject`.
  ///
  /// An object of this type can be created from `Wizard` with `object.slots().reject()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct Reject<'a>(&'a ::wizard::Wizard);
  impl<'a> ::qt_core::connection::Receiver for Reject<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reject()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizard::next`.
  ///
  /// An object of this type can be created from `Wizard` with `object.slots().next()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct Next<'a>(&'a ::wizard::Wizard);
  impl<'a> ::qt_core::connection::Receiver for Next<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1next()\0"
    }
  }
  /// Represents a built-in Qt slot `QWizard::accept`.
  ///
  /// An object of this type can be created from `Wizard` with `object.slots().accept()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Wizard` object.
  pub struct Accept<'a>(&'a ::wizard::Wizard);
  impl<'a> ::qt_core::connection::Receiver for Accept<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1accept()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QWizard::open`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn open(&self) -> Open {
      Open(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizard::back`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn back(&self) -> Back {
      Back(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizard::showExtension`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_extension(&self) -> ShowExtension {
      ShowExtension(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizard::exec`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exec(&self) -> Exec {
      Exec(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizard::restart`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn restart(&self) -> Restart {
      Restart(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizard::reject`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reject(&self) -> Reject {
      Reject(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizard::next`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn next(&self) -> Next {
      Next(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWizard::accept`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accept(&self) -> Accept {
      Accept(self.0)
    }
  }
  impl ::wizard::Wizard {
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

/// C++ type: <span style='color: green;'>```QWizard::WizardButton```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WizardButton {
  /// C++ enum variant: <span style='color: green;'>```NoButton = -1```</span>
  NoButton = -1,
  /// C++ enum variant: <span style='color: green;'>```BackButton = 0```</span>
  BackButton = 0,
  /// C++ enum variant: <span style='color: green;'>```NextButton = 1```</span>
  NextButton = 1,
  /// C++ enum variant: <span style='color: green;'>```CommitButton = 2```</span>
  CommitButton = 2,
  /// C++ enum variant: <span style='color: green;'>```FinishButton = 3```</span>
  FinishButton = 3,
  /// C++ enum variant: <span style='color: green;'>```CancelButton = 4```</span>
  CancelButton = 4,
  /// C++ enum variant: <span style='color: green;'>```HelpButton = 5```</span>
  HelpButton = 5,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```CustomButton1 = 6```</span>
  /// - <span style='color: green;'>```NStandardButtons = 6```</span>
  ///
  CustomButton1 = 6,
  /// C++ enum variant: <span style='color: green;'>```CustomButton2 = 7```</span>
  CustomButton2 = 7,
  /// C++ enum variant: <span style='color: green;'>```CustomButton3 = 8```</span>
  CustomButton3 = 8,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Stretch = 9```</span>
  /// - <span style='color: green;'>```NButtons = 9```</span>
  ///
  Stretch = 9,
}

/// C++ type: <span style='color: green;'>```QWizard::WizardOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WizardOption {
  /// C++ enum variant: <span style='color: green;'>```IndependentPages = 1```</span>
  IndependentPages = 1,
  /// C++ enum variant: <span style='color: green;'>```IgnoreSubTitles = 2```</span>
  IgnoreSubTitles = 2,
  /// C++ enum variant: <span style='color: green;'>```ExtendedWatermarkPixmap = 4```</span>
  ExtendedWatermarkPixmap = 4,
  /// C++ enum variant: <span style='color: green;'>```NoDefaultButton = 8```</span>
  NoDefaultButton = 8,
  /// C++ enum variant: <span style='color: green;'>```NoBackButtonOnStartPage = 16```</span>
  NoBackButtonOnStartPage = 16,
  /// C++ enum variant: <span style='color: green;'>```NoBackButtonOnLastPage = 32```</span>
  NoBackButtonOnLastPage = 32,
  /// C++ enum variant: <span style='color: green;'>```DisabledBackButtonOnLastPage = 64```</span>
  DisabledBackButtonOnLastPage = 64,
  /// C++ enum variant: <span style='color: green;'>```HaveNextButtonOnLastPage = 128```</span>
  HaveNextButtonOnLastPage = 128,
  /// C++ enum variant: <span style='color: green;'>```HaveFinishButtonOnEarlyPages = 256```</span>
  HaveFinishButtonOnEarlyPages = 256,
  /// C++ enum variant: <span style='color: green;'>```NoCancelButton = 512```</span>
  NoCancelButton = 512,
  /// C++ enum variant: <span style='color: green;'>```CancelButtonOnLeft = 1024```</span>
  CancelButtonOnLeft = 1024,
  /// C++ enum variant: <span style='color: green;'>```HaveHelpButton = 2048```</span>
  HaveHelpButton = 2048,
  /// C++ enum variant: <span style='color: green;'>```HelpButtonOnRight = 4096```</span>
  HelpButtonOnRight = 4096,
  /// C++ enum variant: <span style='color: green;'>```HaveCustomButton1 = 8192```</span>
  HaveCustomButton1 = 8192,
  /// C++ enum variant: <span style='color: green;'>```HaveCustomButton2 = 16384```</span>
  HaveCustomButton2 = 16384,
  /// C++ enum variant: <span style='color: green;'>```HaveCustomButton3 = 32768```</span>
  HaveCustomButton3 = 32768,
  /// C++ enum variant: <span style='color: green;'>```NoCancelButtonOnLastPage = 65536```</span>
  NoCancelButtonOnLastPage = 65536,
}

impl ::qt_core::flags::FlaggableEnum for WizardOption {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "WizardOption"
  }
}

/// C++ type: <span style='color: green;'>```QWizard::WizardPixmap```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WizardPixmap {
  /// C++ enum variant: <span style='color: green;'>```WatermarkPixmap = 0```</span>
  WatermarkPixmap = 0,
  /// C++ enum variant: <span style='color: green;'>```LogoPixmap = 1```</span>
  LogoPixmap = 1,
  /// C++ enum variant: <span style='color: green;'>```BannerPixmap = 2```</span>
  BannerPixmap = 2,
  /// C++ enum variant: <span style='color: green;'>```BackgroundPixmap = 3```</span>
  BackgroundPixmap = 3,
  /// C++ enum variant: <span style='color: green;'>```NPixmaps = 4```</span>
  NPixmaps = 4,
}

/// C++ type: <span style='color: green;'>```QWizard::WizardStyle```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WizardStyle {
  /// C++ enum variant: <span style='color: green;'>```ClassicStyle = 0```</span>
  ClassicStyle = 0,
  /// C++ enum variant: <span style='color: green;'>```ModernStyle = 1```</span>
  ModernStyle = 1,
  /// C++ enum variant: <span style='color: green;'>```MacStyle = 2```</span>
  MacStyle = 2,
  /// C++ enum variant: <span style='color: green;'>```AeroStyle = 3```</span>
  AeroStyle = 3,
  /// C++ enum variant: <span style='color: green;'>```NStyles = 4```</span>
  NStyles = 4,
}

impl ::cpp_utils::DynamicCast<::wizard::Wizard> for ::dialog::Dialog {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::wizard::Wizard> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QWizard_G_dynamic_cast_QWizard_ptr_QDialog(self as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::wizard::Wizard> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizard_G_dynamic_cast_QWizard_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::wizard::Wizard> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::wizard::Wizard> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QWizard_G_dynamic_cast_QWizard_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::wizard::Wizard> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizard_G_dynamic_cast_QWizard_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::wizard::Wizard {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizard_G_static_cast_QObject_ptr(self as *mut ::wizard::Wizard) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QWizard_G_static_cast_QObject_ptr(self as *const ::wizard::Wizard as *mut ::wizard::Wizard)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::wizard::Wizard {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QWizard_G_static_cast_QPaintDevice_ptr(self as *mut ::wizard::Wizard) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizard_G_static_cast_QPaintDevice_ptr(self as *const ::wizard::Wizard as *mut ::wizard::Wizard) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::dialog::Dialog> for ::wizard::Wizard {
  fn static_cast_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizard_G_static_cast_QDialog_ptr(self as *mut ::wizard::Wizard) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::dialog::Dialog {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QWizard_G_static_cast_QDialog_ptr(self as *const ::wizard::Wizard as *mut ::wizard::Wizard)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::wizard::Wizard {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizard_G_static_cast_QWidget_ptr(self as *mut ::wizard::Wizard) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QWizard_G_static_cast_QWidget_ptr(self as *const ::wizard::Wizard as *mut ::wizard::Wizard)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::wizard::Wizard> for ::dialog::Dialog {
  unsafe fn static_cast_mut(&mut self) -> &mut ::wizard::Wizard {
    let ffi_result = ::ffi::qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QDialog(self as *mut ::dialog::Dialog);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::wizard::Wizard {
    let ffi_result = ::ffi::qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::wizard::Wizard> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::wizard::Wizard {
    let ffi_result =
      ::ffi::qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::wizard::Wizard {
    let ffi_result = ::ffi::qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::wizard::Wizard> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::wizard::Wizard {
    let ffi_result = ::ffi::qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::wizard::Wizard {
    let ffi_result = ::ffi::qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::wizard::Wizard> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::wizard::Wizard {
    let ffi_result = ::ffi::qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::wizard::Wizard {
    let ffi_result = ::ffi::qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::wizard::Wizard {
  type Target = ::dialog::Dialog;
  fn deref(&self) -> &::dialog::Dialog {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QWizard_G_static_cast_QDialog_ptr(self as *const ::wizard::Wizard as *mut ::wizard::Wizard)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::wizard::Wizard {
  fn deref_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWizard_G_static_cast_QDialog_ptr(self as *mut ::wizard::Wizard) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Wizard::set_option](../struct.Wizard.html#method.set_option) method.
  pub trait WizardSetOptionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::wizard::Wizard) -> ();
  }
  impl<'largs> WizardSetOptionArgs<'largs> for ::wizard::WizardOption {
    fn exec(self, original_self: &'largs mut ::wizard::Wizard) -> () {
      let option = self;
      unsafe { ::ffi::qt_widgets_c_QWizard_setOption_option(original_self as *mut ::wizard::Wizard, option) }
    }
  }
  impl<'largs> WizardSetOptionArgs<'largs> for (::wizard::WizardOption, bool) {
    fn exec(self, original_self: &'largs mut ::wizard::Wizard) -> () {
      let option = self.0;
      let on = self.1;
      unsafe { ::ffi::qt_widgets_c_QWizard_setOption_option_on(original_self as *mut ::wizard::Wizard, option, on) }
    }
  }
}
