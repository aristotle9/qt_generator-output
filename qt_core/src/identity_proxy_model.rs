/// C++ type: <span style='color: green;'>```QIdentityProxyModel```</span>
#[repr(C)]
pub struct IdentityProxyModel(u8);

impl IdentityProxyModel {
  /// C++ method: <span style='color: green;'>```QIdentityProxyModel::columnCount```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn column_count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QIdentityProxyModel::columnCount() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn column_count(&self, &::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QIdentityProxyModel::columnCount(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn column_count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::IdentityProxyModelColumnCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual bool QIdentityProxyModel::dropMimeData(const QMimeData* data, Qt::DropAction action, int row, int column, const QModelIndex& parent)```</span>
  ///
  ///
  pub unsafe fn drop_mime_data(&mut self,
                               data: *const ::mime_data::MimeData,
                               action: &::qt::DropAction,
                               row: ::libc::c_int,
                               column: ::libc::c_int,
                               parent: &::model_index::ModelIndex)
                               -> bool {
    ::ffi::qt_core_c_QIdentityProxyModel_dropMimeData(self as *mut ::identity_proxy_model::IdentityProxyModel,
                                                      data,
                                                      action as *const ::qt::DropAction,
                                                      row,
                                                      column,
                                                      parent as *const ::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```QIdentityProxyModel::headerData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn header_data(&self, (::libc::c_int, &::qt::Orientation)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QIdentityProxyModel::headerData(int section, Qt::Orientation orientation) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn header_data(&self, (::libc::c_int, &::qt::Orientation, ::libc::c_int)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QIdentityProxyModel::headerData(int section, Qt::Orientation orientation, int role = ?) const```</span>
  ///
  ///
  pub fn header_data<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::IdentityProxyModelHeaderDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QIdentityProxyModel::index```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index(&self, (::libc::c_int, ::libc::c_int)) -> ::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QIdentityProxyModel::index(int row, int column) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index(&self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> ::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QIdentityProxyModel::index(int row, int column, const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn index<'largs, Args>(&'largs self, args: Args) -> ::model_index::ModelIndex
    where Args: overloading::IdentityProxyModelIndexArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QIdentityProxyModel::insertColumns```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_columns(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QIdentityProxyModel::insertColumns(int column, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_columns(&mut self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QIdentityProxyModel::insertColumns(int column, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn insert_columns<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::IdentityProxyModelInsertColumnsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QIdentityProxyModel::insertRows```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_rows(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QIdentityProxyModel::insertRows(int row, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_rows(&mut self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QIdentityProxyModel::insertRows(int row, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn insert_rows<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::IdentityProxyModelInsertRowsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QIdentityProxyModel::mapFromSource(const QModelIndex& sourceIndex) const```</span>
  ///
  ///
  pub fn map_from_source(&self, source_index: &::model_index::ModelIndex) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QIdentityProxyModel_mapFromSource_to_output(self as *const ::identity_proxy_model::IdentityProxyModel, source_index as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QItemSelection QIdentityProxyModel::mapSelectionFromSource(const QItemSelection& selection) const```</span>
  ///
  ///
  pub fn map_selection_from_source(&self,
                                   selection: &::item_selection::ItemSelection)
                                   -> ::item_selection::ItemSelection {
    {
      let mut object: ::item_selection::ItemSelection =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QIdentityProxyModel_mapSelectionFromSource_to_output(self as *const ::identity_proxy_model::IdentityProxyModel, selection as *const ::item_selection::ItemSelection, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QItemSelection QIdentityProxyModel::mapSelectionToSource(const QItemSelection& selection) const```</span>
  ///
  ///
  pub fn map_selection_to_source(&self,
                                 selection: &::item_selection::ItemSelection)
                                 -> ::item_selection::ItemSelection {
    {
      let mut object: ::item_selection::ItemSelection =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QIdentityProxyModel_mapSelectionToSource_to_output(self as *const ::identity_proxy_model::IdentityProxyModel, selection as *const ::item_selection::ItemSelection, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QIdentityProxyModel::mapToSource(const QModelIndex& proxyIndex) const```</span>
  ///
  ///
  pub fn map_to_source(&self, proxy_index: &::model_index::ModelIndex) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QIdentityProxyModel_mapToSource_to_output(self as *const ::identity_proxy_model::IdentityProxyModel, proxy_index as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QIdentityProxyModel::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_core_c_QIdentityProxyModel_metaObject(self as *const ::identity_proxy_model::IdentityProxyModel)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QIdentityProxyModel::QIdentityProxyModel()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::identity_proxy_model::IdentityProxyModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QIdentityProxyModel::QIdentityProxyModel(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object)
                           -> ::cpp_utils::CppBox<::identity_proxy_model::IdentityProxyModel> {
    let ffi_result = ::ffi::qt_core_c_QIdentityProxyModel_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QIdentityProxyModel::parent(const QModelIndex& child) const```</span>
  ///
  ///
  pub fn parent(&self, child: &::model_index::ModelIndex) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QIdentityProxyModel_parent_to_output(self as *const ::identity_proxy_model::IdentityProxyModel, child as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QIdentityProxyModel::removeColumns```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove_columns(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QIdentityProxyModel::removeColumns(int column, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove_columns(&mut self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QIdentityProxyModel::removeColumns(int column, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn remove_columns<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::IdentityProxyModelRemoveColumnsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QIdentityProxyModel::removeRows```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove_rows(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QIdentityProxyModel::removeRows(int row, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove_rows(&mut self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QIdentityProxyModel::removeRows(int row, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn remove_rows<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::IdentityProxyModelRemoveRowsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QIdentityProxyModel::rowCount```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn row_count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QIdentityProxyModel::rowCount() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn row_count(&self, &::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QIdentityProxyModel::rowCount(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn row_count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::IdentityProxyModelRowCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QIdentityProxyModel::setSourceModel(QAbstractItemModel* sourceModel)```</span>
  ///
  ///
  pub unsafe fn set_source_model(&mut self, source_model: *mut ::abstract_item_model::AbstractItemModel) {
    ::ffi::qt_core_c_QIdentityProxyModel_setSourceModel(self as *mut ::identity_proxy_model::IdentityProxyModel,
                                                        source_model)
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QIdentityProxyModel::sibling(int row, int column, const QModelIndex& idx) const```</span>
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
        ::ffi::qt_core_c_QIdentityProxyModel_sibling_to_output(self as *const ::identity_proxy_model::IdentityProxyModel, row, column, idx as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QIdentityProxyModel::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QIdentityProxyModel_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QIdentityProxyModel::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QIdentityProxyModel_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::identity_proxy_model::IdentityProxyModel {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QIdentityProxyModel_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `IdentityProxyModel`.
  pub struct Signals<'a>(&'a ::identity_proxy_model::IdentityProxyModel);
  /// Represents a built-in Qt signal `QIdentityProxyModel::sourceModelChanged`.
  ///
  /// An object of this type can be created from `IdentityProxyModel` with `object.signals().source_model_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `IdentityProxyModel` object.
  pub struct SourceModelChanged<'a>(&'a ::identity_proxy_model::IdentityProxyModel);
  impl<'a> ::connection::Receiver for SourceModelChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceModelChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for SourceModelChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QIdentityProxyModel::sourceModelChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_model_changed(&self) -> SourceModelChanged {
      SourceModelChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `IdentityProxyModel`.
  pub struct Slots<'a>(&'a ::identity_proxy_model::IdentityProxyModel);
  /// Represents a built-in Qt slot `QIdentityProxyModel::resetInternalData`.
  ///
  /// An object of this type can be created from `IdentityProxyModel` with `object.slots().reset_internal_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `IdentityProxyModel` object.
  pub struct ResetInternalData<'a>(&'a ::identity_proxy_model::IdentityProxyModel);
  impl<'a> ::connection::Receiver for ResetInternalData<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resetInternalData()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QIdentityProxyModel::resetInternalData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset_internal_data(&self) -> ResetInternalData {
      ResetInternalData(self.0)
    }
  }
  impl ::identity_proxy_model::IdentityProxyModel {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::identity_proxy_model::IdentityProxyModel> for ::abstract_item_model::AbstractItemModel {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::identity_proxy_model::IdentityProxyModel> {
let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_G_dynamic_cast_QIdentityProxyModel_ptr_QAbstractItemModel(self as *mut ::abstract_item_model::AbstractItemModel) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::identity_proxy_model::IdentityProxyModel> {
let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_G_dynamic_cast_QIdentityProxyModel_ptr_QAbstractItemModel(self as *const ::abstract_item_model::AbstractItemModel as *mut ::abstract_item_model::AbstractItemModel) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::identity_proxy_model::IdentityProxyModel> for ::abstract_proxy_model::AbstractProxyModel {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::identity_proxy_model::IdentityProxyModel> {
let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_G_dynamic_cast_QIdentityProxyModel_ptr_QAbstractProxyModel(self as *mut ::abstract_proxy_model::AbstractProxyModel) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::identity_proxy_model::IdentityProxyModel> {
let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_G_dynamic_cast_QIdentityProxyModel_ptr_QAbstractProxyModel(self as *const ::abstract_proxy_model::AbstractProxyModel as *mut ::abstract_proxy_model::AbstractProxyModel) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::identity_proxy_model::IdentityProxyModel> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::identity_proxy_model::IdentityProxyModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_G_dynamic_cast_QIdentityProxyModel_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::identity_proxy_model::IdentityProxyModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_G_dynamic_cast_QIdentityProxyModel_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_model::AbstractItemModel> for ::identity_proxy_model::IdentityProxyModel {
fn static_cast_mut(&mut self) -> &mut ::abstract_item_model::AbstractItemModel {
let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_G_static_cast_QAbstractItemModel_ptr(self as *mut ::identity_proxy_model::IdentityProxyModel) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_item_model::AbstractItemModel {
let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_G_static_cast_QAbstractItemModel_ptr(self as *const ::identity_proxy_model::IdentityProxyModel as *mut ::identity_proxy_model::IdentityProxyModel) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::abstract_proxy_model::AbstractProxyModel> for ::identity_proxy_model::IdentityProxyModel {
fn static_cast_mut(&mut self) -> &mut ::abstract_proxy_model::AbstractProxyModel {
let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_G_static_cast_QAbstractProxyModel_ptr(self as *mut ::identity_proxy_model::IdentityProxyModel) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_proxy_model::AbstractProxyModel {
let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_G_static_cast_QAbstractProxyModel_ptr(self as *const ::identity_proxy_model::IdentityProxyModel as *mut ::identity_proxy_model::IdentityProxyModel) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::object::Object> for ::identity_proxy_model::IdentityProxyModel {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_G_static_cast_QObject_ptr(self as *mut ::identity_proxy_model::IdentityProxyModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_G_static_cast_QObject_ptr(self as *const ::identity_proxy_model::IdentityProxyModel as *mut ::identity_proxy_model::IdentityProxyModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::identity_proxy_model::IdentityProxyModel> for ::abstract_item_model::AbstractItemModel {
unsafe fn static_cast_mut(&mut self) -> &mut ::identity_proxy_model::IdentityProxyModel {
let ffi_result = ::ffi::qt_core_c_QIdentityProxyModel_G_static_cast_QIdentityProxyModel_ptr_QAbstractItemModel(self as *mut ::abstract_item_model::AbstractItemModel);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::identity_proxy_model::IdentityProxyModel {
let ffi_result = ::ffi::qt_core_c_QIdentityProxyModel_G_static_cast_QIdentityProxyModel_ptr_QAbstractItemModel(self as *const ::abstract_item_model::AbstractItemModel as *mut ::abstract_item_model::AbstractItemModel);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::identity_proxy_model::IdentityProxyModel> for ::abstract_proxy_model::AbstractProxyModel {
unsafe fn static_cast_mut(&mut self) -> &mut ::identity_proxy_model::IdentityProxyModel {
let ffi_result = ::ffi::qt_core_c_QIdentityProxyModel_G_static_cast_QIdentityProxyModel_ptr_QAbstractProxyModel(self as *mut ::abstract_proxy_model::AbstractProxyModel);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::identity_proxy_model::IdentityProxyModel {
let ffi_result = ::ffi::qt_core_c_QIdentityProxyModel_G_static_cast_QIdentityProxyModel_ptr_QAbstractProxyModel(self as *const ::abstract_proxy_model::AbstractProxyModel as *mut ::abstract_proxy_model::AbstractProxyModel);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::identity_proxy_model::IdentityProxyModel> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::identity_proxy_model::IdentityProxyModel {
    let ffi_result = ::ffi::qt_core_c_QIdentityProxyModel_G_static_cast_QIdentityProxyModel_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::identity_proxy_model::IdentityProxyModel {
    let ffi_result = ::ffi::qt_core_c_QIdentityProxyModel_G_static_cast_QIdentityProxyModel_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::identity_proxy_model::IdentityProxyModel {
  type Target = ::abstract_proxy_model::AbstractProxyModel;
  fn deref(&self) -> &::abstract_proxy_model::AbstractProxyModel {
    let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_G_static_cast_QAbstractProxyModel_ptr(self as *const ::identity_proxy_model::IdentityProxyModel as *mut ::identity_proxy_model::IdentityProxyModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::identity_proxy_model::IdentityProxyModel {
  fn deref_mut(&mut self) -> &mut ::abstract_proxy_model::AbstractProxyModel {
    let ffi_result = unsafe { ::ffi::qt_core_c_QIdentityProxyModel_G_static_cast_QAbstractProxyModel_ptr(self as *mut ::identity_proxy_model::IdentityProxyModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [IdentityProxyModel::column_count](../struct.IdentityProxyModel.html#method.column_count) method.
  pub trait IdentityProxyModelColumnCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::identity_proxy_model::IdentityProxyModel) -> ::libc::c_int;
  }
  impl<'largs> IdentityProxyModelColumnCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::identity_proxy_model::IdentityProxyModel) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QIdentityProxyModel_columnCount_no_args(original_self as *const ::identity_proxy_model::IdentityProxyModel) }
    }
  }
  impl<'largs> IdentityProxyModelColumnCountArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::identity_proxy_model::IdentityProxyModel) -> ::libc::c_int {
      let parent = self;
      unsafe { ::ffi::qt_core_c_QIdentityProxyModel_columnCount_parent(original_self as *const ::identity_proxy_model::IdentityProxyModel, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [IdentityProxyModel::header_data](../struct.IdentityProxyModel.html#method.header_data) method.
  pub trait IdentityProxyModelHeaderDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::identity_proxy_model::IdentityProxyModel) -> ::variant::Variant;
  }
  impl<'largs> IdentityProxyModelHeaderDataArgs<'largs> for (::libc::c_int, &'largs ::qt::Orientation) {
    fn exec(self, original_self: &'largs ::identity_proxy_model::IdentityProxyModel) -> ::variant::Variant {
      let section = self.0;
      let orientation = self.1;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QIdentityProxyModel_headerData_to_output_section_orientation(original_self as *const ::identity_proxy_model::IdentityProxyModel, section, orientation as *const ::qt::Orientation, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> IdentityProxyModelHeaderDataArgs<'largs> for (::libc::c_int, &'largs ::qt::Orientation, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::identity_proxy_model::IdentityProxyModel) -> ::variant::Variant {
      let section = self.0;
      let orientation = self.1;
      let role = self.2;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QIdentityProxyModel_headerData_to_output_section_orientation_role(original_self as *const ::identity_proxy_model::IdentityProxyModel, section, orientation as *const ::qt::Orientation, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [IdentityProxyModel::index](../struct.IdentityProxyModel.html#method.index) method.
  pub trait IdentityProxyModelIndexArgs<'largs> {
    fn exec(self, original_self: &'largs ::identity_proxy_model::IdentityProxyModel) -> ::model_index::ModelIndex;
  }
  impl<'largs> IdentityProxyModelIndexArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::identity_proxy_model::IdentityProxyModel) -> ::model_index::ModelIndex {
      let row = self.0;
      let column = self.1;
      {
        let mut object: ::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QIdentityProxyModel_index_to_output_row_column(original_self as *const ::identity_proxy_model::IdentityProxyModel, row, column, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> IdentityProxyModelIndexArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs ::identity_proxy_model::IdentityProxyModel) -> ::model_index::ModelIndex {
      let row = self.0;
      let column = self.1;
      let parent = self.2;
      {
        let mut object: ::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QIdentityProxyModel_index_to_output_row_column_parent(original_self as *const ::identity_proxy_model::IdentityProxyModel, row, column, parent as *const ::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [IdentityProxyModel::insert_columns](../struct.IdentityProxyModel.html#method.insert_columns) method.
  pub trait IdentityProxyModelInsertColumnsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::identity_proxy_model::IdentityProxyModel) -> bool;
  }
  impl<'largs> IdentityProxyModelInsertColumnsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::identity_proxy_model::IdentityProxyModel) -> bool {
      let column = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_core_c_QIdentityProxyModel_insertColumns_column_count(original_self as *mut ::identity_proxy_model::IdentityProxyModel, column, count) }
    }
  }
  impl<'largs> IdentityProxyModelInsertColumnsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::identity_proxy_model::IdentityProxyModel) -> bool {
      let column = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QIdentityProxyModel_insertColumns_column_count_parent(original_self as *mut ::identity_proxy_model::IdentityProxyModel, column, count, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [IdentityProxyModel::insert_rows](../struct.IdentityProxyModel.html#method.insert_rows) method.
  pub trait IdentityProxyModelInsertRowsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::identity_proxy_model::IdentityProxyModel) -> bool;
  }
  impl<'largs> IdentityProxyModelInsertRowsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::identity_proxy_model::IdentityProxyModel) -> bool {
      let row = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_core_c_QIdentityProxyModel_insertRows_row_count(original_self as *mut ::identity_proxy_model::IdentityProxyModel, row, count) }
    }
  }
  impl<'largs> IdentityProxyModelInsertRowsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::identity_proxy_model::IdentityProxyModel) -> bool {
      let row = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QIdentityProxyModel_insertRows_row_count_parent(original_self as *mut ::identity_proxy_model::IdentityProxyModel, row, count, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [IdentityProxyModel::remove_columns](../struct.IdentityProxyModel.html#method.remove_columns) method.
  pub trait IdentityProxyModelRemoveColumnsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::identity_proxy_model::IdentityProxyModel) -> bool;
  }
  impl<'largs> IdentityProxyModelRemoveColumnsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::identity_proxy_model::IdentityProxyModel) -> bool {
      let column = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_core_c_QIdentityProxyModel_removeColumns_column_count(original_self as *mut ::identity_proxy_model::IdentityProxyModel, column, count) }
    }
  }
  impl<'largs> IdentityProxyModelRemoveColumnsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::identity_proxy_model::IdentityProxyModel) -> bool {
      let column = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QIdentityProxyModel_removeColumns_column_count_parent(original_self as *mut ::identity_proxy_model::IdentityProxyModel, column, count, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [IdentityProxyModel::remove_rows](../struct.IdentityProxyModel.html#method.remove_rows) method.
  pub trait IdentityProxyModelRemoveRowsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::identity_proxy_model::IdentityProxyModel) -> bool;
  }
  impl<'largs> IdentityProxyModelRemoveRowsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::identity_proxy_model::IdentityProxyModel) -> bool {
      let row = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_core_c_QIdentityProxyModel_removeRows_row_count(original_self as *mut ::identity_proxy_model::IdentityProxyModel, row, count) }
    }
  }
  impl<'largs> IdentityProxyModelRemoveRowsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::identity_proxy_model::IdentityProxyModel) -> bool {
      let row = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QIdentityProxyModel_removeRows_row_count_parent(original_self as *mut ::identity_proxy_model::IdentityProxyModel, row, count, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [IdentityProxyModel::row_count](../struct.IdentityProxyModel.html#method.row_count) method.
  pub trait IdentityProxyModelRowCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::identity_proxy_model::IdentityProxyModel) -> ::libc::c_int;
  }
  impl<'largs> IdentityProxyModelRowCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::identity_proxy_model::IdentityProxyModel) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QIdentityProxyModel_rowCount_no_args(original_self as *const ::identity_proxy_model::IdentityProxyModel) }
    }
  }
  impl<'largs> IdentityProxyModelRowCountArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::identity_proxy_model::IdentityProxyModel) -> ::libc::c_int {
      let parent = self;
      unsafe { ::ffi::qt_core_c_QIdentityProxyModel_rowCount_parent(original_self as *const ::identity_proxy_model::IdentityProxyModel, parent as *const ::model_index::ModelIndex) }
    }
  }
}
