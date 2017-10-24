/// C++ type: <span style='color: green;'>```QAbstractTableModel```</span>
#[repr(C)]
pub struct AbstractTableModel(u8);

impl AbstractTableModel {
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractTableModel::dropMimeData(const QMimeData* data, Qt::DropAction action, int row, int column, const QModelIndex& parent)```</span>
  ///
  ///
  pub unsafe fn drop_mime_data(&mut self,
                               data: *const ::mime_data::MimeData,
                               action: &::qt::DropAction,
                               row: ::libc::c_int,
                               column: ::libc::c_int,
                               parent: &::model_index::ModelIndex)
                               -> bool {
    ::ffi::qt_core_c_QAbstractTableModel_dropMimeData(self as *mut ::abstract_table_model::AbstractTableModel,
                                                      data,
                                                      action as *const ::qt::DropAction,
                                                      row,
                                                      column,
                                                      parent as *const ::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```QAbstractTableModel::index```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index(&self, (::libc::c_int, ::libc::c_int)) -> ::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QAbstractTableModel::index(int row, int column) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index(&self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> ::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QAbstractTableModel::index(int row, int column, const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn index<'largs, Args>(&'largs self, args: Args) -> ::model_index::ModelIndex
    where Args: overloading::AbstractTableModelIndexArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAbstractTableModel::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_core_c_QAbstractTableModel_metaObject(self as *const ::abstract_table_model::AbstractTableModel)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QAbstractTableModel::sibling(int row, int column, const QModelIndex& idx) const```</span>
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
        ::ffi::qt_core_c_QAbstractTableModel_sibling_to_output(self as *const ::abstract_table_model::AbstractTableModel, row, column, idx as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractTableModel::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractTableModel_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractTableModel::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractTableModel_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_table_model::AbstractTableModel {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QAbstractTableModel_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractTableModel`.
  pub struct Signals<'a>(&'a ::abstract_table_model::AbstractTableModel);
  /// Represents a built-in Qt signal `QAbstractTableModel::layoutAboutToBeChanged`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().layout_about_to_be_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct LayoutAboutToBeChanged<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for LayoutAboutToBeChanged<'a> {
    type Arguments = (&'static ::list::ListPersistentModelIndex, &'static ::abstract_item_model::LayoutChangeHint);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2layoutAboutToBeChanged(const QList< QPersistentModelIndex >&,QAbstractItemModel::LayoutChangeHint)\0"
    }
  }
  impl<'a> ::connection::Signal for LayoutAboutToBeChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::rowsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().rows_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct RowsAboutToBeRemoved<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for RowsAboutToBeRemoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsAboutToBeRemoved(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for RowsAboutToBeRemoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::columnsInserted`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().columns_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct ColumnsInserted<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for ColumnsInserted<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsInserted(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for ColumnsInserted<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::modelAboutToBeReset`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().model_about_to_be_reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct ModelAboutToBeReset<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for ModelAboutToBeReset<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2modelAboutToBeReset()\0"
    }
  }
  impl<'a> ::connection::Signal for ModelAboutToBeReset<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::rowsInserted`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().rows_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct RowsInserted<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for RowsInserted<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsInserted(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for RowsInserted<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::columnsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().columns_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct ColumnsAboutToBeRemoved<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for ColumnsAboutToBeRemoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsAboutToBeRemoved(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for ColumnsAboutToBeRemoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::headerDataChanged`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().header_data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct HeaderDataChanged<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for HeaderDataChanged<'a> {
    type Arguments = (&'static ::qt::Orientation, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2headerDataChanged(Qt::Orientation,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for HeaderDataChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::rowsMoved`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().rows_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct RowsMoved<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for RowsMoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex,
     ::libc::c_int,
     ::libc::c_int,
     &'static ::model_index::ModelIndex,
     ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsMoved(const QModelIndex&,int,int,const QModelIndex&,int)\0"
    }
  }
  impl<'a> ::connection::Signal for RowsMoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::layoutChanged`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().layout_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct LayoutChanged<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for LayoutChanged<'a> {
    type Arguments = (&'static ::list::ListPersistentModelIndex, &'static ::abstract_item_model::LayoutChangeHint);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2layoutChanged(const QList< QPersistentModelIndex >&,QAbstractItemModel::LayoutChangeHint)\0"
    }
  }
  impl<'a> ::connection::Signal for LayoutChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::rowsAboutToBeMoved`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().rows_about_to_be_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct RowsAboutToBeMoved<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for RowsAboutToBeMoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex,
     ::libc::c_int,
     ::libc::c_int,
     &'static ::model_index::ModelIndex,
     ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsAboutToBeMoved(const QModelIndex&,int,int,const QModelIndex&,int)\0"
    }
  }
  impl<'a> ::connection::Signal for RowsAboutToBeMoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::columnsMoved`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().columns_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct ColumnsMoved<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for ColumnsMoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex,
     ::libc::c_int,
     ::libc::c_int,
     &'static ::model_index::ModelIndex,
     ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsMoved(const QModelIndex&,int,int,const QModelIndex&,int)\0"
    }
  }
  impl<'a> ::connection::Signal for ColumnsMoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::columnsAboutToBeMoved`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().columns_about_to_be_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct ColumnsAboutToBeMoved<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for ColumnsAboutToBeMoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex,
     ::libc::c_int,
     ::libc::c_int,
     &'static ::model_index::ModelIndex,
     ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsAboutToBeMoved(const QModelIndex&,int,int,const QModelIndex&,int)\0"
    }
  }
  impl<'a> ::connection::Signal for ColumnsAboutToBeMoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::rowsAboutToBeInserted`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().rows_about_to_be_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct RowsAboutToBeInserted<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for RowsAboutToBeInserted<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsAboutToBeInserted(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for RowsAboutToBeInserted<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::rowsRemoved`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().rows_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct RowsRemoved<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for RowsRemoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsRemoved(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for RowsRemoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::dataChanged`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct DataChanged<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for DataChanged<'a> {
    type Arguments = (&'static ::model_index::ModelIndex,
     &'static ::model_index::ModelIndex,
     &'static ::vector::VectorCInt);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2dataChanged(const QModelIndex&,const QModelIndex&,const QVector< int >&)\0"
    }
  }
  impl<'a> ::connection::Signal for DataChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::columnsAboutToBeInserted`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().columns_about_to_be_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct ColumnsAboutToBeInserted<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for ColumnsAboutToBeInserted<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsAboutToBeInserted(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for ColumnsAboutToBeInserted<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::modelReset`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().model_reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct ModelReset<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for ModelReset<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2modelReset()\0"
    }
  }
  impl<'a> ::connection::Signal for ModelReset<'a> {}
  /// Represents a built-in Qt signal `QAbstractTableModel::columnsRemoved`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.signals().columns_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct ColumnsRemoved<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for ColumnsRemoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsRemoved(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for ColumnsRemoved<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::layoutAboutToBeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_about_to_be_changed(&self) -> LayoutAboutToBeChanged {
      LayoutAboutToBeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::rowsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_removed(&self) -> RowsAboutToBeRemoved {
      RowsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::columnsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_inserted(&self) -> ColumnsInserted {
      ColumnsInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::modelAboutToBeReset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn model_about_to_be_reset(&self) -> ModelAboutToBeReset {
      ModelAboutToBeReset(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::rowsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_inserted(&self) -> RowsInserted {
      RowsInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::columnsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_removed(&self) -> ColumnsAboutToBeRemoved {
      ColumnsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::headerDataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn header_data_changed(&self) -> HeaderDataChanged {
      HeaderDataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::rowsMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_moved(&self) -> RowsMoved {
      RowsMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::layoutChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_changed(&self) -> LayoutChanged {
      LayoutChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::rowsAboutToBeMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_moved(&self) -> RowsAboutToBeMoved {
      RowsAboutToBeMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::columnsMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_moved(&self) -> ColumnsMoved {
      ColumnsMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::columnsAboutToBeMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_moved(&self) -> ColumnsAboutToBeMoved {
      ColumnsAboutToBeMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::rowsAboutToBeInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_inserted(&self) -> RowsAboutToBeInserted {
      RowsAboutToBeInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::rowsRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_removed(&self) -> RowsRemoved {
      RowsRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::dataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn data_changed(&self) -> DataChanged {
      DataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::columnsAboutToBeInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_inserted(&self) -> ColumnsAboutToBeInserted {
      ColumnsAboutToBeInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::modelReset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn model_reset(&self) -> ModelReset {
      ModelReset(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTableModel::columnsRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_removed(&self) -> ColumnsRemoved {
      ColumnsRemoved(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AbstractTableModel`.
  pub struct Slots<'a>(&'a ::abstract_table_model::AbstractTableModel);
  /// Represents a built-in Qt slot `QAbstractTableModel::resetInternalData`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.slots().reset_internal_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct ResetInternalData<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for ResetInternalData<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resetInternalData()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractTableModel::revert`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.slots().revert()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct Revert<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for Revert<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1revert()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractTableModel::submit`.
  ///
  /// An object of this type can be created from `AbstractTableModel` with `object.slots().submit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTableModel` object.
  pub struct Submit<'a>(&'a ::abstract_table_model::AbstractTableModel);
  impl<'a> ::connection::Receiver for Submit<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1submit()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QAbstractTableModel::resetInternalData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset_internal_data(&self) -> ResetInternalData {
      ResetInternalData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractTableModel::revert`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn revert(&self) -> Revert {
      Revert(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractTableModel::submit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn submit(&self) -> Submit {
      Submit(self.0)
    }
  }
  impl ::abstract_table_model::AbstractTableModel {
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

impl ::cpp_utils::DynamicCast<::abstract_table_model::AbstractTableModel> for ::abstract_item_model::AbstractItemModel {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_table_model::AbstractTableModel> {
let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTableModel_G_dynamic_cast_QAbstractTableModel_ptr_QAbstractItemModel(self as *mut ::abstract_item_model::AbstractItemModel) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_table_model::AbstractTableModel> {
let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTableModel_G_dynamic_cast_QAbstractTableModel_ptr_QAbstractItemModel(self as *const ::abstract_item_model::AbstractItemModel as *mut ::abstract_item_model::AbstractItemModel) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::abstract_table_model::AbstractTableModel> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_table_model::AbstractTableModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTableModel_G_dynamic_cast_QAbstractTableModel_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_table_model::AbstractTableModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTableModel_G_dynamic_cast_QAbstractTableModel_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_model::AbstractItemModel> for ::abstract_table_model::AbstractTableModel {
fn static_cast_mut(&mut self) -> &mut ::abstract_item_model::AbstractItemModel {
let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTableModel_G_static_cast_QAbstractItemModel_ptr(self as *mut ::abstract_table_model::AbstractTableModel) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_item_model::AbstractItemModel {
let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTableModel_G_static_cast_QAbstractItemModel_ptr(self as *const ::abstract_table_model::AbstractTableModel as *mut ::abstract_table_model::AbstractTableModel) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::object::Object> for ::abstract_table_model::AbstractTableModel {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTableModel_G_static_cast_QObject_ptr(self as *mut ::abstract_table_model::AbstractTableModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTableModel_G_static_cast_QObject_ptr(self as *const ::abstract_table_model::AbstractTableModel as *mut ::abstract_table_model::AbstractTableModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_table_model::AbstractTableModel> for ::abstract_item_model::AbstractItemModel {
unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_table_model::AbstractTableModel {
let ffi_result = ::ffi::qt_core_c_QAbstractTableModel_G_static_cast_QAbstractTableModel_ptr_QAbstractItemModel(self as *mut ::abstract_item_model::AbstractItemModel);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::abstract_table_model::AbstractTableModel {
let ffi_result = ::ffi::qt_core_c_QAbstractTableModel_G_static_cast_QAbstractTableModel_ptr_QAbstractItemModel(self as *const ::abstract_item_model::AbstractItemModel as *mut ::abstract_item_model::AbstractItemModel);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::abstract_table_model::AbstractTableModel> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_table_model::AbstractTableModel {
    let ffi_result = ::ffi::qt_core_c_QAbstractTableModel_G_static_cast_QAbstractTableModel_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_table_model::AbstractTableModel {
    let ffi_result = ::ffi::qt_core_c_QAbstractTableModel_G_static_cast_QAbstractTableModel_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_table_model::AbstractTableModel {
  type Target = ::abstract_item_model::AbstractItemModel;
  fn deref(&self) -> &::abstract_item_model::AbstractItemModel {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTableModel_G_static_cast_QAbstractItemModel_ptr(self as *const ::abstract_table_model::AbstractTableModel as *mut ::abstract_table_model::AbstractTableModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_table_model::AbstractTableModel {
  fn deref_mut(&mut self) -> &mut ::abstract_item_model::AbstractItemModel {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTableModel_G_static_cast_QAbstractItemModel_ptr(self as *mut ::abstract_table_model::AbstractTableModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AbstractTableModel::index](../struct.AbstractTableModel.html#method.index) method.
  pub trait AbstractTableModelIndexArgs<'largs> {
    fn exec(self, original_self: &'largs ::abstract_table_model::AbstractTableModel) -> ::model_index::ModelIndex;
  }
  impl<'largs> AbstractTableModelIndexArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::abstract_table_model::AbstractTableModel) -> ::model_index::ModelIndex {
      let row = self.0;
      let column = self.1;
      {
        let mut object: ::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QAbstractTableModel_index_to_output_row_column(original_self as *const ::abstract_table_model::AbstractTableModel, row, column, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> AbstractTableModelIndexArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs ::abstract_table_model::AbstractTableModel) -> ::model_index::ModelIndex {
      let row = self.0;
      let column = self.1;
      let parent = self.2;
      {
        let mut object: ::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QAbstractTableModel_index_to_output_row_column_parent(original_self as *const ::abstract_table_model::AbstractTableModel, row, column, parent as *const ::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
}
