/// C++ type: <span style='color: green;'>```QGraphicsLayoutItem```</span>
#[repr(C)]
pub struct GraphicsLayoutItem(u8);

impl GraphicsLayoutItem {
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsLayoutItem::contentsRect() const```</span>
  ///
  ///
  pub fn contents_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsLayoutItem_contentsRect_to_output(self as *const ::graphics_layout_item::GraphicsLayoutItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLayoutItem::effectiveSizeHint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn effective_size_hint(&self, &::qt_core::qt::SizeHint) -> ::qt_core::size_f::SizeF```<br>
  /// C++ method: <span style='color: green;'>```QSizeF QGraphicsLayoutItem::effectiveSizeHint(Qt::SizeHint which) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn effective_size_hint(&self, (&::qt_core::qt::SizeHint, &::qt_core::size_f::SizeF)) -> ::qt_core::size_f::SizeF```<br>
  /// C++ method: <span style='color: green;'>```QSizeF QGraphicsLayoutItem::effectiveSizeHint(Qt::SizeHint which, const QSizeF& constraint = ?) const```</span>
  ///
  ///
  pub fn effective_size_hint<'largs, Args>(&'largs self, args: Args) -> ::qt_core::size_f::SizeF
    where Args: overloading::GraphicsLayoutItemEffectiveSizeHintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsLayoutItem::geometry() const```</span>
  ///
  ///
  pub fn geometry(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsLayoutItem_geometry_to_output(self as *const ::graphics_layout_item::GraphicsLayoutItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsLayoutItem::getContentsMargins(double* left, double* top, double* right, double* bottom) const```</span>
  ///
  ///
  pub unsafe fn get_contents_margins(&self,
                                     left: *mut ::libc::c_double,
                                     top: *mut ::libc::c_double,
                                     right: *mut ::libc::c_double,
                                     bottom: *mut ::libc::c_double) {
    ::ffi::qt_widgets_c_QGraphicsLayoutItem_getContentsMargins(self as *const ::graphics_layout_item::GraphicsLayoutItem, left, top, right, bottom)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsLayoutItem::graphicsItem() const```</span>
  ///
  ///
  pub fn graphics_item(&self) -> *mut ::graphics_item::GraphicsItem {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_graphicsItem(self as *const ::graphics_layout_item::GraphicsLayoutItem)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsLayoutItem::isLayout() const```</span>
  ///
  ///
  pub fn is_layout(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_isLayout(self as *const ::graphics_layout_item::GraphicsLayoutItem)
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsLayoutItem::maximumHeight() const```</span>
  ///
  ///
  pub fn maximum_height(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_maximumHeight(self as *const ::graphics_layout_item::GraphicsLayoutItem)
    }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QGraphicsLayoutItem::maximumSize() const```</span>
  ///
  ///
  pub fn maximum_size(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsLayoutItem_maximumSize_to_output(self as *const ::graphics_layout_item::GraphicsLayoutItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsLayoutItem::maximumWidth() const```</span>
  ///
  ///
  pub fn maximum_width(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_maximumWidth(self as *const ::graphics_layout_item::GraphicsLayoutItem)
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsLayoutItem::minimumHeight() const```</span>
  ///
  ///
  pub fn minimum_height(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_minimumHeight(self as *const ::graphics_layout_item::GraphicsLayoutItem)
    }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QGraphicsLayoutItem::minimumSize() const```</span>
  ///
  ///
  pub fn minimum_size(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsLayoutItem_minimumSize_to_output(self as *const ::graphics_layout_item::GraphicsLayoutItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsLayoutItem::minimumWidth() const```</span>
  ///
  ///
  pub fn minimum_width(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_minimumWidth(self as *const ::graphics_layout_item::GraphicsLayoutItem)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsLayoutItem::ownedByLayout() const```</span>
  ///
  ///
  pub fn owned_by_layout(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_ownedByLayout(self as *const ::graphics_layout_item::GraphicsLayoutItem)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLayoutItem* QGraphicsLayoutItem::parentLayoutItem() const```</span>
  ///
  ///
  pub fn parent_layout_item(&self) -> *mut ::graphics_layout_item::GraphicsLayoutItem {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLayoutItem_parentLayoutItem(self as *const ::graphics_layout_item::GraphicsLayoutItem) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsLayoutItem::preferredHeight() const```</span>
  ///
  ///
  pub fn preferred_height(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_preferredHeight(self as *const ::graphics_layout_item::GraphicsLayoutItem)
    }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QGraphicsLayoutItem::preferredSize() const```</span>
  ///
  ///
  pub fn preferred_size(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsLayoutItem_preferredSize_to_output(self as *const ::graphics_layout_item::GraphicsLayoutItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsLayoutItem::preferredWidth() const```</span>
  ///
  ///
  pub fn preferred_width(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_preferredWidth(self as *const ::graphics_layout_item::GraphicsLayoutItem)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsLayoutItem::setGeometry(const QRectF& rect)```</span>
  ///
  ///
  pub fn set_geometry(&mut self, rect: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_setGeometry(self as *mut ::graphics_layout_item::GraphicsLayoutItem,
                                                          rect as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setMaximumHeight(double height)```</span>
  ///
  ///
  pub fn set_maximum_height(&mut self, height: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_setMaximumHeight(self as *mut ::graphics_layout_item::GraphicsLayoutItem, height)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLayoutItem::setMaximumSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_maximum_size(&mut self, &::qt_core::size_f::SizeF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setMaximumSize(const QSizeF& size)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_maximum_size(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setMaximumSize(double w, double h)```</span>
  ///
  ///
  pub fn set_maximum_size<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsLayoutItemSetMaximumSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setMaximumWidth(double width)```</span>
  ///
  ///
  pub fn set_maximum_width(&mut self, width: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_setMaximumWidth(self as *mut ::graphics_layout_item::GraphicsLayoutItem,
                                                              width)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setMinimumHeight(double height)```</span>
  ///
  ///
  pub fn set_minimum_height(&mut self, height: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_setMinimumHeight(self as *mut ::graphics_layout_item::GraphicsLayoutItem, height)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLayoutItem::setMinimumSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_minimum_size(&mut self, &::qt_core::size_f::SizeF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setMinimumSize(const QSizeF& size)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_minimum_size(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setMinimumSize(double w, double h)```</span>
  ///
  ///
  pub fn set_minimum_size<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsLayoutItemSetMinimumSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setMinimumWidth(double width)```</span>
  ///
  ///
  pub fn set_minimum_width(&mut self, width: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_setMinimumWidth(self as *mut ::graphics_layout_item::GraphicsLayoutItem,
                                                              width)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setParentLayoutItem(QGraphicsLayoutItem* parent)```</span>
  ///
  ///
  pub unsafe fn set_parent_layout_item(&mut self, parent: *mut ::graphics_layout_item::GraphicsLayoutItem) {
    ::ffi::qt_widgets_c_QGraphicsLayoutItem_setParentLayoutItem(self as *mut ::graphics_layout_item::GraphicsLayoutItem, parent)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setPreferredHeight(double height)```</span>
  ///
  ///
  pub fn set_preferred_height(&mut self, height: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLayoutItem_setPreferredHeight(self as *mut ::graphics_layout_item::GraphicsLayoutItem, height) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLayoutItem::setPreferredSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_preferred_size(&mut self, &::qt_core::size_f::SizeF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setPreferredSize(const QSizeF& size)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_preferred_size(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setPreferredSize(double w, double h)```</span>
  ///
  ///
  pub fn set_preferred_size<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsLayoutItemSetPreferredSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setPreferredWidth(double width)```</span>
  ///
  ///
  pub fn set_preferred_width(&mut self, width: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_setPreferredWidth(self as *mut ::graphics_layout_item::GraphicsLayoutItem, width)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLayoutItem::setSizePolicy```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_size_policy(&mut self, (&::size_policy::Policy, &::size_policy::Policy)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setSizePolicy(QSizePolicy::Policy hPolicy, QSizePolicy::Policy vPolicy)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_size_policy(&mut self, (&::size_policy::Policy, &::size_policy::Policy, &::size_policy::ControlType)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setSizePolicy(QSizePolicy::Policy hPolicy, QSizePolicy::Policy vPolicy, QSizePolicy::ControlType controlType = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_size_policy(&mut self, &::size_policy::SizePolicy) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLayoutItem::setSizePolicy(const QSizePolicy& policy)```</span>
  ///
  ///
  pub fn set_size_policy<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsLayoutItemSetSizePolicyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSizePolicy QGraphicsLayoutItem::sizePolicy() const```</span>
  ///
  ///
  pub fn size_policy(&self) -> ::size_policy::SizePolicy {
    {
      let mut object: ::size_policy::SizePolicy =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsLayoutItem_sizePolicy_to_output(self as *const ::graphics_layout_item::GraphicsLayoutItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsLayoutItem::updateGeometry()```</span>
  ///
  ///
  pub fn update_geometry(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayoutItem_updateGeometry(self as *mut ::graphics_layout_item::GraphicsLayoutItem)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_layout_item::GraphicsLayoutItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsLayoutItem_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsLayoutItem::effective_size_hint](../struct.GraphicsLayoutItem.html#method.effective_size_hint) method.
  pub trait GraphicsLayoutItemEffectiveSizeHintArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_layout_item::GraphicsLayoutItem) -> ::qt_core::size_f::SizeF;
  }
  impl<'largs> GraphicsLayoutItemEffectiveSizeHintArgs<'largs> for &'largs ::qt_core::qt::SizeHint {
    fn exec(self, original_self: &'largs ::graphics_layout_item::GraphicsLayoutItem) -> ::qt_core::size_f::SizeF {
      let which = self;
      {
        let mut object: ::qt_core::size_f::SizeF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsLayoutItem_effectiveSizeHint_to_output_which(original_self as *const ::graphics_layout_item::GraphicsLayoutItem, which as *const ::qt_core::qt::SizeHint, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsLayoutItemEffectiveSizeHintArgs<'largs>
    for (&'largs ::qt_core::qt::SizeHint, &'largs ::qt_core::size_f::SizeF) {
    fn exec(self, original_self: &'largs ::graphics_layout_item::GraphicsLayoutItem) -> ::qt_core::size_f::SizeF {
      let which = self.0;
      let constraint = self.1;
      {
        let mut object: ::qt_core::size_f::SizeF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsLayoutItem_effectiveSizeHint_to_output_which_constraint(original_self as *const ::graphics_layout_item::GraphicsLayoutItem, which as *const ::qt_core::qt::SizeHint, constraint as *const ::qt_core::size_f::SizeF, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsLayoutItem::set_maximum_size](../struct.GraphicsLayoutItem.html#method.set_maximum_size) method.
  pub trait GraphicsLayoutItemSetMaximumSizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_layout_item::GraphicsLayoutItem) -> ();
  }
  impl<'largs> GraphicsLayoutItemSetMaximumSizeArgs<'largs> for &'largs ::qt_core::size_f::SizeF {
    fn exec(self, original_self: &'largs mut ::graphics_layout_item::GraphicsLayoutItem) -> () {
      let size = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsLayoutItem_setMaximumSize_size(original_self as *mut ::graphics_layout_item::GraphicsLayoutItem, size as *const ::qt_core::size_f::SizeF) }
    }
  }
  impl<'largs> GraphicsLayoutItemSetMaximumSizeArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_layout_item::GraphicsLayoutItem) -> () {
      let w = self.0;
      let h = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsLayoutItem_setMaximumSize_w_h(original_self as *mut ::graphics_layout_item::GraphicsLayoutItem, w, h) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsLayoutItem::set_minimum_size](../struct.GraphicsLayoutItem.html#method.set_minimum_size) method.
  pub trait GraphicsLayoutItemSetMinimumSizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_layout_item::GraphicsLayoutItem) -> ();
  }
  impl<'largs> GraphicsLayoutItemSetMinimumSizeArgs<'largs> for &'largs ::qt_core::size_f::SizeF {
    fn exec(self, original_self: &'largs mut ::graphics_layout_item::GraphicsLayoutItem) -> () {
      let size = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsLayoutItem_setMinimumSize_size(original_self as *mut ::graphics_layout_item::GraphicsLayoutItem, size as *const ::qt_core::size_f::SizeF) }
    }
  }
  impl<'largs> GraphicsLayoutItemSetMinimumSizeArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_layout_item::GraphicsLayoutItem) -> () {
      let w = self.0;
      let h = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsLayoutItem_setMinimumSize_w_h(original_self as *mut ::graphics_layout_item::GraphicsLayoutItem, w, h) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsLayoutItem::set_preferred_size](../struct.GraphicsLayoutItem.html#method.set_preferred_size) method.
  pub trait GraphicsLayoutItemSetPreferredSizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_layout_item::GraphicsLayoutItem) -> ();
  }
  impl<'largs> GraphicsLayoutItemSetPreferredSizeArgs<'largs> for &'largs ::qt_core::size_f::SizeF {
    fn exec(self, original_self: &'largs mut ::graphics_layout_item::GraphicsLayoutItem) -> () {
      let size = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsLayoutItem_setPreferredSize_size(original_self as *mut ::graphics_layout_item::GraphicsLayoutItem, size as *const ::qt_core::size_f::SizeF) }
    }
  }
  impl<'largs> GraphicsLayoutItemSetPreferredSizeArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_layout_item::GraphicsLayoutItem) -> () {
      let w = self.0;
      let h = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsLayoutItem_setPreferredSize_w_h(original_self as *mut ::graphics_layout_item::GraphicsLayoutItem, w, h) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsLayoutItem::set_size_policy](../struct.GraphicsLayoutItem.html#method.set_size_policy) method.
  pub trait GraphicsLayoutItemSetSizePolicyArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_layout_item::GraphicsLayoutItem) -> ();
  }
  impl<'largs> GraphicsLayoutItemSetSizePolicyArgs<'largs>
    for (&'largs ::size_policy::Policy, &'largs ::size_policy::Policy) {
    fn exec(self, original_self: &'largs mut ::graphics_layout_item::GraphicsLayoutItem) -> () {
      let h_policy = self.0;
      let v_policy = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsLayoutItem_setSizePolicy_hPolicy_vPolicy(original_self as *mut ::graphics_layout_item::GraphicsLayoutItem, h_policy as *const ::size_policy::Policy, v_policy as *const ::size_policy::Policy) }
    }
  }
  impl<'largs> GraphicsLayoutItemSetSizePolicyArgs<'largs>
    for (&'largs ::size_policy::Policy, &'largs ::size_policy::Policy, &'largs ::size_policy::ControlType) {
    fn exec(self, original_self: &'largs mut ::graphics_layout_item::GraphicsLayoutItem) -> () {
      let h_policy = self.0;
      let v_policy = self.1;
      let control_type = self.2;
      unsafe { ::ffi::qt_widgets_c_QGraphicsLayoutItem_setSizePolicy_hPolicy_vPolicy_controlType(original_self as *mut ::graphics_layout_item::GraphicsLayoutItem, h_policy as *const ::size_policy::Policy, v_policy as *const ::size_policy::Policy, control_type as *const ::size_policy::ControlType) }
    }
  }
  impl<'largs> GraphicsLayoutItemSetSizePolicyArgs<'largs> for &'largs ::size_policy::SizePolicy {
    fn exec(self, original_self: &'largs mut ::graphics_layout_item::GraphicsLayoutItem) -> () {
      let policy = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsLayoutItem_setSizePolicy_policy(original_self as *mut ::graphics_layout_item::GraphicsLayoutItem, policy as *const ::size_policy::SizePolicy) }
    }
  }
}
