/// C++ type: <span style='color: green;'>```QMap<QDate, QTextCharFormat>```</span>
#[repr(C)]
pub struct MapQtCoreDateQtGuiTextCharFormat([u8; ::type_sizes::QT_WIDGETS_MAP_MAP_QT_CORE_DATE_QT_GUI_TEXT_CHAR_FORMAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MapQtCoreDateQtGuiTextCharFormat {
  unsafe fn new_uninitialized() -> MapQtCoreDateQtGuiTextCharFormat {
    MapQtCoreDateQtGuiTextCharFormat(::std::mem::uninitialized())
  }
}

impl MapQtCoreDateQtGuiTextCharFormat {
  /// C++ method: <span style='color: green;'>```void QMap<QDate, QTextCharFormat>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_clear(self as *mut ::map::MapQtCoreDateQtGuiTextCharFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMap<QDate, QTextCharFormat>::contains(const QDate& key) const```</span>
  ///
  ///
  pub fn contains(&self, key: &::qt_core::date::Date) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_contains(self as *const ::map::MapQtCoreDateQtGuiTextCharFormat,
                                                              key as *const ::qt_core::date::Date)
    }
  }

  /// C++ method: <span style='color: green;'>```QMap<QDate, QTextCharFormat>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QMap<QDate, QTextCharFormat>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::qt_core::date::Date) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QMap<QDate, QTextCharFormat>::count(const QDate& key) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::MapQtCoreDateQtGuiTextCharFormatCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QMap<QDate, QTextCharFormat>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_empty(self as *const ::map::MapQtCoreDateQtGuiTextCharFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextCharFormat& QMap<QDate, QTextCharFormat>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::qt_gui::text_char_format::TextCharFormat {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_first_const(self as *const ::map::MapQtCoreDateQtGuiTextCharFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QDate& QMap<QDate, QTextCharFormat>::firstKey() const```</span>
  ///
  ///
  pub fn first_key<'l0>(&'l0 self) -> &'l0 ::qt_core::date::Date {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_firstKey(self as *const ::map::MapQtCoreDateQtGuiTextCharFormat)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat& QMap<QDate, QTextCharFormat>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::text_char_format::TextCharFormat {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_first(self as *mut ::map::MapQtCoreDateQtGuiTextCharFormat)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<QDate, QTextCharFormat>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_isEmpty(self as *const ::map::MapQtCoreDateQtGuiTextCharFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```QMap<QDate, QTextCharFormat>::key```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn key(&self, &::qt_gui::text_char_format::TextCharFormat) -> ::qt_core::date::Date```<br>
  /// C++ method: <span style='color: green;'>```const QDate QMap<QDate, QTextCharFormat>::key(const QTextCharFormat& value) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn key(&self, (&::qt_gui::text_char_format::TextCharFormat, &::qt_core::date::Date)) -> ::qt_core::date::Date```<br>
  /// C++ method: <span style='color: green;'>```const QDate QMap<QDate, QTextCharFormat>::key(const QTextCharFormat& value, const QDate& defaultKey = ?) const```</span>
  ///
  ///
  pub fn key<'largs, Args>(&'largs self, args: Args) -> ::qt_core::date::Date
    where Args: overloading::MapQtCoreDateQtGuiTextCharFormatKeyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTextCharFormat& QMap<QDate, QTextCharFormat>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::qt_gui::text_char_format::TextCharFormat {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_last_const(self as *const ::map::MapQtCoreDateQtGuiTextCharFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QDate& QMap<QDate, QTextCharFormat>::lastKey() const```</span>
  ///
  ///
  pub fn last_key<'l0>(&'l0 self) -> &'l0 ::qt_core::date::Date {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_lastKey(self as *const ::map::MapQtCoreDateQtGuiTextCharFormat)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat& QMap<QDate, QTextCharFormat>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::text_char_format::TextCharFormat {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_last(self as *mut ::map::MapQtCoreDateQtGuiTextCharFormat)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMap<QDate, QTextCharFormat>::QMap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::map::MapQtCoreDateQtGuiTextCharFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMap<QDate, QTextCharFormat>::QMap()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::map::MapQtCoreDateQtGuiTextCharFormat) -> ::map::MapQtCoreDateQtGuiTextCharFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMap<QDate, QTextCharFormat>::QMap(const QMap<QDate, QTextCharFormat>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::map::MapQtCoreDateQtGuiTextCharFormat
    where Args: overloading::MapQtCoreDateQtGuiTextCharFormatNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMap<QDate, QTextCharFormat>& QMap<QDate, QTextCharFormat>::operator=(const QMap<QDate, QTextCharFormat>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::map::MapQtCoreDateQtGuiTextCharFormat)
                             -> &'l0 mut ::map::MapQtCoreDateQtGuiTextCharFormat {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_operator_assign(self as *mut ::map::MapQtCoreDateQtGuiTextCharFormat, other as *const ::map::MapQtCoreDateQtGuiTextCharFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<QDate, QTextCharFormat>::operator==(const QMap<QDate, QTextCharFormat>& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::map::MapQtCoreDateQtGuiTextCharFormat) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_operator_eq(self as *const ::map::MapQtCoreDateQtGuiTextCharFormat, other as *const ::map::MapQtCoreDateQtGuiTextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```const QTextCharFormat QMap<QDate, QTextCharFormat>::operator[](const QDate& key) const```</span>
  ///
  ///
  pub fn op_index(&self, key: &::qt_core::date::Date) -> ::qt_gui::text_char_format::TextCharFormat {
    {
      let mut object: ::qt_gui::text_char_format::TextCharFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_operator_index_to_output(self as *const ::map::MapQtCoreDateQtGuiTextCharFormat, key as *const ::qt_core::date::Date, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat& QMap<QDate, QTextCharFormat>::operator[](const QDate& key)```</span>
  ///
  ///
  pub fn op_index_mut<'l0, 'l1>(&'l0 mut self,
                                key: &'l1 ::qt_core::date::Date)
                                -> &'l0 mut ::qt_gui::text_char_format::TextCharFormat {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_operator_index(self as *mut ::map::MapQtCoreDateQtGuiTextCharFormat, key as *const ::qt_core::date::Date) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<QDate, QTextCharFormat>::operator!=(const QMap<QDate, QTextCharFormat>& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::map::MapQtCoreDateQtGuiTextCharFormat) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_operator_neq(self as *const ::map::MapQtCoreDateQtGuiTextCharFormat, other as *const ::map::MapQtCoreDateQtGuiTextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QMap<QDate, QTextCharFormat>::remove(const QDate& key)```</span>
  ///
  ///
  pub fn remove(&mut self, key: &::qt_core::date::Date) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_remove(self as *mut ::map::MapQtCoreDateQtGuiTextCharFormat,
                                                            key as *const ::qt_core::date::Date)
    }
  }

  /// C++ method: <span style='color: green;'>```int QMap<QDate, QTextCharFormat>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_size(self as *const ::map::MapQtCoreDateQtGuiTextCharFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMap<QDate, QTextCharFormat>::swap(QMap<QDate, QTextCharFormat>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::map::MapQtCoreDateQtGuiTextCharFormat) {
    unsafe {
      ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_swap(self as *mut ::map::MapQtCoreDateQtGuiTextCharFormat,
                                                          other as *mut ::map::MapQtCoreDateQtGuiTextCharFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat QMap<QDate, QTextCharFormat>::take(const QDate& key)```</span>
  ///
  ///
  pub fn take(&mut self, key: &::qt_core::date::Date) -> ::qt_gui::text_char_format::TextCharFormat {
    {
      let mut object: ::qt_gui::text_char_format::TextCharFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_take_to_output(self as *mut ::map::MapQtCoreDateQtGuiTextCharFormat, key as *const ::qt_core::date::Date, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMap<QDate, QTextCharFormat>& QMap<QDate, QTextCharFormat>::unite(const QMap<QDate, QTextCharFormat>& other)```</span>
  ///
  ///
  pub fn unite<'l0, 'l1>(&'l0 mut self,
                         other: &'l1 ::map::MapQtCoreDateQtGuiTextCharFormat)
                         -> &'l0 mut ::map::MapQtCoreDateQtGuiTextCharFormat {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_unite(self as *mut ::map::MapQtCoreDateQtGuiTextCharFormat,
                                                             other as *const ::map::MapQtCoreDateQtGuiTextCharFormat)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMap<QDate, QTextCharFormat>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, &::qt_core::date::Date) -> ::qt_gui::text_char_format::TextCharFormat```<br>
  /// C++ method: <span style='color: green;'>```const QTextCharFormat QMap<QDate, QTextCharFormat>::value(const QDate& key) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (&::qt_core::date::Date, &::qt_gui::text_char_format::TextCharFormat)) -> ::qt_gui::text_char_format::TextCharFormat```<br>
  /// C++ method: <span style='color: green;'>```const QTextCharFormat QMap<QDate, QTextCharFormat>::value(const QDate& key, const QTextCharFormat& defaultValue = ?) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::qt_gui::text_char_format::TextCharFormat
    where Args: overloading::MapQtCoreDateQtGuiTextCharFormatValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::map::MapQtCoreDateQtGuiTextCharFormat {
  /// C++ method: <span style='color: green;'>```[destructor] void QMap<QDate, QTextCharFormat>::~QMap()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_destructor(self as *mut ::map::MapQtCoreDateQtGuiTextCharFormat)
    }
  }
}

/// C++ type: <span style='color: green;'>```QMap<Qt::GestureType, bool>```</span>
#[repr(C)]
pub struct MapQtCoreQtGestureTypeBool([u8; ::type_sizes::QT_WIDGETS_MAP_MAP_QT_CORE_QT_GESTURE_TYPE_BOOL]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MapQtCoreQtGestureTypeBool {
  unsafe fn new_uninitialized() -> MapQtCoreQtGestureTypeBool {
    MapQtCoreQtGestureTypeBool(::std::mem::uninitialized())
  }
}

impl MapQtCoreQtGestureTypeBool {
  /// C++ method: <span style='color: green;'>```void QMap<Qt::GestureType, bool>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_clear(self as *mut ::map::MapQtCoreQtGestureTypeBool) }
  }

  /// C++ method: <span style='color: green;'>```bool QMap<Qt::GestureType, bool>::contains(const Qt::GestureType& key) const```</span>
  ///
  ///
  pub fn contains(&self, key: &::qt_core::qt::GestureType) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_contains(self as *const ::map::MapQtCoreQtGestureTypeBool,
                                                            key as *const ::qt_core::qt::GestureType)
    }
  }

  /// C++ method: <span style='color: green;'>```QMap<Qt::GestureType, bool>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QMap<Qt::GestureType, bool>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::qt_core::qt::GestureType) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QMap<Qt::GestureType, bool>::count(const Qt::GestureType& key) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::MapQtCoreQtGestureTypeBoolCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QMap<Qt::GestureType, bool>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_empty(self as *const ::map::MapQtCoreQtGestureTypeBool) }
  }

  /// C++ method: <span style='color: green;'>```const bool& QMap<Qt::GestureType, bool>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 bool {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_first_const(self as *const ::map::MapQtCoreQtGestureTypeBool)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt::GestureType& QMap<Qt::GestureType, bool>::firstKey() const```</span>
  ///
  ///
  pub fn first_key<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::GestureType {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_firstKey(self as *const ::map::MapQtCoreQtGestureTypeBool)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool& QMap<Qt::GestureType, bool>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut bool {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_first(self as *mut ::map::MapQtCoreQtGestureTypeBool) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<Qt::GestureType, bool>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_isEmpty(self as *const ::map::MapQtCoreQtGestureTypeBool) }
  }

  /// C++ method: <span style='color: green;'>```QMap<Qt::GestureType, bool>::key```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn key(&self, &bool) -> ::qt_core::qt::GestureType```<br>
  /// C++ method: <span style='color: green;'>```const Qt::GestureType QMap<Qt::GestureType, bool>::key(const bool& value) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn key(&self, (&bool, &::qt_core::qt::GestureType)) -> ::qt_core::qt::GestureType```<br>
  /// C++ method: <span style='color: green;'>```const Qt::GestureType QMap<Qt::GestureType, bool>::key(const bool& value, const Qt::GestureType& defaultKey = ?) const```</span>
  ///
  ///
  pub fn key<'largs, Args>(&'largs self, args: Args) -> ::qt_core::qt::GestureType
    where Args: overloading::MapQtCoreQtGestureTypeBoolKeyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const bool& QMap<Qt::GestureType, bool>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 bool {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_last_const(self as *const ::map::MapQtCoreQtGestureTypeBool)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt::GestureType& QMap<Qt::GestureType, bool>::lastKey() const```</span>
  ///
  ///
  pub fn last_key<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::GestureType {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_lastKey(self as *const ::map::MapQtCoreQtGestureTypeBool) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool& QMap<Qt::GestureType, bool>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut bool {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_last(self as *mut ::map::MapQtCoreQtGestureTypeBool) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMap<Qt::GestureType, bool>::QMap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::map::MapQtCoreQtGestureTypeBool```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMap<Qt::GestureType, bool>::QMap()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::map::MapQtCoreQtGestureTypeBool) -> ::map::MapQtCoreQtGestureTypeBool```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMap<Qt::GestureType, bool>::QMap(const QMap<Qt::GestureType, bool>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::map::MapQtCoreQtGestureTypeBool
    where Args: overloading::MapQtCoreQtGestureTypeBoolNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMap<Qt::GestureType, bool>& QMap<Qt::GestureType, bool>::operator=(const QMap<Qt::GestureType, bool>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::map::MapQtCoreQtGestureTypeBool)
                             -> &'l0 mut ::map::MapQtCoreQtGestureTypeBool {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_operator_assign(self as *mut ::map::MapQtCoreQtGestureTypeBool, other as *const ::map::MapQtCoreQtGestureTypeBool)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<Qt::GestureType, bool>::operator==(const QMap<Qt::GestureType, bool>& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::map::MapQtCoreQtGestureTypeBool) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_operator_eq(self as *const ::map::MapQtCoreQtGestureTypeBool,
                                                               other as *const ::map::MapQtCoreQtGestureTypeBool)
    }
  }

  /// C++ method: <span style='color: green;'>```const bool QMap<Qt::GestureType, bool>::operator[](const Qt::GestureType& key) const```</span>
  ///
  ///
  pub fn op_index(&self, key: &::qt_core::qt::GestureType) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_operator_index_const(self as *const ::map::MapQtCoreQtGestureTypeBool, key as *const ::qt_core::qt::GestureType) }
  }

  /// C++ method: <span style='color: green;'>```bool& QMap<Qt::GestureType, bool>::operator[](const Qt::GestureType& key)```</span>
  ///
  ///
  pub fn op_index_mut<'l0, 'l1>(&'l0 mut self, key: &'l1 ::qt_core::qt::GestureType) -> &'l0 mut bool {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_operator_index(self as *mut ::map::MapQtCoreQtGestureTypeBool,
                                                                    key as *const ::qt_core::qt::GestureType)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<Qt::GestureType, bool>::operator!=(const QMap<Qt::GestureType, bool>& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::map::MapQtCoreQtGestureTypeBool) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_operator_neq(self as *const ::map::MapQtCoreQtGestureTypeBool,
                                                                other as *const ::map::MapQtCoreQtGestureTypeBool)
    }
  }

  /// C++ method: <span style='color: green;'>```int QMap<Qt::GestureType, bool>::remove(const Qt::GestureType& key)```</span>
  ///
  ///
  pub fn remove(&mut self, key: &::qt_core::qt::GestureType) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_remove(self as *mut ::map::MapQtCoreQtGestureTypeBool,
                                                          key as *const ::qt_core::qt::GestureType)
    }
  }

  /// C++ method: <span style='color: green;'>```int QMap<Qt::GestureType, bool>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_size(self as *const ::map::MapQtCoreQtGestureTypeBool) }
  }

  /// C++ method: <span style='color: green;'>```void QMap<Qt::GestureType, bool>::swap(QMap<Qt::GestureType, bool>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::map::MapQtCoreQtGestureTypeBool) {
    unsafe {
      ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_swap(self as *mut ::map::MapQtCoreQtGestureTypeBool,
                                                        other as *mut ::map::MapQtCoreQtGestureTypeBool)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMap<Qt::GestureType, bool>::take(const Qt::GestureType& key)```</span>
  ///
  ///
  pub fn take(&mut self, key: &::qt_core::qt::GestureType) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_take(self as *mut ::map::MapQtCoreQtGestureTypeBool,
                                                        key as *const ::qt_core::qt::GestureType)
    }
  }

  /// C++ method: <span style='color: green;'>```QMap<Qt::GestureType, bool>& QMap<Qt::GestureType, bool>::unite(const QMap<Qt::GestureType, bool>& other)```</span>
  ///
  ///
  pub fn unite<'l0, 'l1>(&'l0 mut self,
                         other: &'l1 ::map::MapQtCoreQtGestureTypeBool)
                         -> &'l0 mut ::map::MapQtCoreQtGestureTypeBool {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_unite(self as *mut ::map::MapQtCoreQtGestureTypeBool,
                                                           other as *const ::map::MapQtCoreQtGestureTypeBool)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMap<Qt::GestureType, bool>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, &::qt_core::qt::GestureType) -> bool```<br>
  /// C++ method: <span style='color: green;'>```const bool QMap<Qt::GestureType, bool>::value(const Qt::GestureType& key) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (&::qt_core::qt::GestureType, &bool)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```const bool QMap<Qt::GestureType, bool>::value(const Qt::GestureType& key, const bool& defaultValue = ?) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::MapQtCoreQtGestureTypeBoolValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::map::MapQtCoreQtGestureTypeBool {
  /// C++ method: <span style='color: green;'>```[destructor] void QMap<Qt::GestureType, bool>::~QMap()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_destructor(self as *mut ::map::MapQtCoreQtGestureTypeBool) }
  }
}

/// C++ type: <span style='color: green;'>```QMap<Qt::GestureType, QWidget*>```</span>
#[repr(C)]
pub struct MapQtCoreQtGestureTypeWidgetMutPtr([u8; ::type_sizes::QT_WIDGETS_MAP_MAP_QT_CORE_QT_GESTURE_TYPE_WIDGET_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MapQtCoreQtGestureTypeWidgetMutPtr {
  unsafe fn new_uninitialized() -> MapQtCoreQtGestureTypeWidgetMutPtr {
    MapQtCoreQtGestureTypeWidgetMutPtr(::std::mem::uninitialized())
  }
}

impl MapQtCoreQtGestureTypeWidgetMutPtr {
  /// C++ method: <span style='color: green;'>```void QMap<Qt::GestureType, QWidget*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_clear(self as *mut ::map::MapQtCoreQtGestureTypeWidgetMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMap<Qt::GestureType, QWidget*>::contains(const Qt::GestureType& key) const```</span>
  ///
  ///
  pub fn contains(&self, key: &::qt_core::qt::GestureType) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_contains(self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr, key as *const ::qt_core::qt::GestureType) }
  }

  /// C++ method: <span style='color: green;'>```QMap<Qt::GestureType, QWidget*>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QMap<Qt::GestureType, QWidget*>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::qt_core::qt::GestureType) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QMap<Qt::GestureType, QWidget*>::count(const Qt::GestureType& key) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::MapQtCoreQtGestureTypeWidgetMutPtrCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QMap<Qt::GestureType, QWidget*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_empty(self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* const & QMap<Qt::GestureType, QWidget*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_first_const(self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt::GestureType& QMap<Qt::GestureType, QWidget*>::firstKey() const```</span>
  ///
  ///
  pub fn first_key<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::GestureType {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_firstKey(self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWidget*& QMap<Qt::GestureType, QWidget*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_first(self as *mut ::map::MapQtCoreQtGestureTypeWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<Qt::GestureType, QWidget*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_isEmpty(self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QMap<Qt::GestureType, QWidget*>::key```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn key(&self, &*mut ::widget::Widget) -> ::qt_core::qt::GestureType```<br>
  /// C++ method: <span style='color: green;'>```const Qt::GestureType QMap<Qt::GestureType, QWidget*>::key(QWidget* const & value) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn key(&self, (&*mut ::widget::Widget, &::qt_core::qt::GestureType)) -> ::qt_core::qt::GestureType```<br>
  /// C++ method: <span style='color: green;'>```const Qt::GestureType QMap<Qt::GestureType, QWidget*>::key(QWidget* const & value, const Qt::GestureType& defaultKey = ?) const```</span>
  ///
  ///
  pub unsafe fn key<'largs, Args>(&'largs self, args: Args) -> ::qt_core::qt::GestureType
    where Args: overloading::MapQtCoreQtGestureTypeWidgetMutPtrKeyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWidget* const & QMap<Qt::GestureType, QWidget*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_last_const(self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt::GestureType& QMap<Qt::GestureType, QWidget*>::lastKey() const```</span>
  ///
  ///
  pub fn last_key<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::GestureType {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_lastKey(self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWidget*& QMap<Qt::GestureType, QWidget*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_last(self as *mut ::map::MapQtCoreQtGestureTypeWidgetMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMap<Qt::GestureType, QWidget*>::QMap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::map::MapQtCoreQtGestureTypeWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMap<Qt::GestureType, QWidget*>::QMap()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::map::MapQtCoreQtGestureTypeWidgetMutPtr) -> ::map::MapQtCoreQtGestureTypeWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMap<Qt::GestureType, QWidget*>::QMap(const QMap<Qt::GestureType, QWidget*>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::map::MapQtCoreQtGestureTypeWidgetMutPtr
    where Args: overloading::MapQtCoreQtGestureTypeWidgetMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMap<Qt::GestureType, QWidget*>& QMap<Qt::GestureType, QWidget*>::operator=(const QMap<Qt::GestureType, QWidget*>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::map::MapQtCoreQtGestureTypeWidgetMutPtr)
                             -> &'l0 mut ::map::MapQtCoreQtGestureTypeWidgetMutPtr {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_operator_assign(self as *mut ::map::MapQtCoreQtGestureTypeWidgetMutPtr, other as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<Qt::GestureType, QWidget*>::operator==(const QMap<Qt::GestureType, QWidget*>& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::map::MapQtCoreQtGestureTypeWidgetMutPtr) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_operator_eq(self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr, other as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QMap<Qt::GestureType, QWidget*>::operator[](const Qt::GestureType& key) const```</span>
  ///
  ///
  pub fn op_index(&self, key: &::qt_core::qt::GestureType) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_operator_index_const(self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr, key as *const ::qt_core::qt::GestureType) }
  }

  /// C++ method: <span style='color: green;'>```QWidget*& QMap<Qt::GestureType, QWidget*>::operator[](const Qt::GestureType& key)```</span>
  ///
  ///
  pub fn op_index_mut<'l0, 'l1>(&'l0 mut self, key: &'l1 ::qt_core::qt::GestureType) -> &'l0 mut *mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_operator_index(self as *mut ::map::MapQtCoreQtGestureTypeWidgetMutPtr, key as *const ::qt_core::qt::GestureType) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<Qt::GestureType, QWidget*>::operator!=(const QMap<Qt::GestureType, QWidget*>& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::map::MapQtCoreQtGestureTypeWidgetMutPtr) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_operator_neq(self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr, other as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QMap<Qt::GestureType, QWidget*>::remove(const Qt::GestureType& key)```</span>
  ///
  ///
  pub fn remove(&mut self, key: &::qt_core::qt::GestureType) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_remove(self as *mut ::map::MapQtCoreQtGestureTypeWidgetMutPtr, key as *const ::qt_core::qt::GestureType)
    }
  }

  /// C++ method: <span style='color: green;'>```int QMap<Qt::GestureType, QWidget*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_size(self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMap<Qt::GestureType, QWidget*>::swap(QMap<Qt::GestureType, QWidget*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::map::MapQtCoreQtGestureTypeWidgetMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_swap(self as *mut ::map::MapQtCoreQtGestureTypeWidgetMutPtr, other as *mut ::map::MapQtCoreQtGestureTypeWidgetMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QMap<Qt::GestureType, QWidget*>::take(const Qt::GestureType& key)```</span>
  ///
  ///
  pub fn take(&mut self, key: &::qt_core::qt::GestureType) -> *mut ::widget::Widget {
    unsafe {
      ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_take(self as *mut ::map::MapQtCoreQtGestureTypeWidgetMutPtr,
                                                               key as *const ::qt_core::qt::GestureType)
    }
  }

  /// C++ method: <span style='color: green;'>```QMap<Qt::GestureType, QWidget*>& QMap<Qt::GestureType, QWidget*>::unite(const QMap<Qt::GestureType, QWidget*>& other)```</span>
  ///
  ///
  pub fn unite<'l0, 'l1>(&'l0 mut self,
                         other: &'l1 ::map::MapQtCoreQtGestureTypeWidgetMutPtr)
                         -> &'l0 mut ::map::MapQtCoreQtGestureTypeWidgetMutPtr {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_unite(self as *mut ::map::MapQtCoreQtGestureTypeWidgetMutPtr, other as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWidget* QMap<Qt::GestureType, QWidget*>::value(const Qt::GestureType& key) const```</span>
  ///
  ///
  pub fn value(&self, key: &::qt_core::qt::GestureType) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_value_key(self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr, key as *const ::qt_core::qt::GestureType) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QMap<Qt::GestureType, QWidget*>::value(const Qt::GestureType& key, QWidget* const & defaultValue = ?) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             key: &::qt_core::qt::GestureType,
                             default_value: &*mut ::widget::Widget)
                             -> *mut ::widget::Widget {
    ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_value_key_defaultValue(self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr, key as *const ::qt_core::qt::GestureType, default_value as *const *mut ::widget::Widget)
  }

  /// C++ method: <span style='color: green;'>```QMap<Qt::GestureType, QWidget*>::values```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn values(&self, ()) -> ::list::ListWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QWidget*> QMap<Qt::GestureType, QWidget*>::values() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn values(&self, &::qt_core::qt::GestureType) -> ::list::ListWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QWidget*> QMap<Qt::GestureType, QWidget*>::values(const Qt::GestureType& key) const```</span>
  ///
  ///
  pub fn values<'largs, Args>(&'largs self, args: Args) -> ::list::ListWidgetMutPtr
    where Args: overloading::MapQtCoreQtGestureTypeWidgetMutPtrValuesArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::map::MapQtCoreQtGestureTypeWidgetMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QMap<Qt::GestureType, QWidget*>::~QMap()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_destructor(self as *mut ::map::MapQtCoreQtGestureTypeWidgetMutPtr) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MapQtCoreDateQtGuiTextCharFormat::count](../struct.MapQtCoreDateQtGuiTextCharFormat.html#method.count) method.
  pub trait MapQtCoreDateQtGuiTextCharFormatCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapQtCoreDateQtGuiTextCharFormat) -> ::libc::c_int;
  }
  impl<'largs> MapQtCoreDateQtGuiTextCharFormatCountArgs<'largs> for &'largs ::qt_core::date::Date {
    fn exec(self, original_self: &'largs ::map::MapQtCoreDateQtGuiTextCharFormat) -> ::libc::c_int {
      let key = self;
      unsafe { ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_count_key(original_self as *const ::map::MapQtCoreDateQtGuiTextCharFormat, key as *const ::qt_core::date::Date) }
    }
  }
  impl<'largs> MapQtCoreDateQtGuiTextCharFormatCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::map::MapQtCoreDateQtGuiTextCharFormat) -> ::libc::c_int {

      unsafe { ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_count_no_args(original_self as *const ::map::MapQtCoreDateQtGuiTextCharFormat) }
    }
  }
  /// This trait represents a set of arguments accepted by [MapQtCoreDateQtGuiTextCharFormat::key](../struct.MapQtCoreDateQtGuiTextCharFormat.html#method.key) method.
  pub trait MapQtCoreDateQtGuiTextCharFormatKeyArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapQtCoreDateQtGuiTextCharFormat) -> ::qt_core::date::Date;
  }
  impl<'largs> MapQtCoreDateQtGuiTextCharFormatKeyArgs<'largs> for &'largs ::qt_gui::text_char_format::TextCharFormat {
    fn exec(self, original_self: &'largs ::map::MapQtCoreDateQtGuiTextCharFormat) -> ::qt_core::date::Date {
      let value = self;
      {
        let mut object: ::qt_core::date::Date =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_key_to_output_value(original_self as *const ::map::MapQtCoreDateQtGuiTextCharFormat, value as *const ::qt_gui::text_char_format::TextCharFormat, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MapQtCoreDateQtGuiTextCharFormatKeyArgs<'largs> for (&'largs ::qt_gui::text_char_format::TextCharFormat,&'largs ::qt_core::date::Date) {

  fn exec(self, original_self: &'largs ::map::MapQtCoreDateQtGuiTextCharFormat) -> ::qt_core::date::Date {
    let value = self.0;
let default_key = self.1;
    {
let mut object: ::qt_core::date::Date = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_key_to_output_value_defaultKey(original_self as *const ::map::MapQtCoreDateQtGuiTextCharFormat, value as *const ::qt_gui::text_char_format::TextCharFormat, default_key as *const ::qt_core::date::Date, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [MapQtCoreDateQtGuiTextCharFormat::new](../struct.MapQtCoreDateQtGuiTextCharFormat.html#method.new) method.
  pub trait MapQtCoreDateQtGuiTextCharFormatNewArgs {
    fn exec(self) -> ::map::MapQtCoreDateQtGuiTextCharFormat;
  }
  impl MapQtCoreDateQtGuiTextCharFormatNewArgs for () {
    fn exec(self) -> ::map::MapQtCoreDateQtGuiTextCharFormat {

      {
        let mut object: ::map::MapQtCoreDateQtGuiTextCharFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> MapQtCoreDateQtGuiTextCharFormatNewArgs for &'a ::map::MapQtCoreDateQtGuiTextCharFormat {
    fn exec(self) -> ::map::MapQtCoreDateQtGuiTextCharFormat {
      let other = self;
      {
        let mut object: ::map::MapQtCoreDateQtGuiTextCharFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_constructor_other(other as *const ::map::MapQtCoreDateQtGuiTextCharFormat, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MapQtCoreDateQtGuiTextCharFormat::value](../struct.MapQtCoreDateQtGuiTextCharFormat.html#method.value) method.
  pub trait MapQtCoreDateQtGuiTextCharFormatValueArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::map::MapQtCoreDateQtGuiTextCharFormat)
            -> ::qt_gui::text_char_format::TextCharFormat;
  }
  impl<'largs> MapQtCoreDateQtGuiTextCharFormatValueArgs<'largs> for &'largs ::qt_core::date::Date {
    fn exec(self,
            original_self: &'largs ::map::MapQtCoreDateQtGuiTextCharFormat)
            -> ::qt_gui::text_char_format::TextCharFormat {
      let key = self;
      {
        let mut object: ::qt_gui::text_char_format::TextCharFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_value_to_output_key(original_self as *const ::map::MapQtCoreDateQtGuiTextCharFormat, key as *const ::qt_core::date::Date, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MapQtCoreDateQtGuiTextCharFormatValueArgs<'largs> for (&'largs ::qt_core::date::Date,&'largs ::qt_gui::text_char_format::TextCharFormat) {

  fn exec(self, original_self: &'largs ::map::MapQtCoreDateQtGuiTextCharFormat) -> ::qt_gui::text_char_format::TextCharFormat {
    let key = self.0;
let default_value = self.1;
    {
let mut object: ::qt_gui::text_char_format::TextCharFormat = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_widgets_c_QMap_QDate_QTextCharFormat_value_to_output_key_defaultValue(original_self as *const ::map::MapQtCoreDateQtGuiTextCharFormat, key as *const ::qt_core::date::Date, default_value as *const ::qt_gui::text_char_format::TextCharFormat, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [MapQtCoreQtGestureTypeBool::count](../struct.MapQtCoreQtGestureTypeBool.html#method.count) method.
  pub trait MapQtCoreQtGestureTypeBoolCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeBool) -> ::libc::c_int;
  }
  impl<'largs> MapQtCoreQtGestureTypeBoolCountArgs<'largs> for &'largs ::qt_core::qt::GestureType {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeBool) -> ::libc::c_int {
      let key = self;
      unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_count_key(original_self as *const ::map::MapQtCoreQtGestureTypeBool, key as *const ::qt_core::qt::GestureType) }
    }
  }
  impl<'largs> MapQtCoreQtGestureTypeBoolCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeBool) -> ::libc::c_int {

      unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_count_no_args(original_self as *const ::map::MapQtCoreQtGestureTypeBool) }
    }
  }
  /// This trait represents a set of arguments accepted by [MapQtCoreQtGestureTypeBool::key](../struct.MapQtCoreQtGestureTypeBool.html#method.key) method.
  pub trait MapQtCoreQtGestureTypeBoolKeyArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeBool) -> ::qt_core::qt::GestureType;
  }
  impl<'largs> MapQtCoreQtGestureTypeBoolKeyArgs<'largs> for &'largs bool {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeBool) -> ::qt_core::qt::GestureType {
      let value = self;
      unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_key_value(original_self as *const ::map::MapQtCoreQtGestureTypeBool, value as *const bool) }
    }
  }
  impl<'largs> MapQtCoreQtGestureTypeBoolKeyArgs<'largs> for (&'largs bool, &'largs ::qt_core::qt::GestureType) {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeBool) -> ::qt_core::qt::GestureType {
      let value = self.0;
      let default_key = self.1;
      unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_key_value_defaultKey(original_self as *const ::map::MapQtCoreQtGestureTypeBool, value as *const bool, default_key as *const ::qt_core::qt::GestureType) }
    }
  }
  /// This trait represents a set of arguments accepted by [MapQtCoreQtGestureTypeBool::new](../struct.MapQtCoreQtGestureTypeBool.html#method.new) method.
  pub trait MapQtCoreQtGestureTypeBoolNewArgs {
    fn exec(self) -> ::map::MapQtCoreQtGestureTypeBool;
  }
  impl MapQtCoreQtGestureTypeBoolNewArgs for () {
    fn exec(self) -> ::map::MapQtCoreQtGestureTypeBool {

      {
        let mut object: ::map::MapQtCoreQtGestureTypeBool =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> MapQtCoreQtGestureTypeBoolNewArgs for &'a ::map::MapQtCoreQtGestureTypeBool {
    fn exec(self) -> ::map::MapQtCoreQtGestureTypeBool {
      let other = self;
      {
        let mut object: ::map::MapQtCoreQtGestureTypeBool =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_constructor_other(other as *const ::map::MapQtCoreQtGestureTypeBool, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MapQtCoreQtGestureTypeBool::value](../struct.MapQtCoreQtGestureTypeBool.html#method.value) method.
  pub trait MapQtCoreQtGestureTypeBoolValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeBool) -> bool;
  }
  impl<'largs> MapQtCoreQtGestureTypeBoolValueArgs<'largs> for &'largs ::qt_core::qt::GestureType {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeBool) -> bool {
      let key = self;
      unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_value_key(original_self as *const ::map::MapQtCoreQtGestureTypeBool, key as *const ::qt_core::qt::GestureType) }
    }
  }
  impl<'largs> MapQtCoreQtGestureTypeBoolValueArgs<'largs> for (&'largs ::qt_core::qt::GestureType, &'largs bool) {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeBool) -> bool {
      let key = self.0;
      let default_value = self.1;
      unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_bool_value_key_defaultValue(original_self as *const ::map::MapQtCoreQtGestureTypeBool, key as *const ::qt_core::qt::GestureType, default_value as *const bool) }
    }
  }
  /// This trait represents a set of arguments accepted by [MapQtCoreQtGestureTypeWidgetMutPtr::count](../struct.MapQtCoreQtGestureTypeWidgetMutPtr.html#method.count) method.
  pub trait MapQtCoreQtGestureTypeWidgetMutPtrCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeWidgetMutPtr) -> ::libc::c_int;
  }
  impl<'largs> MapQtCoreQtGestureTypeWidgetMutPtrCountArgs<'largs> for &'largs ::qt_core::qt::GestureType {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeWidgetMutPtr) -> ::libc::c_int {
      let key = self;
      unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_count_key(original_self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr, key as *const ::qt_core::qt::GestureType) }
    }
  }
  impl<'largs> MapQtCoreQtGestureTypeWidgetMutPtrCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeWidgetMutPtr) -> ::libc::c_int {

      unsafe { ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_count_no_args(original_self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr) }
    }
  }
  /// This trait represents a set of arguments accepted by [MapQtCoreQtGestureTypeWidgetMutPtr::key](../struct.MapQtCoreQtGestureTypeWidgetMutPtr.html#method.key) method.
  pub trait MapQtCoreQtGestureTypeWidgetMutPtrKeyArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs ::map::MapQtCoreQtGestureTypeWidgetMutPtr)
                   -> ::qt_core::qt::GestureType;
  }
  impl<'largs> MapQtCoreQtGestureTypeWidgetMutPtrKeyArgs<'largs> for &'largs *mut ::widget::Widget {
    unsafe fn exec(self,
                   original_self: &'largs ::map::MapQtCoreQtGestureTypeWidgetMutPtr)
                   -> ::qt_core::qt::GestureType {
      let value = self;
      ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_key_value(original_self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr, value as *const *mut ::widget::Widget)
    }
  }
  impl<'largs> MapQtCoreQtGestureTypeWidgetMutPtrKeyArgs<'largs>
    for (&'largs *mut ::widget::Widget, &'largs ::qt_core::qt::GestureType) {
    unsafe fn exec(self,
                   original_self: &'largs ::map::MapQtCoreQtGestureTypeWidgetMutPtr)
                   -> ::qt_core::qt::GestureType {
      let value = self.0;
      let default_key = self.1;
      ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_key_value_defaultKey(original_self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr, value as *const *mut ::widget::Widget, default_key as *const ::qt_core::qt::GestureType)
    }
  }
  /// This trait represents a set of arguments accepted by [MapQtCoreQtGestureTypeWidgetMutPtr::new](../struct.MapQtCoreQtGestureTypeWidgetMutPtr.html#method.new) method.
  pub trait MapQtCoreQtGestureTypeWidgetMutPtrNewArgs {
    fn exec(self) -> ::map::MapQtCoreQtGestureTypeWidgetMutPtr;
  }
  impl MapQtCoreQtGestureTypeWidgetMutPtrNewArgs for () {
    fn exec(self) -> ::map::MapQtCoreQtGestureTypeWidgetMutPtr {

      {
        let mut object: ::map::MapQtCoreQtGestureTypeWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> MapQtCoreQtGestureTypeWidgetMutPtrNewArgs for &'a ::map::MapQtCoreQtGestureTypeWidgetMutPtr {
    fn exec(self) -> ::map::MapQtCoreQtGestureTypeWidgetMutPtr {
      let other = self;
      {
        let mut object: ::map::MapQtCoreQtGestureTypeWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_constructor_other(other as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MapQtCoreQtGestureTypeWidgetMutPtr::values](../struct.MapQtCoreQtGestureTypeWidgetMutPtr.html#method.values) method.
  pub trait MapQtCoreQtGestureTypeWidgetMutPtrValuesArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeWidgetMutPtr) -> ::list::ListWidgetMutPtr;
  }
  impl<'largs> MapQtCoreQtGestureTypeWidgetMutPtrValuesArgs<'largs> for &'largs ::qt_core::qt::GestureType {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeWidgetMutPtr) -> ::list::ListWidgetMutPtr {
      let key = self;
      {
        let mut object: ::list::ListWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_values_to_output_key(original_self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr, key as *const ::qt_core::qt::GestureType, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MapQtCoreQtGestureTypeWidgetMutPtrValuesArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::map::MapQtCoreQtGestureTypeWidgetMutPtr) -> ::list::ListWidgetMutPtr {

      {
        let mut object: ::list::ListWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_values_to_output_no_args(original_self as *const ::map::MapQtCoreQtGestureTypeWidgetMutPtr, &mut object);
        }
        object
      }
    }
  }
}
