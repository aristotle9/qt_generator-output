/// C++ type: <span style='color: green;'>```QGraphicsLinearLayout```</span>
#[repr(C)]
pub struct GraphicsLinearLayout(u8);

impl GraphicsLinearLayout {
  /// C++ method: <span style='color: green;'>```void QGraphicsLinearLayout::addItem(QGraphicsLayoutItem* item)```</span>
  ///
  ///
  pub unsafe fn add_item(&mut self, item: *mut ::graphics_layout_item::GraphicsLayoutItem) {
    ::ffi::qt_widgets_c_QGraphicsLinearLayout_addItem(self as *mut ::graphics_linear_layout::GraphicsLinearLayout,
                                                      item)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLinearLayout::addStretch```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_stretch(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLinearLayout::addStretch()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_stretch(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLinearLayout::addStretch(int stretch = ?)```</span>
  ///
  ///
  pub fn add_stretch<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsLinearLayoutAddStretchArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual int QGraphicsLinearLayout::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLinearLayout_count(self as *const ::graphics_linear_layout::GraphicsLinearLayout)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLinearLayout::dump```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn dump(&self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLinearLayout::dump() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn dump(&self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLinearLayout::dump(int indent = ?) const```</span>
  ///
  ///
  pub fn dump<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::GraphicsLinearLayoutDumpArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsLinearLayout::insertItem(int index, QGraphicsLayoutItem* item)```</span>
  ///
  ///
  pub unsafe fn insert_item(&mut self, index: ::libc::c_int, item: *mut ::graphics_layout_item::GraphicsLayoutItem) {
    ::ffi::qt_widgets_c_QGraphicsLinearLayout_insertItem(self as *mut ::graphics_linear_layout::GraphicsLinearLayout,
                                                         index,
                                                         item)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLinearLayout::insertStretch```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_stretch(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLinearLayout::insertStretch(int index)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_stretch(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLinearLayout::insertStretch(int index, int stretch = ?)```</span>
  ///
  ///
  pub fn insert_stretch<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsLinearLayoutInsertStretchArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsLinearLayout::invalidate()```</span>
  ///
  ///
  pub fn invalidate(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLinearLayout_invalidate(self as *mut ::graphics_linear_layout::GraphicsLinearLayout)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QGraphicsLayoutItem* QGraphicsLinearLayout::itemAt(int index) const```</span>
  ///
  ///
  pub fn item_at(&self, index: ::libc::c_int) -> *mut ::graphics_layout_item::GraphicsLayoutItem {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLinearLayout_itemAt(self as *const ::graphics_linear_layout::GraphicsLinearLayout,
                                                       index)
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsLinearLayout::itemSpacing(int index) const```</span>
  ///
  ///
  pub fn item_spacing(&self, index: ::libc::c_int) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_itemSpacing(self as *const ::graphics_linear_layout::GraphicsLinearLayout, index) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLinearLayout::QGraphicsLinearLayout```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_linear_layout::GraphicsLinearLayout>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsLinearLayout::QGraphicsLinearLayout()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::qt::Orientation) -> ::cpp_utils::CppBox<::graphics_linear_layout::GraphicsLinearLayout>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsLinearLayout::QGraphicsLinearLayout(Qt::Orientation orientation)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_linear_layout::GraphicsLinearLayout>
    where Args: overloading::GraphicsLinearLayoutNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QGraphicsLinearLayout::QGraphicsLinearLayout```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::graphics_layout_item::GraphicsLayoutItem) -> ::cpp_utils::CppBox<::graphics_linear_layout::GraphicsLinearLayout>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsLinearLayout::QGraphicsLinearLayout(QGraphicsLayoutItem* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::qt::Orientation, *mut ::graphics_layout_item::GraphicsLayoutItem)) -> ::cpp_utils::CppBox<::graphics_linear_layout::GraphicsLinearLayout>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsLinearLayout::QGraphicsLinearLayout(Qt::Orientation orientation, QGraphicsLayoutItem* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_linear_layout::GraphicsLinearLayout>
    where Args: overloading::GraphicsLinearLayoutNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsLinearLayout::removeAt(int index)```</span>
  ///
  ///
  pub fn remove_at(&mut self, index: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLinearLayout_removeAt(self as *mut ::graphics_linear_layout::GraphicsLinearLayout,
                                                         index)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsLinearLayout::removeItem(QGraphicsLayoutItem* item)```</span>
  ///
  ///
  pub unsafe fn remove_item(&mut self, item: *mut ::graphics_layout_item::GraphicsLayoutItem) {
    ::ffi::qt_widgets_c_QGraphicsLinearLayout_removeItem(self as *mut ::graphics_linear_layout::GraphicsLinearLayout,
                                                         item)
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsLinearLayout::setGeometry(const QRectF& rect)```</span>
  ///
  ///
  pub fn set_geometry(&mut self, rect: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLinearLayout_setGeometry(self as *mut ::graphics_linear_layout::GraphicsLinearLayout, rect as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsLinearLayout::setItemSpacing(int index, double spacing)```</span>
  ///
  ///
  pub fn set_item_spacing(&mut self, index: ::libc::c_int, spacing: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_setItemSpacing(self as *mut ::graphics_linear_layout::GraphicsLinearLayout, index, spacing) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsLinearLayout::setOrientation(Qt::Orientation orientation)```</span>
  ///
  ///
  pub fn set_orientation(&mut self, orientation: &::qt_core::qt::Orientation) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_setOrientation(self as *mut ::graphics_linear_layout::GraphicsLinearLayout, orientation as *const ::qt_core::qt::Orientation) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsLinearLayout::setSpacing(double spacing)```</span>
  ///
  ///
  pub fn set_spacing(&mut self, spacing: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLinearLayout_setSpacing(self as *mut ::graphics_linear_layout::GraphicsLinearLayout, spacing)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsLinearLayout::setStretchFactor(QGraphicsLayoutItem* item, int stretch)```</span>
  ///
  ///
  pub unsafe fn set_stretch_factor(&mut self,
                                   item: *mut ::graphics_layout_item::GraphicsLayoutItem,
                                   stretch: ::libc::c_int) {
    ::ffi::qt_widgets_c_QGraphicsLinearLayout_setStretchFactor(self as *mut ::graphics_linear_layout::GraphicsLinearLayout, item, stretch)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLinearLayout::sizeHint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn size_hint(&self, &::qt_core::qt::SizeHint) -> ::qt_core::size_f::SizeF```<br>
  /// C++ method: <span style='color: green;'>```virtual QSizeF QGraphicsLinearLayout::sizeHint(Qt::SizeHint which) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn size_hint(&self, (&::qt_core::qt::SizeHint, &::qt_core::size_f::SizeF)) -> ::qt_core::size_f::SizeF```<br>
  /// C++ method: <span style='color: green;'>```virtual QSizeF QGraphicsLinearLayout::sizeHint(Qt::SizeHint which, const QSizeF& constraint = ?) const```</span>
  ///
  ///
  pub fn size_hint<'largs, Args>(&'largs self, args: Args) -> ::qt_core::size_f::SizeF
    where Args: overloading::GraphicsLinearLayoutSizeHintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double QGraphicsLinearLayout::spacing() const```</span>
  ///
  ///
  pub fn spacing(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLinearLayout_spacing(self as *const ::graphics_linear_layout::GraphicsLinearLayout)
    }
  }

  /// C++ method: <span style='color: green;'>```int QGraphicsLinearLayout::stretchFactor(QGraphicsLayoutItem* item) const```</span>
  ///
  ///
  pub unsafe fn stretch_factor(&self, item: *mut ::graphics_layout_item::GraphicsLayoutItem) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsLinearLayout_stretchFactor(self as *const ::graphics_linear_layout::GraphicsLinearLayout, item)
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_linear_layout::GraphicsLinearLayout {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsLinearLayout_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_linear_layout::GraphicsLinearLayout> for ::graphics_layout::GraphicsLayout {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_linear_layout::GraphicsLinearLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_G_dynamic_cast_QGraphicsLinearLayout_ptr_QGraphicsLayout(self as *mut ::graphics_layout::GraphicsLayout) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_linear_layout::GraphicsLinearLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_G_dynamic_cast_QGraphicsLinearLayout_ptr_QGraphicsLayout(self as *const ::graphics_layout::GraphicsLayout as *mut ::graphics_layout::GraphicsLayout) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::graphics_linear_layout::GraphicsLinearLayout> for ::graphics_layout_item::GraphicsLayoutItem {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_linear_layout::GraphicsLinearLayout> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_G_dynamic_cast_QGraphicsLinearLayout_ptr_QGraphicsLayoutItem(self as *mut ::graphics_layout_item::GraphicsLayoutItem) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_linear_layout::GraphicsLinearLayout> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_G_dynamic_cast_QGraphicsLinearLayout_ptr_QGraphicsLayoutItem(self as *const ::graphics_layout_item::GraphicsLayoutItem as *mut ::graphics_layout_item::GraphicsLayoutItem) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::graphics_layout::GraphicsLayout> for ::graphics_linear_layout::GraphicsLinearLayout {
  fn static_cast_mut(&mut self) -> &mut ::graphics_layout::GraphicsLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_G_static_cast_QGraphicsLayout_ptr(self as *mut ::graphics_linear_layout::GraphicsLinearLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_layout::GraphicsLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_G_static_cast_QGraphicsLayout_ptr(self as *const ::graphics_linear_layout::GraphicsLinearLayout as *mut ::graphics_linear_layout::GraphicsLinearLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_layout_item::GraphicsLayoutItem> for ::graphics_linear_layout::GraphicsLinearLayout {
fn static_cast_mut(&mut self) -> &mut ::graphics_layout_item::GraphicsLayoutItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_G_static_cast_QGraphicsLayoutItem_ptr(self as *mut ::graphics_linear_layout::GraphicsLinearLayout) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::graphics_layout_item::GraphicsLayoutItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_G_static_cast_QGraphicsLayoutItem_ptr(self as *const ::graphics_linear_layout::GraphicsLinearLayout as *mut ::graphics_linear_layout::GraphicsLinearLayout) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_linear_layout::GraphicsLinearLayout> for ::graphics_layout::GraphicsLayout {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_linear_layout::GraphicsLinearLayout {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsLinearLayout_G_static_cast_QGraphicsLinearLayout_ptr_QGraphicsLayout(self as *mut ::graphics_layout::GraphicsLayout);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_linear_layout::GraphicsLinearLayout {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsLinearLayout_G_static_cast_QGraphicsLinearLayout_ptr_QGraphicsLayout(self as *const ::graphics_layout::GraphicsLayout as *mut ::graphics_layout::GraphicsLayout);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_linear_layout::GraphicsLinearLayout> for ::graphics_layout_item::GraphicsLayoutItem {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_linear_layout::GraphicsLinearLayout {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsLinearLayout_G_static_cast_QGraphicsLinearLayout_ptr_QGraphicsLayoutItem(self as *mut ::graphics_layout_item::GraphicsLayoutItem);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_linear_layout::GraphicsLinearLayout {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsLinearLayout_G_static_cast_QGraphicsLinearLayout_ptr_QGraphicsLayoutItem(self as *const ::graphics_layout_item::GraphicsLayoutItem as *mut ::graphics_layout_item::GraphicsLayoutItem);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::graphics_linear_layout::GraphicsLinearLayout {
  type Target = ::graphics_layout::GraphicsLayout;
  fn deref(&self) -> &::graphics_layout::GraphicsLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_G_static_cast_QGraphicsLayout_ptr(self as *const ::graphics_linear_layout::GraphicsLinearLayout as *mut ::graphics_linear_layout::GraphicsLinearLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_linear_layout::GraphicsLinearLayout {
  fn deref_mut(&mut self) -> &mut ::graphics_layout::GraphicsLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_G_static_cast_QGraphicsLayout_ptr(self as *mut ::graphics_linear_layout::GraphicsLinearLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsLinearLayout::add_stretch](../struct.GraphicsLinearLayout.html#method.add_stretch) method.
  pub trait GraphicsLinearLayoutAddStretchArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_linear_layout::GraphicsLinearLayout) -> ();
  }
  impl<'largs> GraphicsLinearLayoutAddStretchArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::graphics_linear_layout::GraphicsLinearLayout) -> () {

      unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_addStretch_no_args(original_self as *mut ::graphics_linear_layout::GraphicsLinearLayout) }
    }
  }
  impl<'largs> GraphicsLinearLayoutAddStretchArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::graphics_linear_layout::GraphicsLinearLayout) -> () {
      let stretch = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_addStretch_stretch(original_self as *mut ::graphics_linear_layout::GraphicsLinearLayout, stretch) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsLinearLayout::dump](../struct.GraphicsLinearLayout.html#method.dump) method.
  pub trait GraphicsLinearLayoutDumpArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_linear_layout::GraphicsLinearLayout) -> ();
  }
  impl<'largs> GraphicsLinearLayoutDumpArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::graphics_linear_layout::GraphicsLinearLayout) -> () {
      let indent = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_dump_indent(original_self as *const ::graphics_linear_layout::GraphicsLinearLayout, indent) }
    }
  }
  impl<'largs> GraphicsLinearLayoutDumpArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::graphics_linear_layout::GraphicsLinearLayout) -> () {

      unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_dump_no_args(original_self as *const ::graphics_linear_layout::GraphicsLinearLayout) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsLinearLayout::insert_stretch](../struct.GraphicsLinearLayout.html#method.insert_stretch) method.
  pub trait GraphicsLinearLayoutInsertStretchArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_linear_layout::GraphicsLinearLayout) -> ();
  }
  impl<'largs> GraphicsLinearLayoutInsertStretchArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::graphics_linear_layout::GraphicsLinearLayout) -> () {
      let index = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_insertStretch_index(original_self as *mut ::graphics_linear_layout::GraphicsLinearLayout, index) }
    }
  }
  impl<'largs> GraphicsLinearLayoutInsertStretchArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::graphics_linear_layout::GraphicsLinearLayout) -> () {
      let index = self.0;
      let stretch = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_insertStretch_index_stretch(original_self as *mut ::graphics_linear_layout::GraphicsLinearLayout, index, stretch) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsLinearLayout::new](../struct.GraphicsLinearLayout.html#method.new) method.
  pub trait GraphicsLinearLayoutNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_linear_layout::GraphicsLinearLayout>;
  }
  impl GraphicsLinearLayoutNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_linear_layout::GraphicsLinearLayout> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLinearLayout_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> GraphicsLinearLayoutNewArgs for &'a ::qt_core::qt::Orientation {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_linear_layout::GraphicsLinearLayout> {
      let orientation = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsLinearLayout_new_orientation(orientation as *const ::qt_core::qt::Orientation)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsLinearLayout::new_unsafe](../struct.GraphicsLinearLayout.html#method.new_unsafe) method.
  pub trait GraphicsLinearLayoutNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_linear_layout::GraphicsLinearLayout>;
  }
  impl<'a> GraphicsLinearLayoutNewUnsafeArgs
    for (&'a ::qt_core::qt::Orientation, *mut ::graphics_layout_item::GraphicsLayoutItem) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_linear_layout::GraphicsLinearLayout> {
      let orientation = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsLinearLayout_new_orientation_parent(orientation as *const ::qt_core::qt::Orientation, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl GraphicsLinearLayoutNewUnsafeArgs for *mut ::graphics_layout_item::GraphicsLayoutItem {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_linear_layout::GraphicsLinearLayout> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsLinearLayout_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsLinearLayout::size_hint](../struct.GraphicsLinearLayout.html#method.size_hint) method.
  pub trait GraphicsLinearLayoutSizeHintArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_linear_layout::GraphicsLinearLayout) -> ::qt_core::size_f::SizeF;
  }
  impl<'largs> GraphicsLinearLayoutSizeHintArgs<'largs> for &'largs ::qt_core::qt::SizeHint {
    fn exec(self, original_self: &'largs ::graphics_linear_layout::GraphicsLinearLayout) -> ::qt_core::size_f::SizeF {
      let which = self;
      {
        let mut object: ::qt_core::size_f::SizeF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsLinearLayout_sizeHint_to_output_which(original_self as *const ::graphics_linear_layout::GraphicsLinearLayout, which as *const ::qt_core::qt::SizeHint, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsLinearLayoutSizeHintArgs<'largs>
    for (&'largs ::qt_core::qt::SizeHint, &'largs ::qt_core::size_f::SizeF) {
    fn exec(self, original_self: &'largs ::graphics_linear_layout::GraphicsLinearLayout) -> ::qt_core::size_f::SizeF {
      let which = self.0;
      let constraint = self.1;
      {
        let mut object: ::qt_core::size_f::SizeF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsLinearLayout_sizeHint_to_output_which_constraint(original_self as *const ::graphics_linear_layout::GraphicsLinearLayout, which as *const ::qt_core::qt::SizeHint, constraint as *const ::qt_core::size_f::SizeF, &mut object);
        }
        object
      }
    }
  }
}
