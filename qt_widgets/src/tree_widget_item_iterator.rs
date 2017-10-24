/// C++ type: <span style='color: green;'>```QTreeWidgetItemIterator::IteratorFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum IteratorFlag {
  /// C++ enum variant: <span style='color: green;'>```All = 0```</span>
  All = 0,
  /// C++ enum variant: <span style='color: green;'>```Hidden = 1```</span>
  Hidden = 1,
  /// C++ enum variant: <span style='color: green;'>```NotHidden = 2```</span>
  NotHidden = 2,
  /// C++ enum variant: <span style='color: green;'>```Selected = 4```</span>
  Selected = 4,
  /// C++ enum variant: <span style='color: green;'>```Unselected = 8```</span>
  Unselected = 8,
  /// C++ enum variant: <span style='color: green;'>```Selectable = 16```</span>
  Selectable = 16,
  /// C++ enum variant: <span style='color: green;'>```NotSelectable = 32```</span>
  NotSelectable = 32,
  /// C++ enum variant: <span style='color: green;'>```DragEnabled = 64```</span>
  DragEnabled = 64,
  /// C++ enum variant: <span style='color: green;'>```DragDisabled = 128```</span>
  DragDisabled = 128,
  /// C++ enum variant: <span style='color: green;'>```DropEnabled = 256```</span>
  DropEnabled = 256,
  /// C++ enum variant: <span style='color: green;'>```DropDisabled = 512```</span>
  DropDisabled = 512,
  /// C++ enum variant: <span style='color: green;'>```HasChildren = 1024```</span>
  HasChildren = 1024,
  /// C++ enum variant: <span style='color: green;'>```NoChildren = 2048```</span>
  NoChildren = 2048,
  /// C++ enum variant: <span style='color: green;'>```Checked = 4096```</span>
  Checked = 4096,
  /// C++ enum variant: <span style='color: green;'>```NotChecked = 8192```</span>
  NotChecked = 8192,
  /// C++ enum variant: <span style='color: green;'>```Enabled = 16384```</span>
  Enabled = 16384,
  /// C++ enum variant: <span style='color: green;'>```Disabled = 32768```</span>
  Disabled = 32768,
  /// C++ enum variant: <span style='color: green;'>```Editable = 65536```</span>
  Editable = 65536,
  /// C++ enum variant: <span style='color: green;'>```NotEditable = 131072```</span>
  NotEditable = 131072,
  /// C++ enum variant: <span style='color: green;'>```UserFlag = 16777216```</span>
  UserFlag = 16777216,
}

impl ::qt_core::flags::FlaggableEnum for IteratorFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "IteratorFlag"
  }
}

/// C++ type: <span style='color: green;'>```QTreeWidgetItemIterator```</span>
#[repr(C)]
pub struct TreeWidgetItemIterator([u8; ::type_sizes::QT_WIDGETS_TREE_WIDGET_ITEM_ITERATOR_TREE_WIDGET_ITEM_ITERATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TreeWidgetItemIterator {
  unsafe fn new_uninitialized() -> TreeWidgetItemIterator {
    TreeWidgetItemIterator(::std::mem::uninitialized())
  }
}

impl TreeWidgetItemIterator {
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItemIterator::QTreeWidgetItemIterator(const QTreeWidgetItemIterator& it)```</span>
  ///
  ///
  pub fn new(it: &::tree_widget_item_iterator::TreeWidgetItemIterator)
             -> ::tree_widget_item_iterator::TreeWidgetItemIterator {
    {
      let mut object: ::tree_widget_item_iterator::TreeWidgetItemIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItemIterator_constructor_it(it as *const ::tree_widget_item_iterator::TreeWidgetItemIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItemIterator::QTreeWidgetItemIterator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::tree_widget::TreeWidget) -> ::tree_widget_item_iterator::TreeWidgetItemIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItemIterator::QTreeWidgetItemIterator(QTreeWidget* widget)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::tree_widget::TreeWidget, ::qt_core::flags::Flags<::tree_widget_item_iterator::IteratorFlag>)) -> ::tree_widget_item_iterator::TreeWidgetItemIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItemIterator::QTreeWidgetItemIterator(QTreeWidget* widget, QFlags<QTreeWidgetItemIterator::IteratorFlag> flags = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::tree_widget_item::TreeWidgetItem) -> ::tree_widget_item_iterator::TreeWidgetItemIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItemIterator::QTreeWidgetItemIterator(QTreeWidgetItem* item)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::tree_widget_item::TreeWidgetItem, ::qt_core::flags::Flags<::tree_widget_item_iterator::IteratorFlag>)) -> ::tree_widget_item_iterator::TreeWidgetItemIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidgetItemIterator::QTreeWidgetItemIterator(QTreeWidgetItem* item, QFlags<QTreeWidgetItemIterator::IteratorFlag> flags = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::tree_widget_item_iterator::TreeWidgetItemIterator
    where Args: overloading::TreeWidgetItemIteratorNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTreeWidgetItemIterator& QTreeWidgetItemIterator::operator+=(int n)```</span>
  ///
  ///
  pub fn op_add_assign<'l0>(&'l0 mut self,
                            n: ::libc::c_int)
                            -> &'l0 mut ::tree_widget_item_iterator::TreeWidgetItemIterator {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidgetItemIterator_operator_add_assign(self as *mut ::tree_widget_item_iterator::TreeWidgetItemIterator, n) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItemIterator& QTreeWidgetItemIterator::operator=(const QTreeWidgetItemIterator& it)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             it: &'l1 ::tree_widget_item_iterator::TreeWidgetItemIterator)
                             -> &'l0 mut ::tree_widget_item_iterator::TreeWidgetItemIterator {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidgetItemIterator_operator_assign(self as *mut ::tree_widget_item_iterator::TreeWidgetItemIterator, it as *const ::tree_widget_item_iterator::TreeWidgetItemIterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItemIterator& QTreeWidgetItemIterator::operator--()```</span>
  ///
  ///
  pub fn op_dec<'l0>(&'l0 mut self) -> &'l0 mut ::tree_widget_item_iterator::TreeWidgetItemIterator {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidgetItemIterator_operator_dec(self as *mut ::tree_widget_item_iterator::TreeWidgetItemIterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTreeWidgetItemIterator QTreeWidgetItemIterator::operator--(int arg1)```</span>
  ///
  ///
  pub fn op_dec_postfix(&mut self, arg1: ::libc::c_int) -> ::tree_widget_item_iterator::TreeWidgetItemIterator {
    {
      let mut object: ::tree_widget_item_iterator::TreeWidgetItemIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItemIterator_operator_dec_postfix_to_output(self as *mut ::tree_widget_item_iterator::TreeWidgetItemIterator, arg1, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItemIterator& QTreeWidgetItemIterator::operator++()```</span>
  ///
  ///
  pub fn op_inc<'l0>(&'l0 mut self) -> &'l0 mut ::tree_widget_item_iterator::TreeWidgetItemIterator {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidgetItemIterator_operator_inc(self as *mut ::tree_widget_item_iterator::TreeWidgetItemIterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTreeWidgetItemIterator QTreeWidgetItemIterator::operator++(int arg1)```</span>
  ///
  ///
  pub fn op_inc_postfix(&mut self, arg1: ::libc::c_int) -> ::tree_widget_item_iterator::TreeWidgetItemIterator {
    {
      let mut object: ::tree_widget_item_iterator::TreeWidgetItemIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidgetItemIterator_operator_inc_postfix_to_output(self as *mut ::tree_widget_item_iterator::TreeWidgetItemIterator, arg1, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QTreeWidgetItemIterator::operator*() const```</span>
  ///
  ///
  pub fn op_indirection(&self) -> *mut ::tree_widget_item::TreeWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItemIterator_operator_indirection(self as *const ::tree_widget_item_iterator::TreeWidgetItemIterator) }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItemIterator& QTreeWidgetItemIterator::operator-=(int n)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0>(&'l0 mut self,
                            n: ::libc::c_int)
                            -> &'l0 mut ::tree_widget_item_iterator::TreeWidgetItemIterator {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidgetItemIterator_operator_sub_assign(self as *mut ::tree_widget_item_iterator::TreeWidgetItemIterator, n) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl Drop for ::tree_widget_item_iterator::TreeWidgetItemIterator {
  /// C++ method: <span style='color: green;'>```[destructor] void QTreeWidgetItemIterator::~QTreeWidgetItemIterator()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTreeWidgetItemIterator_destructor(self as *mut ::tree_widget_item_iterator::TreeWidgetItemIterator) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TreeWidgetItemIterator::new_unsafe](../struct.TreeWidgetItemIterator.html#method.new_unsafe) method.
  pub trait TreeWidgetItemIteratorNewUnsafeArgs {
    unsafe fn exec(self) -> ::tree_widget_item_iterator::TreeWidgetItemIterator;
  }
  impl TreeWidgetItemIteratorNewUnsafeArgs for *mut ::tree_widget_item::TreeWidgetItem {
    unsafe fn exec(self) -> ::tree_widget_item_iterator::TreeWidgetItemIterator {
      let item = self;
      {
        let mut object: ::tree_widget_item_iterator::TreeWidgetItemIterator =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QTreeWidgetItemIterator_constructor_item(item, &mut object);
        object
      }
    }
  }
  impl TreeWidgetItemIteratorNewUnsafeArgs
    for (*mut ::tree_widget_item::TreeWidgetItem, ::qt_core::flags::Flags<::tree_widget_item_iterator::IteratorFlag>) {
    unsafe fn exec(self) -> ::tree_widget_item_iterator::TreeWidgetItemIterator {
      let item = self.0;
      let flags = self.1;
      {
        let mut object: ::tree_widget_item_iterator::TreeWidgetItemIterator =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QTreeWidgetItemIterator_constructor_item_flags(item,
                                                                           flags.to_int() as ::libc::c_uint,
                                                                           &mut object);
        object
      }
    }
  }
  impl TreeWidgetItemIteratorNewUnsafeArgs for *mut ::tree_widget::TreeWidget {
    unsafe fn exec(self) -> ::tree_widget_item_iterator::TreeWidgetItemIterator {
      let widget = self;
      {
        let mut object: ::tree_widget_item_iterator::TreeWidgetItemIterator =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QTreeWidgetItemIterator_constructor_widget(widget, &mut object);
        object
      }
    }
  }
  impl TreeWidgetItemIteratorNewUnsafeArgs
    for (*mut ::tree_widget::TreeWidget, ::qt_core::flags::Flags<::tree_widget_item_iterator::IteratorFlag>) {
    unsafe fn exec(self) -> ::tree_widget_item_iterator::TreeWidgetItemIterator {
      let widget = self.0;
      let flags = self.1;
      {
        let mut object: ::tree_widget_item_iterator::TreeWidgetItemIterator =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QTreeWidgetItemIterator_constructor_widget_flags(widget,
                                                                             flags.to_int() as ::libc::c_uint,
                                                                             &mut object);
        object
      }
    }
  }
}
