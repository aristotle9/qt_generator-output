/// C++ type: <span style='color: green;'>```QStringListModel```</span>
#[repr(C)]
pub struct StringListModel(u8);

impl StringListModel {
  /// C++ method: <span style='color: green;'>```QStringListModel::data```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn data(&self, &::model_index::ModelIndex) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QStringListModel::data(const QModelIndex& index) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn data(&self, (&::model_index::ModelIndex, ::libc::c_int)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QStringListModel::data(const QModelIndex& index, int role = ?) const```</span>
  ///
  ///
  pub fn data<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::StringListModelDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringListModel::insertRows```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_rows(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStringListModel::insertRows(int row, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_rows(&mut self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStringListModel::insertRows(int row, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn insert_rows<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::StringListModelInsertRowsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QStringListModel::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QStringListModel_metaObject(self as *const ::string_list_model::StringListModel) }
  }

  /// C++ method: <span style='color: green;'>```QStringListModel::QStringListModel```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::string_list_model::StringListModel>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringListModel::QStringListModel()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string_list::StringList) -> ::cpp_utils::CppBox<::string_list_model::StringListModel>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringListModel::QStringListModel(const QStringList& strings)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::string_list_model::StringListModel>
    where Args: overloading::StringListModelNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStringListModel::QStringListModel```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::object::Object) -> ::cpp_utils::CppBox<::string_list_model::StringListModel>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringListModel::QStringListModel(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::string_list::StringList, *mut ::object::Object)) -> ::cpp_utils::CppBox<::string_list_model::StringListModel>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringListModel::QStringListModel(const QStringList& strings, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::string_list_model::StringListModel>
    where Args: overloading::StringListModelNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStringListModel::removeRows```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove_rows(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStringListModel::removeRows(int row, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove_rows(&mut self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStringListModel::removeRows(int row, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn remove_rows<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::StringListModelRemoveRowsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringListModel::rowCount```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn row_count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QStringListModel::rowCount() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn row_count(&self, &::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QStringListModel::rowCount(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn row_count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringListModelRowCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringListModel::setData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::model_index::ModelIndex, &::variant::Variant)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStringListModel::setData(const QModelIndex& index, const QVariant& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::model_index::ModelIndex, &::variant::Variant, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStringListModel::setData(const QModelIndex& index, const QVariant& value, int role = ?)```</span>
  ///
  ///
  pub fn set_data<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::StringListModelSetDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QStringListModel::setStringList(const QStringList& strings)```</span>
  ///
  ///
  pub fn set_string_list(&mut self, strings: &::string_list::StringList) {
    unsafe {
      ::ffi::qt_core_c_QStringListModel_setStringList(self as *mut ::string_list_model::StringListModel,
                                                      strings as *const ::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QStringListModel::sibling(int row, int column, const QModelIndex& idx) const```</span>
  ///
  ///
  pub fn sibling(&self,
                 row: ::libc::c_int,
                 column: ::libc::c_int,
                 idx: &::model_index::ModelIndex)
                 -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStringListModel_sibling_to_output(self as *const ::string_list_model::StringListModel,
                                                            row,
                                                            column,
                                                            idx as *const ::model_index::ModelIndex,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringListModel::sort```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sort(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QStringListModel::sort(int column)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sort(&mut self, (::libc::c_int, &::qt::SortOrder)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QStringListModel::sort(int column, Qt::SortOrder order = ?)```</span>
  ///
  ///
  pub fn sort<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StringListModelSortArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringList QStringListModel::stringList() const```</span>
  ///
  ///
  pub fn string_list(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStringListModel_stringList_to_output(self as *const ::string_list_model::StringListModel,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QStringListModel::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QStringListModel_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QStringListModel::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QStringListModel_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::string_list_model::StringListModel {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QStringListModel_delete
  }
}

impl ::cpp_utils::DynamicCast<::string_list_model::StringListModel> for ::abstract_item_model::AbstractItemModel {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::string_list_model::StringListModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStringListModel_G_dynamic_cast_QStringListModel_ptr_QAbstractItemModel(self as *mut ::abstract_item_model::AbstractItemModel) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::string_list_model::StringListModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStringListModel_G_dynamic_cast_QStringListModel_ptr_QAbstractItemModel(self as *const ::abstract_item_model::AbstractItemModel as *mut ::abstract_item_model::AbstractItemModel) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::string_list_model::StringListModel> for ::abstract_list_model::AbstractListModel {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::string_list_model::StringListModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStringListModel_G_dynamic_cast_QStringListModel_ptr_QAbstractListModel(self as *mut ::abstract_list_model::AbstractListModel) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::string_list_model::StringListModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStringListModel_G_dynamic_cast_QStringListModel_ptr_QAbstractListModel(self as *const ::abstract_list_model::AbstractListModel as *mut ::abstract_list_model::AbstractListModel) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::string_list_model::StringListModel> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::string_list_model::StringListModel> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QStringListModel_G_dynamic_cast_QStringListModel_ptr_QObject(self as *mut ::object::Object)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::string_list_model::StringListModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStringListModel_G_dynamic_cast_QStringListModel_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_model::AbstractItemModel> for ::string_list_model::StringListModel {
  fn static_cast_mut(&mut self) -> &mut ::abstract_item_model::AbstractItemModel {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStringListModel_G_static_cast_QAbstractItemModel_ptr(self as *mut ::string_list_model::StringListModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_item_model::AbstractItemModel {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStringListModel_G_static_cast_QAbstractItemModel_ptr(self as *const ::string_list_model::StringListModel as *mut ::string_list_model::StringListModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_list_model::AbstractListModel> for ::string_list_model::StringListModel {
  fn static_cast_mut(&mut self) -> &mut ::abstract_list_model::AbstractListModel {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStringListModel_G_static_cast_QAbstractListModel_ptr(self as *mut ::string_list_model::StringListModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_list_model::AbstractListModel {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStringListModel_G_static_cast_QAbstractListModel_ptr(self as *const ::string_list_model::StringListModel as *mut ::string_list_model::StringListModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::string_list_model::StringListModel {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QStringListModel_G_static_cast_QObject_ptr(self as *mut ::string_list_model::StringListModel)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStringListModel_G_static_cast_QObject_ptr(self as *const ::string_list_model::StringListModel as *mut ::string_list_model::StringListModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::string_list_model::StringListModel> for ::abstract_item_model::AbstractItemModel {
unsafe fn static_cast_mut(&mut self) -> &mut ::string_list_model::StringListModel {
let ffi_result = ::ffi::qt_core_c_QStringListModel_G_static_cast_QStringListModel_ptr_QAbstractItemModel(self as *mut ::abstract_item_model::AbstractItemModel);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::string_list_model::StringListModel {
let ffi_result = ::ffi::qt_core_c_QStringListModel_G_static_cast_QStringListModel_ptr_QAbstractItemModel(self as *const ::abstract_item_model::AbstractItemModel as *mut ::abstract_item_model::AbstractItemModel);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::string_list_model::StringListModel> for ::abstract_list_model::AbstractListModel {
unsafe fn static_cast_mut(&mut self) -> &mut ::string_list_model::StringListModel {
let ffi_result = ::ffi::qt_core_c_QStringListModel_G_static_cast_QStringListModel_ptr_QAbstractListModel(self as *mut ::abstract_list_model::AbstractListModel);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::string_list_model::StringListModel {
let ffi_result = ::ffi::qt_core_c_QStringListModel_G_static_cast_QStringListModel_ptr_QAbstractListModel(self as *const ::abstract_list_model::AbstractListModel as *mut ::abstract_list_model::AbstractListModel);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::string_list_model::StringListModel> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::string_list_model::StringListModel {
    let ffi_result =
      ::ffi::qt_core_c_QStringListModel_G_static_cast_QStringListModel_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::string_list_model::StringListModel {
    let ffi_result = ::ffi::qt_core_c_QStringListModel_G_static_cast_QStringListModel_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::string_list_model::StringListModel {
  type Target = ::abstract_list_model::AbstractListModel;
  fn deref(&self) -> &::abstract_list_model::AbstractListModel {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStringListModel_G_static_cast_QAbstractListModel_ptr(self as *const ::string_list_model::StringListModel as *mut ::string_list_model::StringListModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::string_list_model::StringListModel {
  fn deref_mut(&mut self) -> &mut ::abstract_list_model::AbstractListModel {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStringListModel_G_static_cast_QAbstractListModel_ptr(self as *mut ::string_list_model::StringListModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StringListModel::data](../struct.StringListModel.html#method.data) method.
  pub trait StringListModelDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_list_model::StringListModel) -> ::variant::Variant;
  }
  impl<'largs> StringListModelDataArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::string_list_model::StringListModel) -> ::variant::Variant {
      let index = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringListModel_data_to_output_index(original_self as *const ::string_list_model::StringListModel, index as *const ::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringListModelDataArgs<'largs> for (&'largs ::model_index::ModelIndex, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_list_model::StringListModel) -> ::variant::Variant {
      let index = self.0;
      let role = self.1;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringListModel_data_to_output_index_role(original_self as *const ::string_list_model::StringListModel, index as *const ::model_index::ModelIndex, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringListModel::insert_rows](../struct.StringListModel.html#method.insert_rows) method.
  pub trait StringListModelInsertRowsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string_list_model::StringListModel) -> bool;
  }
  impl<'largs> StringListModelInsertRowsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string_list_model::StringListModel) -> bool {
      let row = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_core_c_QStringListModel_insertRows_row_count(original_self as *mut ::string_list_model::StringListModel, row, count) }
    }
  }
  impl<'largs> StringListModelInsertRowsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::string_list_model::StringListModel) -> bool {
      let row = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QStringListModel_insertRows_row_count_parent(original_self as *mut ::string_list_model::StringListModel, row, count, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [StringListModel::new](../struct.StringListModel.html#method.new) method.
  pub trait StringListModelNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::string_list_model::StringListModel>;
  }
  impl StringListModelNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::string_list_model::StringListModel> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QStringListModel_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StringListModelNewArgs for &'a ::string_list::StringList {
    fn exec(self) -> ::cpp_utils::CppBox<::string_list_model::StringListModel> {
      let strings = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QStringListModel_new_strings(strings as *const ::string_list::StringList) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [StringListModel::new_unsafe](../struct.StringListModel.html#method.new_unsafe) method.
  pub trait StringListModelNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::string_list_model::StringListModel>;
  }
  impl StringListModelNewUnsafeArgs for *mut ::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::string_list_model::StringListModel> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QStringListModel_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> StringListModelNewUnsafeArgs for (&'a ::string_list::StringList, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::string_list_model::StringListModel> {
      let strings = self.0;
      let parent = self.1;
      let ffi_result =
        ::ffi::qt_core_c_QStringListModel_new_strings_parent(strings as *const ::string_list::StringList, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [StringListModel::remove_rows](../struct.StringListModel.html#method.remove_rows) method.
  pub trait StringListModelRemoveRowsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string_list_model::StringListModel) -> bool;
  }
  impl<'largs> StringListModelRemoveRowsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string_list_model::StringListModel) -> bool {
      let row = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_core_c_QStringListModel_removeRows_row_count(original_self as *mut ::string_list_model::StringListModel, row, count) }
    }
  }
  impl<'largs> StringListModelRemoveRowsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::string_list_model::StringListModel) -> bool {
      let row = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QStringListModel_removeRows_row_count_parent(original_self as *mut ::string_list_model::StringListModel, row, count, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [StringListModel::row_count](../struct.StringListModel.html#method.row_count) method.
  pub trait StringListModelRowCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_list_model::StringListModel) -> ::libc::c_int;
  }
  impl<'largs> StringListModelRowCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::string_list_model::StringListModel) -> ::libc::c_int {

      unsafe {
        ::ffi::qt_core_c_QStringListModel_rowCount_no_args(original_self as *const ::string_list_model::StringListModel)
      }
    }
  }
  impl<'largs> StringListModelRowCountArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::string_list_model::StringListModel) -> ::libc::c_int {
      let parent = self;
      unsafe {
        ::ffi::qt_core_c_QStringListModel_rowCount_parent(original_self as *const ::string_list_model::StringListModel, parent as *const ::model_index::ModelIndex)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringListModel::set_data](../struct.StringListModel.html#method.set_data) method.
  pub trait StringListModelSetDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string_list_model::StringListModel) -> bool;
  }
  impl<'largs> StringListModelSetDataArgs<'largs> for (&'largs ::model_index::ModelIndex, &'largs ::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::string_list_model::StringListModel) -> bool {
      let index = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_core_c_QStringListModel_setData_index_value(original_self as *mut ::string_list_model::StringListModel, index as *const ::model_index::ModelIndex, value as *const ::variant::Variant) }
    }
  }
  impl<'largs> StringListModelSetDataArgs<'largs>
    for (&'largs ::model_index::ModelIndex, &'largs ::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string_list_model::StringListModel) -> bool {
      let index = self.0;
      let value = self.1;
      let role = self.2;
      unsafe { ::ffi::qt_core_c_QStringListModel_setData_index_value_role(original_self as *mut ::string_list_model::StringListModel, index as *const ::model_index::ModelIndex, value as *const ::variant::Variant, role) }
    }
  }
  /// This trait represents a set of arguments accepted by [StringListModel::sort](../struct.StringListModel.html#method.sort) method.
  pub trait StringListModelSortArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string_list_model::StringListModel) -> ();
  }
  impl<'largs> StringListModelSortArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::string_list_model::StringListModel) -> () {
      let column = self;
      unsafe {
        ::ffi::qt_core_c_QStringListModel_sort_column(original_self as *mut ::string_list_model::StringListModel,
                                                      column)
      }
    }
  }
  impl<'largs> StringListModelSortArgs<'largs> for (::libc::c_int, &'largs ::qt::SortOrder) {
    fn exec(self, original_self: &'largs mut ::string_list_model::StringListModel) -> () {
      let column = self.0;
      let order = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringListModel_sort_column_order(original_self as *mut ::string_list_model::StringListModel, column, order as *const ::qt::SortOrder)
      }
    }
  }
}
