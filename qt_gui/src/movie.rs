/// C++ type: <span style='color: green;'>```QMovie::CacheMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CacheMode {
  /// C++ enum variant: <span style='color: green;'>```CacheNone = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```CacheAll = 1```</span>
  All = 1,
}

/// C++ type: <span style='color: green;'>```QMovie```</span>
#[repr(C)]
pub struct Movie(u8);

impl Movie {
  /// C++ method: <span style='color: green;'>```QColor QMovie::backgroundColor() const```</span>
  ///
  ///
  pub fn background_color(&self) -> ::color::Color {
    {
      let mut object: ::color::Color = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QMovie_backgroundColor_to_output(self as *const ::movie::Movie, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMovie::CacheMode QMovie::cacheMode() const```</span>
  ///
  ///
  pub fn cache_mode(&self) -> ::movie::CacheMode {
    unsafe { ::ffi::qt_gui_c_QMovie_cacheMode(self as *const ::movie::Movie) }
  }

  /// C++ method: <span style='color: green;'>```int QMovie::currentFrameNumber() const```</span>
  ///
  ///
  pub fn current_frame_number(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QMovie_currentFrameNumber(self as *const ::movie::Movie) }
  }

  /// C++ method: <span style='color: green;'>```QImage QMovie::currentImage() const```</span>
  ///
  ///
  pub fn current_image(&self) -> ::cpp_utils::CppBox<::image::Image> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMovie_currentImage_as_ptr(self as *const ::movie::Movie) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QPixmap QMovie::currentPixmap() const```</span>
  ///
  ///
  pub fn current_pixmap(&self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMovie_currentPixmap_as_ptr(self as *const ::movie::Movie) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QIODevice* QMovie::device() const```</span>
  ///
  ///
  pub fn device(&self) -> *mut ::qt_core::io_device::IODevice {
    unsafe { ::ffi::qt_gui_c_QMovie_device(self as *const ::movie::Movie) }
  }

  /// C++ method: <span style='color: green;'>```QString QMovie::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QMovie_fileName_to_output(self as *const ::movie::Movie, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QMovie::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QMovie_format_to_output(self as *const ::movie::Movie, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QMovie::frameCount() const```</span>
  ///
  ///
  pub fn frame_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QMovie_frameCount(self as *const ::movie::Movie) }
  }

  /// C++ method: <span style='color: green;'>```QRect QMovie::frameRect() const```</span>
  ///
  ///
  pub fn frame_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QMovie_frameRect_to_output(self as *const ::movie::Movie, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMovie::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QMovie_isValid(self as *const ::movie::Movie) }
  }

  /// C++ method: <span style='color: green;'>```bool QMovie::jumpToFrame(int frameNumber)```</span>
  ///
  ///
  pub fn jump_to_frame(&mut self, frame_number: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_gui_c_QMovie_jumpToFrame(self as *mut ::movie::Movie, frame_number) }
  }

  /// C++ method: <span style='color: green;'>```[slot] bool QMovie::jumpToNextFrame()```</span>
  ///
  ///
  pub fn jump_to_next_frame(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QMovie_jumpToNextFrame(self as *mut ::movie::Movie) }
  }

  /// C++ method: <span style='color: green;'>```int QMovie::loopCount() const```</span>
  ///
  ///
  pub fn loop_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QMovie_loopCount(self as *const ::movie::Movie) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QMovie::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QMovie_metaObject(self as *const ::movie::Movie) }
  }

  /// C++ method: <span style='color: green;'>```QMovie::QMovie```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::movie::Movie>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMovie::QMovie()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::movie::Movie>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMovie::QMovie(const QString& fileName)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, &::qt_core::byte_array::ByteArray)) -> ::cpp_utils::CppBox<::movie::Movie>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMovie::QMovie(const QString& fileName, const QByteArray& format = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::movie::Movie>
    where Args: overloading::MovieNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMovie::QMovie```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::io_device::IODevice) -> ::cpp_utils::CppBox<::movie::Movie>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMovie::QMovie(QIODevice* device)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::qt_core::io_device::IODevice, &::qt_core::byte_array::ByteArray)) -> ::cpp_utils::CppBox<::movie::Movie>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMovie::QMovie(QIODevice* device, const QByteArray& format = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::qt_core::io_device::IODevice, &::qt_core::byte_array::ByteArray, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::movie::Movie>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMovie::QMovie(QIODevice* device, const QByteArray& format = ?, QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::movie::Movie>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMovie::QMovie(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, &::qt_core::byte_array::ByteArray, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::movie::Movie>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMovie::QMovie(const QString& fileName, const QByteArray& format = ?, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::movie::Movie>
    where Args: overloading::MovieNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QMovie::nextFrameDelay() const```</span>
  ///
  ///
  pub fn next_frame_delay(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QMovie_nextFrameDelay(self as *const ::movie::Movie) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QMovie::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QMovie_qt_metacall(self as *mut ::movie::Movie,
                                       arg1 as *const ::qt_core::meta_object::Call,
                                       arg2,
                                       arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QMovie::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QMovie_qt_metacast(self as *mut ::movie::Movie, arg1)
  }

  /// C++ method: <span style='color: green;'>```QSize QMovie::scaledSize()```</span>
  ///
  ///
  pub fn scaled_size(&mut self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QMovie_scaledSize_to_output(self as *mut ::movie::Movie, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QMovie::setBackgroundColor(const QColor& color)```</span>
  ///
  ///
  pub fn set_background_color(&mut self, color: &::color::Color) {
    unsafe { ::ffi::qt_gui_c_QMovie_setBackgroundColor(self as *mut ::movie::Movie, color as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```void QMovie::setCacheMode(QMovie::CacheMode mode)```</span>
  ///
  ///
  pub fn set_cache_mode(&mut self, mode: ::movie::CacheMode) {
    unsafe { ::ffi::qt_gui_c_QMovie_setCacheMode(self as *mut ::movie::Movie, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QMovie::setDevice(QIODevice* device)```</span>
  ///
  ///
  pub unsafe fn set_device(&mut self, device: *mut ::qt_core::io_device::IODevice) {
    ::ffi::qt_gui_c_QMovie_setDevice(self as *mut ::movie::Movie, device)
  }

  /// C++ method: <span style='color: green;'>```void QMovie::setFileName(const QString& fileName)```</span>
  ///
  ///
  pub fn set_file_name(&mut self, file_name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QMovie_setFileName(self as *mut ::movie::Movie,
                                         file_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMovie::setFormat(const QByteArray& format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: &::qt_core::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_gui_c_QMovie_setFormat(self as *mut ::movie::Movie,
                                       format as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QMovie::setPaused(bool paused)```</span>
  ///
  ///
  pub fn set_paused(&mut self, paused: bool) {
    unsafe { ::ffi::qt_gui_c_QMovie_setPaused(self as *mut ::movie::Movie, paused) }
  }

  /// C++ method: <span style='color: green;'>```void QMovie::setScaledSize(const QSize& size)```</span>
  ///
  ///
  pub fn set_scaled_size(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QMovie_setScaledSize(self as *mut ::movie::Movie,
                                           size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QMovie::setSpeed(int percentSpeed)```</span>
  ///
  ///
  pub fn set_speed(&mut self, percent_speed: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QMovie_setSpeed(self as *mut ::movie::Movie, percent_speed) }
  }

  /// C++ method: <span style='color: green;'>```int QMovie::speed() const```</span>
  ///
  ///
  pub fn speed(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QMovie_speed(self as *const ::movie::Movie) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QMovie::start()```</span>
  ///
  ///
  pub fn start(&mut self) {
    unsafe { ::ffi::qt_gui_c_QMovie_start(self as *mut ::movie::Movie) }
  }

  /// C++ method: <span style='color: green;'>```QMovie::MovieState QMovie::state() const```</span>
  ///
  ///
  pub fn state(&self) -> ::movie::MovieState {
    unsafe { ::ffi::qt_gui_c_QMovie_state(self as *const ::movie::Movie) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QMovie::stop()```</span>
  ///
  ///
  pub fn stop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QMovie_stop(self as *mut ::movie::Movie) }
  }

  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QMovie::supportedFormats()```</span>
  ///
  ///
  pub fn supported_formats() -> ::qt_core::list::ListByteArray {
    {
      let mut object: ::qt_core::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QMovie_supportedFormats_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QMovie::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QMovie_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QMovie::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QMovie_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::movie::Movie {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QMovie_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Movie`.
  pub struct Signals<'a>(&'a ::movie::Movie);
  /// Represents a built-in Qt signal `QMovie::finished`.
  ///
  /// An object of this type can be created from `Movie` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Movie` object.
  pub struct Finished<'a>(&'a ::movie::Movie);
  impl<'a> ::qt_core::connection::Receiver for Finished<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2finished()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Finished<'a> {}
  /// Represents a built-in Qt signal `QMovie::updated`.
  ///
  /// An object of this type can be created from `Movie` with `object.signals().updated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Movie` object.
  pub struct Updated<'a>(&'a ::movie::Movie);
  impl<'a> ::qt_core::connection::Receiver for Updated<'a> {
    type Arguments = (&'static ::qt_core::rect::Rect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2updated(const QRect&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Updated<'a> {}
  /// Represents a built-in Qt signal `QMovie::resized`.
  ///
  /// An object of this type can be created from `Movie` with `object.signals().resized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Movie` object.
  pub struct Resized<'a>(&'a ::movie::Movie);
  impl<'a> ::qt_core::connection::Receiver for Resized<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2resized(const QSize&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Resized<'a> {}
  /// Represents a built-in Qt signal `QMovie::started`.
  ///
  /// An object of this type can be created from `Movie` with `object.signals().started()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Movie` object.
  pub struct Started<'a>(&'a ::movie::Movie);
  impl<'a> ::qt_core::connection::Receiver for Started<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2started()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Started<'a> {}
  /// Represents a built-in Qt signal `QMovie::stateChanged`.
  ///
  /// An object of this type can be created from `Movie` with `object.signals().state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Movie` object.
  pub struct StateChanged<'a>(&'a ::movie::Movie);
  impl<'a> ::qt_core::connection::Receiver for StateChanged<'a> {
    type Arguments = (&'static ::movie::MovieState,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2stateChanged(QMovie::MovieState)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for StateChanged<'a> {}
  /// Represents a built-in Qt signal `QMovie::frameChanged`.
  ///
  /// An object of this type can be created from `Movie` with `object.signals().frame_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Movie` object.
  pub struct FrameChanged<'a>(&'a ::movie::Movie);
  impl<'a> ::qt_core::connection::Receiver for FrameChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2frameChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FrameChanged<'a> {}
  /// Represents a built-in Qt signal `QMovie::objectNameChanged`.
  ///
  /// An object of this type can be created from `Movie` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Movie` object.
  pub struct ObjectNameChanged<'a>(&'a ::movie::Movie);
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
  /// Represents a built-in Qt signal `QMovie::error`.
  ///
  /// An object of this type can be created from `Movie` with `object.signals().error()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Movie` object.
  pub struct Error<'a>(&'a ::movie::Movie);
  impl<'a> ::qt_core::connection::Receiver for Error<'a> {
    type Arguments = (&'static ::image_reader::ImageReaderError,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2error(QImageReader::ImageReaderError)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Error<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QMovie::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMovie::updated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn updated(&self) -> Updated {
      Updated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMovie::resized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resized(&self) -> Resized {
      Resized(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMovie::started`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn started(&self) -> Started {
      Started(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMovie::stateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn state_changed(&self) -> StateChanged {
      StateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMovie::frameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn frame_changed(&self) -> FrameChanged {
      FrameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMovie::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMovie::error`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn error(&self) -> Error {
      Error(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Movie`.
  pub struct Slots<'a>(&'a ::movie::Movie);
  /// Represents a built-in Qt slot `QMovie::setSpeed`.
  ///
  /// An object of this type can be created from `Movie` with `object.slots().set_speed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Movie` object.
  pub struct SetSpeed<'a>(&'a ::movie::Movie);
  impl<'a> ::qt_core::connection::Receiver for SetSpeed<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSpeed(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QMovie::stop`.
  ///
  /// An object of this type can be created from `Movie` with `object.slots().stop()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Movie` object.
  pub struct Stop<'a>(&'a ::movie::Movie);
  impl<'a> ::qt_core::connection::Receiver for Stop<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stop()\0"
    }
  }
  /// Represents a built-in Qt slot `QMovie::setPaused`.
  ///
  /// An object of this type can be created from `Movie` with `object.slots().set_paused()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Movie` object.
  pub struct SetPaused<'a>(&'a ::movie::Movie);
  impl<'a> ::qt_core::connection::Receiver for SetPaused<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPaused(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMovie::start`.
  ///
  /// An object of this type can be created from `Movie` with `object.slots().start()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Movie` object.
  pub struct Start<'a>(&'a ::movie::Movie);
  impl<'a> ::qt_core::connection::Receiver for Start<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1start()\0"
    }
  }
  /// Represents a built-in Qt slot `QMovie::jumpToNextFrame`.
  ///
  /// An object of this type can be created from `Movie` with `object.slots().jump_to_next_frame()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Movie` object.
  pub struct JumpToNextFrame<'a>(&'a ::movie::Movie);
  impl<'a> ::qt_core::connection::Receiver for JumpToNextFrame<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1jumpToNextFrame()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QMovie::setSpeed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_speed(&self) -> SetSpeed {
      SetSpeed(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMovie::stop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn stop(&self) -> Stop {
      Stop(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMovie::setPaused`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_paused(&self) -> SetPaused {
      SetPaused(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMovie::start`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn start(&self) -> Start {
      Start(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMovie::jumpToNextFrame`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn jump_to_next_frame(&self) -> JumpToNextFrame {
      JumpToNextFrame(self.0)
    }
  }
  impl ::movie::Movie {
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

/// C++ type: <span style='color: green;'>```QMovie::MovieState```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MovieState {
  /// C++ enum variant: <span style='color: green;'>```NotRunning = 0```</span>
  NotRunning = 0,
  /// C++ enum variant: <span style='color: green;'>```Paused = 1```</span>
  Paused = 1,
  /// C++ enum variant: <span style='color: green;'>```Running = 2```</span>
  Running = 2,
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::movie::Movie {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMovie_G_static_cast_QObject_ptr(self as *mut ::movie::Movie) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QMovie_G_static_cast_QObject_ptr(self as *const ::movie::Movie as *mut ::movie::Movie) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::movie::Movie> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::movie::Movie {
    let ffi_result = ::ffi::qt_gui_c_QMovie_G_static_cast_QMovie_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::movie::Movie {
    let ffi_result = ::ffi::qt_gui_c_QMovie_G_static_cast_QMovie_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::movie::Movie {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QMovie_G_static_cast_QObject_ptr(self as *const ::movie::Movie as *mut ::movie::Movie) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::movie::Movie {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMovie_G_static_cast_QObject_ptr(self as *mut ::movie::Movie) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Movie::new](../struct.Movie.html#method.new) method.
  pub trait MovieNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::movie::Movie>;
  }
  impl<'a> MovieNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::movie::Movie> {
      let file_name = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QMovie_new_fileName(file_name as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> MovieNewArgs for (&'a ::qt_core::string::String, &'a ::qt_core::byte_array::ByteArray) {
    fn exec(self) -> ::cpp_utils::CppBox<::movie::Movie> {
      let file_name = self.0;
      let format = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QMovie_new_fileName_format(file_name as *const ::qt_core::string::String,
                                                   format as *const ::qt_core::byte_array::ByteArray)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl MovieNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::movie::Movie> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QMovie_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Movie::new_unsafe](../struct.Movie.html#method.new_unsafe) method.
  pub trait MovieNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::movie::Movie>;
  }
  impl MovieNewUnsafeArgs for *mut ::qt_core::io_device::IODevice {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::movie::Movie> {
      let device = self;
      let ffi_result = ::ffi::qt_gui_c_QMovie_new_device(device);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> MovieNewUnsafeArgs for (*mut ::qt_core::io_device::IODevice, &'a ::qt_core::byte_array::ByteArray) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::movie::Movie> {
      let device = self.0;
      let format = self.1;
      let ffi_result = ::ffi::qt_gui_c_QMovie_new_device_format(device,
                                                                format as *const ::qt_core::byte_array::ByteArray);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> MovieNewUnsafeArgs
    for (*mut ::qt_core::io_device::IODevice, &'a ::qt_core::byte_array::ByteArray, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::movie::Movie> {
      let device = self.0;
      let format = self.1;
      let parent = self.2;
      let ffi_result =
        ::ffi::qt_gui_c_QMovie_new_device_format_parent(device,
                                                        format as *const ::qt_core::byte_array::ByteArray,
                                                        parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> MovieNewUnsafeArgs
    for (&'a ::qt_core::string::String, &'a ::qt_core::byte_array::ByteArray, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::movie::Movie> {
      let file_name = self.0;
      let format = self.1;
      let parent = self.2;
      let ffi_result =
        ::ffi::qt_gui_c_QMovie_new_fileName_format_parent(file_name as *const ::qt_core::string::String,
                                                          format as *const ::qt_core::byte_array::ByteArray,
                                                          parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl MovieNewUnsafeArgs for *mut ::qt_core::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::movie::Movie> {
      let parent = self;
      let ffi_result = ::ffi::qt_gui_c_QMovie_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
