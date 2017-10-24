/// C++ type: <span style='color: green;'>```QDataWidgetMapper```</span>
#[repr(C)]
pub struct DataWidgetMapper(u8);

impl DataWidgetMapper {
  /// C++ method: <span style='color: green;'>```QDataWidgetMapper::addMapping```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_mapping(&mut self, (*mut ::widget::Widget, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDataWidgetMapper::addMapping(QWidget* widget, int section)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_mapping(&mut self, (*mut ::widget::Widget, ::libc::c_int, &::qt_core::byte_array::ByteArray)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDataWidgetMapper::addMapping(QWidget* widget, int section, const QByteArray& propertyName)```</span>
  ///
  ///
  pub unsafe fn add_mapping<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::DataWidgetMapperAddMappingArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QDataWidgetMapper::clearMapping()```</span>
  ///
  ///
  pub fn clear_mapping(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_clearMapping(self as *mut ::data_widget_mapper::DataWidgetMapper) }
  }

  /// C++ method: <span style='color: green;'>```int QDataWidgetMapper::currentIndex() const```</span>
  ///
  ///
  pub fn current_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_currentIndex(self as *const ::data_widget_mapper::DataWidgetMapper) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemDelegate* QDataWidgetMapper::itemDelegate() const```</span>
  ///
  ///
  pub fn item_delegate(&self) -> *mut ::abstract_item_delegate::AbstractItemDelegate {
    unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_itemDelegate(self as *const ::data_widget_mapper::DataWidgetMapper) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QDataWidgetMapper::mappedPropertyName(QWidget* widget) const```</span>
  ///
  ///
  pub unsafe fn mapped_property_name(&self, widget: *mut ::widget::Widget) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDataWidgetMapper_mappedPropertyName_to_output(self as *const ::data_widget_mapper::DataWidgetMapper, widget, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QDataWidgetMapper::mappedSection(QWidget* widget) const```</span>
  ///
  ///
  pub unsafe fn mapped_section(&self, widget: *mut ::widget::Widget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QDataWidgetMapper_mappedSection(self as *const ::data_widget_mapper::DataWidgetMapper,
                                                        widget)
  }

  /// C++ method: <span style='color: green;'>```QWidget* QDataWidgetMapper::mappedWidgetAt(int section) const```</span>
  ///
  ///
  pub fn mapped_widget_at(&self, section: ::libc::c_int) -> *mut ::widget::Widget {
    unsafe {
      ::ffi::qt_widgets_c_QDataWidgetMapper_mappedWidgetAt(self as *const ::data_widget_mapper::DataWidgetMapper,
                                                           section)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QDataWidgetMapper::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_metaObject(self as *const ::data_widget_mapper::DataWidgetMapper) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemModel* QDataWidgetMapper::model() const```</span>
  ///
  ///
  pub fn model(&self) -> *mut ::qt_core::abstract_item_model::AbstractItemModel {
    unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_model(self as *const ::data_widget_mapper::DataWidgetMapper) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QDataWidgetMapper::QDataWidgetMapper()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::data_widget_mapper::DataWidgetMapper> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QDataWidgetMapper::QDataWidgetMapper(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::data_widget_mapper::DataWidgetMapper> {
    let ffi_result = ::ffi::qt_widgets_c_QDataWidgetMapper_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QDataWidgetMapper::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QDataWidgetMapper_qt_metacall(self as *mut ::data_widget_mapper::DataWidgetMapper,
                                                      arg1 as *const ::qt_core::meta_object::Call,
                                                      arg2,
                                                      arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QDataWidgetMapper::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QDataWidgetMapper_qt_metacast(self as *mut ::data_widget_mapper::DataWidgetMapper, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QDataWidgetMapper::removeMapping(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn remove_mapping(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QDataWidgetMapper_removeMapping(self as *mut ::data_widget_mapper::DataWidgetMapper, widget)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QDataWidgetMapper::revert()```</span>
  ///
  ///
  pub fn revert(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_revert(self as *mut ::data_widget_mapper::DataWidgetMapper) }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QDataWidgetMapper::rootIndex() const```</span>
  ///
  ///
  pub fn root_index(&self) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDataWidgetMapper_rootIndex_to_output(self as *const ::data_widget_mapper::DataWidgetMapper, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QDataWidgetMapper::setCurrentIndex(int index)```</span>
  ///
  ///
  pub fn set_current_index(&mut self, index: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QDataWidgetMapper_setCurrentIndex(self as *mut ::data_widget_mapper::DataWidgetMapper, index)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QDataWidgetMapper::setCurrentModelIndex(const QModelIndex& index)```</span>
  ///
  ///
  pub fn set_current_model_index(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QDataWidgetMapper_setCurrentModelIndex(self as *mut ::data_widget_mapper::DataWidgetMapper,
                                                                 index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDataWidgetMapper::setItemDelegate(QAbstractItemDelegate* delegate)```</span>
  ///
  ///
  pub unsafe fn set_item_delegate(&mut self, delegate: *mut ::abstract_item_delegate::AbstractItemDelegate) {
    ::ffi::qt_widgets_c_QDataWidgetMapper_setItemDelegate(self as *mut ::data_widget_mapper::DataWidgetMapper,
                                                          delegate)
  }

  /// C++ method: <span style='color: green;'>```void QDataWidgetMapper::setModel(QAbstractItemModel* model)```</span>
  ///
  ///
  pub unsafe fn set_model(&mut self, model: *mut ::qt_core::abstract_item_model::AbstractItemModel) {
    ::ffi::qt_widgets_c_QDataWidgetMapper_setModel(self as *mut ::data_widget_mapper::DataWidgetMapper, model)
  }

  /// C++ method: <span style='color: green;'>```void QDataWidgetMapper::setOrientation(Qt::Orientation aOrientation)```</span>
  ///
  ///
  pub fn set_orientation(&mut self, a_orientation: &::qt_core::qt::Orientation) {
    unsafe {
      ::ffi::qt_widgets_c_QDataWidgetMapper_setOrientation(self as *mut ::data_widget_mapper::DataWidgetMapper,
                                                           a_orientation as *const ::qt_core::qt::Orientation)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDataWidgetMapper::setRootIndex(const QModelIndex& index)```</span>
  ///
  ///
  pub fn set_root_index(&mut self, index: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QDataWidgetMapper_setRootIndex(self as *mut ::data_widget_mapper::DataWidgetMapper,
                                                         index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDataWidgetMapper::setSubmitPolicy(QDataWidgetMapper::SubmitPolicy policy)```</span>
  ///
  ///
  pub fn set_submit_policy(&mut self, policy: ::data_widget_mapper::SubmitPolicy) {
    unsafe {
      ::ffi::qt_widgets_c_QDataWidgetMapper_setSubmitPolicy(self as *mut ::data_widget_mapper::DataWidgetMapper, policy)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] bool QDataWidgetMapper::submit()```</span>
  ///
  ///
  pub fn submit(&mut self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_submit(self as *mut ::data_widget_mapper::DataWidgetMapper) }
  }

  /// C++ method: <span style='color: green;'>```QDataWidgetMapper::SubmitPolicy QDataWidgetMapper::submitPolicy() const```</span>
  ///
  ///
  pub fn submit_policy(&self) -> ::data_widget_mapper::SubmitPolicy {
    unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_submitPolicy(self as *const ::data_widget_mapper::DataWidgetMapper) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QDataWidgetMapper::toFirst()```</span>
  ///
  ///
  pub fn to_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_toFirst(self as *mut ::data_widget_mapper::DataWidgetMapper) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QDataWidgetMapper::toLast()```</span>
  ///
  ///
  pub fn to_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_toLast(self as *mut ::data_widget_mapper::DataWidgetMapper) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QDataWidgetMapper::toNext()```</span>
  ///
  ///
  pub fn to_next(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_toNext(self as *mut ::data_widget_mapper::DataWidgetMapper) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QDataWidgetMapper::toPrevious()```</span>
  ///
  ///
  pub fn to_previous(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_toPrevious(self as *mut ::data_widget_mapper::DataWidgetMapper) }
  }

  /// C++ method: <span style='color: green;'>```static QString QDataWidgetMapper::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDataWidgetMapper_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDataWidgetMapper::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDataWidgetMapper_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::data_widget_mapper::DataWidgetMapper {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QDataWidgetMapper_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `DataWidgetMapper`.
  pub struct Signals<'a>(&'a ::data_widget_mapper::DataWidgetMapper);
  /// Represents a built-in Qt signal `QDataWidgetMapper::currentIndexChanged`.
  ///
  /// An object of this type can be created from `DataWidgetMapper` with `object.signals().current_index_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DataWidgetMapper` object.
  pub struct CurrentIndexChanged<'a>(&'a ::data_widget_mapper::DataWidgetMapper);
  impl<'a> ::qt_core::connection::Receiver for CurrentIndexChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentIndexChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentIndexChanged<'a> {}
  /// Represents a built-in Qt signal `QDataWidgetMapper::objectNameChanged`.
  ///
  /// An object of this type can be created from `DataWidgetMapper` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DataWidgetMapper` object.
  pub struct ObjectNameChanged<'a>(&'a ::data_widget_mapper::DataWidgetMapper);
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
    /// Returns an object representing a built-in Qt signal `QDataWidgetMapper::currentIndexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_index_changed(&self) -> CurrentIndexChanged {
      CurrentIndexChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDataWidgetMapper::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `DataWidgetMapper`.
  pub struct Slots<'a>(&'a ::data_widget_mapper::DataWidgetMapper);
  /// Represents a built-in Qt slot `QDataWidgetMapper::revert`.
  ///
  /// An object of this type can be created from `DataWidgetMapper` with `object.slots().revert()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DataWidgetMapper` object.
  pub struct Revert<'a>(&'a ::data_widget_mapper::DataWidgetMapper);
  impl<'a> ::qt_core::connection::Receiver for Revert<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1revert()\0"
    }
  }
  /// Represents a built-in Qt slot `QDataWidgetMapper::toNext`.
  ///
  /// An object of this type can be created from `DataWidgetMapper` with `object.slots().to_next()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DataWidgetMapper` object.
  pub struct ToNext<'a>(&'a ::data_widget_mapper::DataWidgetMapper);
  impl<'a> ::qt_core::connection::Receiver for ToNext<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1toNext()\0"
    }
  }
  /// Represents a built-in Qt slot `QDataWidgetMapper::toFirst`.
  ///
  /// An object of this type can be created from `DataWidgetMapper` with `object.slots().to_first()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DataWidgetMapper` object.
  pub struct ToFirst<'a>(&'a ::data_widget_mapper::DataWidgetMapper);
  impl<'a> ::qt_core::connection::Receiver for ToFirst<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1toFirst()\0"
    }
  }
  /// Represents a built-in Qt slot `QDataWidgetMapper::setCurrentModelIndex`.
  ///
  /// An object of this type can be created from `DataWidgetMapper` with `object.slots().set_current_model_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DataWidgetMapper` object.
  pub struct SetCurrentModelIndex<'a>(&'a ::data_widget_mapper::DataWidgetMapper);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentModelIndex<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentModelIndex(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDataWidgetMapper::setCurrentIndex`.
  ///
  /// An object of this type can be created from `DataWidgetMapper` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DataWidgetMapper` object.
  pub struct SetCurrentIndex<'a>(&'a ::data_widget_mapper::DataWidgetMapper);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QDataWidgetMapper::submit`.
  ///
  /// An object of this type can be created from `DataWidgetMapper` with `object.slots().submit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DataWidgetMapper` object.
  pub struct Submit<'a>(&'a ::data_widget_mapper::DataWidgetMapper);
  impl<'a> ::qt_core::connection::Receiver for Submit<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1submit()\0"
    }
  }
  /// Represents a built-in Qt slot `QDataWidgetMapper::toLast`.
  ///
  /// An object of this type can be created from `DataWidgetMapper` with `object.slots().to_last()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DataWidgetMapper` object.
  pub struct ToLast<'a>(&'a ::data_widget_mapper::DataWidgetMapper);
  impl<'a> ::qt_core::connection::Receiver for ToLast<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1toLast()\0"
    }
  }
  /// Represents a built-in Qt slot `QDataWidgetMapper::toPrevious`.
  ///
  /// An object of this type can be created from `DataWidgetMapper` with `object.slots().to_previous()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DataWidgetMapper` object.
  pub struct ToPrevious<'a>(&'a ::data_widget_mapper::DataWidgetMapper);
  impl<'a> ::qt_core::connection::Receiver for ToPrevious<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1toPrevious()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QDataWidgetMapper::revert`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn revert(&self) -> Revert {
      Revert(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDataWidgetMapper::toNext`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn to_next(&self) -> ToNext {
      ToNext(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDataWidgetMapper::toFirst`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn to_first(&self) -> ToFirst {
      ToFirst(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDataWidgetMapper::setCurrentModelIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_model_index(&self) -> SetCurrentModelIndex {
      SetCurrentModelIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDataWidgetMapper::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDataWidgetMapper::submit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn submit(&self) -> Submit {
      Submit(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDataWidgetMapper::toLast`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn to_last(&self) -> ToLast {
      ToLast(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDataWidgetMapper::toPrevious`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn to_previous(&self) -> ToPrevious {
      ToPrevious(self.0)
    }
  }
  impl ::data_widget_mapper::DataWidgetMapper {
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

/// C++ type: <span style='color: green;'>```QDataWidgetMapper::SubmitPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SubmitPolicy {
  /// C++ enum variant: <span style='color: green;'>```AutoSubmit = 0```</span>
  Auto = 0,
  /// C++ enum variant: <span style='color: green;'>```ManualSubmit = 1```</span>
  Manual = 1,
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::data_widget_mapper::DataWidgetMapper {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_G_static_cast_QObject_ptr(self as *mut ::data_widget_mapper::DataWidgetMapper) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_G_static_cast_QObject_ptr(self as *const ::data_widget_mapper::DataWidgetMapper as *mut ::data_widget_mapper::DataWidgetMapper) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::data_widget_mapper::DataWidgetMapper> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::data_widget_mapper::DataWidgetMapper {
    let ffi_result = ::ffi::qt_widgets_c_QDataWidgetMapper_G_static_cast_QDataWidgetMapper_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::data_widget_mapper::DataWidgetMapper {
    let ffi_result = ::ffi::qt_widgets_c_QDataWidgetMapper_G_static_cast_QDataWidgetMapper_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::data_widget_mapper::DataWidgetMapper {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_G_static_cast_QObject_ptr(self as *const ::data_widget_mapper::DataWidgetMapper as *mut ::data_widget_mapper::DataWidgetMapper) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::data_widget_mapper::DataWidgetMapper {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDataWidgetMapper_G_static_cast_QObject_ptr(self as *mut ::data_widget_mapper::DataWidgetMapper) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [DataWidgetMapper::add_mapping](../struct.DataWidgetMapper.html#method.add_mapping) method.
  pub trait DataWidgetMapperAddMappingArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::data_widget_mapper::DataWidgetMapper) -> ();
  }
  impl<'largs> DataWidgetMapperAddMappingArgs<'largs> for (*mut ::widget::Widget, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::data_widget_mapper::DataWidgetMapper) -> () {
      let widget = self.0;
      let section = self.1;
      ::ffi::qt_widgets_c_QDataWidgetMapper_addMapping_widget_section(original_self as *mut ::data_widget_mapper::DataWidgetMapper, widget, section)
    }
  }
  impl<'largs> DataWidgetMapperAddMappingArgs<'largs>
    for (*mut ::widget::Widget, ::libc::c_int, &'largs ::qt_core::byte_array::ByteArray) {
    unsafe fn exec(self, original_self: &'largs mut ::data_widget_mapper::DataWidgetMapper) -> () {
      let widget = self.0;
      let section = self.1;
      let property_name = self.2;
      ::ffi::qt_widgets_c_QDataWidgetMapper_addMapping_widget_section_propertyName(original_self as *mut ::data_widget_mapper::DataWidgetMapper, widget, section, property_name as *const ::qt_core::byte_array::ByteArray)
    }
  }
}
