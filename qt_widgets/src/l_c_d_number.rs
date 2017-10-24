/// C++ type: <span style='color: green;'>```QLCDNumber```</span>
#[repr(C)]
pub struct LCDNumber(u8);

impl LCDNumber {
  /// C++ method: <span style='color: green;'>```QLCDNumber::checkOverflow```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn check_overflow(&self, ::libc::c_double) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLCDNumber::checkOverflow(double num) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn check_overflow(&self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLCDNumber::checkOverflow(int num) const```</span>
  ///
  ///
  pub fn check_overflow<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::LCDNumberCheckOverflowArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QLCDNumber::digitCount() const```</span>
  ///
  ///
  pub fn digit_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_digitCount(self as *const ::l_c_d_number::LCDNumber) }
  }

  /// C++ method: <span style='color: green;'>```QLCDNumber::display```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn display(&mut self, &::qt_core::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QLCDNumber::display(const QString& str)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn display(&mut self, ::libc::c_double) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QLCDNumber::display(double num)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn display(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QLCDNumber::display(int num)```</span>
  ///
  ///
  pub fn display<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::LCDNumberDisplayArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QLCDNumber::intValue() const```</span>
  ///
  ///
  pub fn int_value(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_intValue(self as *const ::l_c_d_number::LCDNumber) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QLCDNumber::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_metaObject(self as *const ::l_c_d_number::LCDNumber) }
  }

  /// C++ method: <span style='color: green;'>```QLCDNumber::Mode QLCDNumber::mode() const```</span>
  ///
  ///
  pub fn mode(&self) -> ::l_c_d_number::Mode {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_mode(self as *const ::l_c_d_number::LCDNumber) }
  }

  /// C++ method: <span style='color: green;'>```QLCDNumber::QLCDNumber```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::l_c_d_number::LCDNumber>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLCDNumber::QLCDNumber()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::libc::c_uint) -> ::cpp_utils::CppBox<::l_c_d_number::LCDNumber>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLCDNumber::QLCDNumber(unsigned int numDigits)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::l_c_d_number::LCDNumber>
    where Args: overloading::LCDNumberNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QLCDNumber::QLCDNumber```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::l_c_d_number::LCDNumber>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLCDNumber::QLCDNumber(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((::libc::c_uint, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::l_c_d_number::LCDNumber>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLCDNumber::QLCDNumber(unsigned int numDigits, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::l_c_d_number::LCDNumber>
    where Args: overloading::LCDNumberNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QLCDNumber::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QLCDNumber_qt_metacall(self as *mut ::l_c_d_number::LCDNumber,
                                               arg1 as *const ::qt_core::meta_object::Call,
                                               arg2,
                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QLCDNumber::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QLCDNumber_qt_metacast(self as *mut ::l_c_d_number::LCDNumber, arg1)
  }

  /// C++ method: <span style='color: green;'>```QLCDNumber::SegmentStyle QLCDNumber::segmentStyle() const```</span>
  ///
  ///
  pub fn segment_style(&self) -> ::l_c_d_number::SegmentStyle {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_segmentStyle(self as *const ::l_c_d_number::LCDNumber) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLCDNumber::setBinMode()```</span>
  ///
  ///
  pub fn set_bin_mode(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_setBinMode(self as *mut ::l_c_d_number::LCDNumber) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLCDNumber::setDecMode()```</span>
  ///
  ///
  pub fn set_dec_mode(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_setDecMode(self as *mut ::l_c_d_number::LCDNumber) }
  }

  /// C++ method: <span style='color: green;'>```void QLCDNumber::setDigitCount(int nDigits)```</span>
  ///
  ///
  pub fn set_digit_count(&mut self, n_digits: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_setDigitCount(self as *mut ::l_c_d_number::LCDNumber, n_digits) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLCDNumber::setHexMode()```</span>
  ///
  ///
  pub fn set_hex_mode(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_setHexMode(self as *mut ::l_c_d_number::LCDNumber) }
  }

  /// C++ method: <span style='color: green;'>```void QLCDNumber::setMode(QLCDNumber::Mode arg1)```</span>
  ///
  ///
  pub fn set_mode(&mut self, arg1: ::l_c_d_number::Mode) {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_setMode(self as *mut ::l_c_d_number::LCDNumber, arg1) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLCDNumber::setOctMode()```</span>
  ///
  ///
  pub fn set_oct_mode(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_setOctMode(self as *mut ::l_c_d_number::LCDNumber) }
  }

  /// C++ method: <span style='color: green;'>```void QLCDNumber::setSegmentStyle(QLCDNumber::SegmentStyle arg1)```</span>
  ///
  ///
  pub fn set_segment_style(&mut self, arg1: ::l_c_d_number::SegmentStyle) {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_setSegmentStyle(self as *mut ::l_c_d_number::LCDNumber, arg1) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLCDNumber::setSmallDecimalPoint(bool arg1)```</span>
  ///
  ///
  pub fn set_small_decimal_point(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_setSmallDecimalPoint(self as *mut ::l_c_d_number::LCDNumber, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QLCDNumber::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLCDNumber_sizeHint_to_output(self as *const ::l_c_d_number::LCDNumber, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QLCDNumber::smallDecimalPoint() const```</span>
  ///
  ///
  pub fn small_decimal_point(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_smallDecimalPoint(self as *const ::l_c_d_number::LCDNumber) }
  }

  /// C++ method: <span style='color: green;'>```static QString QLCDNumber::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QLCDNumber_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QLCDNumber::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QLCDNumber_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QLCDNumber::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QLCDNumber_value(self as *const ::l_c_d_number::LCDNumber) }
  }
}

impl ::cpp_utils::CppDeletable for ::l_c_d_number::LCDNumber {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QLCDNumber_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `LCDNumber`.
  pub struct Signals<'a>(&'a ::l_c_d_number::LCDNumber);
  /// Represents a built-in Qt signal `QLCDNumber::overflow`.
  ///
  /// An object of this type can be created from `LCDNumber` with `object.signals().overflow()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LCDNumber` object.
  pub struct Overflow<'a>(&'a ::l_c_d_number::LCDNumber);
  impl<'a> ::qt_core::connection::Receiver for Overflow<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2overflow()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Overflow<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QLCDNumber::overflow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn overflow(&self) -> Overflow {
      Overflow(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `LCDNumber`.
  pub struct Slots<'a>(&'a ::l_c_d_number::LCDNumber);
  /// Represents a built-in Qt slot `QLCDNumber::setBinMode`.
  ///
  /// An object of this type can be created from `LCDNumber` with `object.slots().set_bin_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LCDNumber` object.
  pub struct SetBinMode<'a>(&'a ::l_c_d_number::LCDNumber);
  impl<'a> ::qt_core::connection::Receiver for SetBinMode<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBinMode()\0"
    }
  }
  /// Represents a built-in Qt slot `QLCDNumber::display`.
  ///
  /// An object of this type can be created from `LCDNumber` with `object.slots().display_qt_core_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LCDNumber` object.
  pub struct DisplayQtCoreStringRef<'a>(&'a ::l_c_d_number::LCDNumber);
  impl<'a> ::qt_core::connection::Receiver for DisplayQtCoreStringRef<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1display(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QLCDNumber::display`.
  ///
  /// An object of this type can be created from `LCDNumber` with `object.slots().display_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LCDNumber` object.
  pub struct DisplayCInt<'a>(&'a ::l_c_d_number::LCDNumber);
  impl<'a> ::qt_core::connection::Receiver for DisplayCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1display(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QLCDNumber::display`.
  ///
  /// An object of this type can be created from `LCDNumber` with `object.slots().display_c_double()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LCDNumber` object.
  pub struct DisplayCDouble<'a>(&'a ::l_c_d_number::LCDNumber);
  impl<'a> ::qt_core::connection::Receiver for DisplayCDouble<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1display(double)\0"
    }
  }
  /// Represents a built-in Qt slot `QLCDNumber::setDecMode`.
  ///
  /// An object of this type can be created from `LCDNumber` with `object.slots().set_dec_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LCDNumber` object.
  pub struct SetDecMode<'a>(&'a ::l_c_d_number::LCDNumber);
  impl<'a> ::qt_core::connection::Receiver for SetDecMode<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDecMode()\0"
    }
  }
  /// Represents a built-in Qt slot `QLCDNumber::setSmallDecimalPoint`.
  ///
  /// An object of this type can be created from `LCDNumber` with `object.slots().set_small_decimal_point()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LCDNumber` object.
  pub struct SetSmallDecimalPoint<'a>(&'a ::l_c_d_number::LCDNumber);
  impl<'a> ::qt_core::connection::Receiver for SetSmallDecimalPoint<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSmallDecimalPoint(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QLCDNumber::setOctMode`.
  ///
  /// An object of this type can be created from `LCDNumber` with `object.slots().set_oct_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LCDNumber` object.
  pub struct SetOctMode<'a>(&'a ::l_c_d_number::LCDNumber);
  impl<'a> ::qt_core::connection::Receiver for SetOctMode<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setOctMode()\0"
    }
  }
  /// Represents a built-in Qt slot `QLCDNumber::setHexMode`.
  ///
  /// An object of this type can be created from `LCDNumber` with `object.slots().set_hex_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LCDNumber` object.
  pub struct SetHexMode<'a>(&'a ::l_c_d_number::LCDNumber);
  impl<'a> ::qt_core::connection::Receiver for SetHexMode<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHexMode()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QLCDNumber::setBinMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_bin_mode(&self) -> SetBinMode {
      SetBinMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLCDNumber::display`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn display_qt_core_string_ref(&self) -> DisplayQtCoreStringRef {
      DisplayQtCoreStringRef(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLCDNumber::display`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn display_c_int(&self) -> DisplayCInt {
      DisplayCInt(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLCDNumber::display`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn display_c_double(&self) -> DisplayCDouble {
      DisplayCDouble(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLCDNumber::setDecMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_dec_mode(&self) -> SetDecMode {
      SetDecMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLCDNumber::setSmallDecimalPoint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_small_decimal_point(&self) -> SetSmallDecimalPoint {
      SetSmallDecimalPoint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLCDNumber::setOctMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_oct_mode(&self) -> SetOctMode {
      SetOctMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLCDNumber::setHexMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hex_mode(&self) -> SetHexMode {
      SetHexMode(self.0)
    }
  }
  impl ::l_c_d_number::LCDNumber {
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

/// C++ type: <span style='color: green;'>```QLCDNumber::Mode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Mode {
  /// C++ enum variant: <span style='color: green;'>```Hex = 0```</span>
  Hex = 0,
  /// C++ enum variant: <span style='color: green;'>```Dec = 1```</span>
  Dec = 1,
  /// C++ enum variant: <span style='color: green;'>```Oct = 2```</span>
  Oct = 2,
  /// C++ enum variant: <span style='color: green;'>```Bin = 3```</span>
  Bin = 3,
}

/// C++ type: <span style='color: green;'>```QLCDNumber::SegmentStyle```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SegmentStyle {
  /// C++ enum variant: <span style='color: green;'>```Outline = 0```</span>
  Outline = 0,
  /// C++ enum variant: <span style='color: green;'>```Filled = 1```</span>
  Filled = 1,
  /// C++ enum variant: <span style='color: green;'>```Flat = 2```</span>
  Flat = 2,
}

impl ::cpp_utils::DynamicCast<::l_c_d_number::LCDNumber> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::l_c_d_number::LCDNumber> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLCDNumber_G_dynamic_cast_QLCDNumber_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::l_c_d_number::LCDNumber> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLCDNumber_G_dynamic_cast_QLCDNumber_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::l_c_d_number::LCDNumber> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::l_c_d_number::LCDNumber> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLCDNumber_G_dynamic_cast_QLCDNumber_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::l_c_d_number::LCDNumber> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLCDNumber_G_dynamic_cast_QLCDNumber_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::l_c_d_number::LCDNumber {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QObject_ptr(self as *mut ::l_c_d_number::LCDNumber) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QObject_ptr(self as *const ::l_c_d_number::LCDNumber as *mut ::l_c_d_number::LCDNumber) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::l_c_d_number::LCDNumber {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QPaintDevice_ptr(self as *mut ::l_c_d_number::LCDNumber) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QPaintDevice_ptr(self as *const ::l_c_d_number::LCDNumber as *mut ::l_c_d_number::LCDNumber) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::l_c_d_number::LCDNumber {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QFrame_ptr(self as *mut ::l_c_d_number::LCDNumber) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QFrame_ptr(self as *const ::l_c_d_number::LCDNumber as *mut ::l_c_d_number::LCDNumber) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::l_c_d_number::LCDNumber {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QWidget_ptr(self as *mut ::l_c_d_number::LCDNumber) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QWidget_ptr(self as *const ::l_c_d_number::LCDNumber as *mut ::l_c_d_number::LCDNumber) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::l_c_d_number::LCDNumber> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::l_c_d_number::LCDNumber {
    let ffi_result = ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::l_c_d_number::LCDNumber {
    let ffi_result = ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::l_c_d_number::LCDNumber> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::l_c_d_number::LCDNumber {
    let ffi_result =
      ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::l_c_d_number::LCDNumber {
    let ffi_result = ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::l_c_d_number::LCDNumber> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::l_c_d_number::LCDNumber {
    let ffi_result = ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::l_c_d_number::LCDNumber {
    let ffi_result = ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::l_c_d_number::LCDNumber> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::l_c_d_number::LCDNumber {
    let ffi_result =
      ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::l_c_d_number::LCDNumber {
    let ffi_result = ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QLCDNumber_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::l_c_d_number::LCDNumber {
  type Target = ::frame::Frame;
  fn deref(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QFrame_ptr(self as *const ::l_c_d_number::LCDNumber as *mut ::l_c_d_number::LCDNumber) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::l_c_d_number::LCDNumber {
  fn deref_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLCDNumber_G_static_cast_QFrame_ptr(self as *mut ::l_c_d_number::LCDNumber) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [LCDNumber::check_overflow](../struct.LCDNumber.html#method.check_overflow) method.
  pub trait LCDNumberCheckOverflowArgs<'largs> {
    fn exec(self, original_self: &'largs ::l_c_d_number::LCDNumber) -> bool;
  }
  impl<'largs> LCDNumberCheckOverflowArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs ::l_c_d_number::LCDNumber) -> bool {
      let num = self;
      unsafe {
        ::ffi::qt_widgets_c_QLCDNumber_checkOverflow_double(original_self as *const ::l_c_d_number::LCDNumber, num)
      }
    }
  }
  impl<'largs> LCDNumberCheckOverflowArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::l_c_d_number::LCDNumber) -> bool {
      let num = self;
      unsafe {
        ::ffi::qt_widgets_c_QLCDNumber_checkOverflow_int(original_self as *const ::l_c_d_number::LCDNumber, num)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [LCDNumber::display](../struct.LCDNumber.html#method.display) method.
  pub trait LCDNumberDisplayArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::l_c_d_number::LCDNumber) -> ();
  }
  impl<'largs> LCDNumberDisplayArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::l_c_d_number::LCDNumber) -> () {
      let str = self;
      unsafe {
        ::ffi::qt_widgets_c_QLCDNumber_display_QString(original_self as *mut ::l_c_d_number::LCDNumber,
                                                       str as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> LCDNumberDisplayArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::l_c_d_number::LCDNumber) -> () {
      let num = self;
      unsafe { ::ffi::qt_widgets_c_QLCDNumber_display_double(original_self as *mut ::l_c_d_number::LCDNumber, num) }
    }
  }
  impl<'largs> LCDNumberDisplayArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::l_c_d_number::LCDNumber) -> () {
      let num = self;
      unsafe { ::ffi::qt_widgets_c_QLCDNumber_display_int(original_self as *mut ::l_c_d_number::LCDNumber, num) }
    }
  }
  /// This trait represents a set of arguments accepted by [LCDNumber::new](../struct.LCDNumber.html#method.new) method.
  pub trait LCDNumberNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::l_c_d_number::LCDNumber>;
  }
  impl LCDNumberNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::l_c_d_number::LCDNumber> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QLCDNumber_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl LCDNumberNewArgs for ::libc::c_uint {
    fn exec(self) -> ::cpp_utils::CppBox<::l_c_d_number::LCDNumber> {
      let num_digits = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QLCDNumber_new_numDigits(num_digits) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [LCDNumber::new_unsafe](../struct.LCDNumber.html#method.new_unsafe) method.
  pub trait LCDNumberNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::l_c_d_number::LCDNumber>;
  }
  impl LCDNumberNewUnsafeArgs for (::libc::c_uint, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::l_c_d_number::LCDNumber> {
      let num_digits = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QLCDNumber_new_numDigits_parent(num_digits, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl LCDNumberNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::l_c_d_number::LCDNumber> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QLCDNumber_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
