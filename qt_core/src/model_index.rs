/// C++ type: <span style='color: green;'>```QModelIndex```</span>
#[repr(C)]
pub struct ModelIndex([u8; ::type_sizes::QT_CORE_MODEL_INDEX_MODEL_INDEX]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ModelIndex {
  unsafe fn new_uninitialized() -> ModelIndex {
    ModelIndex(::std::mem::uninitialized())
  }
}

impl ModelIndex {
  /// C++ method: <span style='color: green;'>```QModelIndex QModelIndex::child(int row, int column) const```</span>
  ///
  ///
  pub fn child(&self, row: ::libc::c_int, column: ::libc::c_int) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QModelIndex_child_to_output(self as *const ::model_index::ModelIndex,
                                                     row,
                                                     column,
                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QModelIndex::column() const```</span>
  ///
  ///
  pub fn column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QModelIndex_column(self as *const ::model_index::ModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex::data```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn data(&self, ()) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QModelIndex::data() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn data(&self, ::libc::c_int) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QModelIndex::data(int role = ?) const```</span>
  ///
  ///
  pub fn data<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::ModelIndexDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```quintptr QModelIndex::internalId() const```</span>
  ///
  ///
  pub fn internal_id(&self) -> usize {
    unsafe { ::ffi::qt_core_c_QModelIndex_internalId(self as *const ::model_index::ModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```void* QModelIndex::internalPointer() const```</span>
  ///
  ///
  pub fn internal_pointer(&self) -> *mut ::libc::c_void {
    unsafe { ::ffi::qt_core_c_QModelIndex_internalPointer(self as *const ::model_index::ModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```bool QModelIndex::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QModelIndex_isValid(self as *const ::model_index::ModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```const QAbstractItemModel* QModelIndex::model() const```</span>
  ///
  ///
  pub fn model(&self) -> *const ::abstract_item_model::AbstractItemModel {
    unsafe { ::ffi::qt_core_c_QModelIndex_model(self as *const ::model_index::ModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QModelIndex::QModelIndex()```</span>
  ///
  ///
  pub fn new() -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QModelIndex_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QModelIndex::operator==(const QModelIndex& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QModelIndex_operator_eq(self as *const ::model_index::ModelIndex,
                                               other as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QModelIndex::operator<(const QModelIndex& other) const```</span>
  ///
  ///
  pub fn op_lt(&self, other: &::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QModelIndex_operator_lt(self as *const ::model_index::ModelIndex,
                                               other as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QModelIndex::operator!=(const QModelIndex& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QModelIndex_operator_neq(self as *const ::model_index::ModelIndex,
                                                other as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QModelIndex::parent() const```</span>
  ///
  ///
  pub fn parent(&self) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QModelIndex_parent_to_output(self as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QModelIndex::row() const```</span>
  ///
  ///
  pub fn row(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QModelIndex_row(self as *const ::model_index::ModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QModelIndex::sibling(int row, int column) const```</span>
  ///
  ///
  pub fn sibling(&self, row: ::libc::c_int, column: ::libc::c_int) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QModelIndex_sibling_to_output(self as *const ::model_index::ModelIndex,
                                                       row,
                                                       column,
                                                       &mut object);
      }
      object
    }
  }
}

impl Drop for ::model_index::ModelIndex {
  /// C++ method: <span style='color: green;'>```[destructor] void QModelIndex::~QModelIndex()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QModelIndex_destructor(self as *mut ::model_index::ModelIndex) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ModelIndex::data](../struct.ModelIndex.html#method.data) method.
  pub trait ModelIndexDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::model_index::ModelIndex) -> ::variant::Variant;
  }
  impl<'largs> ModelIndexDataArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::model_index::ModelIndex) -> ::variant::Variant {

      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QModelIndex_data_to_output_no_args(original_self as *const ::model_index::ModelIndex,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ModelIndexDataArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::model_index::ModelIndex) -> ::variant::Variant {
      let role = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QModelIndex_data_to_output_role(original_self as *const ::model_index::ModelIndex,
                                                           role,
                                                           &mut object);
        }
        object
      }
    }
  }
}
