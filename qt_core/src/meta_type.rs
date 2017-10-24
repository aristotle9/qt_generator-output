/// C++ type: <span style='color: green;'>```QMetaType```</span>
#[repr(C)]
pub struct MetaType([u8; ::type_sizes::QT_CORE_META_TYPE_META_TYPE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MetaType {
  unsafe fn new_uninitialized() -> MetaType {
    MetaType(::std::mem::uninitialized())
  }
}

impl MetaType {
  /// C++ method: <span style='color: green;'>```static bool QMetaType::compare(const void* lhs, const void* rhs, int typeId, int* result)```</span>
  ///
  ///
  pub unsafe fn compare(lhs: *const ::libc::c_void,
                        rhs: *const ::libc::c_void,
                        type_id: ::libc::c_int,
                        result: *mut ::libc::c_int)
                        -> bool {
    ::ffi::qt_core_c_QMetaType_compare(lhs, rhs, type_id, result)
  }

  /// C++ method: <span style='color: green;'>```QMetaType::construct```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn construct(&self, *mut ::libc::c_void) -> *mut ::libc::c_void```<br>
  /// C++ method: <span style='color: green;'>```void* QMetaType::construct(void* where) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn construct(&self, (*mut ::libc::c_void, *const ::libc::c_void)) -> *mut ::libc::c_void```<br>
  /// C++ method: <span style='color: green;'>```void* QMetaType::construct(void* where, const void* copy = ?) const```</span>
  ///
  ///
  pub unsafe fn construct<'largs, Args>(&'largs self, args: Args) -> *mut ::libc::c_void
    where Args: overloading::MetaTypeConstructArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static void* QMetaType::construct(int type, void* where, const void* copy)```</span>
  ///
  ///
  pub unsafe fn construct_static(type_: ::libc::c_int,
                                 where_: *mut ::libc::c_void,
                                 copy: *const ::libc::c_void)
                                 -> *mut ::libc::c_void {
    ::ffi::qt_core_c_QMetaType_construct_type_where_copy(type_, where_, copy)
  }

  /// C++ method: <span style='color: green;'>```static bool QMetaType::convert(const void* from, int fromTypeId, void* to, int toTypeId)```</span>
  ///
  ///
  pub unsafe fn convert(from: *const ::libc::c_void,
                        from_type_id: ::libc::c_int,
                        to: *mut ::libc::c_void,
                        to_type_id: ::libc::c_int)
                        -> bool {
    ::ffi::qt_core_c_QMetaType_convert(from, from_type_id, to, to_type_id)
  }

  /// C++ method: <span style='color: green;'>```void* QMetaType::create() const```</span>
  ///
  ///
  pub fn create(&self) -> *mut ::libc::c_void {
    unsafe { ::ffi::qt_core_c_QMetaType_create_no_args(self as *const ::meta_type::MetaType) }
  }

  /// C++ method: <span style='color: green;'>```void* QMetaType::create(const void* copy = ?) const```</span>
  ///
  ///
  pub unsafe fn create_c_void_ptr(&self, copy: *const ::libc::c_void) -> *mut ::libc::c_void {
    ::ffi::qt_core_c_QMetaType_create_copy(self as *const ::meta_type::MetaType, copy)
  }

  /// C++ method: <span style='color: green;'>```static void* QMetaType::create(int type)```</span>
  ///
  ///
  pub fn create_static_c_int(type_: ::libc::c_int) -> *mut ::libc::c_void {
    unsafe { ::ffi::qt_core_c_QMetaType_create_type(type_) }
  }

  /// C++ method: <span style='color: green;'>```static void* QMetaType::create(int type, const void* copy = ?)```</span>
  ///
  ///
  pub unsafe fn create_static_c_int_c_void_ptr(type_: ::libc::c_int,
                                               copy: *const ::libc::c_void)
                                               -> *mut ::libc::c_void {
    ::ffi::qt_core_c_QMetaType_create_type_copy(type_, copy)
  }

  /// C++ method: <span style='color: green;'>```static bool QMetaType::debugStream(QDebug& dbg, const void* rhs, int typeId)```</span>
  ///
  ///
  pub unsafe fn debug_stream(dbg: &mut ::debug::Debug, rhs: *const ::libc::c_void, type_id: ::libc::c_int) -> bool {
    ::ffi::qt_core_c_QMetaType_debugStream(dbg as *mut ::debug::Debug, rhs, type_id)
  }

  /// C++ method: <span style='color: green;'>```void QMetaType::destroy(void* data) const```</span>
  ///
  ///
  pub unsafe fn destroy(&self, data: *mut ::libc::c_void) {
    ::ffi::qt_core_c_QMetaType_destroy_data(self as *const ::meta_type::MetaType, data)
  }

  /// C++ method: <span style='color: green;'>```static void QMetaType::destroy(int type, void* data)```</span>
  ///
  ///
  pub unsafe fn destroy_static(type_: ::libc::c_int, data: *mut ::libc::c_void) {
    ::ffi::qt_core_c_QMetaType_destroy_type_data(type_, data)
  }

  /// C++ method: <span style='color: green;'>```void QMetaType::destruct(void* data) const```</span>
  ///
  ///
  pub unsafe fn destruct(&self, data: *mut ::libc::c_void) {
    ::ffi::qt_core_c_QMetaType_destruct_data(self as *const ::meta_type::MetaType, data)
  }

  /// C++ method: <span style='color: green;'>```static void QMetaType::destruct(int type, void* where)```</span>
  ///
  ///
  pub unsafe fn destruct_static(type_: ::libc::c_int, where_: *mut ::libc::c_void) {
    ::ffi::qt_core_c_QMetaType_destruct_type_where(type_, where_)
  }

  /// C++ method: <span style='color: green;'>```static bool QMetaType::equals(const void* lhs, const void* rhs, int typeId, int* result)```</span>
  ///
  ///
  pub unsafe fn equals(lhs: *const ::libc::c_void,
                       rhs: *const ::libc::c_void,
                       type_id: ::libc::c_int,
                       result: *mut ::libc::c_int)
                       -> bool {
    ::ffi::qt_core_c_QMetaType_equals(lhs, rhs, type_id, result)
  }

  /// C++ method: <span style='color: green;'>```QFlags<QMetaType::TypeFlag> QMetaType::flags() const```</span>
  ///
  ///
  pub fn flags(&self) -> ::flags::Flags<::meta_type::TypeFlag> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMetaType_flags(self as *const ::meta_type::MetaType) };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```static bool QMetaType::hasRegisteredComparators(int typeId)```</span>
  ///
  ///
  pub fn has_registered_comparators(type_id: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaType_hasRegisteredComparators(type_id) }
  }

  /// C++ method: <span style='color: green;'>```static bool QMetaType::hasRegisteredConverterFunction(int fromTypeId, int toTypeId)```</span>
  ///
  ///
  pub fn has_registered_converter_function(from_type_id: ::libc::c_int, to_type_id: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaType_hasRegisteredConverterFunction(from_type_id, to_type_id) }
  }

  /// C++ method: <span style='color: green;'>```static bool QMetaType::hasRegisteredDebugStreamOperator(int typeId)```</span>
  ///
  ///
  pub fn has_registered_debug_stream_operator(type_id: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaType_hasRegisteredDebugStreamOperator(type_id) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaType::isRegistered() const```</span>
  ///
  ///
  pub fn is_registered(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaType_isRegistered_no_args(self as *const ::meta_type::MetaType) }
  }

  /// C++ method: <span style='color: green;'>```static bool QMetaType::isRegistered(int type)```</span>
  ///
  ///
  pub fn is_registered_static(type_: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaType_isRegistered_type(type_) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaType::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaType_isValid(self as *const ::meta_type::MetaType) }
  }

  /// C++ method: <span style='color: green;'>```static bool QMetaType::load(QDataStream& stream, int type, void* data)```</span>
  ///
  ///
  pub unsafe fn load(stream: &mut ::data_stream::DataStream, type_: ::libc::c_int, data: *mut ::libc::c_void) -> bool {
    ::ffi::qt_core_c_QMetaType_load(stream as *mut ::data_stream::DataStream, type_, data)
  }

  /// C++ method: <span style='color: green;'>```const QMetaObject* QMetaType::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QMetaType_metaObject(self as *const ::meta_type::MetaType) }
  }

  /// C++ method: <span style='color: green;'>```static const QMetaObject* QMetaType::metaObjectForType(int type)```</span>
  ///
  ///
  pub fn meta_object_for_type(type_: ::libc::c_int) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QMetaType_metaObjectForType(type_) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMetaType::QMetaType(const int type)```</span>
  ///
  ///
  pub fn new(type_: ::libc::c_int) -> ::meta_type::MetaType {
    {
      let mut object: ::meta_type::MetaType =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaType_constructor(type_, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static int QMetaType::registerNormalizedTypedef(const QByteArray& normalizedTypeName, int aliasId)```</span>
  ///
  ///
  pub fn register_normalized_typedef(normalized_type_name: &::byte_array::ByteArray,
                                     alias_id: ::libc::c_int)
                                     -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QMetaType_registerNormalizedTypedef(normalized_type_name as *const ::byte_array::ByteArray,
                                                           alias_id)
    }
  }

  /// C++ method: <span style='color: green;'>```static int QMetaType::registerType(const char* typeName, void (*FN_PTR)(void*) deleter, void* (*FN_PTR)(const void*) creator)```</span>
  ///
  ///
  pub unsafe fn register_type(type_name: *const ::libc::c_char,
                              deleter: extern "C" fn(*mut ::libc::c_void),
                              creator: extern "C" fn(*const ::libc::c_void) -> *mut ::libc::c_void)
                              -> ::libc::c_int {
    ::ffi::qt_core_c_QMetaType_registerType(type_name, deleter, creator)
  }

  /// C++ method: <span style='color: green;'>```static int QMetaType::registerTypedef(const char* typeName, int aliasId)```</span>
  ///
  ///
  pub unsafe fn register_typedef(type_name: *const ::libc::c_char, alias_id: ::libc::c_int) -> ::libc::c_int {
    ::ffi::qt_core_c_QMetaType_registerTypedef(type_name, alias_id)
  }

  /// C++ method: <span style='color: green;'>```static bool QMetaType::save(QDataStream& stream, int type, const void* data)```</span>
  ///
  ///
  pub unsafe fn save(stream: &mut ::data_stream::DataStream,
                     type_: ::libc::c_int,
                     data: *const ::libc::c_void)
                     -> bool {
    ::ffi::qt_core_c_QMetaType_save(stream as *mut ::data_stream::DataStream, type_, data)
  }

  /// C++ method: <span style='color: green;'>```int QMetaType::sizeOf() const```</span>
  ///
  ///
  pub fn size_of(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaType_sizeOf_no_args(self as *const ::meta_type::MetaType) }
  }

  /// C++ method: <span style='color: green;'>```static int QMetaType::sizeOf(int type)```</span>
  ///
  ///
  pub fn size_of_static(type_: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaType_sizeOf_type(type_) }
  }

  /// C++ method: <span style='color: green;'>```static int QMetaType::type(const QByteArray& typeName)```</span>
  ///
  ///
  pub fn type_(type_name: &::byte_array::ByteArray) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaType_type_QByteArray(type_name as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```static QFlags<QMetaType::TypeFlag> QMetaType::typeFlags(int type)```</span>
  ///
  ///
  pub fn type_flags(type_: ::libc::c_int) -> ::flags::Flags<::meta_type::TypeFlag> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMetaType_typeFlags(type_) };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```static const char* QMetaType::typeName(int type)```</span>
  ///
  ///
  pub fn type_name(type_: ::libc::c_int) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMetaType_typeName(type_) }
  }

  /// C++ method: <span style='color: green;'>```static int QMetaType::type(const char* typeName)```</span>
  ///
  ///
  pub unsafe fn type_unsafe(type_name: *const ::libc::c_char) -> ::libc::c_int {
    ::ffi::qt_core_c_QMetaType_type_char(type_name)
  }

  /// C++ method: <span style='color: green;'>```static bool QMetaType::unregisterType(int type)```</span>
  ///
  ///
  pub fn unregister_type(type_: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaType_unregisterType(type_) }
  }
}

impl Drop for ::meta_type::MetaType {
  /// C++ method: <span style='color: green;'>```[destructor] void QMetaType::~QMetaType()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMetaType_destructor(self as *mut ::meta_type::MetaType) }
  }
}

/// C++ type: <span style='color: green;'>```QMetaType::Type```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Type {
  /// C++ enum variant: <span style='color: green;'>```UnknownType = 0```</span>
  UnknownType = 0,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Bool = 1```</span>
  /// - <span style='color: green;'>```FirstCoreType = 1```</span>
  ///
  Bool = 1,
  /// C++ enum variant: <span style='color: green;'>```Int = 2```</span>
  Int = 2,
  /// C++ enum variant: <span style='color: green;'>```UInt = 3```</span>
  UInt = 3,
  /// C++ enum variant: <span style='color: green;'>```LongLong = 4```</span>
  LongLong = 4,
  /// C++ enum variant: <span style='color: green;'>```ULongLong = 5```</span>
  ULongLong = 5,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Double = 6```</span>
  /// - <span style='color: green;'>```QReal = 6```</span>
  ///
  Double = 6,
  /// C++ enum variant: <span style='color: green;'>```QChar = 7```</span>
  QChar = 7,
  /// C++ enum variant: <span style='color: green;'>```QVariantMap = 8```</span>
  QVariantMap = 8,
  /// C++ enum variant: <span style='color: green;'>```QVariantList = 9```</span>
  QVariantList = 9,
  /// C++ enum variant: <span style='color: green;'>```QString = 10```</span>
  QString = 10,
  /// C++ enum variant: <span style='color: green;'>```QStringList = 11```</span>
  QStringList = 11,
  /// C++ enum variant: <span style='color: green;'>```QByteArray = 12```</span>
  QByteArray = 12,
  /// C++ enum variant: <span style='color: green;'>```QBitArray = 13```</span>
  QBitArray = 13,
  /// C++ enum variant: <span style='color: green;'>```QDate = 14```</span>
  QDate = 14,
  /// C++ enum variant: <span style='color: green;'>```QTime = 15```</span>
  QTime = 15,
  /// C++ enum variant: <span style='color: green;'>```QDateTime = 16```</span>
  QDateTime = 16,
  /// C++ enum variant: <span style='color: green;'>```QUrl = 17```</span>
  QUrl = 17,
  /// C++ enum variant: <span style='color: green;'>```QLocale = 18```</span>
  QLocale = 18,
  /// C++ enum variant: <span style='color: green;'>```QRect = 19```</span>
  QRect = 19,
  /// C++ enum variant: <span style='color: green;'>```QRectF = 20```</span>
  QRectF = 20,
  /// C++ enum variant: <span style='color: green;'>```QSize = 21```</span>
  QSize = 21,
  /// C++ enum variant: <span style='color: green;'>```QSizeF = 22```</span>
  QSizeF = 22,
  /// C++ enum variant: <span style='color: green;'>```QLine = 23```</span>
  QLine = 23,
  /// C++ enum variant: <span style='color: green;'>```QLineF = 24```</span>
  QLineF = 24,
  /// C++ enum variant: <span style='color: green;'>```QPoint = 25```</span>
  QPoint = 25,
  /// C++ enum variant: <span style='color: green;'>```QPointF = 26```</span>
  QPointF = 26,
  /// C++ enum variant: <span style='color: green;'>```QRegExp = 27```</span>
  QRegExp = 27,
  /// C++ enum variant: <span style='color: green;'>```QVariantHash = 28```</span>
  QVariantHash = 28,
  /// C++ enum variant: <span style='color: green;'>```QEasingCurve = 29```</span>
  QEasingCurve = 29,
  /// C++ enum variant: <span style='color: green;'>```QUuid = 30```</span>
  QUuid = 30,
  /// C++ enum variant: <span style='color: green;'>```VoidStar = 31```</span>
  VoidStar = 31,
  /// C++ enum variant: <span style='color: green;'>```Long = 32```</span>
  Long = 32,
  /// C++ enum variant: <span style='color: green;'>```Short = 33```</span>
  Short = 33,
  /// C++ enum variant: <span style='color: green;'>```Char = 34```</span>
  Char = 34,
  /// C++ enum variant: <span style='color: green;'>```ULong = 35```</span>
  ULong = 35,
  /// C++ enum variant: <span style='color: green;'>```UShort = 36```</span>
  UShort = 36,
  /// C++ enum variant: <span style='color: green;'>```UChar = 37```</span>
  UChar = 37,
  /// C++ enum variant: <span style='color: green;'>```Float = 38```</span>
  Float = 38,
  /// C++ enum variant: <span style='color: green;'>```QObjectStar = 39```</span>
  QObjectStar = 39,
  /// C++ enum variant: <span style='color: green;'>```SChar = 40```</span>
  SChar = 40,
  /// C++ enum variant: <span style='color: green;'>```QVariant = 41```</span>
  QVariant = 41,
  /// C++ enum variant: <span style='color: green;'>```QModelIndex = 42```</span>
  QModelIndex = 42,
  /// C++ enum variant: <span style='color: green;'>```Void = 43```</span>
  Void = 43,
  /// C++ enum variant: <span style='color: green;'>```QRegularExpression = 44```</span>
  QRegularExpression = 44,
  /// C++ enum variant: <span style='color: green;'>```QJsonValue = 45```</span>
  QJsonValue = 45,
  /// C++ enum variant: <span style='color: green;'>```QJsonObject = 46```</span>
  QJsonObject = 46,
  /// C++ enum variant: <span style='color: green;'>```QJsonArray = 47```</span>
  QJsonArray = 47,
  /// C++ enum variant: <span style='color: green;'>```QJsonDocument = 48```</span>
  QJsonDocument = 48,
  /// C++ enum variant: <span style='color: green;'>```QByteArrayList = 49```</span>
  QByteArrayList = 49,
  /// C++ enum variant: <span style='color: green;'>```QPersistentModelIndex = 50```</span>
  QPersistentModelIndex = 50,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Nullptr = 51```</span>
  /// - <span style='color: green;'>```LastCoreType = 51```</span>
  ///
  Nullptr = 51,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```QFont = 64```</span>
  /// - <span style='color: green;'>```FirstGuiType = 64```</span>
  ///
  QFont = 64,
  /// C++ enum variant: <span style='color: green;'>```QPixmap = 65```</span>
  QPixmap = 65,
  /// C++ enum variant: <span style='color: green;'>```QBrush = 66```</span>
  QBrush = 66,
  /// C++ enum variant: <span style='color: green;'>```QColor = 67```</span>
  QColor = 67,
  /// C++ enum variant: <span style='color: green;'>```QPalette = 68```</span>
  QPalette = 68,
  /// C++ enum variant: <span style='color: green;'>```QIcon = 69```</span>
  QIcon = 69,
  /// C++ enum variant: <span style='color: green;'>```QImage = 70```</span>
  QImage = 70,
  /// C++ enum variant: <span style='color: green;'>```QPolygon = 71```</span>
  QPolygon = 71,
  /// C++ enum variant: <span style='color: green;'>```QRegion = 72```</span>
  QRegion = 72,
  /// C++ enum variant: <span style='color: green;'>```QBitmap = 73```</span>
  QBitmap = 73,
  /// C++ enum variant: <span style='color: green;'>```QCursor = 74```</span>
  QCursor = 74,
  /// C++ enum variant: <span style='color: green;'>```QKeySequence = 75```</span>
  QKeySequence = 75,
  /// C++ enum variant: <span style='color: green;'>```QPen = 76```</span>
  QPen = 76,
  /// C++ enum variant: <span style='color: green;'>```QTextLength = 77```</span>
  QTextLength = 77,
  /// C++ enum variant: <span style='color: green;'>```QTextFormat = 78```</span>
  QTextFormat = 78,
  /// C++ enum variant: <span style='color: green;'>```QMatrix = 79```</span>
  QMatrix = 79,
  /// C++ enum variant: <span style='color: green;'>```QTransform = 80```</span>
  QTransform = 80,
  /// C++ enum variant: <span style='color: green;'>```QMatrix4x4 = 81```</span>
  QMatrix4X4 = 81,
  /// C++ enum variant: <span style='color: green;'>```QVector2D = 82```</span>
  QVector2D = 82,
  /// C++ enum variant: <span style='color: green;'>```QVector3D = 83```</span>
  QVector3D = 83,
  /// C++ enum variant: <span style='color: green;'>```QVector4D = 84```</span>
  QVector4D = 84,
  /// C++ enum variant: <span style='color: green;'>```QQuaternion = 85```</span>
  QQuaternion = 85,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```QPolygonF = 86```</span>
  /// - <span style='color: green;'>```LastGuiType = 86```</span>
  ///
  QPolygonF = 86,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```QSizePolicy = 121```</span>
  /// - <span style='color: green;'>```FirstWidgetsType = 121```</span>
  /// - <span style='color: green;'>```LastWidgetsType = 121```</span>
  /// - <span style='color: green;'>```HighestInternalId = 121```</span>
  ///
  QSizePolicy = 121,
  /// C++ enum variant: <span style='color: green;'>```User = 1024```</span>
  User = 1024,
}

/// C++ type: <span style='color: green;'>```QMetaType::TypeFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TypeFlag {
  /// C++ enum variant: <span style='color: green;'>```NeedsConstruction = 1```</span>
  NeedsConstruction = 1,
  /// C++ enum variant: <span style='color: green;'>```NeedsDestruction = 2```</span>
  NeedsDestruction = 2,
  /// C++ enum variant: <span style='color: green;'>```MovableType = 4```</span>
  MovableType = 4,
  /// C++ enum variant: <span style='color: green;'>```PointerToQObject = 8```</span>
  PointerToQObject = 8,
  /// C++ enum variant: <span style='color: green;'>```IsEnumeration = 16```</span>
  IsEnumeration = 16,
  /// C++ enum variant: <span style='color: green;'>```SharedPointerToQObject = 32```</span>
  SharedPointerToQObject = 32,
  /// C++ enum variant: <span style='color: green;'>```WeakPointerToQObject = 64```</span>
  WeakPointerToQObject = 64,
  /// C++ enum variant: <span style='color: green;'>```TrackingPointerToQObject = 128```</span>
  TrackingPointerToQObject = 128,
  /// C++ enum variant: <span style='color: green;'>```WasDeclaredAsMetaType = 256```</span>
  WasDeclaredAsMetaType = 256,
  /// C++ enum variant: <span style='color: green;'>```IsGadget = 512```</span>
  IsGadget = 512,
}

impl ::flags::FlaggableEnum for TypeFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "TypeFlag"
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MetaType::construct](../struct.MetaType.html#method.construct) method.
  pub trait MetaTypeConstructArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::meta_type::MetaType) -> *mut ::libc::c_void;
  }
  impl<'largs> MetaTypeConstructArgs<'largs> for *mut ::libc::c_void {
    unsafe fn exec(self, original_self: &'largs ::meta_type::MetaType) -> *mut ::libc::c_void {
      let where_ = self;
      ::ffi::qt_core_c_QMetaType_construct_where(original_self as *const ::meta_type::MetaType, where_)
    }
  }
  impl<'largs> MetaTypeConstructArgs<'largs> for (*mut ::libc::c_void, *const ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs ::meta_type::MetaType) -> *mut ::libc::c_void {
      let where_ = self.0;
      let copy = self.1;
      ::ffi::qt_core_c_QMetaType_construct_where_copy(original_self as *const ::meta_type::MetaType, where_, copy)
    }
  }
}
