/// C++ type: <span style='color: green;'>```QBoxLayout```</span>
#[repr(C)]
pub struct BoxLayout(u8);

impl BoxLayout {
  /// C++ method: <span style='color: green;'>```virtual void QBoxLayout::addItem(QLayoutItem* arg1)```</span>
  ///
  ///
  pub unsafe fn add_item(&mut self, arg1: *mut ::layout_item::LayoutItem) {
    ::ffi::qt_widgets_c_QBoxLayout_addItem(self as *mut ::box_layout::BoxLayout, arg1)
  }

  /// C++ method: <span style='color: green;'>```QBoxLayout::addLayout```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_layout(&mut self, *mut ::layout::Layout) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBoxLayout::addLayout(QLayout* layout)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_layout(&mut self, (*mut ::layout::Layout, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBoxLayout::addLayout(QLayout* layout, int stretch = ?)```</span>
  ///
  ///
  pub unsafe fn add_layout<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::BoxLayoutAddLayoutArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QBoxLayout::addSpacerItem(QSpacerItem* spacerItem)```</span>
  ///
  ///
  pub unsafe fn add_spacer_item(&mut self, spacer_item: *mut ::spacer_item::SpacerItem) {
    ::ffi::qt_widgets_c_QBoxLayout_addSpacerItem(self as *mut ::box_layout::BoxLayout, spacer_item)
  }

  /// C++ method: <span style='color: green;'>```void QBoxLayout::addSpacing(int size)```</span>
  ///
  ///
  pub fn add_spacing(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_addSpacing(self as *mut ::box_layout::BoxLayout, size) }
  }

  /// C++ method: <span style='color: green;'>```QBoxLayout::addStretch```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_stretch(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBoxLayout::addStretch()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_stretch(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBoxLayout::addStretch(int stretch = ?)```</span>
  ///
  ///
  pub fn add_stretch<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::BoxLayoutAddStretchArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QBoxLayout::addStrut(int arg1)```</span>
  ///
  ///
  pub fn add_strut(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_addStrut(self as *mut ::box_layout::BoxLayout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QBoxLayout::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_count(self as *const ::box_layout::BoxLayout) }
  }

  /// C++ method: <span style='color: green;'>```QBoxLayout::Direction QBoxLayout::direction() const```</span>
  ///
  ///
  pub fn direction(&self) -> ::box_layout::Direction {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_direction(self as *const ::box_layout::BoxLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QBoxLayout::hasHeightForWidth() const```</span>
  ///
  ///
  pub fn has_height_for_width(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_hasHeightForWidth(self as *const ::box_layout::BoxLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QBoxLayout::heightForWidth(int arg1) const```</span>
  ///
  ///
  pub fn height_for_width(&self, arg1: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_heightForWidth(self as *const ::box_layout::BoxLayout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QBoxLayout::insertItem(int index, QLayoutItem* arg2)```</span>
  ///
  ///
  pub unsafe fn insert_item(&mut self, index: ::libc::c_int, arg2: *mut ::layout_item::LayoutItem) {
    ::ffi::qt_widgets_c_QBoxLayout_insertItem(self as *mut ::box_layout::BoxLayout, index, arg2)
  }

  /// C++ method: <span style='color: green;'>```QBoxLayout::insertLayout```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_layout(&mut self, (::libc::c_int, *mut ::layout::Layout)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBoxLayout::insertLayout(int index, QLayout* layout)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_layout(&mut self, (::libc::c_int, *mut ::layout::Layout, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBoxLayout::insertLayout(int index, QLayout* layout, int stretch = ?)```</span>
  ///
  ///
  pub unsafe fn insert_layout<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::BoxLayoutInsertLayoutArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QBoxLayout::insertSpacerItem(int index, QSpacerItem* spacerItem)```</span>
  ///
  ///
  pub unsafe fn insert_spacer_item(&mut self, index: ::libc::c_int, spacer_item: *mut ::spacer_item::SpacerItem) {
    ::ffi::qt_widgets_c_QBoxLayout_insertSpacerItem(self as *mut ::box_layout::BoxLayout, index, spacer_item)
  }

  /// C++ method: <span style='color: green;'>```void QBoxLayout::insertSpacing(int index, int size)```</span>
  ///
  ///
  pub fn insert_spacing(&mut self, index: ::libc::c_int, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_insertSpacing(self as *mut ::box_layout::BoxLayout, index, size) }
  }

  /// C++ method: <span style='color: green;'>```QBoxLayout::insertStretch```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_stretch(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBoxLayout::insertStretch(int index)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_stretch(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBoxLayout::insertStretch(int index, int stretch = ?)```</span>
  ///
  ///
  pub fn insert_stretch<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::BoxLayoutInsertStretchArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QBoxLayout::invalidate()```</span>
  ///
  ///
  pub fn invalidate(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_invalidate(self as *mut ::box_layout::BoxLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual QLayoutItem* QBoxLayout::itemAt(int arg1) const```</span>
  ///
  ///
  pub fn item_at(&self, arg1: ::libc::c_int) -> *mut ::layout_item::LayoutItem {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_itemAt(self as *const ::box_layout::BoxLayout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QBoxLayout::maximumSize() const```</span>
  ///
  ///
  pub fn maximum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QBoxLayout_maximumSize_to_output(self as *const ::box_layout::BoxLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QBoxLayout::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_metaObject(self as *const ::box_layout::BoxLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QBoxLayout::minimumHeightForWidth(int arg1) const```</span>
  ///
  ///
  pub fn minimum_height_for_width(&self, arg1: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_minimumHeightForWidth(self as *const ::box_layout::BoxLayout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QBoxLayout::minimumSize() const```</span>
  ///
  ///
  pub fn minimum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QBoxLayout_minimumSize_to_output(self as *const ::box_layout::BoxLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QBoxLayout::QBoxLayout(QBoxLayout::Direction arg1)```</span>
  ///
  ///
  pub fn new(arg1: ::box_layout::Direction) -> ::cpp_utils::CppBox<::box_layout::BoxLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QBoxLayout_new_arg1(arg1) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QBoxLayout::QBoxLayout(QBoxLayout::Direction arg1, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(arg1: ::box_layout::Direction,
                           parent: *mut ::widget::Widget)
                           -> ::cpp_utils::CppBox<::box_layout::BoxLayout> {
    let ffi_result = ::ffi::qt_widgets_c_QBoxLayout_new_arg1_parent(arg1, parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QBoxLayout::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QBoxLayout_qt_metacall(self as *mut ::box_layout::BoxLayout,
                                               arg1 as *const ::qt_core::meta_object::Call,
                                               arg2,
                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QBoxLayout::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QBoxLayout_qt_metacast(self as *mut ::box_layout::BoxLayout, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QBoxLayout::setDirection(QBoxLayout::Direction arg1)```</span>
  ///
  ///
  pub fn set_direction(&mut self, arg1: ::box_layout::Direction) {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_setDirection(self as *mut ::box_layout::BoxLayout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QBoxLayout::setGeometry(const QRect& arg1)```</span>
  ///
  ///
  pub fn set_geometry(&mut self, arg1: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_widgets_c_QBoxLayout_setGeometry(self as *mut ::box_layout::BoxLayout,
                                                 arg1 as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QBoxLayout::setSpacing(int spacing)```</span>
  ///
  ///
  pub fn set_spacing(&mut self, spacing: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_setSpacing(self as *mut ::box_layout::BoxLayout, spacing) }
  }

  /// C++ method: <span style='color: green;'>```void QBoxLayout::setStretch(int index, int stretch)```</span>
  ///
  ///
  pub fn set_stretch(&mut self, index: ::libc::c_int, stretch: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_setStretch(self as *mut ::box_layout::BoxLayout, index, stretch) }
  }

  /// C++ method: <span style='color: green;'>```QBoxLayout::setStretchFactor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_stretch_factor(&mut self, (*mut ::layout::Layout, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QBoxLayout::setStretchFactor(QLayout* l, int stretch)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_stretch_factor(&mut self, (*mut ::widget::Widget, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QBoxLayout::setStretchFactor(QWidget* w, int stretch)```</span>
  ///
  ///
  pub unsafe fn set_stretch_factor<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::BoxLayoutSetStretchFactorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QSize QBoxLayout::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QBoxLayout_sizeHint_to_output(self as *const ::box_layout::BoxLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QBoxLayout::spacing() const```</span>
  ///
  ///
  pub fn spacing(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_spacing(self as *const ::box_layout::BoxLayout) }
  }

  /// C++ method: <span style='color: green;'>```int QBoxLayout::stretch(int index) const```</span>
  ///
  ///
  pub fn stretch(&self, index: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_stretch(self as *const ::box_layout::BoxLayout, index) }
  }

  /// C++ method: <span style='color: green;'>```virtual QLayoutItem* QBoxLayout::takeAt(int arg1)```</span>
  ///
  ///
  pub fn take_at(&mut self, arg1: ::libc::c_int) -> *mut ::layout_item::LayoutItem {
    unsafe { ::ffi::qt_widgets_c_QBoxLayout_takeAt(self as *mut ::box_layout::BoxLayout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```static QString QBoxLayout::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QBoxLayout_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QBoxLayout::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QBoxLayout_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::box_layout::BoxLayout {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QBoxLayout_delete
  }
}

/// C++ type: <span style='color: green;'>```QBoxLayout::Direction```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Direction {
  /// C++ enum variant: <span style='color: green;'>```LeftToRight = 0```</span>
  LeftToRight = 0,
  /// C++ enum variant: <span style='color: green;'>```RightToLeft = 1```</span>
  RightToLeft = 1,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```TopToBottom = 2```</span>
  /// - <span style='color: green;'>```Down = 2```</span>
  ///
  TopToBottom = 2,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```BottomToTop = 3```</span>
  /// - <span style='color: green;'>```Up = 3```</span>
  ///
  BottomToTop = 3,
}

impl ::cpp_utils::DynamicCast<::box_layout::BoxLayout> for ::layout::Layout {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::box_layout::BoxLayout> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QBoxLayout_G_dynamic_cast_QBoxLayout_ptr_QLayout(self as *mut ::layout::Layout) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::box_layout::BoxLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QBoxLayout_G_dynamic_cast_QBoxLayout_ptr_QLayout(self as *const ::layout::Layout as *mut ::layout::Layout) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::box_layout::BoxLayout> for ::layout_item::LayoutItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::box_layout::BoxLayout> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QBoxLayout_G_dynamic_cast_QBoxLayout_ptr_QLayoutItem(self as *mut ::layout_item::LayoutItem)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::box_layout::BoxLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QBoxLayout_G_dynamic_cast_QBoxLayout_ptr_QLayoutItem(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::box_layout::BoxLayout {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QBoxLayout_G_static_cast_QObject_ptr(self as *mut ::box_layout::BoxLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QBoxLayout_G_static_cast_QObject_ptr(self as *const ::box_layout::BoxLayout as *mut ::box_layout::BoxLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::layout::Layout> for ::box_layout::BoxLayout {
  fn static_cast_mut(&mut self) -> &mut ::layout::Layout {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QBoxLayout_G_static_cast_QLayout_ptr(self as *mut ::box_layout::BoxLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout::Layout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QBoxLayout_G_static_cast_QLayout_ptr(self as *const ::box_layout::BoxLayout as *mut ::box_layout::BoxLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::layout_item::LayoutItem> for ::box_layout::BoxLayout {
  fn static_cast_mut(&mut self) -> &mut ::layout_item::LayoutItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QBoxLayout_G_static_cast_QLayoutItem_ptr(self as *mut ::box_layout::BoxLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout_item::LayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QBoxLayout_G_static_cast_QLayoutItem_ptr(self as *const ::box_layout::BoxLayout as *mut ::box_layout::BoxLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::box_layout::BoxLayout> for ::layout::Layout {
  unsafe fn static_cast_mut(&mut self) -> &mut ::box_layout::BoxLayout {
    let ffi_result =
      ::ffi::qt_widgets_c_QBoxLayout_G_static_cast_QBoxLayout_ptr_QLayout(self as *mut ::layout::Layout);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::box_layout::BoxLayout {
    let ffi_result = ::ffi::qt_widgets_c_QBoxLayout_G_static_cast_QBoxLayout_ptr_QLayout(self as *const ::layout::Layout as *mut ::layout::Layout);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::box_layout::BoxLayout> for ::layout_item::LayoutItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::box_layout::BoxLayout {
    let ffi_result =
      ::ffi::qt_widgets_c_QBoxLayout_G_static_cast_QBoxLayout_ptr_QLayoutItem(self as *mut ::layout_item::LayoutItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::box_layout::BoxLayout {
    let ffi_result = ::ffi::qt_widgets_c_QBoxLayout_G_static_cast_QBoxLayout_ptr_QLayoutItem(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::box_layout::BoxLayout> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::box_layout::BoxLayout {
    let ffi_result =
      ::ffi::qt_widgets_c_QBoxLayout_G_static_cast_QBoxLayout_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::box_layout::BoxLayout {
    let ffi_result = ::ffi::qt_widgets_c_QBoxLayout_G_static_cast_QBoxLayout_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::box_layout::BoxLayout {
  type Target = ::layout::Layout;
  fn deref(&self) -> &::layout::Layout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QBoxLayout_G_static_cast_QLayout_ptr(self as *const ::box_layout::BoxLayout as *mut ::box_layout::BoxLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::box_layout::BoxLayout {
  fn deref_mut(&mut self) -> &mut ::layout::Layout {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QBoxLayout_G_static_cast_QLayout_ptr(self as *mut ::box_layout::BoxLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [BoxLayout::add_layout](../struct.BoxLayout.html#method.add_layout) method.
  pub trait BoxLayoutAddLayoutArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> ();
  }
  impl<'largs> BoxLayoutAddLayoutArgs<'largs> for *mut ::layout::Layout {
    unsafe fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> () {
      let layout = self;
      ::ffi::qt_widgets_c_QBoxLayout_addLayout_layout(original_self as *mut ::box_layout::BoxLayout, layout)
    }
  }
  impl<'largs> BoxLayoutAddLayoutArgs<'largs> for (*mut ::layout::Layout, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> () {
      let layout = self.0;
      let stretch = self.1;
      ::ffi::qt_widgets_c_QBoxLayout_addLayout_layout_stretch(original_self as *mut ::box_layout::BoxLayout,
                                                              layout,
                                                              stretch)
    }
  }
  /// This trait represents a set of arguments accepted by [BoxLayout::add_stretch](../struct.BoxLayout.html#method.add_stretch) method.
  pub trait BoxLayoutAddStretchArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> ();
  }
  impl<'largs> BoxLayoutAddStretchArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> () {

      unsafe { ::ffi::qt_widgets_c_QBoxLayout_addStretch_no_args(original_self as *mut ::box_layout::BoxLayout) }
    }
  }
  impl<'largs> BoxLayoutAddStretchArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> () {
      let stretch = self;
      unsafe {
        ::ffi::qt_widgets_c_QBoxLayout_addStretch_stretch(original_self as *mut ::box_layout::BoxLayout, stretch)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [BoxLayout::insert_layout](../struct.BoxLayout.html#method.insert_layout) method.
  pub trait BoxLayoutInsertLayoutArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> ();
  }
  impl<'largs> BoxLayoutInsertLayoutArgs<'largs> for (::libc::c_int, *mut ::layout::Layout) {
    unsafe fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> () {
      let index = self.0;
      let layout = self.1;
      ::ffi::qt_widgets_c_QBoxLayout_insertLayout_index_layout(original_self as *mut ::box_layout::BoxLayout,
                                                               index,
                                                               layout)
    }
  }
  impl<'largs> BoxLayoutInsertLayoutArgs<'largs> for (::libc::c_int, *mut ::layout::Layout, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> () {
      let index = self.0;
      let layout = self.1;
      let stretch = self.2;
      ::ffi::qt_widgets_c_QBoxLayout_insertLayout_index_layout_stretch(original_self as *mut ::box_layout::BoxLayout,
                                                                       index,
                                                                       layout,
                                                                       stretch)
    }
  }
  /// This trait represents a set of arguments accepted by [BoxLayout::insert_stretch](../struct.BoxLayout.html#method.insert_stretch) method.
  pub trait BoxLayoutInsertStretchArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> ();
  }
  impl<'largs> BoxLayoutInsertStretchArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> () {
      let index = self;
      unsafe {
        ::ffi::qt_widgets_c_QBoxLayout_insertStretch_index(original_self as *mut ::box_layout::BoxLayout, index)
      }
    }
  }
  impl<'largs> BoxLayoutInsertStretchArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> () {
      let index = self.0;
      let stretch = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QBoxLayout_insertStretch_index_stretch(original_self as *mut ::box_layout::BoxLayout,
                                                                   index,
                                                                   stretch)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [BoxLayout::set_stretch_factor](../struct.BoxLayout.html#method.set_stretch_factor) method.
  pub trait BoxLayoutSetStretchFactorArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> bool;
  }
  impl<'largs> BoxLayoutSetStretchFactorArgs<'largs> for (*mut ::layout::Layout, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> bool {
      let l = self.0;
      let stretch = self.1;
      ::ffi::qt_widgets_c_QBoxLayout_setStretchFactor_l_stretch(original_self as *mut ::box_layout::BoxLayout,
                                                                l,
                                                                stretch)
    }
  }
  impl<'largs> BoxLayoutSetStretchFactorArgs<'largs> for (*mut ::widget::Widget, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::box_layout::BoxLayout) -> bool {
      let w = self.0;
      let stretch = self.1;
      ::ffi::qt_widgets_c_QBoxLayout_setStretchFactor_w_stretch(original_self as *mut ::box_layout::BoxLayout,
                                                                w,
                                                                stretch)
    }
  }
}
