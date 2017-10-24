/// C++ type: <span style='color: green;'>```QPixmap```</span>
#[repr(C)]
pub struct Pixmap(u8);

impl Pixmap {
  /// C++ method: <span style='color: green;'>```QVariant QPixmap::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPixmap_convert_to_QVariant_to_output(self as *const ::pixmap::Pixmap, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QPixmap::cacheKey() const```</span>
  ///
  ///
  pub fn cache_key(&self) -> i64 {
    unsafe { ::ffi::qt_gui_c_QPixmap_cacheKey(self as *const ::pixmap::Pixmap) }
  }

  /// C++ method: <span style='color: green;'>```QPixmap::copy```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn copy(&self, ()) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::copy() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn copy(&self, &::qt_core::rect::Rect) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::copy(const QRect& rect = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn copy(&self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::copy(int x, int y, int width, int height) const```</span>
  ///
  ///
  pub fn copy<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::pixmap::Pixmap>
    where Args: overloading::PixmapCopyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPixmap::createHeuristicMask```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_heuristic_mask(&self, ()) -> ::cpp_utils::CppBox<::bitmap::Bitmap>```<br>
  /// C++ method: <span style='color: green;'>```QBitmap QPixmap::createHeuristicMask() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_heuristic_mask(&self, bool) -> ::cpp_utils::CppBox<::bitmap::Bitmap>```<br>
  /// C++ method: <span style='color: green;'>```QBitmap QPixmap::createHeuristicMask(bool clipTight = ?) const```</span>
  ///
  ///
  pub fn create_heuristic_mask<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::bitmap::Bitmap>
    where Args: overloading::PixmapCreateHeuristicMaskArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPixmap::createMaskFromColor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_mask_from_color(&self, &::color::Color) -> ::cpp_utils::CppBox<::bitmap::Bitmap>```<br>
  /// C++ method: <span style='color: green;'>```QBitmap QPixmap::createMaskFromColor(const QColor& maskColor) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_mask_from_color(&self, (&::color::Color, &::qt_core::qt::MaskMode)) -> ::cpp_utils::CppBox<::bitmap::Bitmap>```<br>
  /// C++ method: <span style='color: green;'>```QBitmap QPixmap::createMaskFromColor(const QColor& maskColor, Qt::MaskMode mode = ?) const```</span>
  ///
  ///
  pub fn create_mask_from_color<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::bitmap::Bitmap>
    where Args: overloading::PixmapCreateMaskFromColorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static int QPixmap::defaultDepth()```</span>
  ///
  ///
  pub fn default_depth() -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPixmap_defaultDepth() }
  }

  /// C++ method: <span style='color: green;'>```int QPixmap::depth() const```</span>
  ///
  ///
  pub fn depth(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPixmap_depth(self as *const ::pixmap::Pixmap) }
  }

  /// C++ method: <span style='color: green;'>```void QPixmap::detach()```</span>
  ///
  ///
  pub fn detach(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPixmap_detach(self as *mut ::pixmap::Pixmap) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QPixmap::devType() const```</span>
  ///
  ///
  pub fn dev_type(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPixmap_devType(self as *const ::pixmap::Pixmap) }
  }

  /// C++ method: <span style='color: green;'>```double QPixmap::devicePixelRatio() const```</span>
  ///
  ///
  pub fn device_pixel_ratio(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPixmap_devicePixelRatio(self as *const ::pixmap::Pixmap) }
  }

  /// C++ method: <span style='color: green;'>```QPixmap::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPixmap::fill()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, &::color::Color) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPixmap::fill(const QColor& fillColor = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PixmapFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPixmap::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill_unsafe(&mut self, (*const ::paint_device::PaintDevice, &::qt_core::point::Point)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPixmap::fill(const QPaintDevice* device, const QPoint& ofs)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill_unsafe(&mut self, (*const ::paint_device::PaintDevice, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPixmap::fill(const QPaintDevice* device, int xofs, int yofs)```</span>
  ///
  ///
  pub unsafe fn fill_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PixmapFillUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPixmap::grabWidget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn grab_widget(*mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```static QPixmap QPixmap::grabWidget(QObject* widget)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn grab_widget((*mut ::qt_core::object::Object, &::qt_core::rect::Rect)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```static QPixmap QPixmap::grabWidget(QObject* widget, const QRect& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn grab_widget((*mut ::qt_core::object::Object, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```static QPixmap QPixmap::grabWidget(QObject* widget, int x = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn grab_widget((*mut ::qt_core::object::Object, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```static QPixmap QPixmap::grabWidget(QObject* widget, int x = ?, int y = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn grab_widget((*mut ::qt_core::object::Object, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```static QPixmap QPixmap::grabWidget(QObject* widget, int x = ?, int y = ?, int w = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn grab_widget((*mut ::qt_core::object::Object, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```static QPixmap QPixmap::grabWidget(QObject* widget, int x = ?, int y = ?, int w = ?, int h = ?)```</span>
  ///
  ///
  pub unsafe fn grab_widget<Args>(args: Args) -> ::cpp_utils::CppBox<::pixmap::Pixmap>
    where Args: overloading::PixmapGrabWidgetArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPixmap::grabWindow```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn grab_window(::libc::c_ulonglong) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```static QPixmap QPixmap::grabWindow(unsigned long long arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn grab_window((::libc::c_ulonglong, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```static QPixmap QPixmap::grabWindow(unsigned long long arg1, int x = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn grab_window((::libc::c_ulonglong, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```static QPixmap QPixmap::grabWindow(unsigned long long arg1, int x = ?, int y = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn grab_window((::libc::c_ulonglong, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```static QPixmap QPixmap::grabWindow(unsigned long long arg1, int x = ?, int y = ?, int w = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn grab_window((::libc::c_ulonglong, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```static QPixmap QPixmap::grabWindow(unsigned long long arg1, int x = ?, int y = ?, int w = ?, int h = ?)```</span>
  ///
  ///
  pub fn grab_window<Args>(args: Args) -> ::cpp_utils::CppBox<::pixmap::Pixmap>
    where Args: overloading::PixmapGrabWindowArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QPixmap::hasAlpha() const```</span>
  ///
  ///
  pub fn has_alpha(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPixmap_hasAlpha(self as *const ::pixmap::Pixmap) }
  }

  /// C++ method: <span style='color: green;'>```bool QPixmap::hasAlphaChannel() const```</span>
  ///
  ///
  pub fn has_alpha_channel(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPixmap_hasAlphaChannel(self as *const ::pixmap::Pixmap) }
  }

  /// C++ method: <span style='color: green;'>```int QPixmap::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPixmap_height(self as *const ::pixmap::Pixmap) }
  }

  /// C++ method: <span style='color: green;'>```bool QPixmap::isDetached() const```</span>
  ///
  ///
  pub fn is_detached(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPixmap_isDetached(self as *const ::pixmap::Pixmap) }
  }

  /// C++ method: <span style='color: green;'>```bool QPixmap::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPixmap_isNull(self as *const ::pixmap::Pixmap) }
  }

  /// C++ method: <span style='color: green;'>```bool QPixmap::isQBitmap() const```</span>
  ///
  ///
  pub fn is_q_bitmap(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPixmap_isQBitmap(self as *const ::pixmap::Pixmap) }
  }

  /// C++ method: <span style='color: green;'>```QBitmap QPixmap::mask() const```</span>
  ///
  ///
  pub fn mask(&self) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_mask_as_ptr(self as *const ::pixmap::Pixmap) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QPixmap::QPixmap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPixmap::QPixmap()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPixmap::QPixmap(const QPixmap& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::qt_core::size::Size) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPixmap::QPixmap(const QSize& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPixmap::QPixmap(int w, int h)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::pixmap::Pixmap>
    where Args: overloading::PixmapNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPixmap& QPixmap::operator=(const QPixmap& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::pixmap::Pixmap) -> &'l0 mut ::pixmap::Pixmap {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QPixmap_operator_assign(self as *mut ::pixmap::Pixmap,
                                              arg1 as *const ::pixmap::Pixmap)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QPixmap::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPixmap_operator_not(self as *const ::pixmap::Pixmap) }
  }

  /// C++ method: <span style='color: green;'>```virtual QPaintEngine* QPixmap::paintEngine() const```</span>
  ///
  ///
  pub fn paint_engine(&self) -> *mut ::paint_engine::PaintEngine {
    unsafe { ::ffi::qt_gui_c_QPixmap_paintEngine(self as *const ::pixmap::Pixmap) }
  }

  /// C++ method: <span style='color: green;'>```QRect QPixmap::rect() const```</span>
  ///
  ///
  pub fn rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPixmap_rect_to_output(self as *const ::pixmap::Pixmap, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPixmap::save(const QString& fileName) const```</span>
  ///
  ///
  pub fn save(&self, file_name: &::qt_core::string::String) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPixmap_save_fileName(self as *const ::pixmap::Pixmap,
                                            file_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QPixmap::save```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn save_unsafe(&self, *mut ::qt_core::io_device::IODevice) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPixmap::save(QIODevice* device) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn save_unsafe(&self, (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPixmap::save(QIODevice* device, const char* format = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn save_unsafe(&self, (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPixmap::save(QIODevice* device, const char* format = ?, int quality = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn save_unsafe(&self, (&::qt_core::string::String, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPixmap::save(const QString& fileName, const char* format = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn save_unsafe(&self, (&::qt_core::string::String, *const ::libc::c_char, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPixmap::save(const QString& fileName, const char* format = ?, int quality = ?) const```</span>
  ///
  ///
  pub unsafe fn save_unsafe<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::PixmapSaveUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPixmap::scaled```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scaled(&self, &::qt_core::size::Size) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::scaled(const QSize& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scaled(&self, (&::qt_core::size::Size, &::qt_core::qt::AspectRatioMode)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::scaled(const QSize& s, Qt::AspectRatioMode aspectMode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn scaled(&self, (&::qt_core::size::Size, &::qt_core::qt::AspectRatioMode, &::qt_core::qt::TransformationMode)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::scaled(const QSize& s, Qt::AspectRatioMode aspectMode = ?, Qt::TransformationMode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn scaled(&self, (::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::scaled(int w, int h) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn scaled(&self, (::libc::c_int, ::libc::c_int, &::qt_core::qt::AspectRatioMode)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::scaled(int w, int h, Qt::AspectRatioMode aspectMode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn scaled(&self, (::libc::c_int, ::libc::c_int, &::qt_core::qt::AspectRatioMode, &::qt_core::qt::TransformationMode)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::scaled(int w, int h, Qt::AspectRatioMode aspectMode = ?, Qt::TransformationMode mode = ?) const```</span>
  ///
  ///
  pub fn scaled<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::pixmap::Pixmap>
    where Args: overloading::PixmapScaledArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPixmap::scaledToHeight```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scaled_to_height(&self, ::libc::c_int) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::scaledToHeight(int h) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scaled_to_height(&self, (::libc::c_int, &::qt_core::qt::TransformationMode)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::scaledToHeight(int h, Qt::TransformationMode mode = ?) const```</span>
  ///
  ///
  pub fn scaled_to_height<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::pixmap::Pixmap>
    where Args: overloading::PixmapScaledToHeightArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPixmap::scaledToWidth```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scaled_to_width(&self, ::libc::c_int) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::scaledToWidth(int w) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scaled_to_width(&self, (::libc::c_int, &::qt_core::qt::TransformationMode)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::scaledToWidth(int w, Qt::TransformationMode mode = ?) const```</span>
  ///
  ///
  pub fn scaled_to_width<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::pixmap::Pixmap>
    where Args: overloading::PixmapScaledToWidthArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPixmap::scroll```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scroll(&mut self, (::libc::c_int, ::libc::c_int, &::qt_core::rect::Rect)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPixmap::scroll(int dx, int dy, const QRect& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scroll(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPixmap::scroll(int dx, int dy, int x, int y, int width, int height)```</span>
  ///
  ///
  pub fn scroll<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PixmapScrollArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPixmap::scroll```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scroll_unsafe(&mut self, (::libc::c_int, ::libc::c_int, &::qt_core::rect::Rect, *mut ::region::Region)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPixmap::scroll(int dx, int dy, const QRect& rect, QRegion* exposed = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scroll_unsafe(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, *mut ::region::Region)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPixmap::scroll(int dx, int dy, int x, int y, int width, int height, QRegion* exposed = ?)```</span>
  ///
  ///
  pub unsafe fn scroll_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PixmapScrollUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPixmap::setDevicePixelRatio(double scaleFactor)```</span>
  ///
  ///
  pub fn set_device_pixel_ratio(&mut self, scale_factor: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPixmap_setDevicePixelRatio(self as *mut ::pixmap::Pixmap, scale_factor) }
  }

  /// C++ method: <span style='color: green;'>```void QPixmap::setMask(const QBitmap& arg1)```</span>
  ///
  ///
  pub fn set_mask(&mut self, arg1: &::bitmap::Bitmap) {
    unsafe {
      ::ffi::qt_gui_c_QPixmap_setMask(self as *mut ::pixmap::Pixmap,
                                      arg1 as *const ::bitmap::Bitmap)
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QPixmap::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPixmap_size_to_output(self as *const ::pixmap::Pixmap, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QPixmap::swap(QPixmap& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::pixmap::Pixmap) {
    unsafe {
      ::ffi::qt_gui_c_QPixmap_swap(self as *mut ::pixmap::Pixmap,
                                   other as *mut ::pixmap::Pixmap)
    }
  }

  /// C++ method: <span style='color: green;'>```QImage QPixmap::toImage() const```</span>
  ///
  ///
  pub fn to_image(&self) -> ::cpp_utils::CppBox<::image::Image> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_toImage_as_ptr(self as *const ::pixmap::Pixmap) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QPixmap::transformed```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn transformed(&self, &::matrix::Matrix) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::transformed(const QMatrix& arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn transformed(&self, (&::matrix::Matrix, &::qt_core::qt::TransformationMode)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::transformed(const QMatrix& arg1, Qt::TransformationMode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn transformed(&self, &::transform::Transform) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::transformed(const QTransform& arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn transformed(&self, (&::transform::Transform, &::qt_core::qt::TransformationMode)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QPixmap::transformed(const QTransform& arg1, Qt::TransformationMode mode = ?) const```</span>
  ///
  ///
  pub fn transformed<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::pixmap::Pixmap>
    where Args: overloading::PixmapTransformedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPixmap::trueMatrix```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn true_matrix((&::matrix::Matrix, ::libc::c_int, ::libc::c_int)) -> ::matrix::Matrix```<br>
  /// C++ method: <span style='color: green;'>```static QMatrix QPixmap::trueMatrix(const QMatrix& m, int w, int h)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn true_matrix((&::transform::Transform, ::libc::c_int, ::libc::c_int)) -> ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```static QTransform QPixmap::trueMatrix(const QTransform& m, int w, int h)```</span>
  ///
  ///
  pub fn true_matrix<Args>(args: Args) -> Args::ReturnType
    where Args: overloading::PixmapTrueMatrixArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QPixmap::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPixmap_width(self as *const ::pixmap::Pixmap) }
  }
}

impl ::cpp_utils::CppDeletable for ::pixmap::Pixmap {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QPixmap_delete
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
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::pixmap::Pixmap)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QPixmap& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::pixmap::Pixmap)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QPixmap& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```void swap(QPixmap& value1, QPixmap& value2)```</span>
///
///
pub fn swap(value1: &mut ::pixmap::Pixmap, value2: &mut ::pixmap::Pixmap) {
  unsafe {
    ::ffi::qt_gui_c_QPixmap_G_swap(value1 as *mut ::pixmap::Pixmap,
                                   value2 as *mut ::pixmap::Pixmap)
  }
}

impl ::cpp_utils::DynamicCast<::pixmap::Pixmap> for ::paint_device::PaintDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::pixmap::Pixmap> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QPixmap_G_dynamic_cast_QPixmap_ptr(self as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::pixmap::Pixmap> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_G_dynamic_cast_QPixmap_ptr(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::paint_device::PaintDevice> for ::pixmap::Pixmap {
  fn static_cast_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_G_static_cast_QPaintDevice_ptr(self as *mut ::pixmap::Pixmap) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QPixmap_G_static_cast_QPaintDevice_ptr(self as *const ::pixmap::Pixmap as *mut ::pixmap::Pixmap)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::pixmap::Pixmap> for ::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::pixmap::Pixmap {
    let ffi_result = ::ffi::qt_gui_c_QPixmap_G_static_cast_QPixmap_ptr(self as *mut ::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::pixmap::Pixmap {
    let ffi_result = ::ffi::qt_gui_c_QPixmap_G_static_cast_QPixmap_ptr(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::pixmap::Pixmap {
  type Target = ::paint_device::PaintDevice;
  fn deref(&self) -> &::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QPixmap_G_static_cast_QPaintDevice_ptr(self as *const ::pixmap::Pixmap as *mut ::pixmap::Pixmap)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::pixmap::Pixmap {
  fn deref_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_G_static_cast_QPaintDevice_ptr(self as *mut ::pixmap::Pixmap) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Pixmap::copy](../struct.Pixmap.html#method.copy) method.
  pub trait PixmapCopyArgs<'largs> {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap>;
  }
  impl<'largs> PixmapCopyArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_copy_as_ptr_no_args(original_self as *const ::pixmap::Pixmap) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> PixmapCopyArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let rect = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QPixmap_copy_as_ptr_rect(original_self as *const ::pixmap::Pixmap,
                                                 rect as *const ::qt_core::rect::Rect)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> PixmapCopyArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let x = self.0;
      let y = self.1;
      let width = self.2;
      let height = self.3;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QPixmap_copy_as_ptr_x_y_width_height(original_self as *const ::pixmap::Pixmap,
                                                             x,
                                                             y,
                                                             width,
                                                             height)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::create_heuristic_mask](../struct.Pixmap.html#method.create_heuristic_mask) method.
  pub trait PixmapCreateHeuristicMaskArgs<'largs> {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::bitmap::Bitmap>;
  }
  impl<'largs> PixmapCreateHeuristicMaskArgs<'largs> for bool {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {
      let clip_tight = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QPixmap_createHeuristicMask_as_ptr_clipTight(original_self as *const ::pixmap::Pixmap,
                                                                       clip_tight)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> PixmapCreateHeuristicMaskArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {

      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QPixmap_createHeuristicMask_as_ptr_no_args(original_self as *const ::pixmap::Pixmap) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::create_mask_from_color](../struct.Pixmap.html#method.create_mask_from_color) method.
  pub trait PixmapCreateMaskFromColorArgs<'largs> {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::bitmap::Bitmap>;
  }
  impl<'largs> PixmapCreateMaskFromColorArgs<'largs> for &'largs ::color::Color {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {
      let mask_color = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QPixmap_createMaskFromColor_as_ptr_maskColor(original_self as *const ::pixmap::Pixmap,
                                                                       mask_color as *const ::color::Color)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> PixmapCreateMaskFromColorArgs<'largs> for (&'largs ::color::Color, &'largs ::qt_core::qt::MaskMode) {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::bitmap::Bitmap> {
      let mask_color = self.0;
      let mode = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QPixmap_createMaskFromColor_as_ptr_maskColor_mode(original_self as *const ::pixmap::Pixmap,
                                                                            mask_color as *const ::color::Color,
                                                                            mode as *const ::qt_core::qt::MaskMode)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::fill](../struct.Pixmap.html#method.fill) method.
  pub trait PixmapFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::pixmap::Pixmap) -> ();
  }
  impl<'largs> PixmapFillArgs<'largs> for &'largs ::color::Color {
    fn exec(self, original_self: &'largs mut ::pixmap::Pixmap) -> () {
      let fill_color = self;
      unsafe {
        ::ffi::qt_gui_c_QPixmap_fill_fillColor(original_self as *mut ::pixmap::Pixmap,
                                               fill_color as *const ::color::Color)
      }
    }
  }
  impl<'largs> PixmapFillArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::pixmap::Pixmap) -> () {

      unsafe { ::ffi::qt_gui_c_QPixmap_fill_no_args(original_self as *mut ::pixmap::Pixmap) }
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::fill_unsafe](../struct.Pixmap.html#method.fill_unsafe) method.
  pub trait PixmapFillUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::pixmap::Pixmap) -> ();
  }
  impl<'largs> PixmapFillUnsafeArgs<'largs> for (*const ::paint_device::PaintDevice, &'largs ::qt_core::point::Point) {
    unsafe fn exec(self, original_self: &'largs mut ::pixmap::Pixmap) -> () {
      let device = self.0;
      let ofs = self.1;
      ::ffi::qt_gui_c_QPixmap_fill_device_ofs(original_self as *mut ::pixmap::Pixmap,
                                              device,
                                              ofs as *const ::qt_core::point::Point)
    }
  }
  impl<'largs> PixmapFillUnsafeArgs<'largs> for (*const ::paint_device::PaintDevice, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::pixmap::Pixmap) -> () {
      let device = self.0;
      let xofs = self.1;
      let yofs = self.2;
      ::ffi::qt_gui_c_QPixmap_fill_device_xofs_yofs(original_self as *mut ::pixmap::Pixmap, device, xofs, yofs)
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::grab_widget](../struct.Pixmap.html#method.grab_widget) method.
  pub trait PixmapGrabWidgetArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap>;
  }
  impl PixmapGrabWidgetArgs for *mut ::qt_core::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let widget = self;
      let ffi_result = ::ffi::qt_gui_c_QPixmap_grabWidget_as_ptr_widget(widget);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> PixmapGrabWidgetArgs for (*mut ::qt_core::object::Object, &'a ::qt_core::rect::Rect) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let widget = self.0;
      let rect = self.1;
      let ffi_result = ::ffi::qt_gui_c_QPixmap_grabWidget_as_ptr_widget_rect(widget,
                                                                             rect as *const ::qt_core::rect::Rect);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl PixmapGrabWidgetArgs for (*mut ::qt_core::object::Object, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let widget = self.0;
      let x = self.1;
      let ffi_result = ::ffi::qt_gui_c_QPixmap_grabWidget_as_ptr_widget_x(widget, x);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl PixmapGrabWidgetArgs for (*mut ::qt_core::object::Object, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let widget = self.0;
      let x = self.1;
      let y = self.2;
      let ffi_result = ::ffi::qt_gui_c_QPixmap_grabWidget_as_ptr_widget_x_y(widget, x, y);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl PixmapGrabWidgetArgs for (*mut ::qt_core::object::Object, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let widget = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let ffi_result = ::ffi::qt_gui_c_QPixmap_grabWidget_as_ptr_widget_x_y_w(widget, x, y, w);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl PixmapGrabWidgetArgs
    for (*mut ::qt_core::object::Object, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let widget = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let ffi_result = ::ffi::qt_gui_c_QPixmap_grabWidget_as_ptr_widget_x_y_w_h(widget, x, y, w, h);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::grab_window](../struct.Pixmap.html#method.grab_window) method.
  pub trait PixmapGrabWindowArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap>;
  }
  impl PixmapGrabWindowArgs for ::libc::c_ulonglong {
    fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_grabWindow_as_ptr_arg1(arg1) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl PixmapGrabWindowArgs for (::libc::c_ulonglong, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let arg1 = self.0;
      let x = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_grabWindow_as_ptr_arg1_x(arg1, x) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl PixmapGrabWindowArgs for (::libc::c_ulonglong, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let arg1 = self.0;
      let x = self.1;
      let y = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_grabWindow_as_ptr_arg1_x_y(arg1, x, y) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl PixmapGrabWindowArgs for (::libc::c_ulonglong, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let arg1 = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_grabWindow_as_ptr_arg1_x_y_w(arg1, x, y, w) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl PixmapGrabWindowArgs for (::libc::c_ulonglong, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let arg1 = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_grabWindow_as_ptr_arg1_x_y_w_h(arg1, x, y, w, h) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::new](../struct.Pixmap.html#method.new) method.
  pub trait PixmapNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap>;
  }
  impl<'a> PixmapNewArgs for &'a ::pixmap::Pixmap {
    fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_new_QPixmap(arg1 as *const ::pixmap::Pixmap) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> PixmapNewArgs for &'a ::qt_core::size::Size {
    fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_new_QSize(arg1 as *const ::qt_core::size::Size) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl PixmapNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let w = self.0;
      let h = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_new_int_int(w, h) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl PixmapNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::save_unsafe](../struct.Pixmap.html#method.save_unsafe) method.
  pub trait PixmapSaveUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> bool;
  }
  impl<'largs> PixmapSaveUnsafeArgs<'largs> for *mut ::qt_core::io_device::IODevice {
    unsafe fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> bool {
      let device = self;
      ::ffi::qt_gui_c_QPixmap_save_device(original_self as *const ::pixmap::Pixmap, device)
    }
  }
  impl<'largs> PixmapSaveUnsafeArgs<'largs> for (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> bool {
      let device = self.0;
      let format = self.1;
      ::ffi::qt_gui_c_QPixmap_save_device_format(original_self as *const ::pixmap::Pixmap, device, format)
    }
  }
  impl<'largs> PixmapSaveUnsafeArgs<'largs>
    for (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> bool {
      let device = self.0;
      let format = self.1;
      let quality = self.2;
      ::ffi::qt_gui_c_QPixmap_save_device_format_quality(original_self as *const ::pixmap::Pixmap,
                                                         device,
                                                         format,
                                                         quality)
    }
  }
  impl<'largs> PixmapSaveUnsafeArgs<'largs> for (&'largs ::qt_core::string::String, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> bool {
      let file_name = self.0;
      let format = self.1;
      ::ffi::qt_gui_c_QPixmap_save_fileName_format(original_self as *const ::pixmap::Pixmap,
                                                   file_name as *const ::qt_core::string::String,
                                                   format)
    }
  }
  impl<'largs> PixmapSaveUnsafeArgs<'largs>
    for (&'largs ::qt_core::string::String, *const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> bool {
      let file_name = self.0;
      let format = self.1;
      let quality = self.2;
      ::ffi::qt_gui_c_QPixmap_save_fileName_format_quality(original_self as *const ::pixmap::Pixmap,
                                                           file_name as *const ::qt_core::string::String,
                                                           format,
                                                           quality)
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::scaled](../struct.Pixmap.html#method.scaled) method.
  pub trait PixmapScaledArgs<'largs> {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap>;
  }
  impl<'largs> PixmapScaledArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let s = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QPixmap_scaled_as_ptr_s(original_self as *const ::pixmap::Pixmap,
                                                s as *const ::qt_core::size::Size)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> PixmapScaledArgs<'largs> for (&'largs ::qt_core::size::Size, &'largs ::qt_core::qt::AspectRatioMode) {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let s = self.0;
      let aspect_mode = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QPixmap_scaled_as_ptr_s_aspectMode(original_self as *const ::pixmap::Pixmap,
                                                             s as *const ::qt_core::size::Size,
                                                             aspect_mode as *const ::qt_core::qt::AspectRatioMode)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> PixmapScaledArgs<'largs>
    for (&'largs ::qt_core::size::Size,
                                                 &'largs ::qt_core::qt::AspectRatioMode,
                                                 &'largs ::qt_core::qt::TransformationMode) {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let s = self.0;
      let aspect_mode = self.1;
      let mode = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QPixmap_scaled_as_ptr_s_aspectMode_mode(original_self as *const ::pixmap::Pixmap, s as *const ::qt_core::size::Size, aspect_mode as *const ::qt_core::qt::AspectRatioMode, mode as *const ::qt_core::qt::TransformationMode)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> PixmapScaledArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let w = self.0;
      let h = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QPixmap_scaled_as_ptr_w_h(original_self as *const ::pixmap::Pixmap, w, h) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> PixmapScaledArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::qt::AspectRatioMode) {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let w = self.0;
      let h = self.1;
      let aspect_mode = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QPixmap_scaled_as_ptr_w_h_aspectMode(original_self as *const ::pixmap::Pixmap,
                                                               w,
                                                               h,
                                                               aspect_mode as *const ::qt_core::qt::AspectRatioMode)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> PixmapScaledArgs<'largs>
    for (::libc::c_int,
                                                 ::libc::c_int,
                                                 &'largs ::qt_core::qt::AspectRatioMode,
                                                 &'largs ::qt_core::qt::TransformationMode) {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let w = self.0;
      let h = self.1;
      let aspect_mode = self.2;
      let mode = self.3;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_scaled_as_ptr_w_h_aspectMode_mode(original_self as *const ::pixmap::Pixmap, w, h, aspect_mode as *const ::qt_core::qt::AspectRatioMode, mode as *const ::qt_core::qt::TransformationMode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::scaled_to_height](../struct.Pixmap.html#method.scaled_to_height) method.
  pub trait PixmapScaledToHeightArgs<'largs> {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap>;
  }
  impl<'largs> PixmapScaledToHeightArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let h = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QPixmap_scaledToHeight_as_ptr_h(original_self as *const ::pixmap::Pixmap, h) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> PixmapScaledToHeightArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::TransformationMode) {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let h = self.0;
      let mode = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QPixmap_scaledToHeight_as_ptr_h_mode(original_self as *const ::pixmap::Pixmap,
                                                               h,
                                                               mode as *const ::qt_core::qt::TransformationMode)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::scaled_to_width](../struct.Pixmap.html#method.scaled_to_width) method.
  pub trait PixmapScaledToWidthArgs<'largs> {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap>;
  }
  impl<'largs> PixmapScaledToWidthArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let w = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QPixmap_scaledToWidth_as_ptr_w(original_self as *const ::pixmap::Pixmap, w) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> PixmapScaledToWidthArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::TransformationMode) {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let w = self.0;
      let mode = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QPixmap_scaledToWidth_as_ptr_w_mode(original_self as *const ::pixmap::Pixmap,
                                                              w,
                                                              mode as *const ::qt_core::qt::TransformationMode)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::scroll](../struct.Pixmap.html#method.scroll) method.
  pub trait PixmapScrollArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::pixmap::Pixmap) -> ();
  }
  impl<'largs> PixmapScrollArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::rect::Rect) {
    fn exec(self, original_self: &'largs mut ::pixmap::Pixmap) -> () {
      let dx = self.0;
      let dy = self.1;
      let rect = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPixmap_scroll_dx_dy_rect(original_self as *mut ::pixmap::Pixmap,
                                                  dx,
                                                  dy,
                                                  rect as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> PixmapScrollArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::pixmap::Pixmap) -> () {
      let dx = self.0;
      let dy = self.1;
      let x = self.2;
      let y = self.3;
      let width = self.4;
      let height = self.5;
      unsafe {
        ::ffi::qt_gui_c_QPixmap_scroll_dx_dy_x_y_width_height(original_self as *mut ::pixmap::Pixmap,
                                                              dx,
                                                              dy,
                                                              x,
                                                              y,
                                                              width,
                                                              height)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::scroll_unsafe](../struct.Pixmap.html#method.scroll_unsafe) method.
  pub trait PixmapScrollUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::pixmap::Pixmap) -> ();
  }
  impl<'largs> PixmapScrollUnsafeArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::rect::Rect, *mut ::region::Region) {
    unsafe fn exec(self, original_self: &'largs mut ::pixmap::Pixmap) -> () {
      let dx = self.0;
      let dy = self.1;
      let rect = self.2;
      let exposed = self.3;
      ::ffi::qt_gui_c_QPixmap_scroll_dx_dy_rect_exposed(original_self as *mut ::pixmap::Pixmap,
                                                        dx,
                                                        dy,
                                                        rect as *const ::qt_core::rect::Rect,
                                                        exposed)
    }
  }
  impl<'largs> PixmapScrollUnsafeArgs<'largs>
    for (::libc::c_int,
                                                       ::libc::c_int,
                                                       ::libc::c_int,
                                                       ::libc::c_int,
                                                       ::libc::c_int,
                                                       ::libc::c_int,
                                                       *mut ::region::Region) {
    unsafe fn exec(self, original_self: &'largs mut ::pixmap::Pixmap) -> () {
      let dx = self.0;
      let dy = self.1;
      let x = self.2;
      let y = self.3;
      let width = self.4;
      let height = self.5;
      let exposed = self.6;
      ::ffi::qt_gui_c_QPixmap_scroll_dx_dy_x_y_width_height_exposed(original_self as *mut ::pixmap::Pixmap,
                                                                    dx,
                                                                    dy,
                                                                    x,
                                                                    y,
                                                                    width,
                                                                    height,
                                                                    exposed)
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::transformed](../struct.Pixmap.html#method.transformed) method.
  pub trait PixmapTransformedArgs<'largs> {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap>;
  }
  impl<'largs> PixmapTransformedArgs<'largs> for &'largs ::matrix::Matrix {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let arg1 = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QPixmap_transformed_as_ptr_QMatrix(original_self as *const ::pixmap::Pixmap,
                                                           arg1 as *const ::matrix::Matrix)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> PixmapTransformedArgs<'largs> for (&'largs ::matrix::Matrix, &'largs ::qt_core::qt::TransformationMode) {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let arg1 = self.0;
      let mode = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_transformed_as_ptr_QMatrix_Qt_TransformationMode(original_self as *const ::pixmap::Pixmap, arg1 as *const ::matrix::Matrix, mode as *const ::qt_core::qt::TransformationMode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> PixmapTransformedArgs<'largs> for &'largs ::transform::Transform {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let arg1 = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QPixmap_transformed_as_ptr_QTransform(original_self as *const ::pixmap::Pixmap,
                                                              arg1 as *const ::transform::Transform)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> PixmapTransformedArgs<'largs>
    for (&'largs ::transform::Transform, &'largs ::qt_core::qt::TransformationMode) {
    fn exec(self, original_self: &'largs ::pixmap::Pixmap) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let arg1 = self.0;
      let mode = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPixmap_transformed_as_ptr_QTransform_Qt_TransformationMode(original_self as *const ::pixmap::Pixmap, arg1 as *const ::transform::Transform, mode as *const ::qt_core::qt::TransformationMode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Pixmap::true_matrix](../struct.Pixmap.html#method.true_matrix) method.
  pub trait PixmapTrueMatrixArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> PixmapTrueMatrixArgs for (&'a ::matrix::Matrix, ::libc::c_int, ::libc::c_int) {
    type ReturnType = ::matrix::Matrix;
    fn exec(self) -> ::matrix::Matrix {
      let m = self.0;
      let w = self.1;
      let h = self.2;
      {
        let mut object: ::matrix::Matrix =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixmap_trueMatrix_to_output_QMatrix_int_int(m as *const ::matrix::Matrix, w, h, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixmapTrueMatrixArgs for (&'a ::transform::Transform, ::libc::c_int, ::libc::c_int) {
    type ReturnType = ::transform::Transform;
    fn exec(self) -> ::transform::Transform {
      let m = self.0;
      let w = self.1;
      let h = self.2;
      {
        let mut object: ::transform::Transform =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixmap_trueMatrix_to_output_QTransform_int_int(m as *const ::transform::Transform,
                                                                          w,
                                                                          h,
                                                                          &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::pixmap::Pixmap) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QPixmap_G_operator_shl(arg1 as *mut ::qt_core::data_stream::DataStream,
                                               arg2 as *const ::pixmap::Pixmap)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::pixmap::Pixmap) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixmap_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug,
                                                           arg2 as *const ::pixmap::Pixmap,
                                                           &mut object);
        }
        object
      }
    }
  }
}
