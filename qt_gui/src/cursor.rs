/// C++ type: <span style='color: green;'>```QCursor```</span>
#[repr(C)]
pub struct Cursor(u8);

impl Cursor {
  /// C++ method: <span style='color: green;'>```QVariant QCursor::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QCursor_convert_to_QVariant_to_output(self as *const ::cursor::Cursor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QBitmap* QCursor::bitmap() const```</span>
  ///
  ///
  pub fn bitmap(&self) -> *const ::bitmap::Bitmap {
    unsafe { ::ffi::qt_gui_c_QCursor_bitmap(self as *const ::cursor::Cursor) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QCursor::hotSpot() const```</span>
  ///
  ///
  pub fn hot_spot(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QCursor_hotSpot_to_output(self as *const ::cursor::Cursor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QBitmap* QCursor::mask() const```</span>
  ///
  ///
  pub fn mask(&self) -> *const ::bitmap::Bitmap {
    unsafe { ::ffi::qt_gui_c_QCursor_mask(self as *const ::cursor::Cursor) }
  }

  /// C++ method: <span style='color: green;'>```QCursor::QCursor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::cursor::Cursor>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCursor::QCursor()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::qt::CursorShape) -> ::cpp_utils::CppBox<::cursor::Cursor>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCursor::QCursor(Qt::CursorShape shape)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::bitmap::Bitmap, &::bitmap::Bitmap)) -> ::cpp_utils::CppBox<::cursor::Cursor>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCursor::QCursor(const QBitmap& bitmap, const QBitmap& mask)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::bitmap::Bitmap, &::bitmap::Bitmap, ::libc::c_int)) -> ::cpp_utils::CppBox<::cursor::Cursor>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCursor::QCursor(const QBitmap& bitmap, const QBitmap& mask, int hotX = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::bitmap::Bitmap, &::bitmap::Bitmap, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::cursor::Cursor>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCursor::QCursor(const QBitmap& bitmap, const QBitmap& mask, int hotX = ?, int hotY = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new(&::cursor::Cursor) -> ::cpp_utils::CppBox<::cursor::Cursor>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCursor::QCursor(const QCursor& cursor)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new(&::pixmap::Pixmap) -> ::cpp_utils::CppBox<::cursor::Cursor>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCursor::QCursor(const QPixmap& pixmap)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new((&::pixmap::Pixmap, ::libc::c_int)) -> ::cpp_utils::CppBox<::cursor::Cursor>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCursor::QCursor(const QPixmap& pixmap, int hotX = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn new((&::pixmap::Pixmap, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::cursor::Cursor>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCursor::QCursor(const QPixmap& pixmap, int hotX = ?, int hotY = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::cursor::Cursor>
    where Args: overloading::CursorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QCursor& QCursor::operator=(const QCursor& cursor)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, cursor: &'l1 ::cursor::Cursor) -> &'l0 mut ::cursor::Cursor {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QCursor_operator_assign(self as *mut ::cursor::Cursor,
                                              cursor as *const ::cursor::Cursor)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPixmap QCursor::pixmap() const```</span>
  ///
  ///
  pub fn pixmap(&self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QCursor_pixmap_as_ptr(self as *const ::cursor::Cursor) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```static QPoint QCursor::pos()```</span>
  ///
  ///
  pub fn pos() -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QCursor_pos_to_output_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QPoint QCursor::pos(const QScreen* screen)```</span>
  ///
  ///
  pub unsafe fn pos_unsafe(screen: *const ::screen::Screen) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QCursor_pos_to_output_screen(screen, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCursor::setPos```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_pos(&::qt_core::point::Point) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QCursor::setPos(const QPoint& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_pos((::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QCursor::setPos(int x, int y)```</span>
  ///
  ///
  pub fn set_pos<Args>(args: Args) -> ()
    where Args: overloading::CursorSetPosArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QCursor::setPos```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_pos_unsafe((*mut ::screen::Screen, &::qt_core::point::Point)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QCursor::setPos(QScreen* screen, const QPoint& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_pos_unsafe((*mut ::screen::Screen, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QCursor::setPos(QScreen* screen, int x, int y)```</span>
  ///
  ///
  pub unsafe fn set_pos_unsafe<Args>(args: Args) -> ()
    where Args: overloading::CursorSetPosUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QCursor::setShape(Qt::CursorShape newShape)```</span>
  ///
  ///
  pub fn set_shape(&mut self, new_shape: &::qt_core::qt::CursorShape) {
    unsafe {
      ::ffi::qt_gui_c_QCursor_setShape(self as *mut ::cursor::Cursor,
                                       new_shape as *const ::qt_core::qt::CursorShape)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCursor::swap(QCursor& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::cursor::Cursor) {
    unsafe {
      ::ffi::qt_gui_c_QCursor_swap(self as *mut ::cursor::Cursor,
                                   other as *mut ::cursor::Cursor)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::cursor::Cursor {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QCursor_delete
  }
}

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::cursor::Cursor)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& outS, const QCursor& cursor)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::cursor::Cursor)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QCursor& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& inS, QCursor& cursor)```</span>
///
///
pub fn op_shr<'l0, 'l1>(in_s: &'l0 mut ::qt_core::data_stream::DataStream,
                        cursor: &'l1 mut ::cursor::Cursor)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QCursor_G_operator_shr(in_s as *mut ::qt_core::data_stream::DataStream,
                                           cursor as *mut ::cursor::Cursor)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```void swap(QCursor& value1, QCursor& value2)```</span>
///
///
pub fn swap(value1: &mut ::cursor::Cursor, value2: &mut ::cursor::Cursor) {
  unsafe {
    ::ffi::qt_gui_c_QCursor_G_swap(value1 as *mut ::cursor::Cursor,
                                   value2 as *mut ::cursor::Cursor)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Cursor::new](../struct.Cursor.html#method.new) method.
  pub trait CursorNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::cursor::Cursor>;
  }
  impl<'a> CursorNewArgs for (&'a ::bitmap::Bitmap, &'a ::bitmap::Bitmap) {
    fn exec(self) -> ::cpp_utils::CppBox<::cursor::Cursor> {
      let bitmap = self.0;
      let mask = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QCursor_new_bitmap_mask(bitmap as *const ::bitmap::Bitmap,
                                                mask as *const ::bitmap::Bitmap)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> CursorNewArgs for (&'a ::bitmap::Bitmap, &'a ::bitmap::Bitmap, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::cursor::Cursor> {
      let bitmap = self.0;
      let mask = self.1;
      let hot_x = self.2;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QCursor_new_bitmap_mask_hotX(bitmap as *const ::bitmap::Bitmap,
                                                     mask as *const ::bitmap::Bitmap,
                                                     hot_x)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> CursorNewArgs for (&'a ::bitmap::Bitmap, &'a ::bitmap::Bitmap, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::cursor::Cursor> {
      let bitmap = self.0;
      let mask = self.1;
      let hot_x = self.2;
      let hot_y = self.3;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QCursor_new_bitmap_mask_hotX_hotY(bitmap as *const ::bitmap::Bitmap,
                                                          mask as *const ::bitmap::Bitmap,
                                                          hot_x,
                                                          hot_y)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> CursorNewArgs for &'a ::cursor::Cursor {
    fn exec(self) -> ::cpp_utils::CppBox<::cursor::Cursor> {
      let cursor = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QCursor_new_cursor(cursor as *const ::cursor::Cursor) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl CursorNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::cursor::Cursor> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QCursor_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> CursorNewArgs for &'a ::pixmap::Pixmap {
    fn exec(self) -> ::cpp_utils::CppBox<::cursor::Cursor> {
      let pixmap = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QCursor_new_pixmap(pixmap as *const ::pixmap::Pixmap) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> CursorNewArgs for (&'a ::pixmap::Pixmap, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::cursor::Cursor> {
      let pixmap = self.0;
      let hot_x = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QCursor_new_pixmap_hotX(pixmap as *const ::pixmap::Pixmap, hot_x) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> CursorNewArgs for (&'a ::pixmap::Pixmap, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::cursor::Cursor> {
      let pixmap = self.0;
      let hot_x = self.1;
      let hot_y = self.2;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QCursor_new_pixmap_hotX_hotY(pixmap as *const ::pixmap::Pixmap, hot_x, hot_y) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> CursorNewArgs for &'a ::qt_core::qt::CursorShape {
    fn exec(self) -> ::cpp_utils::CppBox<::cursor::Cursor> {
      let shape = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QCursor_new_shape(shape as *const ::qt_core::qt::CursorShape) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Cursor::set_pos](../struct.Cursor.html#method.set_pos) method.
  pub trait CursorSetPosArgs {
    fn exec(self) -> ();
  }
  impl<'a> CursorSetPosArgs for &'a ::qt_core::point::Point {
    fn exec(self) -> () {
      let p = self;
      unsafe { ::ffi::qt_gui_c_QCursor_setPos_p(p as *const ::qt_core::point::Point) }
    }
  }
  impl CursorSetPosArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> () {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_gui_c_QCursor_setPos_x_y(x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [Cursor::set_pos_unsafe](../struct.Cursor.html#method.set_pos_unsafe) method.
  pub trait CursorSetPosUnsafeArgs {
    unsafe fn exec(self) -> ();
  }
  impl<'a> CursorSetPosUnsafeArgs for (*mut ::screen::Screen, &'a ::qt_core::point::Point) {
    unsafe fn exec(self) -> () {
      let screen = self.0;
      let p = self.1;
      ::ffi::qt_gui_c_QCursor_setPos_screen_p(screen, p as *const ::qt_core::point::Point)
    }
  }
  impl CursorSetPosUnsafeArgs for (*mut ::screen::Screen, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let screen = self.0;
      let x = self.1;
      let y = self.2;
      ::ffi::qt_gui_c_QCursor_setPos_screen_x_y(screen, x, y)
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::cursor::Cursor) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let out_s = self.0;
      let cursor = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QCursor_G_operator_shl(out_s as *mut ::qt_core::data_stream::DataStream,
                                               cursor as *const ::cursor::Cursor)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::cursor::Cursor) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QCursor_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug,
                                                           arg2 as *const ::cursor::Cursor,
                                                           &mut object);
        }
        object
      }
    }
  }
}
