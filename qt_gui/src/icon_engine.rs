/// C++ type: <span style='color: green;'>```QIconEngine::AvailableSizesArgument```</span>
#[repr(C)]
pub struct AvailableSizesArgument(u8);

impl AvailableSizesArgument {
  /// C++ method: <span style='color: green;'>```const QIcon::Mode& QIconEngine::AvailableSizesArgument::mode() const```</span>
  ///
  ///
  pub fn mode<'l0>(&'l0 self) -> &'l0 ::icon::Mode {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_AvailableSizesArgument_mode(self as *const ::icon_engine::AvailableSizesArgument)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QIcon::Mode& QIconEngine::AvailableSizesArgument::mode_mut()```</span>
  ///
  ///
  pub fn mode_mut<'l0>(&'l0 mut self) -> &'l0 mut ::icon::Mode {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_AvailableSizesArgument_mode_mut(self as *mut ::icon_engine::AvailableSizesArgument)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QIconEngine::AvailableSizesArgument::set_mode(QIcon::Mode value)```</span>
  ///
  ///
  pub fn set_mode(&mut self, value: &::icon::Mode) {
    unsafe {
      ::ffi::qt_gui_c_QIconEngine_AvailableSizesArgument_set_mode(self as *mut ::icon_engine::AvailableSizesArgument,
                                                                  value as *const ::icon::Mode)
    }
  }

  /// C++ method: <span style='color: green;'>```void QIconEngine::AvailableSizesArgument::set_sizes(QList<QSize> value)```</span>
  ///
  ///
  pub fn set_sizes(&mut self, value: &::list::ListQtCoreSize) {
    unsafe {
      ::ffi::qt_gui_c_QIconEngine_AvailableSizesArgument_set_sizes(self as *mut ::icon_engine::AvailableSizesArgument,
                                                                   value as *const ::list::ListQtCoreSize)
    }
  }

  /// C++ method: <span style='color: green;'>```void QIconEngine::AvailableSizesArgument::set_state(QIcon::State value)```</span>
  ///
  ///
  pub fn set_state(&mut self, value: &::icon::State) {
    unsafe {
      ::ffi::qt_gui_c_QIconEngine_AvailableSizesArgument_set_state(self as *mut ::icon_engine::AvailableSizesArgument,
                                                                   value as *const ::icon::State)
    }
  }

  /// C++ method: <span style='color: green;'>```const QList<QSize>& QIconEngine::AvailableSizesArgument::sizes() const```</span>
  ///
  ///
  pub fn sizes<'l0>(&'l0 self) -> &'l0 ::list::ListQtCoreSize {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_AvailableSizesArgument_sizes(self as *const ::icon_engine::AvailableSizesArgument)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QSize>& QIconEngine::AvailableSizesArgument::sizes_mut()```</span>
  ///
  ///
  pub fn sizes_mut<'l0>(&'l0 mut self) -> &'l0 mut ::list::ListQtCoreSize {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_AvailableSizesArgument_sizes_mut(self as *mut ::icon_engine::AvailableSizesArgument)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QIcon::State& QIconEngine::AvailableSizesArgument::state() const```</span>
  ///
  ///
  pub fn state<'l0>(&'l0 self) -> &'l0 ::icon::State {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_AvailableSizesArgument_state(self as *const ::icon_engine::AvailableSizesArgument)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QIcon::State& QIconEngine::AvailableSizesArgument::state_mut()```</span>
  ///
  ///
  pub fn state_mut<'l0>(&'l0 mut self) -> &'l0 mut ::icon::State {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_AvailableSizesArgument_state_mut(self as *mut ::icon_engine::AvailableSizesArgument)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::icon_engine::AvailableSizesArgument {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QIconEngine_AvailableSizesArgument_delete
  }
}

/// C++ type: <span style='color: green;'>```QIconEngine```</span>
#[repr(C)]
pub struct IconEngine(u8);

impl IconEngine {
  /// C++ method: <span style='color: green;'>```virtual QSize QIconEngine::actualSize(const QSize& size, QIcon::Mode mode, QIcon::State state)```</span>
  ///
  ///
  pub fn actual_size(&mut self,
                     size: &::qt_core::size::Size,
                     mode: &::icon::Mode,
                     state: &::icon::State)
                     -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_actualSize_to_output(self as *mut ::icon_engine::IconEngine,
                                                         size as *const ::qt_core::size::Size,
                                                         mode as *const ::icon::Mode,
                                                         state as *const ::icon::State,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QIconEngine::addFile(const QString& fileName, const QSize& size, QIcon::Mode mode, QIcon::State state)```</span>
  ///
  ///
  pub fn add_file(&mut self,
                  file_name: &::qt_core::string::String,
                  size: &::qt_core::size::Size,
                  mode: &::icon::Mode,
                  state: &::icon::State) {
    unsafe {
      ::ffi::qt_gui_c_QIconEngine_addFile(self as *mut ::icon_engine::IconEngine,
                                          file_name as *const ::qt_core::string::String,
                                          size as *const ::qt_core::size::Size,
                                          mode as *const ::icon::Mode,
                                          state as *const ::icon::State)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QIconEngine::addPixmap(const QPixmap& pixmap, QIcon::Mode mode, QIcon::State state)```</span>
  ///
  ///
  pub fn add_pixmap(&mut self, pixmap: &::pixmap::Pixmap, mode: &::icon::Mode, state: &::icon::State) {
    unsafe {
      ::ffi::qt_gui_c_QIconEngine_addPixmap(self as *mut ::icon_engine::IconEngine,
                                            pixmap as *const ::pixmap::Pixmap,
                                            mode as *const ::icon::Mode,
                                            state as *const ::icon::State)
    }
  }

  /// C++ method: <span style='color: green;'>```QIconEngine::availableSizes```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn available_sizes(&self, ()) -> ::list::ListQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```virtual QList<QSize> QIconEngine::availableSizes() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn available_sizes(&self, &::icon::Mode) -> ::list::ListQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```virtual QList<QSize> QIconEngine::availableSizes(QIcon::Mode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn available_sizes(&self, (&::icon::Mode, &::icon::State)) -> ::list::ListQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```virtual QList<QSize> QIconEngine::availableSizes(QIcon::Mode mode = ?, QIcon::State state = ?) const```</span>
  ///
  ///
  pub fn available_sizes<'largs, Args>(&'largs self, args: Args) -> ::list::ListQtCoreSize
    where Args: overloading::IconEngineAvailableSizesArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```pure virtual QIconEngine* QIconEngine::clone() const```</span>
  ///
  ///
  pub fn clone(&self) -> *mut ::icon_engine::IconEngine {
    unsafe { ::ffi::qt_gui_c_QIconEngine_clone(self as *const ::icon_engine::IconEngine) }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QIconEngine::iconName() const```</span>
  ///
  ///
  pub fn icon_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_iconName_to_output(self as *const ::icon_engine::IconEngine, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QIconEngine::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QIconEngine_isNull(self as *const ::icon_engine::IconEngine) }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QIconEngine::key() const```</span>
  ///
  ///
  pub fn key(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_key_to_output(self as *const ::icon_engine::IconEngine, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QIconEngine::paint(QPainter* painter, const QRect& rect, QIcon::Mode mode, QIcon::State state)```</span>
  ///
  ///
  pub unsafe fn paint(&mut self,
                      painter: *mut ::painter::Painter,
                      rect: &::qt_core::rect::Rect,
                      mode: &::icon::Mode,
                      state: &::icon::State) {
    ::ffi::qt_gui_c_QIconEngine_paint(self as *mut ::icon_engine::IconEngine,
                                      painter,
                                      rect as *const ::qt_core::rect::Rect,
                                      mode as *const ::icon::Mode,
                                      state as *const ::icon::State)
  }

  /// C++ method: <span style='color: green;'>```virtual QPixmap QIconEngine::pixmap(const QSize& size, QIcon::Mode mode, QIcon::State state)```</span>
  ///
  ///
  pub fn pixmap(&mut self,
                size: &::qt_core::size::Size,
                mode: &::icon::Mode,
                state: &::icon::State)
                -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QIconEngine_pixmap_as_ptr(self as *mut ::icon_engine::IconEngine,
                                                size as *const ::qt_core::size::Size,
                                                mode as *const ::icon::Mode,
                                                state as *const ::icon::State)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QIconEngine::read(QDataStream& in)```</span>
  ///
  ///
  pub fn read(&mut self, in_: &mut ::qt_core::data_stream::DataStream) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QIconEngine_read(self as *mut ::icon_engine::IconEngine,
                                       in_ as *mut ::qt_core::data_stream::DataStream)
    }
  }

  /// C++ method: <span style='color: green;'>```QPixmap QIconEngine::scaledPixmap(const QSize& size, QIcon::Mode mode, QIcon::State state, double scale)```</span>
  ///
  ///
  pub fn scaled_pixmap(&mut self,
                       size: &::qt_core::size::Size,
                       mode: &::icon::Mode,
                       state: &::icon::State,
                       scale: ::libc::c_double)
                       -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QIconEngine_scaledPixmap_as_ptr(self as *mut ::icon_engine::IconEngine,
                                                      size as *const ::qt_core::size::Size,
                                                      mode as *const ::icon::Mode,
                                                      state as *const ::icon::State,
                                                      scale)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QIconEngine::virtual_hook(int id, void* data)```</span>
  ///
  ///
  pub unsafe fn virtual_hook(&mut self, id: ::libc::c_int, data: *mut ::libc::c_void) {
    ::ffi::qt_gui_c_QIconEngine_virtual_hook(self as *mut ::icon_engine::IconEngine, id, data)
  }

  /// C++ method: <span style='color: green;'>```virtual bool QIconEngine::write(QDataStream& out) const```</span>
  ///
  ///
  pub fn write(&self, out: &mut ::qt_core::data_stream::DataStream) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QIconEngine_write(self as *const ::icon_engine::IconEngine,
                                        out as *mut ::qt_core::data_stream::DataStream)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::icon_engine::IconEngine {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QIconEngine_delete
  }
}

/// C++ type: <span style='color: green;'>```QIconEngine::IconEngineHook```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum IconEngineHook {
  /// C++ enum variant: <span style='color: green;'>```AvailableSizesHook = 1```</span>
  AvailableSizes = 1,
  /// C++ enum variant: <span style='color: green;'>```IconNameHook = 2```</span>
  IconName = 2,
  /// C++ enum variant: <span style='color: green;'>```IsNullHook = 3```</span>
  IsNull = 3,
  /// C++ enum variant: <span style='color: green;'>```ScaledPixmapHook = 4```</span>
  ScaledPixmap = 4,
}

/// C++ type: <span style='color: green;'>```QIconEngine::ScaledPixmapArgument```</span>
#[repr(C)]
pub struct ScaledPixmapArgument(u8);

impl ScaledPixmapArgument {
  /// C++ method: <span style='color: green;'>```const QIcon::Mode& QIconEngine::ScaledPixmapArgument::mode() const```</span>
  ///
  ///
  pub fn mode<'l0>(&'l0 self) -> &'l0 ::icon::Mode {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_mode(self as *const ::icon_engine::ScaledPixmapArgument)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QIcon::Mode& QIconEngine::ScaledPixmapArgument::mode_mut()```</span>
  ///
  ///
  pub fn mode_mut<'l0>(&'l0 mut self) -> &'l0 mut ::icon::Mode {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_mode_mut(self as *mut ::icon_engine::ScaledPixmapArgument)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPixmap& QIconEngine::ScaledPixmapArgument::pixmap() const```</span>
  ///
  ///
  pub fn pixmap<'l0>(&'l0 self) -> &'l0 ::pixmap::Pixmap {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_pixmap(self as *const ::icon_engine::ScaledPixmapArgument)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPixmap& QIconEngine::ScaledPixmapArgument::pixmap_mut()```</span>
  ///
  ///
  pub fn pixmap_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pixmap::Pixmap {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_pixmap_mut(self as *mut ::icon_engine::ScaledPixmapArgument)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double QIconEngine::ScaledPixmapArgument::scale() const```</span>
  ///
  ///
  pub fn scale(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_scale(self as *const ::icon_engine::ScaledPixmapArgument)
    }
  }

  /// C++ method: <span style='color: green;'>```void QIconEngine::ScaledPixmapArgument::set_mode(QIcon::Mode value)```</span>
  ///
  ///
  pub fn set_mode(&mut self, value: &::icon::Mode) {
    unsafe {
      ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_set_mode(self as *mut ::icon_engine::ScaledPixmapArgument,
                                                                value as *const ::icon::Mode)
    }
  }

  /// C++ method: <span style='color: green;'>```void QIconEngine::ScaledPixmapArgument::set_pixmap(QPixmap value)```</span>
  ///
  ///
  pub fn set_pixmap(&mut self, value: &::pixmap::Pixmap) {
    unsafe {
      ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_set_pixmap(self as *mut ::icon_engine::ScaledPixmapArgument,
                                                                  value as *const ::pixmap::Pixmap)
    }
  }

  /// C++ method: <span style='color: green;'>```void QIconEngine::ScaledPixmapArgument::set_scale(double value)```</span>
  ///
  ///
  pub fn set_scale(&mut self, value: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_set_scale(self as *mut ::icon_engine::ScaledPixmapArgument,
                                                                 value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QIconEngine::ScaledPixmapArgument::set_size(QSize value)```</span>
  ///
  ///
  pub fn set_size(&mut self, value: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_set_size(self as *mut ::icon_engine::ScaledPixmapArgument,
                                                                value as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QIconEngine::ScaledPixmapArgument::set_state(QIcon::State value)```</span>
  ///
  ///
  pub fn set_state(&mut self, value: &::icon::State) {
    unsafe {
      ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_set_state(self as *mut ::icon_engine::ScaledPixmapArgument,
                                                                 value as *const ::icon::State)
    }
  }

  /// C++ method: <span style='color: green;'>```const QSize& QIconEngine::ScaledPixmapArgument::size() const```</span>
  ///
  ///
  pub fn size<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_size(self as *const ::icon_engine::ScaledPixmapArgument)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QIconEngine::ScaledPixmapArgument::size_mut()```</span>
  ///
  ///
  pub fn size_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_size_mut(self as *mut ::icon_engine::ScaledPixmapArgument)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QIcon::State& QIconEngine::ScaledPixmapArgument::state() const```</span>
  ///
  ///
  pub fn state<'l0>(&'l0 self) -> &'l0 ::icon::State {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_state(self as *const ::icon_engine::ScaledPixmapArgument)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QIcon::State& QIconEngine::ScaledPixmapArgument::state_mut()```</span>
  ///
  ///
  pub fn state_mut<'l0>(&'l0 mut self) -> &'l0 mut ::icon::State {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_state_mut(self as *mut ::icon_engine::ScaledPixmapArgument)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::icon_engine::ScaledPixmapArgument {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QIconEngine_ScaledPixmapArgument_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [IconEngine::available_sizes](../struct.IconEngine.html#method.available_sizes) method.
  pub trait IconEngineAvailableSizesArgs<'largs> {
    fn exec(self, original_self: &'largs ::icon_engine::IconEngine) -> ::list::ListQtCoreSize;
  }
  impl<'largs> IconEngineAvailableSizesArgs<'largs> for &'largs ::icon::Mode {
    fn exec(self, original_self: &'largs ::icon_engine::IconEngine) -> ::list::ListQtCoreSize {
      let mode = self;
      {
        let mut object: ::list::ListQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIconEngine_availableSizes_to_output_mode(original_self as *const ::icon_engine::IconEngine, mode as *const ::icon::Mode, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> IconEngineAvailableSizesArgs<'largs> for (&'largs ::icon::Mode, &'largs ::icon::State) {
    fn exec(self, original_self: &'largs ::icon_engine::IconEngine) -> ::list::ListQtCoreSize {
      let mode = self.0;
      let state = self.1;
      {
        let mut object: ::list::ListQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIconEngine_availableSizes_to_output_mode_state(original_self as *const ::icon_engine::IconEngine, mode as *const ::icon::Mode, state as *const ::icon::State, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> IconEngineAvailableSizesArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::icon_engine::IconEngine) -> ::list::ListQtCoreSize {

      {
        let mut object: ::list::ListQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QIconEngine_availableSizes_to_output_no_args(original_self as *const ::icon_engine::IconEngine, &mut object);
        }
        object
      }
    }
  }
}
