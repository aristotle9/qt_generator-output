/// C++ type: <span style='color: green;'>```QLabel```</span>
#[repr(C)]
pub struct Label(u8);

impl Label {
  /// C++ method: <span style='color: green;'>```QWidget* QLabel::buddy() const```</span>
  ///
  ///
  pub fn buddy(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QLabel_buddy(self as *const ::label::Label) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLabel::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLabel_clear(self as *mut ::label::Label) }
  }

  /// C++ method: <span style='color: green;'>```bool QLabel::hasScaledContents() const```</span>
  ///
  ///
  pub fn has_scaled_contents(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLabel_hasScaledContents(self as *const ::label::Label) }
  }

  /// C++ method: <span style='color: green;'>```bool QLabel::hasSelectedText() const```</span>
  ///
  ///
  pub fn has_selected_text(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLabel_hasSelectedText(self as *const ::label::Label) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QLabel::heightForWidth(int arg1) const```</span>
  ///
  ///
  pub fn height_for_width(&self, arg1: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLabel_heightForWidth(self as *const ::label::Label, arg1) }
  }

  /// C++ method: <span style='color: green;'>```int QLabel::indent() const```</span>
  ///
  ///
  pub fn indent(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLabel_indent(self as *const ::label::Label) }
  }

  /// C++ method: <span style='color: green;'>```int QLabel::margin() const```</span>
  ///
  ///
  pub fn margin(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLabel_margin(self as *const ::label::Label) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QLabel::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QLabel_metaObject(self as *const ::label::Label) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QLabel::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLabel_minimumSizeHint_to_output(self as *const ::label::Label, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMovie* QLabel::movie() const```</span>
  ///
  ///
  pub fn movie(&self) -> *mut ::qt_gui::movie::Movie {
    unsafe { ::ffi::qt_widgets_c_QLabel_movie(self as *const ::label::Label) }
  }

  /// C++ method: <span style='color: green;'>```bool QLabel::openExternalLinks() const```</span>
  ///
  ///
  pub fn open_external_links(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLabel_openExternalLinks(self as *const ::label::Label) }
  }

  /// C++ method: <span style='color: green;'>```const QPicture* QLabel::picture() const```</span>
  ///
  ///
  pub fn picture(&self) -> *const ::qt_gui::picture::Picture {
    unsafe { ::ffi::qt_widgets_c_QLabel_picture(self as *const ::label::Label) }
  }

  /// C++ method: <span style='color: green;'>```const QPixmap* QLabel::pixmap() const```</span>
  ///
  ///
  pub fn pixmap(&self) -> *const ::qt_gui::pixmap::Pixmap {
    unsafe { ::ffi::qt_widgets_c_QLabel_pixmap(self as *const ::label::Label) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QLabel::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QLabel_qt_metacall(self as *mut ::label::Label,
                                           arg1 as *const ::qt_core::meta_object::Call,
                                           arg2,
                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QLabel::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QLabel_qt_metacast(self as *mut ::label::Label, arg1)
  }

  /// C++ method: <span style='color: green;'>```QString QLabel::selectedText() const```</span>
  ///
  ///
  pub fn selected_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLabel_selectedText_to_output(self as *const ::label::Label, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QLabel::selectionStart() const```</span>
  ///
  ///
  pub fn selection_start(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLabel_selectionStart(self as *const ::label::Label) }
  }

  /// C++ method: <span style='color: green;'>```void QLabel::setBuddy(QWidget* arg1)```</span>
  ///
  ///
  pub unsafe fn set_buddy(&mut self, arg1: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QLabel_setBuddy(self as *mut ::label::Label, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QLabel::setIndent(int arg1)```</span>
  ///
  ///
  pub fn set_indent(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QLabel_setIndent(self as *mut ::label::Label, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QLabel::setMargin(int arg1)```</span>
  ///
  ///
  pub fn set_margin(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QLabel_setMargin(self as *mut ::label::Label, arg1) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLabel::setMovie(QMovie* movie)```</span>
  ///
  ///
  pub unsafe fn set_movie(&mut self, movie: *mut ::qt_gui::movie::Movie) {
    ::ffi::qt_widgets_c_QLabel_setMovie(self as *mut ::label::Label, movie)
  }

  /// C++ method: <span style='color: green;'>```QLabel::setNum```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_num(&mut self, ::libc::c_double) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QLabel::setNum(double arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_num(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QLabel::setNum(int arg1)```</span>
  ///
  ///
  pub fn set_num<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::LabelSetNumArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QLabel::setOpenExternalLinks(bool open)```</span>
  ///
  ///
  pub fn set_open_external_links(&mut self, open: bool) {
    unsafe { ::ffi::qt_widgets_c_QLabel_setOpenExternalLinks(self as *mut ::label::Label, open) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLabel::setPicture(const QPicture& arg1)```</span>
  ///
  ///
  pub fn set_picture(&mut self, arg1: &::qt_gui::picture::Picture) {
    unsafe {
      ::ffi::qt_widgets_c_QLabel_setPicture(self as *mut ::label::Label,
                                            arg1 as *const ::qt_gui::picture::Picture)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLabel::setPixmap(const QPixmap& arg1)```</span>
  ///
  ///
  pub fn set_pixmap(&mut self, arg1: &::qt_gui::pixmap::Pixmap) {
    unsafe {
      ::ffi::qt_widgets_c_QLabel_setPixmap(self as *mut ::label::Label,
                                           arg1 as *const ::qt_gui::pixmap::Pixmap)
    }
  }

  /// C++ method: <span style='color: green;'>```void QLabel::setScaledContents(bool arg1)```</span>
  ///
  ///
  pub fn set_scaled_contents(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QLabel_setScaledContents(self as *mut ::label::Label, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QLabel::setSelection(int arg1, int arg2)```</span>
  ///
  ///
  pub fn set_selection(&mut self, arg1: ::libc::c_int, arg2: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QLabel_setSelection(self as *mut ::label::Label, arg1, arg2) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLabel::setText(const QString& arg1)```</span>
  ///
  ///
  pub fn set_text(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QLabel_setText(self as *mut ::label::Label,
                                         arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QLabel::setTextFormat(Qt::TextFormat arg1)```</span>
  ///
  ///
  pub fn set_text_format(&mut self, arg1: &::qt_core::qt::TextFormat) {
    unsafe {
      ::ffi::qt_widgets_c_QLabel_setTextFormat(self as *mut ::label::Label,
                                               arg1 as *const ::qt_core::qt::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QLabel::setWordWrap(bool on)```</span>
  ///
  ///
  pub fn set_word_wrap(&mut self, on: bool) {
    unsafe { ::ffi::qt_widgets_c_QLabel_setWordWrap(self as *mut ::label::Label, on) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QLabel::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLabel_sizeHint_to_output(self as *const ::label::Label, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QLabel::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLabel_text_to_output(self as *const ::label::Label, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QLabel::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QLabel_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QLabel::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QLabel_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QLabel::wordWrap() const```</span>
  ///
  ///
  pub fn word_wrap(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLabel_wordWrap(self as *const ::label::Label) }
  }
}

impl ::cpp_utils::CppDeletable for ::label::Label {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QLabel_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Label`.
  pub struct Signals<'a>(&'a ::label::Label);
  /// Represents a built-in Qt signal `QLabel::linkActivated`.
  ///
  /// An object of this type can be created from `Label` with `object.signals().link_activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Label` object.
  pub struct LinkActivated<'a>(&'a ::label::Label);
  impl<'a> ::qt_core::connection::Receiver for LinkActivated<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2linkActivated(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LinkActivated<'a> {}
  /// Represents a built-in Qt signal `QLabel::linkHovered`.
  ///
  /// An object of this type can be created from `Label` with `object.signals().link_hovered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Label` object.
  pub struct LinkHovered<'a>(&'a ::label::Label);
  impl<'a> ::qt_core::connection::Receiver for LinkHovered<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2linkHovered(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LinkHovered<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QLabel::linkActivated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn link_activated(&self) -> LinkActivated {
      LinkActivated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QLabel::linkHovered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn link_hovered(&self) -> LinkHovered {
      LinkHovered(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Label`.
  pub struct Slots<'a>(&'a ::label::Label);
  /// Represents a built-in Qt slot `QLabel::setMovie`.
  ///
  /// An object of this type can be created from `Label` with `object.slots().set_movie()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Label` object.
  pub struct SetMovie<'a>(&'a ::label::Label);
  impl<'a> ::qt_core::connection::Receiver for SetMovie<'a> {
    type Arguments = (*mut ::qt_gui::movie::Movie,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMovie(QMovie*)\0"
    }
  }
  /// Represents a built-in Qt slot `QLabel::setText`.
  ///
  /// An object of this type can be created from `Label` with `object.slots().set_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Label` object.
  pub struct SetText<'a>(&'a ::label::Label);
  impl<'a> ::qt_core::connection::Receiver for SetText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QLabel::clear`.
  ///
  /// An object of this type can be created from `Label` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Label` object.
  pub struct Clear<'a>(&'a ::label::Label);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QLabel::setPixmap`.
  ///
  /// An object of this type can be created from `Label` with `object.slots().set_pixmap()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Label` object.
  pub struct SetPixmap<'a>(&'a ::label::Label);
  impl<'a> ::qt_core::connection::Receiver for SetPixmap<'a> {
    type Arguments = (&'static ::qt_gui::pixmap::Pixmap,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPixmap(const QPixmap&)\0"
    }
  }
  /// Represents a built-in Qt slot `QLabel::setPicture`.
  ///
  /// An object of this type can be created from `Label` with `object.slots().set_picture()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Label` object.
  pub struct SetPicture<'a>(&'a ::label::Label);
  impl<'a> ::qt_core::connection::Receiver for SetPicture<'a> {
    type Arguments = (&'static ::qt_gui::picture::Picture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPicture(const QPicture&)\0"
    }
  }
  /// Represents a built-in Qt slot `QLabel::setNum`.
  ///
  /// An object of this type can be created from `Label` with `object.slots().set_num_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Label` object.
  pub struct SetNumCInt<'a>(&'a ::label::Label);
  impl<'a> ::qt_core::connection::Receiver for SetNumCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setNum(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QLabel::setNum`.
  ///
  /// An object of this type can be created from `Label` with `object.slots().set_num_c_double()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Label` object.
  pub struct SetNumCDouble<'a>(&'a ::label::Label);
  impl<'a> ::qt_core::connection::Receiver for SetNumCDouble<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setNum(double)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QLabel::setMovie`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_movie(&self) -> SetMovie {
      SetMovie(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLabel::setText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_text(&self) -> SetText {
      SetText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLabel::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLabel::setPixmap`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_pixmap(&self) -> SetPixmap {
      SetPixmap(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLabel::setPicture`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_picture(&self) -> SetPicture {
      SetPicture(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLabel::setNum`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_num_c_int(&self) -> SetNumCInt {
      SetNumCInt(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLabel::setNum`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_num_c_double(&self) -> SetNumCDouble {
      SetNumCDouble(self.0)
    }
  }
  impl ::label::Label {
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

impl ::cpp_utils::DynamicCast<::label::Label> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::label::Label> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLabel_G_dynamic_cast_QLabel_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::label::Label> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLabel_G_dynamic_cast_QLabel_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::label::Label> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::label::Label> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLabel_G_dynamic_cast_QLabel_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::label::Label> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLabel_G_dynamic_cast_QLabel_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::label::Label {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLabel_G_static_cast_QObject_ptr(self as *mut ::label::Label) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QLabel_G_static_cast_QObject_ptr(self as *const ::label::Label as *mut ::label::Label)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::label::Label {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLabel_G_static_cast_QPaintDevice_ptr(self as *mut ::label::Label) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QLabel_G_static_cast_QPaintDevice_ptr(self as *const ::label::Label as *mut ::label::Label)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::label::Label {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLabel_G_static_cast_QFrame_ptr(self as *mut ::label::Label) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QLabel_G_static_cast_QFrame_ptr(self as *const ::label::Label as *mut ::label::Label)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::label::Label {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLabel_G_static_cast_QWidget_ptr(self as *mut ::label::Label) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QLabel_G_static_cast_QWidget_ptr(self as *const ::label::Label as *mut ::label::Label)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::label::Label> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::label::Label {
    let ffi_result = ::ffi::qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::label::Label {
    let ffi_result = ::ffi::qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::label::Label> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::label::Label {
    let ffi_result =
      ::ffi::qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::label::Label {
    let ffi_result = ::ffi::qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::label::Label> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::label::Label {
    let ffi_result = ::ffi::qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::label::Label {
    let ffi_result = ::ffi::qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::label::Label> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::label::Label {
    let ffi_result = ::ffi::qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::label::Label {
    let ffi_result = ::ffi::qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::label::Label {
  type Target = ::frame::Frame;
  fn deref(&self) -> &::frame::Frame {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QLabel_G_static_cast_QFrame_ptr(self as *const ::label::Label as *mut ::label::Label)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::label::Label {
  fn deref_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLabel_G_static_cast_QFrame_ptr(self as *mut ::label::Label) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Label::set_num](../struct.Label.html#method.set_num) method.
  pub trait LabelSetNumArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::label::Label) -> ();
  }
  impl<'largs> LabelSetNumArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::label::Label) -> () {
      let arg1 = self;
      unsafe { ::ffi::qt_widgets_c_QLabel_setNum_double(original_self as *mut ::label::Label, arg1) }
    }
  }
  impl<'largs> LabelSetNumArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::label::Label) -> () {
      let arg1 = self;
      unsafe { ::ffi::qt_widgets_c_QLabel_setNum_int(original_self as *mut ::label::Label, arg1) }
    }
  }
}
