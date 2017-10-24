/// C++ type: <span style='color: green;'>```QClipboard```</span>
#[repr(C)]
pub struct Clipboard(u8);

impl Clipboard {
  /// C++ method: <span style='color: green;'>```QClipboard::clear```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn clear(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QClipboard::clear()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn clear(&mut self, ::clipboard::Mode) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QClipboard::clear(QClipboard::Mode mode = ?)```</span>
  ///
  ///
  pub fn clear<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ClipboardClearArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QClipboard::image```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn image(&self, ()) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QClipboard::image() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn image(&self, ::clipboard::Mode) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QClipboard::image(QClipboard::Mode mode = ?) const```</span>
  ///
  ///
  pub fn image<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::image::Image>
    where Args: overloading::ClipboardImageArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QClipboard::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QClipboard_metaObject(self as *const ::clipboard::Clipboard) }
  }

  /// C++ method: <span style='color: green;'>```QClipboard::mimeData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mime_data(&self, ()) -> *const ::qt_core::mime_data::MimeData```<br>
  /// C++ method: <span style='color: green;'>```const QMimeData* QClipboard::mimeData() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mime_data(&self, ::clipboard::Mode) -> *const ::qt_core::mime_data::MimeData```<br>
  /// C++ method: <span style='color: green;'>```const QMimeData* QClipboard::mimeData(QClipboard::Mode mode = ?) const```</span>
  ///
  ///
  pub fn mime_data<'largs, Args>(&'largs self, args: Args) -> *const ::qt_core::mime_data::MimeData
    where Args: overloading::ClipboardMimeDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QClipboard::ownsClipboard() const```</span>
  ///
  ///
  pub fn owns_clipboard(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QClipboard_ownsClipboard(self as *const ::clipboard::Clipboard) }
  }

  /// C++ method: <span style='color: green;'>```bool QClipboard::ownsFindBuffer() const```</span>
  ///
  ///
  pub fn owns_find_buffer(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QClipboard_ownsFindBuffer(self as *const ::clipboard::Clipboard) }
  }

  /// C++ method: <span style='color: green;'>```bool QClipboard::ownsSelection() const```</span>
  ///
  ///
  pub fn owns_selection(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QClipboard_ownsSelection(self as *const ::clipboard::Clipboard) }
  }

  /// C++ method: <span style='color: green;'>```QClipboard::pixmap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn pixmap(&self, ()) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QClipboard::pixmap() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn pixmap(&self, ::clipboard::Mode) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QClipboard::pixmap(QClipboard::Mode mode = ?) const```</span>
  ///
  ///
  pub fn pixmap<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::pixmap::Pixmap>
    where Args: overloading::ClipboardPixmapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual int QClipboard::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QClipboard_qt_metacall(self as *mut ::clipboard::Clipboard,
                                           arg1 as *const ::qt_core::meta_object::Call,
                                           arg2,
                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QClipboard::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QClipboard_qt_metacast(self as *mut ::clipboard::Clipboard, arg1)
  }

  /// C++ method: <span style='color: green;'>```QClipboard::setImage```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_image(&mut self, &::image::Image) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QClipboard::setImage(const QImage& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_image(&mut self, (&::image::Image, ::clipboard::Mode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QClipboard::setImage(const QImage& arg1, QClipboard::Mode mode = ?)```</span>
  ///
  ///
  pub fn set_image<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ClipboardSetImageArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QClipboard::setMimeData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_mime_data(&mut self, *mut ::qt_core::mime_data::MimeData) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QClipboard::setMimeData(QMimeData* data)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_mime_data(&mut self, (*mut ::qt_core::mime_data::MimeData, ::clipboard::Mode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QClipboard::setMimeData(QMimeData* data, QClipboard::Mode mode = ?)```</span>
  ///
  ///
  pub unsafe fn set_mime_data<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ClipboardSetMimeDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QClipboard::setPixmap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_pixmap(&mut self, &::pixmap::Pixmap) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QClipboard::setPixmap(const QPixmap& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_pixmap(&mut self, (&::pixmap::Pixmap, ::clipboard::Mode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QClipboard::setPixmap(const QPixmap& arg1, QClipboard::Mode mode = ?)```</span>
  ///
  ///
  pub fn set_pixmap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ClipboardSetPixmapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QClipboard::setText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_text(&mut self, &::qt_core::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QClipboard::setText(const QString& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_text(&mut self, (&::qt_core::string::String, ::clipboard::Mode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QClipboard::setText(const QString& arg1, QClipboard::Mode mode = ?)```</span>
  ///
  ///
  pub fn set_text<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ClipboardSetTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QClipboard::supportsFindBuffer() const```</span>
  ///
  ///
  pub fn supports_find_buffer(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QClipboard_supportsFindBuffer(self as *const ::clipboard::Clipboard) }
  }

  /// C++ method: <span style='color: green;'>```bool QClipboard::supportsSelection() const```</span>
  ///
  ///
  pub fn supports_selection(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QClipboard_supportsSelection(self as *const ::clipboard::Clipboard) }
  }

  /// C++ method: <span style='color: green;'>```QClipboard::text```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn text(&self, ()) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QClipboard::text() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn text(&self, ::clipboard::Mode) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QClipboard::text(QClipboard::Mode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn text(&self, &mut ::qt_core::string::String) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QClipboard::text(QString& subtype) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn text(&self, (&mut ::qt_core::string::String, ::clipboard::Mode)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QClipboard::text(QString& subtype, QClipboard::Mode mode = ?) const```</span>
  ///
  ///
  pub fn text<'largs, Args>(&'largs self, args: Args) -> ::qt_core::string::String
    where Args: overloading::ClipboardTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QClipboard::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QClipboard_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QClipboard::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QClipboard_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Clipboard`.
  pub struct Signals<'a>(&'a ::clipboard::Clipboard);
  /// Represents a built-in Qt signal `QClipboard::changed`.
  ///
  /// An object of this type can be created from `Clipboard` with `object.signals().changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Clipboard` object.
  pub struct Changed<'a>(&'a ::clipboard::Clipboard);
  impl<'a> ::qt_core::connection::Receiver for Changed<'a> {
    type Arguments = (&'static ::clipboard::Mode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2changed(QClipboard::Mode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Changed<'a> {}
  /// Represents a built-in Qt signal `QClipboard::objectNameChanged`.
  ///
  /// An object of this type can be created from `Clipboard` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Clipboard` object.
  pub struct ObjectNameChanged<'a>(&'a ::clipboard::Clipboard);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  /// Represents a built-in Qt signal `QClipboard::dataChanged`.
  ///
  /// An object of this type can be created from `Clipboard` with `object.signals().data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Clipboard` object.
  pub struct DataChanged<'a>(&'a ::clipboard::Clipboard);
  impl<'a> ::qt_core::connection::Receiver for DataChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2dataChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DataChanged<'a> {}
  /// Represents a built-in Qt signal `QClipboard::findBufferChanged`.
  ///
  /// An object of this type can be created from `Clipboard` with `object.signals().find_buffer_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Clipboard` object.
  pub struct FindBufferChanged<'a>(&'a ::clipboard::Clipboard);
  impl<'a> ::qt_core::connection::Receiver for FindBufferChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2findBufferChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FindBufferChanged<'a> {}
  /// Represents a built-in Qt signal `QClipboard::selectionChanged`.
  ///
  /// An object of this type can be created from `Clipboard` with `object.signals().selection_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Clipboard` object.
  pub struct SelectionChanged<'a>(&'a ::clipboard::Clipboard);
  impl<'a> ::qt_core::connection::Receiver for SelectionChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2selectionChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SelectionChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QClipboard::changed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn changed(&self) -> Changed {
      Changed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QClipboard::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QClipboard::dataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn data_changed(&self) -> DataChanged {
      DataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QClipboard::findBufferChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn find_buffer_changed(&self) -> FindBufferChanged {
      FindBufferChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QClipboard::selectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn selection_changed(&self) -> SelectionChanged {
      SelectionChanged(self.0)
    }
  }
  impl ::clipboard::Clipboard {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QClipboard::Mode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Mode {
  /// C++ enum variant: <span style='color: green;'>```Clipboard = 0```</span>
  Clipboard = 0,
  /// C++ enum variant: <span style='color: green;'>```Selection = 1```</span>
  Selection = 1,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```FindBuffer = 2```</span>
  /// - <span style='color: green;'>```LastMode = 2```</span>
  ///
  FindBuffer = 2,
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::clipboard::Clipboard {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QClipboard_G_static_cast_QObject_ptr(self as *mut ::clipboard::Clipboard) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QClipboard_G_static_cast_QObject_ptr(self as *const ::clipboard::Clipboard as *mut ::clipboard::Clipboard) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::clipboard::Clipboard> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::clipboard::Clipboard {
    let ffi_result = ::ffi::qt_gui_c_QClipboard_G_static_cast_QClipboard_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::clipboard::Clipboard {
    let ffi_result = ::ffi::qt_gui_c_QClipboard_G_static_cast_QClipboard_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::clipboard::Clipboard {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QClipboard_G_static_cast_QObject_ptr(self as *const ::clipboard::Clipboard as *mut ::clipboard::Clipboard) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::clipboard::Clipboard {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QClipboard_G_static_cast_QObject_ptr(self as *mut ::clipboard::Clipboard) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Clipboard::clear](../struct.Clipboard.html#method.clear) method.
  pub trait ClipboardClearArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> ();
  }
  impl<'largs> ClipboardClearArgs<'largs> for ::clipboard::Mode {
    fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> () {
      let mode = self;
      unsafe { ::ffi::qt_gui_c_QClipboard_clear_mode(original_self as *mut ::clipboard::Clipboard, mode) }
    }
  }
  impl<'largs> ClipboardClearArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> () {

      unsafe { ::ffi::qt_gui_c_QClipboard_clear_no_args(original_self as *mut ::clipboard::Clipboard) }
    }
  }
  /// This trait represents a set of arguments accepted by [Clipboard::image](../struct.Clipboard.html#method.image) method.
  pub trait ClipboardImageArgs<'largs> {
    fn exec(self, original_self: &'largs ::clipboard::Clipboard) -> ::cpp_utils::CppBox<::image::Image>;
  }
  impl<'largs> ClipboardImageArgs<'largs> for ::clipboard::Mode {
    fn exec(self, original_self: &'largs ::clipboard::Clipboard) -> ::cpp_utils::CppBox<::image::Image> {
      let mode = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QClipboard_image_as_ptr_mode(original_self as *const ::clipboard::Clipboard, mode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ClipboardImageArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::clipboard::Clipboard) -> ::cpp_utils::CppBox<::image::Image> {

      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QClipboard_image_as_ptr_no_args(original_self as *const ::clipboard::Clipboard) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Clipboard::mime_data](../struct.Clipboard.html#method.mime_data) method.
  pub trait ClipboardMimeDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::clipboard::Clipboard) -> *const ::qt_core::mime_data::MimeData;
  }
  impl<'largs> ClipboardMimeDataArgs<'largs> for ::clipboard::Mode {
    fn exec(self, original_self: &'largs ::clipboard::Clipboard) -> *const ::qt_core::mime_data::MimeData {
      let mode = self;
      unsafe { ::ffi::qt_gui_c_QClipboard_mimeData_mode(original_self as *const ::clipboard::Clipboard, mode) }
    }
  }
  impl<'largs> ClipboardMimeDataArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::clipboard::Clipboard) -> *const ::qt_core::mime_data::MimeData {

      unsafe { ::ffi::qt_gui_c_QClipboard_mimeData_no_args(original_self as *const ::clipboard::Clipboard) }
    }
  }
  /// This trait represents a set of arguments accepted by [Clipboard::pixmap](../struct.Clipboard.html#method.pixmap) method.
  pub trait ClipboardPixmapArgs<'largs> {
    fn exec(self, original_self: &'largs ::clipboard::Clipboard) -> ::cpp_utils::CppBox<::pixmap::Pixmap>;
  }
  impl<'largs> ClipboardPixmapArgs<'largs> for ::clipboard::Mode {
    fn exec(self, original_self: &'largs ::clipboard::Clipboard) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let mode = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QClipboard_pixmap_as_ptr_mode(original_self as *const ::clipboard::Clipboard, mode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ClipboardPixmapArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::clipboard::Clipboard) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {

      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QClipboard_pixmap_as_ptr_no_args(original_self as *const ::clipboard::Clipboard) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Clipboard::set_image](../struct.Clipboard.html#method.set_image) method.
  pub trait ClipboardSetImageArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> ();
  }
  impl<'largs> ClipboardSetImageArgs<'largs> for &'largs ::image::Image {
    fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_gui_c_QClipboard_setImage_arg1(original_self as *mut ::clipboard::Clipboard,
                                                 arg1 as *const ::image::Image)
      }
    }
  }
  impl<'largs> ClipboardSetImageArgs<'largs> for (&'largs ::image::Image, ::clipboard::Mode) {
    fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> () {
      let arg1 = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_gui_c_QClipboard_setImage_arg1_mode(original_self as *mut ::clipboard::Clipboard,
                                                      arg1 as *const ::image::Image,
                                                      mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Clipboard::set_mime_data](../struct.Clipboard.html#method.set_mime_data) method.
  pub trait ClipboardSetMimeDataArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> ();
  }
  impl<'largs> ClipboardSetMimeDataArgs<'largs> for *mut ::qt_core::mime_data::MimeData {
    unsafe fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> () {
      let data = self;
      ::ffi::qt_gui_c_QClipboard_setMimeData_data(original_self as *mut ::clipboard::Clipboard, data)
    }
  }
  impl<'largs> ClipboardSetMimeDataArgs<'largs> for (*mut ::qt_core::mime_data::MimeData, ::clipboard::Mode) {
    unsafe fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> () {
      let data = self.0;
      let mode = self.1;
      ::ffi::qt_gui_c_QClipboard_setMimeData_data_mode(original_self as *mut ::clipboard::Clipboard, data, mode)
    }
  }
  /// This trait represents a set of arguments accepted by [Clipboard::set_pixmap](../struct.Clipboard.html#method.set_pixmap) method.
  pub trait ClipboardSetPixmapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> ();
  }
  impl<'largs> ClipboardSetPixmapArgs<'largs> for &'largs ::pixmap::Pixmap {
    fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_gui_c_QClipboard_setPixmap_arg1(original_self as *mut ::clipboard::Clipboard,
                                                  arg1 as *const ::pixmap::Pixmap)
      }
    }
  }
  impl<'largs> ClipboardSetPixmapArgs<'largs> for (&'largs ::pixmap::Pixmap, ::clipboard::Mode) {
    fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> () {
      let arg1 = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_gui_c_QClipboard_setPixmap_arg1_mode(original_self as *mut ::clipboard::Clipboard,
                                                       arg1 as *const ::pixmap::Pixmap,
                                                       mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Clipboard::set_text](../struct.Clipboard.html#method.set_text) method.
  pub trait ClipboardSetTextArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> ();
  }
  impl<'largs> ClipboardSetTextArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_gui_c_QClipboard_setText_arg1(original_self as *mut ::clipboard::Clipboard,
                                                arg1 as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> ClipboardSetTextArgs<'largs> for (&'largs ::qt_core::string::String, ::clipboard::Mode) {
    fn exec(self, original_self: &'largs mut ::clipboard::Clipboard) -> () {
      let arg1 = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_gui_c_QClipboard_setText_arg1_mode(original_self as *mut ::clipboard::Clipboard,
                                                     arg1 as *const ::qt_core::string::String,
                                                     mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Clipboard::text](../struct.Clipboard.html#method.text) method.
  pub trait ClipboardTextArgs<'largs> {
    fn exec(self, original_self: &'largs ::clipboard::Clipboard) -> ::qt_core::string::String;
  }
  impl<'largs> ClipboardTextArgs<'largs> for ::clipboard::Mode {
    fn exec(self, original_self: &'largs ::clipboard::Clipboard) -> ::qt_core::string::String {
      let mode = self;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QClipboard_text_to_output_mode(original_self as *const ::clipboard::Clipboard,
                                                         mode,
                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ClipboardTextArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::clipboard::Clipboard) -> ::qt_core::string::String {

      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QClipboard_text_to_output_no_args(original_self as *const ::clipboard::Clipboard,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ClipboardTextArgs<'largs> for &'largs mut ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::clipboard::Clipboard) -> ::qt_core::string::String {
      let subtype = self;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QClipboard_text_to_output_subtype(original_self as *const ::clipboard::Clipboard,
                                                            subtype as *mut ::qt_core::string::String,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ClipboardTextArgs<'largs> for (&'largs mut ::qt_core::string::String, ::clipboard::Mode) {
    fn exec(self, original_self: &'largs ::clipboard::Clipboard) -> ::qt_core::string::String {
      let subtype = self.0;
      let mode = self.1;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QClipboard_text_to_output_subtype_mode(original_self as *const ::clipboard::Clipboard,
                                                                 subtype as *mut ::qt_core::string::String,
                                                                 mode,
                                                                 &mut object);
        }
        object
      }
    }
  }
}
