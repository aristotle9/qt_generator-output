/// C++ type: <span style='color: green;'>```QIcon```</span>
#[repr(C)]
pub struct Icon([u8; ::type_sizes::QT_GUI_ICON_ICON]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Icon {
  unsafe fn new_uninitialized() -> Icon {
    Icon(::std::mem::uninitialized())
  }
}

impl Icon {
  /// C++ method: <span style='color: green;'>```QIcon::actualSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn actual_size(&self, &::qt_core::size::Size) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```QSize QIcon::actualSize(const QSize& size) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn actual_size(&self, (&::qt_core::size::Size, ::icon::Mode)) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```QSize QIcon::actualSize(const QSize& size, QIcon::Mode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn actual_size(&self, (&::qt_core::size::Size, ::icon::Mode, ::icon::State)) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```QSize QIcon::actualSize(const QSize& size, QIcon::Mode mode = ?, QIcon::State state = ?) const```</span>
  ///
  ///
  pub fn actual_size<'largs, Args>(&'largs self, args: Args) -> ::qt_core::size::Size
    where Args: overloading::IconActualSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QIcon::actualSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn actual_size_unsafe(&self, (*mut ::window::Window, &::qt_core::size::Size)) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```QSize QIcon::actualSize(QWindow* window, const QSize& size) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn actual_size_unsafe(&self, (*mut ::window::Window, &::qt_core::size::Size, ::icon::Mode)) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```QSize QIcon::actualSize(QWindow* window, const QSize& size, QIcon::Mode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn actual_size_unsafe(&self, (*mut ::window::Window, &::qt_core::size::Size, ::icon::Mode, ::icon::State)) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```QSize QIcon::actualSize(QWindow* window, const QSize& size, QIcon::Mode mode = ?, QIcon::State state = ?) const```</span>
  ///
  ///
  pub unsafe fn actual_size_unsafe<'largs, Args>(&'largs self, args: Args) -> ::qt_core::size::Size
    where Args: overloading::IconActualSizeUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QIcon::addFile```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_file(&mut self, &::qt_core::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QIcon::addFile(const QString& fileName)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_file(&mut self, (&::qt_core::string::String, &::qt_core::size::Size)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QIcon::addFile(const QString& fileName, const QSize& size = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_file(&mut self, (&::qt_core::string::String, &::qt_core::size::Size, ::icon::Mode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QIcon::addFile(const QString& fileName, const QSize& size = ?, QIcon::Mode mode = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn add_file(&mut self, (&::qt_core::string::String, &::qt_core::size::Size, ::icon::Mode, ::icon::State)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QIcon::addFile(const QString& fileName, const QSize& size = ?, QIcon::Mode mode = ?, QIcon::State state = ?)```</span>
  ///
  ///
  pub fn add_file<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::IconAddFileArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QIcon::addPixmap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_pixmap(&mut self, &::pixmap::Pixmap) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QIcon::addPixmap(const QPixmap& pixmap)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_pixmap(&mut self, (&::pixmap::Pixmap, ::icon::Mode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QIcon::addPixmap(const QPixmap& pixmap, QIcon::Mode mode = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_pixmap(&mut self, (&::pixmap::Pixmap, ::icon::Mode, ::icon::State)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QIcon::addPixmap(const QPixmap& pixmap, QIcon::Mode mode = ?, QIcon::State state = ?)```</span>
  ///
  ///
  pub fn add_pixmap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::IconAddPixmapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVariant QIcon::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QIcon_convert_to_QVariant_to_output(self as *const ::icon::Icon, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QIcon::availableSizes```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn available_sizes(&self, ()) -> ::list::ListQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QList<QSize> QIcon::availableSizes() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn available_sizes(&self, ::icon::Mode) -> ::list::ListQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QList<QSize> QIcon::availableSizes(QIcon::Mode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn available_sizes(&self, (::icon::Mode, ::icon::State)) -> ::list::ListQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QList<QSize> QIcon::availableSizes(QIcon::Mode mode = ?, QIcon::State state = ?) const```</span>
  ///
  ///
  pub fn available_sizes<'largs, Args>(&'largs self, args: Args) -> ::list::ListQtCoreSize
    where Args: overloading::IconAvailableSizesArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```qint64 QIcon::cacheKey() const```</span>
  ///
  ///
  pub fn cache_key(&self) -> i64 {
    unsafe { ::ffi::qt_gui_c_QIcon_cacheKey(self as *const ::icon::Icon) }
  }

  /// C++ method: <span style='color: green;'>```void QIcon::detach()```</span>
  ///
  ///
  pub fn detach(&mut self) {
    unsafe { ::ffi::qt_gui_c_QIcon_detach(self as *mut ::icon::Icon) }
  }

  /// C++ method: <span style='color: green;'>```QIcon::fromTheme```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_theme(&::qt_core::string::String) -> ::icon::Icon```<br>
  /// C++ method: <span style='color: green;'>```static QIcon QIcon::fromTheme(const QString& name)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_theme((&::qt_core::string::String, &::icon::Icon)) -> ::icon::Icon```<br>
  /// C++ method: <span style='color: green;'>```static QIcon QIcon::fromTheme(const QString& name, const QIcon& fallback)```</span>
  ///
  ///
  pub fn from_theme<Args>(args: Args) -> ::icon::Icon
    where Args: overloading::IconFromThemeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static bool QIcon::hasThemeIcon(const QString& name)```</span>
  ///
  ///
  pub fn has_theme_icon(name: &::qt_core::string::String) -> bool {
    unsafe { ::ffi::qt_gui_c_QIcon_hasThemeIcon(name as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```bool QIcon::isDetached() const```</span>
  ///
  ///
  pub fn is_detached(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QIcon_isDetached(self as *const ::icon::Icon) }
  }

  /// C++ method: <span style='color: green;'>```bool QIcon::isMask() const```</span>
  ///
  ///
  pub fn is_mask(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QIcon_isMask(self as *const ::icon::Icon) }
  }

  /// C++ method: <span style='color: green;'>```bool QIcon::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QIcon_isNull(self as *const ::icon::Icon) }
  }

  /// C++ method: <span style='color: green;'>```QString QIcon::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QIcon_name_to_output(self as *const ::icon::Icon, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QIcon::QIcon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::icon::Icon```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QIcon::QIcon()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::icon::Icon) -> ::icon::Icon```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QIcon::QIcon(const QIcon& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::pixmap::Pixmap) -> ::icon::Icon```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QIcon::QIcon(const QPixmap& pixmap)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::icon::Icon```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QIcon::QIcon(const QString& fileName)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::icon::Icon
    where Args: overloading::IconNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QIcon::QIcon(QIconEngine* engine)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(engine: *mut ::icon_engine::IconEngine) -> ::icon::Icon {
    {
      let mut object: ::icon::Icon = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QIcon_constructor_engine(engine, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QIcon& QIcon::operator=(const QIcon& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::icon::Icon) -> &'l0 mut ::icon::Icon {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QIcon_operator_assign(self as *mut ::icon::Icon, other as *const ::icon::Icon) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QIcon::pixmap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn pixmap(&self, &::qt_core::size::Size) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QIcon::pixmap(const QSize& size) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn pixmap(&self, (&::qt_core::size::Size, ::icon::Mode)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QIcon::pixmap(const QSize& size, QIcon::Mode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn pixmap(&self, (&::qt_core::size::Size, ::icon::Mode, ::icon::State)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QIcon::pixmap(const QSize& size, QIcon::Mode mode = ?, QIcon::State state = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn pixmap(&self, ::libc::c_int) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QIcon::pixmap(int extent) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn pixmap(&self, (::libc::c_int, ::icon::Mode)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QIcon::pixmap(int extent, QIcon::Mode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn pixmap(&self, (::libc::c_int, ::icon::Mode, ::icon::State)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QIcon::pixmap(int extent, QIcon::Mode mode = ?, QIcon::State state = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn pixmap(&self, (::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QIcon::pixmap(int w, int h) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn pixmap(&self, (::libc::c_int, ::libc::c_int, ::icon::Mode)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QIcon::pixmap(int w, int h, QIcon::Mode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn pixmap(&self, (::libc::c_int, ::libc::c_int, ::icon::Mode, ::icon::State)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QIcon::pixmap(int w, int h, QIcon::Mode mode = ?, QIcon::State state = ?) const```</span>
  ///
  ///
  pub fn pixmap<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::pixmap::Pixmap>
    where Args: overloading::IconPixmapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QIcon::pixmap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn pixmap_unsafe(&self, (*mut ::window::Window, &::qt_core::size::Size)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QIcon::pixmap(QWindow* window, const QSize& size) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn pixmap_unsafe(&self, (*mut ::window::Window, &::qt_core::size::Size, ::icon::Mode)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QIcon::pixmap(QWindow* window, const QSize& size, QIcon::Mode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn pixmap_unsafe(&self, (*mut ::window::Window, &::qt_core::size::Size, ::icon::Mode, ::icon::State)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QIcon::pixmap(QWindow* window, const QSize& size, QIcon::Mode mode = ?, QIcon::State state = ?) const```</span>
  ///
  ///
  pub unsafe fn pixmap_unsafe<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::pixmap::Pixmap>
    where Args: overloading::IconPixmapUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QIcon::setIsMask(bool isMask)```</span>
  ///
  ///
  pub fn set_is_mask(&mut self, is_mask: bool) {
    unsafe { ::ffi::qt_gui_c_QIcon_setIsMask(self as *mut ::icon::Icon, is_mask) }
  }

  /// C++ method: <span style='color: green;'>```static void QIcon::setThemeName(const QString& path)```</span>
  ///
  ///
  pub fn set_theme_name(path: &::qt_core::string::String) {
    unsafe { ::ffi::qt_gui_c_QIcon_setThemeName(path as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static void QIcon::setThemeSearchPaths(const QStringList& searchpath)```</span>
  ///
  ///
  pub fn set_theme_search_paths(searchpath: &::qt_core::string_list::StringList) {
    unsafe { ::ffi::qt_gui_c_QIcon_setThemeSearchPaths(searchpath as *const ::qt_core::string_list::StringList) }
  }

  /// C++ method: <span style='color: green;'>```void QIcon::swap(QIcon& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::icon::Icon) {
    unsafe { ::ffi::qt_gui_c_QIcon_swap(self as *mut ::icon::Icon, other as *mut ::icon::Icon) }
  }

  /// C++ method: <span style='color: green;'>```static QString QIcon::themeName()```</span>
  ///
  ///
  pub fn theme_name() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QIcon_themeName_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QStringList QIcon::themeSearchPaths()```</span>
  ///
  ///
  pub fn theme_search_paths() -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QIcon_themeSearchPaths_to_output(&mut object);
      }
      object
    }
  }
}

impl Drop for ::icon::Icon {
  /// C++ method: <span style='color: green;'>```[destructor] void QIcon::~QIcon()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QIcon_destructor(self as *mut ::icon::Icon) }
  }
}

/// C++ type: <span style='color: green;'>```QIcon::Mode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Mode {
  /// C++ enum variant: <span style='color: green;'>```Normal = 0```</span>
  Normal = 0,
  /// C++ enum variant: <span style='color: green;'>```Disabled = 1```</span>
  Disabled = 1,
  /// C++ enum variant: <span style='color: green;'>```Active = 2```</span>
  Active = 2,
  /// C++ enum variant: <span style='color: green;'>```Selected = 3```</span>
  Selected = 3,
}

/// C++ type: <span style='color: green;'>```QIcon::State```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum State {
  /// C++ enum variant: <span style='color: green;'>```On = 0```</span>
  On = 0,
  /// C++ enum variant: <span style='color: green;'>```Off = 1```</span>
  Off = 1,
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QIcon& arg2)```</span>
///
///
pub fn op_shl(dbg: &::qt_core::debug::Debug, arg2: &::icon::Icon) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QIcon_G_operator_shl_to_output(dbg as *const ::qt_core::debug::Debug,
                                                     arg2 as *const ::icon::Icon,
                                                     &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```QString qt_findAtNxFile(const QString& baseFileName, double targetDevicePixelRatio)```</span>
///
///
pub fn qt_find_at_nx_file(base_file_name: &::qt_core::string::String,
                          target_device_pixel_ratio: ::libc::c_double)
                          -> ::qt_core::string::String {
  {
    let mut object: ::qt_core::string::String =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QIcon_G_qt_findAtNxFile_to_output_baseFileName_targetDevicePixelRatio(base_file_name as *const ::qt_core::string::String, target_device_pixel_ratio, &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```QString qt_findAtNxFile(const QString& baseFileName, double targetDevicePixelRatio, double* sourceDevicePixelRatio = ?)```</span>
///
///
pub unsafe fn qt_find_at_nx_file_unsafe(base_file_name: &::qt_core::string::String,
                                        target_device_pixel_ratio: ::libc::c_double,
                                        source_device_pixel_ratio: *mut ::libc::c_double)
                                        -> ::qt_core::string::String {
  {
    let mut object: ::qt_core::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
    ::ffi::qt_gui_c_QIcon_G_qt_findAtNxFile_to_output_baseFileName_targetDevicePixelRatio_sourceDevicePixelRatio(base_file_name as *const ::qt_core::string::String, target_device_pixel_ratio, source_device_pixel_ratio, &mut object);
    object
  }
}

/// C++ method: <span style='color: green;'>```void swap(QIcon& value1, QIcon& value2)```</span>
///
///
pub fn swap(value1: &mut ::icon::Icon, value2: &mut ::icon::Icon) {
  unsafe { ::ffi::qt_gui_c_QIcon_G_swap(value1 as *mut ::icon::Icon, value2 as *mut ::icon::Icon) }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Icon::actual_size](../struct.Icon.html#method.actual_size) method.
  pub trait IconActualSizeArgs<'largs> {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::qt_core::size::Size;
  }
  impl<'largs> IconActualSizeArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::qt_core::size::Size {
      let size = self;
      {
        let mut object: ::qt_core::size::Size =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIcon_actualSize_to_output_size(original_self as *const ::icon::Icon,
                                                          size as *const ::qt_core::size::Size,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> IconActualSizeArgs<'largs> for (&'largs ::qt_core::size::Size, ::icon::Mode) {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::qt_core::size::Size {
      let size = self.0;
      let mode = self.1;
      {
        let mut object: ::qt_core::size::Size =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIcon_actualSize_to_output_size_mode(original_self as *const ::icon::Icon,
                                                               size as *const ::qt_core::size::Size,
                                                               mode,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> IconActualSizeArgs<'largs> for (&'largs ::qt_core::size::Size, ::icon::Mode, ::icon::State) {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::qt_core::size::Size {
      let size = self.0;
      let mode = self.1;
      let state = self.2;
      {
        let mut object: ::qt_core::size::Size =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIcon_actualSize_to_output_size_mode_state(original_self as *const ::icon::Icon,
                                                                     size as *const ::qt_core::size::Size,
                                                                     mode,
                                                                     state,
                                                                     &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Icon::actual_size_unsafe](../struct.Icon.html#method.actual_size_unsafe) method.
  pub trait IconActualSizeUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::icon::Icon) -> ::qt_core::size::Size;
  }
  impl<'largs> IconActualSizeUnsafeArgs<'largs> for (*mut ::window::Window, &'largs ::qt_core::size::Size) {
    unsafe fn exec(self, original_self: &'largs ::icon::Icon) -> ::qt_core::size::Size {
      let window = self.0;
      let size = self.1;
      {
        let mut object: ::qt_core::size::Size = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_gui_c_QIcon_actualSize_to_output_window_size(original_self as *const ::icon::Icon,
                                                               window,
                                                               size as *const ::qt_core::size::Size,
                                                               &mut object);
        object
      }
    }
  }
  impl<'largs> IconActualSizeUnsafeArgs<'largs> for (*mut ::window::Window, &'largs ::qt_core::size::Size, ::icon::Mode) {
    unsafe fn exec(self, original_self: &'largs ::icon::Icon) -> ::qt_core::size::Size {
      let window = self.0;
      let size = self.1;
      let mode = self.2;
      {
        let mut object: ::qt_core::size::Size = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_gui_c_QIcon_actualSize_to_output_window_size_mode(original_self as *const ::icon::Icon,
                                                                    window,
                                                                    size as *const ::qt_core::size::Size,
                                                                    mode,
                                                                    &mut object);
        object
      }
    }
  }
  impl<'largs> IconActualSizeUnsafeArgs<'largs>
    for (*mut ::window::Window, &'largs ::qt_core::size::Size, ::icon::Mode, ::icon::State) {
    unsafe fn exec(self, original_self: &'largs ::icon::Icon) -> ::qt_core::size::Size {
      let window = self.0;
      let size = self.1;
      let mode = self.2;
      let state = self.3;
      {
        let mut object: ::qt_core::size::Size = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_gui_c_QIcon_actualSize_to_output_window_size_mode_state(original_self as *const ::icon::Icon,
                                                                          window,
                                                                          size as *const ::qt_core::size::Size,
                                                                          mode,
                                                                          state,
                                                                          &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Icon::add_file](../struct.Icon.html#method.add_file) method.
  pub trait IconAddFileArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::icon::Icon) -> ();
  }
  impl<'largs> IconAddFileArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::icon::Icon) -> () {
      let file_name = self;
      unsafe {
        ::ffi::qt_gui_c_QIcon_addFile_fileName(original_self as *mut ::icon::Icon,
                                               file_name as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> IconAddFileArgs<'largs> for (&'largs ::qt_core::string::String, &'largs ::qt_core::size::Size) {
    fn exec(self, original_self: &'largs mut ::icon::Icon) -> () {
      let file_name = self.0;
      let size = self.1;
      unsafe {
        ::ffi::qt_gui_c_QIcon_addFile_fileName_size(original_self as *mut ::icon::Icon,
                                                    file_name as *const ::qt_core::string::String,
                                                    size as *const ::qt_core::size::Size)
      }
    }
  }
  impl<'largs> IconAddFileArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::size::Size, ::icon::Mode) {
    fn exec(self, original_self: &'largs mut ::icon::Icon) -> () {
      let file_name = self.0;
      let size = self.1;
      let mode = self.2;
      unsafe {
        ::ffi::qt_gui_c_QIcon_addFile_fileName_size_mode(original_self as *mut ::icon::Icon,
                                                         file_name as *const ::qt_core::string::String,
                                                         size as *const ::qt_core::size::Size,
                                                         mode)
      }
    }
  }
  impl<'largs> IconAddFileArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::size::Size, ::icon::Mode, ::icon::State) {
    fn exec(self, original_self: &'largs mut ::icon::Icon) -> () {
      let file_name = self.0;
      let size = self.1;
      let mode = self.2;
      let state = self.3;
      unsafe {
        ::ffi::qt_gui_c_QIcon_addFile_fileName_size_mode_state(original_self as *mut ::icon::Icon,
                                                               file_name as *const ::qt_core::string::String,
                                                               size as *const ::qt_core::size::Size,
                                                               mode,
                                                               state)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Icon::add_pixmap](../struct.Icon.html#method.add_pixmap) method.
  pub trait IconAddPixmapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::icon::Icon) -> ();
  }
  impl<'largs> IconAddPixmapArgs<'largs> for &'largs ::pixmap::Pixmap {
    fn exec(self, original_self: &'largs mut ::icon::Icon) -> () {
      let pixmap = self;
      unsafe {
        ::ffi::qt_gui_c_QIcon_addPixmap_pixmap(original_self as *mut ::icon::Icon,
                                               pixmap as *const ::pixmap::Pixmap)
      }
    }
  }
  impl<'largs> IconAddPixmapArgs<'largs> for (&'largs ::pixmap::Pixmap, ::icon::Mode) {
    fn exec(self, original_self: &'largs mut ::icon::Icon) -> () {
      let pixmap = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_gui_c_QIcon_addPixmap_pixmap_mode(original_self as *mut ::icon::Icon,
                                                    pixmap as *const ::pixmap::Pixmap,
                                                    mode)
      }
    }
  }
  impl<'largs> IconAddPixmapArgs<'largs> for (&'largs ::pixmap::Pixmap, ::icon::Mode, ::icon::State) {
    fn exec(self, original_self: &'largs mut ::icon::Icon) -> () {
      let pixmap = self.0;
      let mode = self.1;
      let state = self.2;
      unsafe {
        ::ffi::qt_gui_c_QIcon_addPixmap_pixmap_mode_state(original_self as *mut ::icon::Icon,
                                                          pixmap as *const ::pixmap::Pixmap,
                                                          mode,
                                                          state)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Icon::available_sizes](../struct.Icon.html#method.available_sizes) method.
  pub trait IconAvailableSizesArgs<'largs> {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::list::ListQtCoreSize;
  }
  impl<'largs> IconAvailableSizesArgs<'largs> for ::icon::Mode {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::list::ListQtCoreSize {
      let mode = self;
      {
        let mut object: ::list::ListQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIcon_availableSizes_to_output_mode(original_self as *const ::icon::Icon, mode, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> IconAvailableSizesArgs<'largs> for (::icon::Mode, ::icon::State) {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::list::ListQtCoreSize {
      let mode = self.0;
      let state = self.1;
      {
        let mut object: ::list::ListQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIcon_availableSizes_to_output_mode_state(original_self as *const ::icon::Icon,
                                                                    mode,
                                                                    state,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'largs> IconAvailableSizesArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::list::ListQtCoreSize {

      {
        let mut object: ::list::ListQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIcon_availableSizes_to_output_no_args(original_self as *const ::icon::Icon, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Icon::from_theme](../struct.Icon.html#method.from_theme) method.
  pub trait IconFromThemeArgs {
    fn exec(self) -> ::icon::Icon;
  }
  impl<'a> IconFromThemeArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::icon::Icon {
      let name = self;
      {
        let mut object: ::icon::Icon = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIcon_fromTheme_to_output_name(name as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> IconFromThemeArgs for (&'a ::qt_core::string::String, &'a ::icon::Icon) {
    fn exec(self) -> ::icon::Icon {
      let name = self.0;
      let fallback = self.1;
      {
        let mut object: ::icon::Icon = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIcon_fromTheme_to_output_name_fallback(name as *const ::qt_core::string::String,
                                                                  fallback as *const ::icon::Icon,
                                                                  &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Icon::new](../struct.Icon.html#method.new) method.
  pub trait IconNewArgs {
    fn exec(self) -> ::icon::Icon;
  }
  impl<'a> IconNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::icon::Icon {
      let file_name = self;
      {
        let mut object: ::icon::Icon = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIcon_constructor_fileName(file_name as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl IconNewArgs for () {
    fn exec(self) -> ::icon::Icon {

      {
        let mut object: ::icon::Icon = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIcon_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> IconNewArgs for &'a ::icon::Icon {
    fn exec(self) -> ::icon::Icon {
      let other = self;
      {
        let mut object: ::icon::Icon = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIcon_constructor_other(other as *const ::icon::Icon, &mut object);
        }
        object
      }
    }
  }
  impl<'a> IconNewArgs for &'a ::pixmap::Pixmap {
    fn exec(self) -> ::icon::Icon {
      let pixmap = self;
      {
        let mut object: ::icon::Icon = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIcon_constructor_pixmap(pixmap as *const ::pixmap::Pixmap, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Icon::pixmap](../struct.Icon.html#method.pixmap) method.
  pub trait IconPixmapArgs<'largs> {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::cpp_utils::CppBox<::pixmap::Pixmap>;
  }
  impl<'largs> IconPixmapArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let extent = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QIcon_pixmap_as_ptr_extent(original_self as *const ::icon::Icon, extent) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> IconPixmapArgs<'largs> for (::libc::c_int, ::icon::Mode) {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let extent = self.0;
      let mode = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QIcon_pixmap_as_ptr_extent_mode(original_self as *const ::icon::Icon, extent, mode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> IconPixmapArgs<'largs> for (::libc::c_int, ::icon::Mode, ::icon::State) {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let extent = self.0;
      let mode = self.1;
      let state = self.2;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QIcon_pixmap_as_ptr_extent_mode_state(original_self as *const ::icon::Icon, extent, mode, state)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> IconPixmapArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let size = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QIcon_pixmap_as_ptr_size(original_self as *const ::icon::Icon,
                                                 size as *const ::qt_core::size::Size)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> IconPixmapArgs<'largs> for (&'largs ::qt_core::size::Size, ::icon::Mode) {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let size = self.0;
      let mode = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QIcon_pixmap_as_ptr_size_mode(original_self as *const ::icon::Icon,
                                                      size as *const ::qt_core::size::Size,
                                                      mode)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> IconPixmapArgs<'largs> for (&'largs ::qt_core::size::Size, ::icon::Mode, ::icon::State) {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let size = self.0;
      let mode = self.1;
      let state = self.2;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QIcon_pixmap_as_ptr_size_mode_state(original_self as *const ::icon::Icon,
                                                            size as *const ::qt_core::size::Size,
                                                            mode,
                                                            state)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> IconPixmapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let w = self.0;
      let h = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QIcon_pixmap_as_ptr_w_h(original_self as *const ::icon::Icon, w, h) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> IconPixmapArgs<'largs> for (::libc::c_int, ::libc::c_int, ::icon::Mode) {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let w = self.0;
      let h = self.1;
      let mode = self.2;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QIcon_pixmap_as_ptr_w_h_mode(original_self as *const ::icon::Icon, w, h, mode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> IconPixmapArgs<'largs> for (::libc::c_int, ::libc::c_int, ::icon::Mode, ::icon::State) {
    fn exec(self, original_self: &'largs ::icon::Icon) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let w = self.0;
      let h = self.1;
      let mode = self.2;
      let state = self.3;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QIcon_pixmap_as_ptr_w_h_mode_state(original_self as *const ::icon::Icon, w, h, mode, state)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Icon::pixmap_unsafe](../struct.Icon.html#method.pixmap_unsafe) method.
  pub trait IconPixmapUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::icon::Icon) -> ::cpp_utils::CppBox<::pixmap::Pixmap>;
  }
  impl<'largs> IconPixmapUnsafeArgs<'largs> for (*mut ::window::Window, &'largs ::qt_core::size::Size) {
    unsafe fn exec(self, original_self: &'largs ::icon::Icon) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let window = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_gui_c_QIcon_pixmap_as_ptr_window_size(original_self as *const ::icon::Icon,
                                                                       window,
                                                                       size as *const ::qt_core::size::Size);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'largs> IconPixmapUnsafeArgs<'largs> for (*mut ::window::Window, &'largs ::qt_core::size::Size, ::icon::Mode) {
    unsafe fn exec(self, original_self: &'largs ::icon::Icon) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let window = self.0;
      let size = self.1;
      let mode = self.2;
      let ffi_result = ::ffi::qt_gui_c_QIcon_pixmap_as_ptr_window_size_mode(original_self as *const ::icon::Icon,
                                                                            window,
                                                                            size as *const ::qt_core::size::Size,
                                                                            mode);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'largs> IconPixmapUnsafeArgs<'largs>
    for (*mut ::window::Window, &'largs ::qt_core::size::Size, ::icon::Mode, ::icon::State) {
    unsafe fn exec(self, original_self: &'largs ::icon::Icon) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let window = self.0;
      let size = self.1;
      let mode = self.2;
      let state = self.3;
      let ffi_result =
        ::ffi::qt_gui_c_QIcon_pixmap_as_ptr_window_size_mode_state(original_self as *const ::icon::Icon,
                                                                   window,
                                                                   size as *const ::qt_core::size::Size,
                                                                   mode,
                                                                   state);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
