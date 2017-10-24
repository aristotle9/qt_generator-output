/// C++ type: <span style='color: green;'>```QCompleter```</span>
#[repr(C)]
pub struct Completer(u8);

impl Completer {
  /// C++ method: <span style='color: green;'>```QCompleter::complete```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn complete(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QCompleter::complete()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn complete(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QCompleter::complete(const QRect& rect = ?)```</span>
  ///
  ///
  pub fn complete<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::CompleterCompleteArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QCompleter::completionColumn() const```</span>
  ///
  ///
  pub fn completion_column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QCompleter_completionColumn(self as *const ::completer::Completer) }
  }

  /// C++ method: <span style='color: green;'>```int QCompleter::completionCount() const```</span>
  ///
  ///
  pub fn completion_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QCompleter_completionCount(self as *const ::completer::Completer) }
  }

  /// C++ method: <span style='color: green;'>```QCompleter::CompletionMode QCompleter::completionMode() const```</span>
  ///
  ///
  pub fn completion_mode(&self) -> ::completer::CompletionMode {
    unsafe { ::ffi::qt_widgets_c_QCompleter_completionMode(self as *const ::completer::Completer) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemModel* QCompleter::completionModel() const```</span>
  ///
  ///
  pub fn completion_model(&self) -> *mut ::qt_core::abstract_item_model::AbstractItemModel {
    unsafe { ::ffi::qt_widgets_c_QCompleter_completionModel(self as *const ::completer::Completer) }
  }

  /// C++ method: <span style='color: green;'>```QString QCompleter::completionPrefix() const```</span>
  ///
  ///
  pub fn completion_prefix(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCompleter_completionPrefix_to_output(self as *const ::completer::Completer, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QCompleter::completionRole() const```</span>
  ///
  ///
  pub fn completion_role(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QCompleter_completionRole(self as *const ::completer::Completer) }
  }

  /// C++ method: <span style='color: green;'>```QString QCompleter::currentCompletion() const```</span>
  ///
  ///
  pub fn current_completion(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCompleter_currentCompletion_to_output(self as *const ::completer::Completer, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QCompleter::currentIndex() const```</span>
  ///
  ///
  pub fn current_index(&self) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCompleter_currentIndex_to_output(self as *const ::completer::Completer, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QCompleter::currentRow() const```</span>
  ///
  ///
  pub fn current_row(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QCompleter_currentRow(self as *const ::completer::Completer) }
  }

  /// C++ method: <span style='color: green;'>```int QCompleter::maxVisibleItems() const```</span>
  ///
  ///
  pub fn max_visible_items(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QCompleter_maxVisibleItems(self as *const ::completer::Completer) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QCompleter::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QCompleter_metaObject(self as *const ::completer::Completer) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemModel* QCompleter::model() const```</span>
  ///
  ///
  pub fn model(&self) -> *mut ::qt_core::abstract_item_model::AbstractItemModel {
    unsafe { ::ffi::qt_widgets_c_QCompleter_model(self as *const ::completer::Completer) }
  }

  /// C++ method: <span style='color: green;'>```QCompleter::ModelSorting QCompleter::modelSorting() const```</span>
  ///
  ///
  pub fn model_sorting(&self) -> ::completer::ModelSorting {
    unsafe { ::ffi::qt_widgets_c_QCompleter_modelSorting(self as *const ::completer::Completer) }
  }

  /// C++ method: <span style='color: green;'>```QCompleter::QCompleter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::completer::Completer>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCompleter::QCompleter()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string_list::StringList) -> ::cpp_utils::CppBox<::completer::Completer>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCompleter::QCompleter(const QStringList& completions)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::completer::Completer>
    where Args: overloading::CompleterNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QCompleter::QCompleter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::abstract_item_model::AbstractItemModel) -> ::cpp_utils::CppBox<::completer::Completer>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCompleter::QCompleter(QAbstractItemModel* model)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::qt_core::abstract_item_model::AbstractItemModel, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::completer::Completer>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCompleter::QCompleter(QAbstractItemModel* model, QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::completer::Completer>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCompleter::QCompleter(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string_list::StringList, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::completer::Completer>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCompleter::QCompleter(const QStringList& completions, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::completer::Completer>
    where Args: overloading::CompleterNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual QString QCompleter::pathFromIndex(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn path_from_index(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCompleter_pathFromIndex_to_output(self as *const ::completer::Completer,
                                                               index as *const ::qt_core::model_index::ModelIndex,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemView* QCompleter::popup() const```</span>
  ///
  ///
  pub fn popup(&self) -> *mut ::abstract_item_view::AbstractItemView {
    unsafe { ::ffi::qt_widgets_c_QCompleter_popup(self as *const ::completer::Completer) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QCompleter::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QCompleter_qt_metacall(self as *mut ::completer::Completer,
                                               arg1 as *const ::qt_core::meta_object::Call,
                                               arg2,
                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QCompleter::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QCompleter_qt_metacast(self as *mut ::completer::Completer, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QCompleter::setCaseSensitivity(Qt::CaseSensitivity caseSensitivity)```</span>
  ///
  ///
  pub fn set_case_sensitivity(&mut self, case_sensitivity: &::qt_core::qt::CaseSensitivity) {
    unsafe {
      ::ffi::qt_widgets_c_QCompleter_setCaseSensitivity(self as *mut ::completer::Completer,
                                                        case_sensitivity as *const ::qt_core::qt::CaseSensitivity)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCompleter::setCompletionColumn(int column)```</span>
  ///
  ///
  pub fn set_completion_column(&mut self, column: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QCompleter_setCompletionColumn(self as *mut ::completer::Completer, column) }
  }

  /// C++ method: <span style='color: green;'>```void QCompleter::setCompletionMode(QCompleter::CompletionMode mode)```</span>
  ///
  ///
  pub fn set_completion_mode(&mut self, mode: ::completer::CompletionMode) {
    unsafe { ::ffi::qt_widgets_c_QCompleter_setCompletionMode(self as *mut ::completer::Completer, mode) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QCompleter::setCompletionPrefix(const QString& prefix)```</span>
  ///
  ///
  pub fn set_completion_prefix(&mut self, prefix: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QCompleter_setCompletionPrefix(self as *mut ::completer::Completer,
                                                         prefix as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCompleter::setCompletionRole(int role)```</span>
  ///
  ///
  pub fn set_completion_role(&mut self, role: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QCompleter_setCompletionRole(self as *mut ::completer::Completer, role) }
  }

  /// C++ method: <span style='color: green;'>```bool QCompleter::setCurrentRow(int row)```</span>
  ///
  ///
  pub fn set_current_row(&mut self, row: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_widgets_c_QCompleter_setCurrentRow(self as *mut ::completer::Completer, row) }
  }

  /// C++ method: <span style='color: green;'>```void QCompleter::setMaxVisibleItems(int maxItems)```</span>
  ///
  ///
  pub fn set_max_visible_items(&mut self, max_items: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QCompleter_setMaxVisibleItems(self as *mut ::completer::Completer, max_items) }
  }

  /// C++ method: <span style='color: green;'>```void QCompleter::setModel(QAbstractItemModel* c)```</span>
  ///
  ///
  pub unsafe fn set_model(&mut self, c: *mut ::qt_core::abstract_item_model::AbstractItemModel) {
    ::ffi::qt_widgets_c_QCompleter_setModel(self as *mut ::completer::Completer, c)
  }

  /// C++ method: <span style='color: green;'>```void QCompleter::setModelSorting(QCompleter::ModelSorting sorting)```</span>
  ///
  ///
  pub fn set_model_sorting(&mut self, sorting: ::completer::ModelSorting) {
    unsafe { ::ffi::qt_widgets_c_QCompleter_setModelSorting(self as *mut ::completer::Completer, sorting) }
  }

  /// C++ method: <span style='color: green;'>```void QCompleter::setPopup(QAbstractItemView* popup)```</span>
  ///
  ///
  pub unsafe fn set_popup(&mut self, popup: *mut ::abstract_item_view::AbstractItemView) {
    ::ffi::qt_widgets_c_QCompleter_setPopup(self as *mut ::completer::Completer, popup)
  }

  /// C++ method: <span style='color: green;'>```void QCompleter::setWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QCompleter_setWidget(self as *mut ::completer::Completer, widget)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QCompleter::setWrapAround(bool wrap)```</span>
  ///
  ///
  pub fn set_wrap_around(&mut self, wrap: bool) {
    unsafe { ::ffi::qt_widgets_c_QCompleter_setWrapAround(self as *mut ::completer::Completer, wrap) }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList QCompleter::splitPath(const QString& path) const```</span>
  ///
  ///
  pub fn split_path(&self, path: &::qt_core::string::String) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCompleter_splitPath_to_output(self as *const ::completer::Completer,
                                                           path as *const ::qt_core::string::String,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QCompleter::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QCompleter_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QCompleter::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QCompleter_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QCompleter::widget() const```</span>
  ///
  ///
  pub fn widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QCompleter_widget(self as *const ::completer::Completer) }
  }

  /// C++ method: <span style='color: green;'>```bool QCompleter::wrapAround() const```</span>
  ///
  ///
  pub fn wrap_around(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QCompleter_wrapAround(self as *const ::completer::Completer) }
  }
}

impl ::cpp_utils::CppDeletable for ::completer::Completer {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QCompleter_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Completer`.
  pub struct Signals<'a>(&'a ::completer::Completer);
  /// Represents a built-in Qt signal `QCompleter::activated`.
  ///
  /// An object of this type can be created from `Completer` with `object.signals().activated_qt_core_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Completer` object.
  pub struct ActivatedQtCoreStringRef<'a>(&'a ::completer::Completer);
  impl<'a> ::qt_core::connection::Receiver for ActivatedQtCoreStringRef<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activated(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActivatedQtCoreStringRef<'a> {}
  /// Represents a built-in Qt signal `QCompleter::activated`.
  ///
  /// An object of this type can be created from `Completer` with `object.signals().activated_qt_core_model_index_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Completer` object.
  pub struct ActivatedQtCoreModelIndexRef<'a>(&'a ::completer::Completer);
  impl<'a> ::qt_core::connection::Receiver for ActivatedQtCoreModelIndexRef<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activated(const QModelIndex&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActivatedQtCoreModelIndexRef<'a> {}
  /// Represents a built-in Qt signal `QCompleter::highlighted`.
  ///
  /// An object of this type can be created from `Completer` with `object.signals().highlighted_qt_core_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Completer` object.
  pub struct HighlightedQtCoreStringRef<'a>(&'a ::completer::Completer);
  impl<'a> ::qt_core::connection::Receiver for HighlightedQtCoreStringRef<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2highlighted(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HighlightedQtCoreStringRef<'a> {}
  /// Represents a built-in Qt signal `QCompleter::highlighted`.
  ///
  /// An object of this type can be created from `Completer` with `object.signals().highlighted_qt_core_model_index_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Completer` object.
  pub struct HighlightedQtCoreModelIndexRef<'a>(&'a ::completer::Completer);
  impl<'a> ::qt_core::connection::Receiver for HighlightedQtCoreModelIndexRef<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2highlighted(const QModelIndex&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HighlightedQtCoreModelIndexRef<'a> {}
  /// Represents a built-in Qt signal `QCompleter::objectNameChanged`.
  ///
  /// An object of this type can be created from `Completer` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Completer` object.
  pub struct ObjectNameChanged<'a>(&'a ::completer::Completer);
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
    /// Returns an object representing a built-in Qt signal `QCompleter::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated_qt_core_string_ref(&self) -> ActivatedQtCoreStringRef {
      ActivatedQtCoreStringRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCompleter::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated_qt_core_model_index_ref(&self) -> ActivatedQtCoreModelIndexRef {
      ActivatedQtCoreModelIndexRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCompleter::highlighted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn highlighted_qt_core_string_ref(&self) -> HighlightedQtCoreStringRef {
      HighlightedQtCoreStringRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCompleter::highlighted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn highlighted_qt_core_model_index_ref(&self) -> HighlightedQtCoreModelIndexRef {
      HighlightedQtCoreModelIndexRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCompleter::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Completer`.
  pub struct Slots<'a>(&'a ::completer::Completer);
  /// Represents a built-in Qt slot `QCompleter::setWrapAround`.
  ///
  /// An object of this type can be created from `Completer` with `object.slots().set_wrap_around()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Completer` object.
  pub struct SetWrapAround<'a>(&'a ::completer::Completer);
  impl<'a> ::qt_core::connection::Receiver for SetWrapAround<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWrapAround(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QCompleter::setCompletionPrefix`.
  ///
  /// An object of this type can be created from `Completer` with `object.slots().set_completion_prefix()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Completer` object.
  pub struct SetCompletionPrefix<'a>(&'a ::completer::Completer);
  impl<'a> ::qt_core::connection::Receiver for SetCompletionPrefix<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCompletionPrefix(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QCompleter::complete`.
  ///
  /// An object of this type can be created from `Completer` with `object.slots().complete()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Completer` object.
  pub struct Complete<'a>(&'a ::completer::Completer);
  impl<'a> ::qt_core::connection::Receiver for Complete<'a> {
    type Arguments = (&'static ::qt_core::rect::Rect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1complete(const QRect&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QCompleter::setWrapAround`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_wrap_around(&self) -> SetWrapAround {
      SetWrapAround(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCompleter::setCompletionPrefix`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_completion_prefix(&self) -> SetCompletionPrefix {
      SetCompletionPrefix(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCompleter::complete`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn complete(&self) -> Complete {
      Complete(self.0)
    }
  }
  impl ::completer::Completer {
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

/// C++ type: <span style='color: green;'>```QCompleter::CompletionMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CompletionMode {
  /// C++ enum variant: <span style='color: green;'>```PopupCompletion = 0```</span>
  Popup = 0,
  /// C++ enum variant: <span style='color: green;'>```UnfilteredPopupCompletion = 1```</span>
  UnfilteredPopup = 1,
  /// C++ enum variant: <span style='color: green;'>```InlineCompletion = 2```</span>
  Inline = 2,
}

/// C++ type: <span style='color: green;'>```QCompleter::ModelSorting```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ModelSorting {
  /// C++ enum variant: <span style='color: green;'>```UnsortedModel = 0```</span>
  Unsorted = 0,
  /// C++ enum variant: <span style='color: green;'>```CaseSensitivelySortedModel = 1```</span>
  CaseSensitivelySorted = 1,
  /// C++ enum variant: <span style='color: green;'>```CaseInsensitivelySortedModel = 2```</span>
  CaseInsensitivelySorted = 2,
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::completer::Completer {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QCompleter_G_static_cast_QObject_ptr(self as *mut ::completer::Completer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCompleter_G_static_cast_QObject_ptr(self as *const ::completer::Completer as *mut ::completer::Completer) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::completer::Completer> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::completer::Completer {
    let ffi_result =
      ::ffi::qt_widgets_c_QCompleter_G_static_cast_QCompleter_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::completer::Completer {
    let ffi_result = ::ffi::qt_widgets_c_QCompleter_G_static_cast_QCompleter_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::completer::Completer {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCompleter_G_static_cast_QObject_ptr(self as *const ::completer::Completer as *mut ::completer::Completer) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::completer::Completer {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QCompleter_G_static_cast_QObject_ptr(self as *mut ::completer::Completer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Completer::complete](../struct.Completer.html#method.complete) method.
  pub trait CompleterCompleteArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::completer::Completer) -> ();
  }
  impl<'largs> CompleterCompleteArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::completer::Completer) -> () {

      unsafe { ::ffi::qt_widgets_c_QCompleter_complete_no_args(original_self as *mut ::completer::Completer) }
    }
  }
  impl<'largs> CompleterCompleteArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::completer::Completer) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_widgets_c_QCompleter_complete_rect(original_self as *mut ::completer::Completer,
                                                     rect as *const ::qt_core::rect::Rect)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Completer::new](../struct.Completer.html#method.new) method.
  pub trait CompleterNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::completer::Completer>;
  }
  impl<'a> CompleterNewArgs for &'a ::qt_core::string_list::StringList {
    fn exec(self) -> ::cpp_utils::CppBox<::completer::Completer> {
      let completions = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QCompleter_new_completions(completions as *const ::qt_core::string_list::StringList)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl CompleterNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::completer::Completer> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QCompleter_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Completer::new_unsafe](../struct.Completer.html#method.new_unsafe) method.
  pub trait CompleterNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::completer::Completer>;
  }
  impl<'a> CompleterNewUnsafeArgs for (&'a ::qt_core::string_list::StringList, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::completer::Completer> {
      let completions = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QCompleter_new_completions_parent(completions as *const ::qt_core::string_list::StringList, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl CompleterNewUnsafeArgs for *mut ::qt_core::abstract_item_model::AbstractItemModel {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::completer::Completer> {
      let model = self;
      let ffi_result = ::ffi::qt_widgets_c_QCompleter_new_model(model);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl CompleterNewUnsafeArgs
    for (*mut ::qt_core::abstract_item_model::AbstractItemModel, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::completer::Completer> {
      let model = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QCompleter_new_model_parent(model, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl CompleterNewUnsafeArgs for *mut ::qt_core::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::completer::Completer> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QCompleter_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
