/// C++ type: <span style='color: green;'>```QPersistentModelIndex```</span>
#[repr(C)]
pub struct PersistentModelIndex([u8; ::type_sizes::QT_CORE_PERSISTENT_MODEL_INDEX_PERSISTENT_MODEL_INDEX]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PersistentModelIndex {
  unsafe fn new_uninitialized() -> PersistentModelIndex {
    PersistentModelIndex(::std::mem::uninitialized())
  }
}

impl PersistentModelIndex {
  /// C++ method: <span style='color: green;'>```const QModelIndex& QPersistentModelIndex::operator const QModelIndex &() const```</span>
  ///
  ///
  pub fn as_q_model_index_ref<'l0>(&'l0 self) -> &'l0 ::model_index::ModelIndex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPersistentModelIndex_convert_to_QModelIndex_ref(self as *const ::persistent_model_index::PersistentModelIndex) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QPersistentModelIndex::child(int row, int column) const```</span>
  ///
  ///
  pub fn child(&self, row: ::libc::c_int, column: ::libc::c_int) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QPersistentModelIndex_child_to_output(self as *const ::persistent_model_index::PersistentModelIndex, row, column, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QPersistentModelIndex::column() const```</span>
  ///
  ///
  pub fn column(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QPersistentModelIndex_column(self as *const ::persistent_model_index::PersistentModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QPersistentModelIndex::data```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn data(&self, ()) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QPersistentModelIndex::data() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn data(&self, ::libc::c_int) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QPersistentModelIndex::data(int role = ?) const```</span>
  ///
  ///
  pub fn data<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::PersistentModelIndexDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QPersistentModelIndex::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe {
      ::ffi::qt_core_c_QPersistentModelIndex_isValid(self as *const ::persistent_model_index::PersistentModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```const QAbstractItemModel* QPersistentModelIndex::model() const```</span>
  ///
  ///
  pub fn model(&self) -> *const ::abstract_item_model::AbstractItemModel {
    unsafe {
      ::ffi::qt_core_c_QPersistentModelIndex_model(self as *const ::persistent_model_index::PersistentModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QPersistentModelIndex::QPersistentModelIndex```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::persistent_model_index::PersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPersistentModelIndex::QPersistentModelIndex()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::model_index::ModelIndex) -> ::persistent_model_index::PersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPersistentModelIndex::QPersistentModelIndex(const QModelIndex& index)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::persistent_model_index::PersistentModelIndex) -> ::persistent_model_index::PersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPersistentModelIndex::QPersistentModelIndex(const QPersistentModelIndex& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::persistent_model_index::PersistentModelIndex
    where Args: overloading::PersistentModelIndexNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPersistentModelIndex::operator=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::model_index::ModelIndex) -> &'l0 mut ::persistent_model_index::PersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QPersistentModelIndex& QPersistentModelIndex::operator=(const QModelIndex& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::persistent_model_index::PersistentModelIndex) -> &'l0 mut ::persistent_model_index::PersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QPersistentModelIndex& QPersistentModelIndex::operator=(const QPersistentModelIndex& other)```</span>
  ///
  ///
  pub fn op_assign<'largs, Args>(&'largs mut self,
                                 args: Args)
                                 -> &'largs mut ::persistent_model_index::PersistentModelIndex
    where Args: overloading::PersistentModelIndexOpAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPersistentModelIndex::operator==```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_eq(&self, &::model_index::ModelIndex) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPersistentModelIndex::operator==(const QModelIndex& other) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_eq(&self, &::persistent_model_index::PersistentModelIndex) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPersistentModelIndex::operator==(const QPersistentModelIndex& other) const```</span>
  ///
  ///
  pub fn op_eq<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::PersistentModelIndexOpEqArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QPersistentModelIndex::operator<(const QPersistentModelIndex& other) const```</span>
  ///
  ///
  pub fn op_lt(&self, other: &::persistent_model_index::PersistentModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QPersistentModelIndex_operator_lt(self as *const ::persistent_model_index::PersistentModelIndex, other as *const ::persistent_model_index::PersistentModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QPersistentModelIndex::operator!=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_neq(&self, &::model_index::ModelIndex) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPersistentModelIndex::operator!=(const QModelIndex& other) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_neq(&self, &::persistent_model_index::PersistentModelIndex) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPersistentModelIndex::operator!=(const QPersistentModelIndex& other) const```</span>
  ///
  ///
  pub fn op_neq<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::PersistentModelIndexOpNeqArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QModelIndex QPersistentModelIndex::parent() const```</span>
  ///
  ///
  pub fn parent(&self) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QPersistentModelIndex_parent_to_output(self as *const ::persistent_model_index::PersistentModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QPersistentModelIndex::row() const```</span>
  ///
  ///
  pub fn row(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QPersistentModelIndex_row(self as *const ::persistent_model_index::PersistentModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QPersistentModelIndex::sibling(int row, int column) const```</span>
  ///
  ///
  pub fn sibling(&self, row: ::libc::c_int, column: ::libc::c_int) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QPersistentModelIndex_sibling_to_output(self as *const ::persistent_model_index::PersistentModelIndex, row, column, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QPersistentModelIndex::swap(QPersistentModelIndex& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::persistent_model_index::PersistentModelIndex) {
    unsafe {
      ::ffi::qt_core_c_QPersistentModelIndex_swap(self as *mut ::persistent_model_index::PersistentModelIndex,
                                                  other as *mut ::persistent_model_index::PersistentModelIndex)
    }
  }
}

impl Drop for ::persistent_model_index::PersistentModelIndex {
  /// C++ method: <span style='color: green;'>```[destructor] void QPersistentModelIndex::~QPersistentModelIndex()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QPersistentModelIndex_destructor(self as *mut ::persistent_model_index::PersistentModelIndex)
    }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PersistentModelIndex::data](../struct.PersistentModelIndex.html#method.data) method.
  pub trait PersistentModelIndexDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::persistent_model_index::PersistentModelIndex) -> ::variant::Variant;
  }
  impl<'largs> PersistentModelIndexDataArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::persistent_model_index::PersistentModelIndex) -> ::variant::Variant {

      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPersistentModelIndex_data_to_output_no_args(original_self as *const ::persistent_model_index::PersistentModelIndex, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PersistentModelIndexDataArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::persistent_model_index::PersistentModelIndex) -> ::variant::Variant {
      let role = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPersistentModelIndex_data_to_output_role(original_self as *const ::persistent_model_index::PersistentModelIndex, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PersistentModelIndex::new](../struct.PersistentModelIndex.html#method.new) method.
  pub trait PersistentModelIndexNewArgs {
    fn exec(self) -> ::persistent_model_index::PersistentModelIndex;
  }
  impl<'a> PersistentModelIndexNewArgs for &'a ::model_index::ModelIndex {
    fn exec(self) -> ::persistent_model_index::PersistentModelIndex {
      let index = self;
      {
        let mut object: ::persistent_model_index::PersistentModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPersistentModelIndex_constructor_index(index as *const ::model_index::ModelIndex,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl PersistentModelIndexNewArgs for () {
    fn exec(self) -> ::persistent_model_index::PersistentModelIndex {

      {
        let mut object: ::persistent_model_index::PersistentModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPersistentModelIndex_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PersistentModelIndexNewArgs for &'a ::persistent_model_index::PersistentModelIndex {
    fn exec(self) -> ::persistent_model_index::PersistentModelIndex {
      let other = self;
      {
        let mut object: ::persistent_model_index::PersistentModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPersistentModelIndex_constructor_other(other as *const ::persistent_model_index::PersistentModelIndex, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PersistentModelIndex::op_assign](../struct.PersistentModelIndex.html#method.op_assign) method.
  pub trait PersistentModelIndexOpAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::persistent_model_index::PersistentModelIndex)
            -> &'largs mut ::persistent_model_index::PersistentModelIndex;
  }
  impl<'largs> PersistentModelIndexOpAssignArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self,
            original_self: &'largs mut ::persistent_model_index::PersistentModelIndex)
            -> &'largs mut ::persistent_model_index::PersistentModelIndex {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QPersistentModelIndex_operator_assign_QModelIndex(original_self as *mut ::persistent_model_index::PersistentModelIndex, other as *const ::model_index::ModelIndex) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> PersistentModelIndexOpAssignArgs<'largs> for &'largs ::persistent_model_index::PersistentModelIndex {
    fn exec(self,
            original_self: &'largs mut ::persistent_model_index::PersistentModelIndex)
            -> &'largs mut ::persistent_model_index::PersistentModelIndex {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QPersistentModelIndex_operator_assign_QPersistentModelIndex(original_self as *mut ::persistent_model_index::PersistentModelIndex, other as *const ::persistent_model_index::PersistentModelIndex) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [PersistentModelIndex::op_eq](../struct.PersistentModelIndex.html#method.op_eq) method.
  pub trait PersistentModelIndexOpEqArgs<'largs> {
    fn exec(self, original_self: &'largs ::persistent_model_index::PersistentModelIndex) -> bool;
  }
  impl<'largs> PersistentModelIndexOpEqArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::persistent_model_index::PersistentModelIndex) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QPersistentModelIndex_operator_eq_QModelIndex(original_self as *const ::persistent_model_index::PersistentModelIndex, other as *const ::model_index::ModelIndex) }
    }
  }
  impl<'largs> PersistentModelIndexOpEqArgs<'largs> for &'largs ::persistent_model_index::PersistentModelIndex {
    fn exec(self, original_self: &'largs ::persistent_model_index::PersistentModelIndex) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QPersistentModelIndex_operator_eq_QPersistentModelIndex(original_self as *const ::persistent_model_index::PersistentModelIndex, other as *const ::persistent_model_index::PersistentModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [PersistentModelIndex::op_neq](../struct.PersistentModelIndex.html#method.op_neq) method.
  pub trait PersistentModelIndexOpNeqArgs<'largs> {
    fn exec(self, original_self: &'largs ::persistent_model_index::PersistentModelIndex) -> bool;
  }
  impl<'largs> PersistentModelIndexOpNeqArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::persistent_model_index::PersistentModelIndex) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QPersistentModelIndex_operator_neq_QModelIndex(original_self as *const ::persistent_model_index::PersistentModelIndex, other as *const ::model_index::ModelIndex) }
    }
  }
  impl<'largs> PersistentModelIndexOpNeqArgs<'largs> for &'largs ::persistent_model_index::PersistentModelIndex {
    fn exec(self, original_self: &'largs ::persistent_model_index::PersistentModelIndex) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QPersistentModelIndex_operator_neq_QPersistentModelIndex(original_self as *const ::persistent_model_index::PersistentModelIndex, other as *const ::persistent_model_index::PersistentModelIndex) }
    }
  }
}
