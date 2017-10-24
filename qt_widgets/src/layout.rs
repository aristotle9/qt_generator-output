/// C++ type: <span style='color: green;'>```QLayout```</span>
#[repr(C)]
pub struct Layout(u8);

impl Layout {
  /// C++ method: <span style='color: green;'>```bool QLayout::activate()```</span>
  ///
  ///
  pub fn activate(&mut self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLayout_activate(self as *mut ::layout::Layout) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QLayout::addItem(QLayoutItem* arg1)```</span>
  ///
  ///
  pub unsafe fn add_item(&mut self, arg1: *mut ::layout_item::LayoutItem) {
    ::ffi::qt_widgets_c_QLayout_addItem(self as *mut ::layout::Layout, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QLayout::addWidget(QWidget* w)```</span>
  ///
  ///
  pub unsafe fn add_widget(&mut self, w: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QLayout_addWidget(self as *mut ::layout::Layout, w)
  }

  /// C++ method: <span style='color: green;'>```static QSize QLayout::closestAcceptableSize(const QWidget* w, const QSize& s)```</span>
  ///
  ///
  pub unsafe fn closest_acceptable_size(w: *const ::widget::Widget,
                                        s: &::qt_core::size::Size)
                                        -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QLayout_closestAcceptableSize_to_output(w, s as *const ::qt_core::size::Size, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMargins QLayout::contentsMargins() const```</span>
  ///
  ///
  pub fn contents_margins(&self) -> ::qt_core::margins::Margins {
    {
      let mut object: ::qt_core::margins::Margins =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLayout_contentsMargins_to_output(self as *const ::layout::Layout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QLayout::contentsRect() const```</span>
  ///
  ///
  pub fn contents_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLayout_contentsRect_to_output(self as *const ::layout::Layout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QLayout::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLayout_count(self as *const ::layout::Layout) }
  }

  /// C++ method: <span style='color: green;'>```virtual QRect QLayout::geometry() const```</span>
  ///
  ///
  pub fn geometry(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLayout_geometry_to_output(self as *const ::layout::Layout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QLayout::getContentsMargins(int* left, int* top, int* right, int* bottom) const```</span>
  ///
  ///
  pub unsafe fn get_contents_margins(&self,
                                     left: *mut ::libc::c_int,
                                     top: *mut ::libc::c_int,
                                     right: *mut ::libc::c_int,
                                     bottom: *mut ::libc::c_int) {
    ::ffi::qt_widgets_c_QLayout_getContentsMargins(self as *const ::layout::Layout, left, top, right, bottom)
  }

  /// C++ method: <span style='color: green;'>```virtual int QLayout::indexOf(QWidget* arg1) const```</span>
  ///
  ///
  pub unsafe fn index_of(&self, arg1: *mut ::widget::Widget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QLayout_indexOf(self as *const ::layout::Layout, arg1)
  }

  /// C++ method: <span style='color: green;'>```virtual void QLayout::invalidate()```</span>
  ///
  ///
  pub fn invalidate(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLayout_invalidate(self as *mut ::layout::Layout) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QLayout::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLayout_isEmpty(self as *const ::layout::Layout) }
  }

  /// C++ method: <span style='color: green;'>```bool QLayout::isEnabled() const```</span>
  ///
  ///
  pub fn is_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLayout_isEnabled(self as *const ::layout::Layout) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QLayoutItem* QLayout::itemAt(int index) const```</span>
  ///
  ///
  pub fn item_at(&self, index: ::libc::c_int) -> *mut ::layout_item::LayoutItem {
    unsafe { ::ffi::qt_widgets_c_QLayout_itemAt(self as *const ::layout::Layout, index) }
  }

  /// C++ method: <span style='color: green;'>```virtual QLayout* QLayout::layout()```</span>
  ///
  ///
  pub fn layout(&mut self) -> *mut ::layout::Layout {
    unsafe { ::ffi::qt_widgets_c_QLayout_layout(self as *mut ::layout::Layout) }
  }

  /// C++ method: <span style='color: green;'>```int QLayout::margin() const```</span>
  ///
  ///
  pub fn margin(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLayout_margin(self as *const ::layout::Layout) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QLayout::maximumSize() const```</span>
  ///
  ///
  pub fn maximum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLayout_maximumSize_to_output(self as *const ::layout::Layout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QLayout::menuBar() const```</span>
  ///
  ///
  pub fn menu_bar(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QLayout_menuBar(self as *const ::layout::Layout) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QLayout::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QLayout_metaObject(self as *const ::layout::Layout) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QLayout::minimumSize() const```</span>
  ///
  ///
  pub fn minimum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLayout_minimumSize_to_output(self as *const ::layout::Layout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QLayout::parentWidget() const```</span>
  ///
  ///
  pub fn parent_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QLayout_parentWidget(self as *const ::layout::Layout) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QLayout::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QLayout_qt_metacall(self as *mut ::layout::Layout,
                                            arg1 as *const ::qt_core::meta_object::Call,
                                            arg2,
                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QLayout::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QLayout_qt_metacast(self as *mut ::layout::Layout, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QLayout::removeItem(QLayoutItem* arg1)```</span>
  ///
  ///
  pub unsafe fn remove_item(&mut self, arg1: *mut ::layout_item::LayoutItem) {
    ::ffi::qt_widgets_c_QLayout_removeItem(self as *mut ::layout::Layout, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QLayout::removeWidget(QWidget* w)```</span>
  ///
  ///
  pub unsafe fn remove_widget(&mut self, w: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QLayout_removeWidget(self as *mut ::layout::Layout, w)
  }

  /// C++ method: <span style='color: green;'>```QLayout::setContentsMargins```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_contents_margins(&mut self, &::qt_core::margins::Margins) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLayout::setContentsMargins(const QMargins& margins)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_contents_margins(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLayout::setContentsMargins(int left, int top, int right, int bottom)```</span>
  ///
  ///
  pub fn set_contents_margins<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::LayoutSetContentsMarginsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QLayout::setEnabled(bool arg1)```</span>
  ///
  ///
  pub fn set_enabled(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QLayout_setEnabled(self as *mut ::layout::Layout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QLayout::setGeometry(const QRect& arg1)```</span>
  ///
  ///
  pub fn set_geometry(&mut self, arg1: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_widgets_c_QLayout_setGeometry(self as *mut ::layout::Layout,
                                              arg1 as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QLayout::setMargin(int arg1)```</span>
  ///
  ///
  pub fn set_margin(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QLayout_setMargin(self as *mut ::layout::Layout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QLayout::setMenuBar(QWidget* w)```</span>
  ///
  ///
  pub unsafe fn set_menu_bar(&mut self, w: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QLayout_setMenuBar(self as *mut ::layout::Layout, w)
  }

  /// C++ method: <span style='color: green;'>```void QLayout::setSizeConstraint(QLayout::SizeConstraint arg1)```</span>
  ///
  ///
  pub fn set_size_constraint(&mut self, arg1: ::layout::SizeConstraint) {
    unsafe { ::ffi::qt_widgets_c_QLayout_setSizeConstraint(self as *mut ::layout::Layout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QLayout::setSpacing(int arg1)```</span>
  ///
  ///
  pub fn set_spacing(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QLayout_setSpacing(self as *mut ::layout::Layout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```QLayout::SizeConstraint QLayout::sizeConstraint() const```</span>
  ///
  ///
  pub fn size_constraint(&self) -> ::layout::SizeConstraint {
    unsafe { ::ffi::qt_widgets_c_QLayout_sizeConstraint(self as *const ::layout::Layout) }
  }

  /// C++ method: <span style='color: green;'>```int QLayout::spacing() const```</span>
  ///
  ///
  pub fn spacing(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLayout_spacing(self as *const ::layout::Layout) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QLayoutItem* QLayout::takeAt(int index)```</span>
  ///
  ///
  pub fn take_at(&mut self, index: ::libc::c_int) -> *mut ::layout_item::LayoutItem {
    unsafe { ::ffi::qt_widgets_c_QLayout_takeAt(self as *mut ::layout::Layout, index) }
  }

  /// C++ method: <span style='color: green;'>```int QLayout::totalHeightForWidth(int w) const```</span>
  ///
  ///
  pub fn total_height_for_width(&self, w: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLayout_totalHeightForWidth(self as *const ::layout::Layout, w) }
  }

  /// C++ method: <span style='color: green;'>```QSize QLayout::totalMaximumSize() const```</span>
  ///
  ///
  pub fn total_maximum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLayout_totalMaximumSize_to_output(self as *const ::layout::Layout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QLayout::totalMinimumSize() const```</span>
  ///
  ///
  pub fn total_minimum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLayout_totalMinimumSize_to_output(self as *const ::layout::Layout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QLayout::totalSizeHint() const```</span>
  ///
  ///
  pub fn total_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLayout_totalSizeHint_to_output(self as *const ::layout::Layout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QLayout::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QLayout_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QLayout::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QLayout_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QLayout::update()```</span>
  ///
  ///
  pub fn update(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLayout_update(self as *mut ::layout::Layout) }
  }
}

impl ::cpp_utils::CppDeletable for ::layout::Layout {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QLayout_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Layout`.
  pub struct Signals<'a>(&'a ::layout::Layout);
  /// Represents a built-in Qt signal `QLayout::objectNameChanged`.
  ///
  /// An object of this type can be created from `Layout` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Layout` object.
  pub struct ObjectNameChanged<'a>(&'a ::layout::Layout);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QLayout::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::layout::Layout {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QLayout::SizeConstraint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SizeConstraint {
  /// C++ enum variant: <span style='color: green;'>```SetDefaultConstraint = 0```</span>
  DefaultConstraint = 0,
  /// C++ enum variant: <span style='color: green;'>```SetNoConstraint = 1```</span>
  NoConstraint = 1,
  /// C++ enum variant: <span style='color: green;'>```SetMinimumSize = 2```</span>
  MinimumSize = 2,
  /// C++ enum variant: <span style='color: green;'>```SetFixedSize = 3```</span>
  FixedSize = 3,
  /// C++ enum variant: <span style='color: green;'>```SetMaximumSize = 4```</span>
  MaximumSize = 4,
  /// C++ enum variant: <span style='color: green;'>```SetMinAndMaxSize = 5```</span>
  MinAndMaxSize = 5,
}

impl ::cpp_utils::DynamicCast<::layout::Layout> for ::layout_item::LayoutItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::layout::Layout> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLayout_G_dynamic_cast_QLayout_ptr(self as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::layout::Layout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLayout_G_dynamic_cast_QLayout_ptr(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::layout::Layout {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLayout_G_static_cast_QObject_ptr(self as *mut ::layout::Layout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QLayout_G_static_cast_QObject_ptr(self as *const ::layout::Layout as *mut ::layout::Layout)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::layout_item::LayoutItem> for ::layout::Layout {
  fn static_cast_mut(&mut self) -> &mut ::layout_item::LayoutItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLayout_G_static_cast_QLayoutItem_ptr(self as *mut ::layout::Layout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout_item::LayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLayout_G_static_cast_QLayoutItem_ptr(self as *const ::layout::Layout as *mut ::layout::Layout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::layout::Layout> for ::layout_item::LayoutItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::layout::Layout {
    let ffi_result =
      ::ffi::qt_widgets_c_QLayout_G_static_cast_QLayout_ptr_QLayoutItem(self as *mut ::layout_item::LayoutItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::layout::Layout {
    let ffi_result = ::ffi::qt_widgets_c_QLayout_G_static_cast_QLayout_ptr_QLayoutItem(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::layout::Layout> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::layout::Layout {
    let ffi_result =
      ::ffi::qt_widgets_c_QLayout_G_static_cast_QLayout_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::layout::Layout {
    let ffi_result = ::ffi::qt_widgets_c_QLayout_G_static_cast_QLayout_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Layout::set_contents_margins](../struct.Layout.html#method.set_contents_margins) method.
  pub trait LayoutSetContentsMarginsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::layout::Layout) -> ();
  }
  impl<'largs> LayoutSetContentsMarginsArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::layout::Layout) -> () {
      let left = self.0;
      let top = self.1;
      let right = self.2;
      let bottom = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QLayout_setContentsMargins_left_top_right_bottom(original_self as *mut ::layout::Layout,
                                                                             left,
                                                                             top,
                                                                             right,
                                                                             bottom)
      }
    }
  }
  impl<'largs> LayoutSetContentsMarginsArgs<'largs> for &'largs ::qt_core::margins::Margins {
    fn exec(self, original_self: &'largs mut ::layout::Layout) -> () {
      let margins = self;
      unsafe {
        ::ffi::qt_widgets_c_QLayout_setContentsMargins_margins(original_self as *mut ::layout::Layout,
                                                               margins as *const ::qt_core::margins::Margins)
      }
    }
  }
}
