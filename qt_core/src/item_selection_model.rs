/// C++ type: <span style='color: green;'>```QItemSelectionModel```</span>
#[repr(C)]
pub struct ItemSelectionModel(u8);

impl ItemSelectionModel {
  /// C++ method: <span style='color: green;'>```virtual [slot] void QItemSelectionModel::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QItemSelectionModel_clear(self as *mut ::item_selection_model::ItemSelectionModel) }
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QItemSelectionModel::clearCurrentIndex()```</span>
  ///
  ///
  pub fn clear_current_index(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QItemSelectionModel_clearCurrentIndex(self as *mut ::item_selection_model::ItemSelectionModel)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QItemSelectionModel::clearSelection()```</span>
  ///
  ///
  pub fn clear_selection(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QItemSelectionModel_clearSelection(self as *mut ::item_selection_model::ItemSelectionModel)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QItemSelectionModel::columnIntersectsSelection(int column, const QModelIndex& parent) const```</span>
  ///
  ///
  pub fn column_intersects_selection(&self, column: ::libc::c_int, parent: &::model_index::ModelIndex) -> bool {
    unsafe { ::ffi::qt_core_c_QItemSelectionModel_columnIntersectsSelection(self as *const ::item_selection_model::ItemSelectionModel, column, parent as *const ::model_index::ModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QItemSelectionModel::currentIndex() const```</span>
  ///
  ///
  pub fn current_index(&self) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QItemSelectionModel_currentIndex_to_output(self as *const ::item_selection_model::ItemSelectionModel, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QItemSelectionModel::hasSelection() const```</span>
  ///
  ///
  pub fn has_selection(&self) -> bool {
    unsafe {
      ::ffi::qt_core_c_QItemSelectionModel_hasSelection(self as *const ::item_selection_model::ItemSelectionModel)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QItemSelectionModel::isColumnSelected(int column, const QModelIndex& parent) const```</span>
  ///
  ///
  pub fn is_column_selected(&self, column: ::libc::c_int, parent: &::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QItemSelectionModel_isColumnSelected(self as *const ::item_selection_model::ItemSelectionModel,
                                                            column,
                                                            parent as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QItemSelectionModel::isRowSelected(int row, const QModelIndex& parent) const```</span>
  ///
  ///
  pub fn is_row_selected(&self, row: ::libc::c_int, parent: &::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QItemSelectionModel_isRowSelected(self as *const ::item_selection_model::ItemSelectionModel,
                                                         row,
                                                         parent as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QItemSelectionModel::isSelected(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn is_selected(&self, index: &::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QItemSelectionModel_isSelected(self as *const ::item_selection_model::ItemSelectionModel,
                                                      index as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QItemSelectionModel::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_core_c_QItemSelectionModel_metaObject(self as *const ::item_selection_model::ItemSelectionModel)
    }
  }

  /// C++ method: <span style='color: green;'>```const QAbstractItemModel* QItemSelectionModel::model() const```</span>
  ///
  ///
  pub fn model(&self) -> *const ::abstract_item_model::AbstractItemModel {
    unsafe {
      ::ffi::qt_core_c_QItemSelectionModel_model_const(self as *const ::item_selection_model::ItemSelectionModel)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemModel* QItemSelectionModel::model()```</span>
  ///
  ///
  pub fn model_mut(&mut self) -> *mut ::abstract_item_model::AbstractItemModel {
    unsafe { ::ffi::qt_core_c_QItemSelectionModel_model(self as *mut ::item_selection_model::ItemSelectionModel) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QItemSelectionModel::QItemSelectionModel()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::item_selection_model::ItemSelectionModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QItemSelectionModel_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QItemSelectionModel::QItemSelectionModel```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::abstract_item_model::AbstractItemModel) -> ::cpp_utils::CppBox<::item_selection_model::ItemSelectionModel>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QItemSelectionModel::QItemSelectionModel(QAbstractItemModel* model = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::abstract_item_model::AbstractItemModel, *mut ::object::Object)) -> ::cpp_utils::CppBox<::item_selection_model::ItemSelectionModel>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QItemSelectionModel::QItemSelectionModel(QAbstractItemModel* model, QObject* parent)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::item_selection_model::ItemSelectionModel>
    where Args: overloading::ItemSelectionModelNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual [slot] void QItemSelectionModel::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_core_c_QItemSelectionModel_reset(self as *mut ::item_selection_model::ItemSelectionModel) }
  }

  /// C++ method: <span style='color: green;'>```bool QItemSelectionModel::rowIntersectsSelection(int row, const QModelIndex& parent) const```</span>
  ///
  ///
  pub fn row_intersects_selection(&self, row: ::libc::c_int, parent: &::model_index::ModelIndex) -> bool {
    unsafe { ::ffi::qt_core_c_QItemSelectionModel_rowIntersectsSelection(self as *const ::item_selection_model::ItemSelectionModel, row, parent as *const ::model_index::ModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```QItemSelectionModel::selectedColumns```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn selected_columns(&self, ()) -> ::list::ListModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QModelIndex> QItemSelectionModel::selectedColumns() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn selected_columns(&self, ::libc::c_int) -> ::list::ListModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QModelIndex> QItemSelectionModel::selectedColumns(int row = ?) const```</span>
  ///
  ///
  pub fn selected_columns<'largs, Args>(&'largs self, args: Args) -> ::list::ListModelIndex
    where Args: overloading::ItemSelectionModelSelectedColumnsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QModelIndex> QItemSelectionModel::selectedIndexes() const```</span>
  ///
  ///
  pub fn selected_indexes(&self) -> ::list::ListModelIndex {
    {
      let mut object: ::list::ListModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QItemSelectionModel_selectedIndexes_to_output(self as *const ::item_selection_model::ItemSelectionModel, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QItemSelectionModel::selectedRows```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn selected_rows(&self, ()) -> ::list::ListModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QModelIndex> QItemSelectionModel::selectedRows() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn selected_rows(&self, ::libc::c_int) -> ::list::ListModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QModelIndex> QItemSelectionModel::selectedRows(int column = ?) const```</span>
  ///
  ///
  pub fn selected_rows<'largs, Args>(&'largs self, args: Args) -> ::list::ListModelIndex
    where Args: overloading::ItemSelectionModelSelectedRowsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QItemSelection QItemSelectionModel::selection() const```</span>
  ///
  ///
  pub fn selection(&self) -> ::item_selection::ItemSelection {
    {
      let mut object: ::item_selection::ItemSelection =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QItemSelectionModel_selection_to_output(self as *const ::item_selection_model::ItemSelectionModel, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QItemSelectionModel::setModel(QAbstractItemModel* model)```</span>
  ///
  ///
  pub unsafe fn set_model(&mut self, model: *mut ::abstract_item_model::AbstractItemModel) {
    ::ffi::qt_core_c_QItemSelectionModel_setModel(self as *mut ::item_selection_model::ItemSelectionModel,
                                                  model)
  }

  /// C++ method: <span style='color: green;'>```static QString QItemSelectionModel::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QItemSelectionModel_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QItemSelectionModel::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QItemSelectionModel_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::item_selection_model::ItemSelectionModel {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QItemSelectionModel_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ItemSelectionModel`.
  pub struct Signals<'a>(&'a ::item_selection_model::ItemSelectionModel);
  /// Represents a built-in Qt signal `QItemSelectionModel::objectNameChanged`.
  ///
  /// An object of this type can be created from `ItemSelectionModel` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ItemSelectionModel` object.
  pub struct ObjectNameChanged<'a>(&'a ::item_selection_model::ItemSelectionModel);
  impl<'a> ::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::connection::Signal for ObjectNameChanged<'a> {}
  /// Represents a built-in Qt signal `QItemSelectionModel::currentColumnChanged`.
  ///
  /// An object of this type can be created from `ItemSelectionModel` with `object.signals().current_column_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ItemSelectionModel` object.
  pub struct CurrentColumnChanged<'a>(&'a ::item_selection_model::ItemSelectionModel);
  impl<'a> ::connection::Receiver for CurrentColumnChanged<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, &'static ::model_index::ModelIndex);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentColumnChanged(const QModelIndex&,const QModelIndex&)\0"
    }
  }
  impl<'a> ::connection::Signal for CurrentColumnChanged<'a> {}
  /// Represents a built-in Qt signal `QItemSelectionModel::selectionChanged`.
  ///
  /// An object of this type can be created from `ItemSelectionModel` with `object.signals().selection_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ItemSelectionModel` object.
  pub struct SelectionChanged<'a>(&'a ::item_selection_model::ItemSelectionModel);
  impl<'a> ::connection::Receiver for SelectionChanged<'a> {
    type Arguments = (&'static ::item_selection::ItemSelection, &'static ::item_selection::ItemSelection);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2selectionChanged(const QItemSelection&,const QItemSelection&)\0"
    }
  }
  impl<'a> ::connection::Signal for SelectionChanged<'a> {}
  /// Represents a built-in Qt signal `QItemSelectionModel::currentChanged`.
  ///
  /// An object of this type can be created from `ItemSelectionModel` with `object.signals().current_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ItemSelectionModel` object.
  pub struct CurrentChanged<'a>(&'a ::item_selection_model::ItemSelectionModel);
  impl<'a> ::connection::Receiver for CurrentChanged<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, &'static ::model_index::ModelIndex);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentChanged(const QModelIndex&,const QModelIndex&)\0"
    }
  }
  impl<'a> ::connection::Signal for CurrentChanged<'a> {}
  /// Represents a built-in Qt signal `QItemSelectionModel::currentRowChanged`.
  ///
  /// An object of this type can be created from `ItemSelectionModel` with `object.signals().current_row_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ItemSelectionModel` object.
  pub struct CurrentRowChanged<'a>(&'a ::item_selection_model::ItemSelectionModel);
  impl<'a> ::connection::Receiver for CurrentRowChanged<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, &'static ::model_index::ModelIndex);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentRowChanged(const QModelIndex&,const QModelIndex&)\0"
    }
  }
  impl<'a> ::connection::Signal for CurrentRowChanged<'a> {}
  /// Represents a built-in Qt signal `QItemSelectionModel::modelChanged`.
  ///
  /// An object of this type can be created from `ItemSelectionModel` with `object.signals().model_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ItemSelectionModel` object.
  pub struct ModelChanged<'a>(&'a ::item_selection_model::ItemSelectionModel);
  impl<'a> ::connection::Receiver for ModelChanged<'a> {
    type Arguments = (*mut ::abstract_item_model::AbstractItemModel,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2modelChanged(QAbstractItemModel*)\0"
    }
  }
  impl<'a> ::connection::Signal for ModelChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QItemSelectionModel::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QItemSelectionModel::currentColumnChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_column_changed(&self) -> CurrentColumnChanged {
      CurrentColumnChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QItemSelectionModel::selectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn selection_changed(&self) -> SelectionChanged {
      SelectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QItemSelectionModel::currentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_changed(&self) -> CurrentChanged {
      CurrentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QItemSelectionModel::currentRowChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_row_changed(&self) -> CurrentRowChanged {
      CurrentRowChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QItemSelectionModel::modelChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn model_changed(&self) -> ModelChanged {
      ModelChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ItemSelectionModel`.
  pub struct Slots<'a>(&'a ::item_selection_model::ItemSelectionModel);
  /// Represents a built-in Qt slot `QItemSelectionModel::clear`.
  ///
  /// An object of this type can be created from `ItemSelectionModel` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ItemSelectionModel` object.
  pub struct Clear<'a>(&'a ::item_selection_model::ItemSelectionModel);
  impl<'a> ::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QItemSelectionModel::clearSelection`.
  ///
  /// An object of this type can be created from `ItemSelectionModel` with `object.slots().clear_selection()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ItemSelectionModel` object.
  pub struct ClearSelection<'a>(&'a ::item_selection_model::ItemSelectionModel);
  impl<'a> ::connection::Receiver for ClearSelection<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clearSelection()\0"
    }
  }
  /// Represents a built-in Qt slot `QItemSelectionModel::reset`.
  ///
  /// An object of this type can be created from `ItemSelectionModel` with `object.slots().reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ItemSelectionModel` object.
  pub struct Reset<'a>(&'a ::item_selection_model::ItemSelectionModel);
  impl<'a> ::connection::Receiver for Reset<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reset()\0"
    }
  }
  /// Represents a built-in Qt slot `QItemSelectionModel::clearCurrentIndex`.
  ///
  /// An object of this type can be created from `ItemSelectionModel` with `object.slots().clear_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ItemSelectionModel` object.
  pub struct ClearCurrentIndex<'a>(&'a ::item_selection_model::ItemSelectionModel);
  impl<'a> ::connection::Receiver for ClearCurrentIndex<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clearCurrentIndex()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QItemSelectionModel::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QItemSelectionModel::clearSelection`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_selection(&self) -> ClearSelection {
      ClearSelection(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QItemSelectionModel::reset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset(&self) -> Reset {
      Reset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QItemSelectionModel::clearCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_current_index(&self) -> ClearCurrentIndex {
      ClearCurrentIndex(self.0)
    }
  }
  impl ::item_selection_model::ItemSelectionModel {
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

/// C++ type: <span style='color: green;'>```QItemSelectionModel::SelectionFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SelectionFlag {
  /// C++ enum variant: <span style='color: green;'>```NoUpdate = 0```</span>
  NoUpdate = 0,
  /// C++ enum variant: <span style='color: green;'>```Clear = 1```</span>
  Clear = 1,
  /// C++ enum variant: <span style='color: green;'>```Select = 2```</span>
  Select = 2,
  /// C++ enum variant: <span style='color: green;'>```ClearAndSelect = 3```</span>
  ClearAndSelect = 3,
  /// C++ enum variant: <span style='color: green;'>```Deselect = 4```</span>
  Deselect = 4,
  /// C++ enum variant: <span style='color: green;'>```Toggle = 8```</span>
  Toggle = 8,
  /// C++ enum variant: <span style='color: green;'>```Current = 16```</span>
  Current = 16,
  /// C++ enum variant: <span style='color: green;'>```SelectCurrent = 18```</span>
  SelectCurrent = 18,
  /// C++ enum variant: <span style='color: green;'>```ToggleCurrent = 24```</span>
  ToggleCurrent = 24,
  /// C++ enum variant: <span style='color: green;'>```Rows = 32```</span>
  Rows = 32,
  /// C++ enum variant: <span style='color: green;'>```Columns = 64```</span>
  Columns = 64,
}

/// C++ method: <span style='color: green;'>```unsigned int qHash(const QItemSelectionRange& arg1)```</span>
///
///
pub fn hash(arg1: &::item_selection_range::ItemSelectionRange) -> ::libc::c_uint {
  unsafe { ::ffi::qt_core_c_QItemSelectionModel_G_qHash(arg1 as *const ::item_selection_range::ItemSelectionRange) }
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QItemSelectionRange& arg2)```</span>
///
///
pub fn op_shl(arg1: &::debug::Debug, arg2: &::item_selection_range::ItemSelectionRange) -> ::debug::Debug {
  {
    let mut object: ::debug::Debug = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QItemSelectionModel_G_operator_shl_to_output(arg1 as *const ::debug::Debug, arg2 as *const ::item_selection_range::ItemSelectionRange, &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```void swap(QItemSelection& value1, QItemSelection& value2)```</span>
///
///
pub fn swap(value1: &mut ::item_selection::ItemSelection, value2: &mut ::item_selection::ItemSelection) {
  unsafe {
    ::ffi::qt_core_c_QItemSelectionModel_G_swap(value1 as *mut ::item_selection::ItemSelection,
                                                value2 as *mut ::item_selection::ItemSelection)
  }
}

impl ::cpp_utils::DynamicCast<::item_selection_model::ItemSelectionModel> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::item_selection_model::ItemSelectionModel> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QItemSelectionModel_G_dynamic_cast_QItemSelectionModel_ptr(self as *mut ::object::Object)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::item_selection_model::ItemSelectionModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QItemSelectionModel_G_dynamic_cast_QItemSelectionModel_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::item_selection_model::ItemSelectionModel {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QItemSelectionModel_G_static_cast_QObject_ptr(self as *mut ::item_selection_model::ItemSelectionModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QItemSelectionModel_G_static_cast_QObject_ptr(self as *const ::item_selection_model::ItemSelectionModel as *mut ::item_selection_model::ItemSelectionModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::item_selection_model::ItemSelectionModel> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::item_selection_model::ItemSelectionModel {
    let ffi_result =
      ::ffi::qt_core_c_QItemSelectionModel_G_static_cast_QItemSelectionModel_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::item_selection_model::ItemSelectionModel {
    let ffi_result = ::ffi::qt_core_c_QItemSelectionModel_G_static_cast_QItemSelectionModel_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::item_selection_model::ItemSelectionModel {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QItemSelectionModel_G_static_cast_QObject_ptr(self as *const ::item_selection_model::ItemSelectionModel as *mut ::item_selection_model::ItemSelectionModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::item_selection_model::ItemSelectionModel {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QItemSelectionModel_G_static_cast_QObject_ptr(self as *mut ::item_selection_model::ItemSelectionModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ItemSelectionModel::new_unsafe](../struct.ItemSelectionModel.html#method.new_unsafe) method.
  pub trait ItemSelectionModelNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::item_selection_model::ItemSelectionModel>;
  }
  impl ItemSelectionModelNewUnsafeArgs for *mut ::abstract_item_model::AbstractItemModel {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::item_selection_model::ItemSelectionModel> {
      let model = self;
      let ffi_result = ::ffi::qt_core_c_QItemSelectionModel_new_model(model);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ItemSelectionModelNewUnsafeArgs for (*mut ::abstract_item_model::AbstractItemModel, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::item_selection_model::ItemSelectionModel> {
      let model = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_core_c_QItemSelectionModel_new_model_parent(model, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [ItemSelectionModel::selected_columns](../struct.ItemSelectionModel.html#method.selected_columns) method.
  pub trait ItemSelectionModelSelectedColumnsArgs<'largs> {
    fn exec(self, original_self: &'largs ::item_selection_model::ItemSelectionModel) -> ::list::ListModelIndex;
  }
  impl<'largs> ItemSelectionModelSelectedColumnsArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::item_selection_model::ItemSelectionModel) -> ::list::ListModelIndex {

      {
        let mut object: ::list::ListModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QItemSelectionModel_selectedColumns_to_output_no_args(original_self as *const ::item_selection_model::ItemSelectionModel, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ItemSelectionModelSelectedColumnsArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::item_selection_model::ItemSelectionModel) -> ::list::ListModelIndex {
      let row = self;
      {
        let mut object: ::list::ListModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QItemSelectionModel_selectedColumns_to_output_row(original_self as *const ::item_selection_model::ItemSelectionModel, row, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ItemSelectionModel::selected_rows](../struct.ItemSelectionModel.html#method.selected_rows) method.
  pub trait ItemSelectionModelSelectedRowsArgs<'largs> {
    fn exec(self, original_self: &'largs ::item_selection_model::ItemSelectionModel) -> ::list::ListModelIndex;
  }
  impl<'largs> ItemSelectionModelSelectedRowsArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::item_selection_model::ItemSelectionModel) -> ::list::ListModelIndex {
      let column = self;
      {
        let mut object: ::list::ListModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QItemSelectionModel_selectedRows_to_output_column(original_self as *const ::item_selection_model::ItemSelectionModel, column, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ItemSelectionModelSelectedRowsArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::item_selection_model::ItemSelectionModel) -> ::list::ListModelIndex {

      {
        let mut object: ::list::ListModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QItemSelectionModel_selectedRows_to_output_no_args(original_self as *const ::item_selection_model::ItemSelectionModel, &mut object);
        }
        object
      }
    }
  }
}
