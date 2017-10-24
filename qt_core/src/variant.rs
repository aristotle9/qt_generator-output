/// C++ type: <span style='color: green;'>```QVariant::Type```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Type {
  /// C++ enum variant: <span style='color: green;'>```LastType = -1```</span>
  LastType = -1,
  /// C++ enum variant: <span style='color: green;'>```Invalid = 0```</span>
  Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Bool = 1```</span>
  Bool = 1,
  /// C++ enum variant: <span style='color: green;'>```Int = 2```</span>
  Int = 2,
  /// C++ enum variant: <span style='color: green;'>```UInt = 3```</span>
  UInt = 3,
  /// C++ enum variant: <span style='color: green;'>```LongLong = 4```</span>
  LongLong = 4,
  /// C++ enum variant: <span style='color: green;'>```ULongLong = 5```</span>
  ULongLong = 5,
  /// C++ enum variant: <span style='color: green;'>```Double = 6```</span>
  Double = 6,
  /// C++ enum variant: <span style='color: green;'>```Char = 7```</span>
  Char = 7,
  /// C++ enum variant: <span style='color: green;'>```Map = 8```</span>
  Map = 8,
  /// C++ enum variant: <span style='color: green;'>```List = 9```</span>
  List = 9,
  /// C++ enum variant: <span style='color: green;'>```String = 10```</span>
  String = 10,
  /// C++ enum variant: <span style='color: green;'>```StringList = 11```</span>
  StringList = 11,
  /// C++ enum variant: <span style='color: green;'>```ByteArray = 12```</span>
  ByteArray = 12,
  /// C++ enum variant: <span style='color: green;'>```BitArray = 13```</span>
  BitArray = 13,
  /// C++ enum variant: <span style='color: green;'>```Date = 14```</span>
  Date = 14,
  /// C++ enum variant: <span style='color: green;'>```Time = 15```</span>
  Time = 15,
  /// C++ enum variant: <span style='color: green;'>```DateTime = 16```</span>
  DateTime = 16,
  /// C++ enum variant: <span style='color: green;'>```Url = 17```</span>
  Url = 17,
  /// C++ enum variant: <span style='color: green;'>```Locale = 18```</span>
  Locale = 18,
  /// C++ enum variant: <span style='color: green;'>```Rect = 19```</span>
  Rect = 19,
  /// C++ enum variant: <span style='color: green;'>```RectF = 20```</span>
  RectF = 20,
  /// C++ enum variant: <span style='color: green;'>```Size = 21```</span>
  Size = 21,
  /// C++ enum variant: <span style='color: green;'>```SizeF = 22```</span>
  SizeF = 22,
  /// C++ enum variant: <span style='color: green;'>```Line = 23```</span>
  Line = 23,
  /// C++ enum variant: <span style='color: green;'>```LineF = 24```</span>
  LineF = 24,
  /// C++ enum variant: <span style='color: green;'>```Point = 25```</span>
  Point = 25,
  /// C++ enum variant: <span style='color: green;'>```PointF = 26```</span>
  PointF = 26,
  /// C++ enum variant: <span style='color: green;'>```RegExp = 27```</span>
  RegExp = 27,
  /// C++ enum variant: <span style='color: green;'>```Hash = 28```</span>
  Hash = 28,
  /// C++ enum variant: <span style='color: green;'>```EasingCurve = 29```</span>
  EasingCurve = 29,
  /// C++ enum variant: <span style='color: green;'>```Uuid = 30```</span>
  Uuid = 30,
  /// C++ enum variant: <span style='color: green;'>```ModelIndex = 42```</span>
  ModelIndex = 42,
  /// C++ enum variant: <span style='color: green;'>```RegularExpression = 44```</span>
  RegularExpression = 44,
  /// C++ enum variant: <span style='color: green;'>```PersistentModelIndex = 50```</span>
  PersistentModelIndex = 50,
  /// C++ enum variant: <span style='color: green;'>```LastCoreType = 51```</span>
  LastCoreType = 51,
  /// C++ enum variant: <span style='color: green;'>```Font = 64```</span>
  Font = 64,
  /// C++ enum variant: <span style='color: green;'>```Pixmap = 65```</span>
  Pixmap = 65,
  /// C++ enum variant: <span style='color: green;'>```Brush = 66```</span>
  Brush = 66,
  /// C++ enum variant: <span style='color: green;'>```Color = 67```</span>
  Color = 67,
  /// C++ enum variant: <span style='color: green;'>```Palette = 68```</span>
  Palette = 68,
  /// C++ enum variant: <span style='color: green;'>```Icon = 69```</span>
  Icon = 69,
  /// C++ enum variant: <span style='color: green;'>```Image = 70```</span>
  Image = 70,
  /// C++ enum variant: <span style='color: green;'>```Polygon = 71```</span>
  Polygon = 71,
  /// C++ enum variant: <span style='color: green;'>```Region = 72```</span>
  Region = 72,
  /// C++ enum variant: <span style='color: green;'>```Bitmap = 73```</span>
  Bitmap = 73,
  /// C++ enum variant: <span style='color: green;'>```Cursor = 74```</span>
  Cursor = 74,
  /// C++ enum variant: <span style='color: green;'>```KeySequence = 75```</span>
  KeySequence = 75,
  /// C++ enum variant: <span style='color: green;'>```Pen = 76```</span>
  Pen = 76,
  /// C++ enum variant: <span style='color: green;'>```TextLength = 77```</span>
  TextLength = 77,
  /// C++ enum variant: <span style='color: green;'>```TextFormat = 78```</span>
  TextFormat = 78,
  /// C++ enum variant: <span style='color: green;'>```Matrix = 79```</span>
  Matrix = 79,
  /// C++ enum variant: <span style='color: green;'>```Transform = 80```</span>
  Transform = 80,
  /// C++ enum variant: <span style='color: green;'>```Matrix4x4 = 81```</span>
  Matrix4X4 = 81,
  /// C++ enum variant: <span style='color: green;'>```Vector2D = 82```</span>
  Vector2D = 82,
  /// C++ enum variant: <span style='color: green;'>```Vector3D = 83```</span>
  Vector3D = 83,
  /// C++ enum variant: <span style='color: green;'>```Vector4D = 84```</span>
  Vector4D = 84,
  /// C++ enum variant: <span style='color: green;'>```Quaternion = 85```</span>
  Quaternion = 85,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```PolygonF = 86```</span>
  /// - <span style='color: green;'>```LastGuiType = 86```</span>
  ///
  PolygonF = 86,
  /// C++ enum variant: <span style='color: green;'>```SizePolicy = 121```</span>
  SizePolicy = 121,
  /// C++ enum variant: <span style='color: green;'>```UserType = 1024```</span>
  UserType = 1024,
}

/// C++ type: <span style='color: green;'>```QVariant```</span>
#[repr(C)]
pub struct Variant([u8; ::type_sizes::QT_CORE_VARIANT_VARIANT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Variant {
  unsafe fn new_uninitialized() -> Variant {
    Variant(::std::mem::uninitialized())
  }
}

impl Variant {
  /// C++ method: <span style='color: green;'>```bool QVariant::canConvert(int targetTypeId) const```</span>
  ///
  ///
  pub fn can_convert(&self, target_type_id: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QVariant_canConvert(self as *const ::variant::Variant, target_type_id) }
  }

  /// C++ method: <span style='color: green;'>```void QVariant::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QVariant_clear(self as *mut ::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```bool QVariant::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVariant_isNull(self as *const ::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```bool QVariant::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVariant_isValid(self as *const ::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```void QVariant::load(QDataStream& ds)```</span>
  ///
  ///
  pub fn load(&mut self, ds: &mut ::data_stream::DataStream) {
    unsafe {
      ::ffi::qt_core_c_QVariant_load(self as *mut ::variant::Variant,
                                     ds as *mut ::data_stream::DataStream)
    }
  }

  /// C++ method: <span style='color: green;'>```static QVariant::Type QVariant::nameToType(const char* name)```</span>
  ///
  ///
  pub unsafe fn name_to_type(name: *const ::libc::c_char) -> ::variant::Type {
    ::ffi::qt_core_c_QVariant_nameToType(name)
  }

  /// C++ method: <span style='color: green;'>```QVariant::QVariant```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new0(()) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new0(&::char::Char) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(QChar qchar)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new0(&mut ::data_stream::DataStream) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(QDataStream& s)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new0(&::latin1_string::Latin1String) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(QLatin1String string)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new0(::variant::Type) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(QVariant::Type type)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new0(bool) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(bool b)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new0(&::bit_array::BitArray) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QBitArray& bitarray)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new0(&::byte_array::ByteArray) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QByteArray& bytearray)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn new0(&::date::Date) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QDate& date)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn new0(&::date_time::DateTime) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QDateTime& datetime)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn new0(&::easing_curve::EasingCurve) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QEasingCurve& easing)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn new0(&::hash::HashStringVariant) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QHash<QString, QVariant>& hash)```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn new0(&::json_array::JsonArray) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QJsonArray& jsonArray)```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn new0(&::json_document::JsonDocument) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QJsonDocument& jsonDocument)```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn new0(&::json_object::JsonObject) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QJsonObject& jsonObject)```</span>
  ///
  ///
  ///
  /// ## Variant 16
  ///
  /// Rust arguments: ```fn new0(&::json_value::JsonValue) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QJsonValue& jsonValue)```</span>
  ///
  ///
  ///
  /// ## Variant 17
  ///
  /// Rust arguments: ```fn new0(&::line::Line) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QLine& line)```</span>
  ///
  ///
  ///
  /// ## Variant 18
  ///
  /// Rust arguments: ```fn new0(&::line_f::LineF) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QLineF& line)```</span>
  ///
  ///
  ///
  /// ## Variant 19
  ///
  /// Rust arguments: ```fn new0(&::list::ListVariant) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QList<QVariant>& list)```</span>
  ///
  ///
  ///
  /// ## Variant 20
  ///
  /// Rust arguments: ```fn new0(&::locale::Locale) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QLocale& locale)```</span>
  ///
  ///
  ///
  /// ## Variant 21
  ///
  /// Rust arguments: ```fn new0(&::map::MapStringVariant) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QMap<QString, QVariant>& map)```</span>
  ///
  ///
  ///
  /// ## Variant 22
  ///
  /// Rust arguments: ```fn new0(&::model_index::ModelIndex) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QModelIndex& modelIndex)```</span>
  ///
  ///
  ///
  /// ## Variant 23
  ///
  /// Rust arguments: ```fn new0(&::persistent_model_index::PersistentModelIndex) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QPersistentModelIndex& modelIndex)```</span>
  ///
  ///
  ///
  /// ## Variant 24
  ///
  /// Rust arguments: ```fn new0(&::point::Point) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QPoint& pt)```</span>
  ///
  ///
  ///
  /// ## Variant 25
  ///
  /// Rust arguments: ```fn new0(&::point_f::PointF) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QPointF& pt)```</span>
  ///
  ///
  ///
  /// ## Variant 26
  ///
  /// Rust arguments: ```fn new0(&::rect::Rect) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QRect& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 27
  ///
  /// Rust arguments: ```fn new0(&::rect_f::RectF) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 28
  ///
  /// Rust arguments: ```fn new0(&::reg_exp::RegExp) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QRegExp& regExp)```</span>
  ///
  ///
  ///
  /// ## Variant 29
  ///
  /// Rust arguments: ```fn new0(&::regular_expression::RegularExpression) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QRegularExpression& re)```</span>
  ///
  ///
  ///
  /// ## Variant 30
  ///
  /// Rust arguments: ```fn new0(&::size::Size) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QSize& size)```</span>
  ///
  ///
  ///
  /// ## Variant 31
  ///
  /// Rust arguments: ```fn new0(&::size_f::SizeF) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QSizeF& size)```</span>
  ///
  ///
  ///
  /// ## Variant 32
  ///
  /// Rust arguments: ```fn new0(&::string::String) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QString& string)```</span>
  ///
  ///
  ///
  /// ## Variant 33
  ///
  /// Rust arguments: ```fn new0(&::string_list::StringList) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QStringList& stringlist)```</span>
  ///
  ///
  ///
  /// ## Variant 34
  ///
  /// Rust arguments: ```fn new0(&::time::Time) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QTime& time)```</span>
  ///
  ///
  ///
  /// ## Variant 35
  ///
  /// Rust arguments: ```fn new0(&::url::Url) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QUrl& url)```</span>
  ///
  ///
  ///
  /// ## Variant 36
  ///
  /// Rust arguments: ```fn new0(&::uuid::Uuid) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QUuid& uuid)```</span>
  ///
  ///
  ///
  /// ## Variant 37
  ///
  /// Rust arguments: ```fn new0(&::variant::Variant) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const QVariant& other)```</span>
  ///
  ///
  ///
  /// ## Variant 38
  ///
  /// Rust arguments: ```fn new0(::libc::c_double) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(double d)```</span>
  ///
  ///
  ///
  /// ## Variant 39
  ///
  /// Rust arguments: ```fn new0(::libc::c_int) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 40
  ///
  /// Rust arguments: ```fn new0(u64) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(qulonglong ull)```</span>
  ///
  ///
  pub fn new0<Args>(args: Args) -> ::variant::Variant
    where Args: overloading::VariantNew0Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVariant::QVariant```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new1(*const ::libc::c_char) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(const char* str)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new1((::libc::c_int, *const ::libc::c_void)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(int typeId, const void* copy)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new1((::libc::c_int, *const ::libc::c_void, ::libc::c_uint)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(int typeId, const void* copy, unsigned int flags)```</span>
  ///
  ///
  pub unsafe fn new1<Args>(args: Args) -> ::variant::Variant
    where Args: overloading::VariantNew1Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVariant::QVariant```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new2(::libc::c_float) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(float f)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new2(i64) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(qlonglong ll)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new2(::libc::c_uint) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVariant::QVariant(unsigned int ui)```</span>
  ///
  ///
  pub fn new2<Args>(args: Args) -> ::variant::Variant
    where Args: overloading::VariantNew2Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVariant& QVariant::operator=(const QVariant& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::variant::Variant) -> &'l0 mut ::variant::Variant {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QVariant_operator_assign(self as *mut ::variant::Variant,
                                                other as *const ::variant::Variant)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVariant::operator==(const QVariant& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::variant::Variant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVariant_operator_eq(self as *const ::variant::Variant,
                                            v as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVariant::operator>=(const QVariant& v) const```</span>
  ///
  ///
  pub fn op_ge(&self, v: &::variant::Variant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVariant_operator_ge(self as *const ::variant::Variant,
                                            v as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVariant::operator>(const QVariant& v) const```</span>
  ///
  ///
  pub fn op_gt(&self, v: &::variant::Variant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVariant_operator_gt(self as *const ::variant::Variant,
                                            v as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVariant::operator<=(const QVariant& v) const```</span>
  ///
  ///
  pub fn op_le(&self, v: &::variant::Variant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVariant_operator_le(self as *const ::variant::Variant,
                                            v as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVariant::operator<(const QVariant& v) const```</span>
  ///
  ///
  pub fn op_lt(&self, v: &::variant::Variant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVariant_operator_lt(self as *const ::variant::Variant,
                                            v as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVariant::operator!=(const QVariant& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::variant::Variant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVariant_operator_neq(self as *const ::variant::Variant,
                                             v as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVariant::save(QDataStream& ds) const```</span>
  ///
  ///
  pub fn save(&self, ds: &mut ::data_stream::DataStream) {
    unsafe {
      ::ffi::qt_core_c_QVariant_save(self as *const ::variant::Variant,
                                     ds as *mut ::data_stream::DataStream)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVariant::swap(QVariant& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::variant::Variant) {
    unsafe {
      ::ffi::qt_core_c_QVariant_swap(self as *mut ::variant::Variant,
                                     other as *mut ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```QBitArray QVariant::toBitArray() const```</span>
  ///
  ///
  pub fn to_bit_array(&self) -> ::bit_array::BitArray {
    {
      let mut object: ::bit_array::BitArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toBitArray_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVariant::toBool() const```</span>
  ///
  ///
  pub fn to_bool(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVariant_toBool(self as *const ::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QVariant::toByteArray() const```</span>
  ///
  ///
  pub fn to_byte_array(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toByteArray_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QChar QVariant::toChar() const```</span>
  ///
  ///
  pub fn to_char(&self) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toChar_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDate QVariant::toDate() const```</span>
  ///
  ///
  pub fn to_date(&self) -> ::date::Date {
    {
      let mut object: ::date::Date = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toDate_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QVariant::toDateTime() const```</span>
  ///
  ///
  pub fn to_date_time(&self) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toDateTime_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QVariant::toDouble() const```</span>
  ///
  ///
  pub fn to_double(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QVariant_toDouble_no_args(self as *const ::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```double QVariant::toDouble(bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn to_double_unsafe(&self, ok: *mut bool) -> ::libc::c_double {
    ::ffi::qt_core_c_QVariant_toDouble_ok(self as *const ::variant::Variant, ok)
  }

  /// C++ method: <span style='color: green;'>```QEasingCurve QVariant::toEasingCurve() const```</span>
  ///
  ///
  pub fn to_easing_curve(&self) -> ::easing_curve::EasingCurve {
    {
      let mut object: ::easing_curve::EasingCurve =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toEasingCurve_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float QVariant::toFloat() const```</span>
  ///
  ///
  pub fn to_float(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_core_c_QVariant_toFloat_no_args(self as *const ::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```float QVariant::toFloat(bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn to_float_unsafe(&self, ok: *mut bool) -> ::libc::c_float {
    ::ffi::qt_core_c_QVariant_toFloat_ok(self as *const ::variant::Variant, ok)
  }

  /// C++ method: <span style='color: green;'>```QHash<QString, QVariant> QVariant::toHash() const```</span>
  ///
  ///
  pub fn to_hash(&self) -> ::hash::HashStringVariant {
    {
      let mut object: ::hash::HashStringVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toHash_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QVariant::toInt() const```</span>
  ///
  ///
  pub fn to_int(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVariant_toInt_no_args(self as *const ::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```int QVariant::toInt(bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn to_int_unsafe(&self, ok: *mut bool) -> ::libc::c_int {
    ::ffi::qt_core_c_QVariant_toInt_ok(self as *const ::variant::Variant, ok)
  }

  /// C++ method: <span style='color: green;'>```QJsonArray QVariant::toJsonArray() const```</span>
  ///
  ///
  pub fn to_json_array(&self) -> ::json_array::JsonArray {
    {
      let mut object: ::json_array::JsonArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toJsonArray_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonDocument QVariant::toJsonDocument() const```</span>
  ///
  ///
  pub fn to_json_document(&self) -> ::json_document::JsonDocument {
    {
      let mut object: ::json_document::JsonDocument =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toJsonDocument_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject QVariant::toJsonObject() const```</span>
  ///
  ///
  pub fn to_json_object(&self) -> ::json_object::JsonObject {
    {
      let mut object: ::json_object::JsonObject =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toJsonObject_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValue QVariant::toJsonValue() const```</span>
  ///
  ///
  pub fn to_json_value(&self) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toJsonValue_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLine QVariant::toLine() const```</span>
  ///
  ///
  pub fn to_line(&self) -> ::line::Line {
    {
      let mut object: ::line::Line = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toLine_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLineF QVariant::toLineF() const```</span>
  ///
  ///
  pub fn to_line_f(&self) -> ::line_f::LineF {
    {
      let mut object: ::line_f::LineF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toLineF_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QVariant> QVariant::toList() const```</span>
  ///
  ///
  pub fn to_list(&self) -> ::list::ListVariant {
    {
      let mut object: ::list::ListVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toList_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLocale QVariant::toLocale() const```</span>
  ///
  ///
  pub fn to_locale(&self) -> ::locale::Locale {
    {
      let mut object: ::locale::Locale =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toLocale_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```qlonglong QVariant::toLongLong() const```</span>
  ///
  ///
  pub fn to_long_long(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QVariant_toLongLong_no_args(self as *const ::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```qlonglong QVariant::toLongLong(bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn to_long_long_unsafe(&self, ok: *mut bool) -> i64 {
    ::ffi::qt_core_c_QVariant_toLongLong_ok(self as *const ::variant::Variant, ok)
  }

  /// C++ method: <span style='color: green;'>```QMap<QString, QVariant> QVariant::toMap() const```</span>
  ///
  ///
  pub fn to_map(&self) -> ::map::MapStringVariant {
    {
      let mut object: ::map::MapStringVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toMap_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QVariant::toModelIndex() const```</span>
  ///
  ///
  pub fn to_model_index(&self) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toModelIndex_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPersistentModelIndex QVariant::toPersistentModelIndex() const```</span>
  ///
  ///
  pub fn to_persistent_model_index(&self) -> ::persistent_model_index::PersistentModelIndex {
    {
      let mut object: ::persistent_model_index::PersistentModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toPersistentModelIndex_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QVariant::toPoint() const```</span>
  ///
  ///
  pub fn to_point(&self) -> ::point::Point {
    {
      let mut object: ::point::Point = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toPoint_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QVariant::toPointF() const```</span>
  ///
  ///
  pub fn to_point_f(&self) -> ::point_f::PointF {
    {
      let mut object: ::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toPointF_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QVariant::toReal() const```</span>
  ///
  ///
  pub fn to_real(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QVariant_toReal_no_args(self as *const ::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```double QVariant::toReal(bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn to_real_unsafe(&self, ok: *mut bool) -> ::libc::c_double {
    ::ffi::qt_core_c_QVariant_toReal_ok(self as *const ::variant::Variant, ok)
  }

  /// C++ method: <span style='color: green;'>```QRect QVariant::toRect() const```</span>
  ///
  ///
  pub fn to_rect(&self) -> ::rect::Rect {
    {
      let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toRect_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QVariant::toRectF() const```</span>
  ///
  ///
  pub fn to_rect_f(&self) -> ::rect_f::RectF {
    {
      let mut object: ::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toRectF_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRegExp QVariant::toRegExp() const```</span>
  ///
  ///
  pub fn to_reg_exp(&self) -> ::reg_exp::RegExp {
    {
      let mut object: ::reg_exp::RegExp =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toRegExp_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRegularExpression QVariant::toRegularExpression() const```</span>
  ///
  ///
  pub fn to_regular_expression(&self) -> ::regular_expression::RegularExpression {
    {
      let mut object: ::regular_expression::RegularExpression =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toRegularExpression_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QVariant::toSize() const```</span>
  ///
  ///
  pub fn to_size(&self) -> ::size::Size {
    {
      let mut object: ::size::Size = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toSize_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QVariant::toSizeF() const```</span>
  ///
  ///
  pub fn to_size_f(&self) -> ::size_f::SizeF {
    {
      let mut object: ::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toSizeF_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QVariant::toString() const```</span>
  ///
  ///
  pub fn to_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toString_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QVariant::toStringList() const```</span>
  ///
  ///
  pub fn to_string_list(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toStringList_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTime QVariant::toTime() const```</span>
  ///
  ///
  pub fn to_time(&self) -> ::time::Time {
    {
      let mut object: ::time::Time = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toTime_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QVariant::toUInt() const```</span>
  ///
  ///
  pub fn to_u_int(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QVariant_toUInt_no_args(self as *const ::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QVariant::toUInt(bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn to_u_int_unsafe(&self, ok: *mut bool) -> ::libc::c_uint {
    ::ffi::qt_core_c_QVariant_toUInt_ok(self as *const ::variant::Variant, ok)
  }

  /// C++ method: <span style='color: green;'>```qulonglong QVariant::toULongLong() const```</span>
  ///
  ///
  pub fn to_u_long_long(&self) -> u64 {
    unsafe { ::ffi::qt_core_c_QVariant_toULongLong_no_args(self as *const ::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```qulonglong QVariant::toULongLong(bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn to_u_long_long_unsafe(&self, ok: *mut bool) -> u64 {
    ::ffi::qt_core_c_QVariant_toULongLong_ok(self as *const ::variant::Variant, ok)
  }

  /// C++ method: <span style='color: green;'>```QUrl QVariant::toUrl() const```</span>
  ///
  ///
  pub fn to_url(&self) -> ::url::Url {
    {
      let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toUrl_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUuid QVariant::toUuid() const```</span>
  ///
  ///
  pub fn to_uuid(&self) -> ::uuid::Uuid {
    {
      let mut object: ::uuid::Uuid = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVariant_toUuid_to_output(self as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant::Type QVariant::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::variant::Type {
    unsafe { ::ffi::qt_core_c_QVariant_type(self as *const ::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```const char* QVariant::typeName() const```</span>
  ///
  ///
  pub fn type_name(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QVariant_typeName(self as *const ::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```static const char* QVariant::typeToName(int typeId)```</span>
  ///
  ///
  pub fn type_to_name(type_id: ::libc::c_int) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QVariant_typeToName(type_id) }
  }

  /// C++ method: <span style='color: green;'>```int QVariant::userType() const```</span>
  ///
  ///
  pub fn user_type(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVariant_userType(self as *const ::variant::Variant) }
  }
}

impl Drop for ::variant::Variant {
  /// C++ method: <span style='color: green;'>```[destructor] void QVariant::~QVariant()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QVariant_destructor(self as *mut ::variant::Variant) }
  }
}

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::variant::Variant)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVariant& p)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::variant::Type)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, const QVariant::Type p)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::variant::Type)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QVariant::Type arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator>>```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shr((&'l0 mut ::data_stream::DataStream, &'l1 mut ::variant::Variant)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVariant& p)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shr((&'l0 mut ::data_stream::DataStream, &'l1 mut ::variant::Type)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& s, QVariant::Type& p)```</span>
///
///
pub fn op_shr<'largs, Args>(args: Args) -> &'largs mut ::data_stream::DataStream
  where Args: overloading::OpShrArgs<'largs>
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```void swap(QVariant& value1, QVariant& value2)```</span>
///
///
pub fn swap(value1: &mut ::variant::Variant, value2: &mut ::variant::Variant) {
  unsafe {
    ::ffi::qt_core_c_QVariant_G_swap(value1 as *mut ::variant::Variant,
                                     value2 as *mut ::variant::Variant)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Variant::new0](../struct.Variant.html#method.new0) method.
  pub trait VariantNew0Args {
    fn exec(self) -> ::variant::Variant;
  }
  impl<'a> VariantNew0Args for &'a ::bit_array::BitArray {
    fn exec(self) -> ::variant::Variant {
      let bitarray = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QBitArray(bitarray as *const ::bit_array::BitArray, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::variant::Variant {
      let bytearray = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QByteArray(bytearray as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::char::Char {
    fn exec(self) -> ::variant::Variant {
      let qchar = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QChar(qchar as *const ::char::Char, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a mut ::data_stream::DataStream {
    fn exec(self) -> ::variant::Variant {
      let s = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QDataStream(s as *mut ::data_stream::DataStream, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::date::Date {
    fn exec(self) -> ::variant::Variant {
      let date = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QDate(date as *const ::date::Date, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::date_time::DateTime {
    fn exec(self) -> ::variant::Variant {
      let datetime = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QDateTime(datetime as *const ::date_time::DateTime, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::easing_curve::EasingCurve {
    fn exec(self) -> ::variant::Variant {
      let easing = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QEasingCurve(easing as *const ::easing_curve::EasingCurve, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::hash::HashStringVariant {
    fn exec(self) -> ::variant::Variant {
      let hash = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QHash_QString_QVariant(hash as *const ::hash::HashStringVariant,
                                                                       &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::json_array::JsonArray {
    fn exec(self) -> ::variant::Variant {
      let json_array = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QJsonArray(json_array as *const ::json_array::JsonArray, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::json_document::JsonDocument {
    fn exec(self) -> ::variant::Variant {
      let json_document = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QJsonDocument(json_document as *const ::json_document::JsonDocument,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::json_object::JsonObject {
    fn exec(self) -> ::variant::Variant {
      let json_object = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QJsonObject(json_object as *const ::json_object::JsonObject,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::json_value::JsonValue {
    fn exec(self) -> ::variant::Variant {
      let json_value = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QJsonValue(json_value as *const ::json_value::JsonValue, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::latin1_string::Latin1String {
    fn exec(self) -> ::variant::Variant {
      let string = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QLatin1String(string as *const ::latin1_string::Latin1String,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::line::Line {
    fn exec(self) -> ::variant::Variant {
      let line = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QLine(line as *const ::line::Line, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::line_f::LineF {
    fn exec(self) -> ::variant::Variant {
      let line = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QLineF(line as *const ::line_f::LineF, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::list::ListVariant {
    fn exec(self) -> ::variant::Variant {
      let list = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QList_QVariant(list as *const ::list::ListVariant, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::locale::Locale {
    fn exec(self) -> ::variant::Variant {
      let locale = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QLocale(locale as *const ::locale::Locale, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::map::MapStringVariant {
    fn exec(self) -> ::variant::Variant {
      let map = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QMap_QString_QVariant(map as *const ::map::MapStringVariant,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::model_index::ModelIndex {
    fn exec(self) -> ::variant::Variant {
      let model_index = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QModelIndex(model_index as *const ::model_index::ModelIndex,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::persistent_model_index::PersistentModelIndex {
    fn exec(self) -> ::variant::Variant {
      let model_index = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QPersistentModelIndex(model_index as *const ::persistent_model_index::PersistentModelIndex, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::point::Point {
    fn exec(self) -> ::variant::Variant {
      let pt = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QPoint(pt as *const ::point::Point, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::point_f::PointF {
    fn exec(self) -> ::variant::Variant {
      let pt = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QPointF(pt as *const ::point_f::PointF, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::rect::Rect {
    fn exec(self) -> ::variant::Variant {
      let rect = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QRect(rect as *const ::rect::Rect, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::rect_f::RectF {
    fn exec(self) -> ::variant::Variant {
      let rect = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QRectF(rect as *const ::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::reg_exp::RegExp {
    fn exec(self) -> ::variant::Variant {
      let reg_exp = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QRegExp(reg_exp as *const ::reg_exp::RegExp, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::regular_expression::RegularExpression {
    fn exec(self) -> ::variant::Variant {
      let re = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QRegularExpression(re as *const ::regular_expression::RegularExpression, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::size::Size {
    fn exec(self) -> ::variant::Variant {
      let size = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QSize(size as *const ::size::Size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::size_f::SizeF {
    fn exec(self) -> ::variant::Variant {
      let size = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QSizeF(size as *const ::size_f::SizeF, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::string::String {
    fn exec(self) -> ::variant::Variant {
      let string = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QString(string as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::string_list::StringList {
    fn exec(self) -> ::variant::Variant {
      let stringlist = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QStringList(stringlist as *const ::string_list::StringList,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::time::Time {
    fn exec(self) -> ::variant::Variant {
      let time = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QTime(time as *const ::time::Time, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::url::Url {
    fn exec(self) -> ::variant::Variant {
      let url = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QUrl(url as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::uuid::Uuid {
    fn exec(self) -> ::variant::Variant {
      let uuid = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QUuid(uuid as *const ::uuid::Uuid, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VariantNew0Args for &'a ::variant::Variant {
    fn exec(self) -> ::variant::Variant {
      let other = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QVariant(other as *const ::variant::Variant, &mut object);
        }
        object
      }
    }
  }
  impl VariantNew0Args for ::variant::Type {
    fn exec(self) -> ::variant::Variant {
      let type_ = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_QVariant_Type(type_, &mut object);
        }
        object
      }
    }
  }
  impl VariantNew0Args for bool {
    fn exec(self) -> ::variant::Variant {
      let b = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_bool(b, &mut object);
        }
        object
      }
    }
  }
  impl VariantNew0Args for ::libc::c_double {
    fn exec(self) -> ::variant::Variant {
      let d = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_double(d, &mut object);
        }
        object
      }
    }
  }
  impl VariantNew0Args for ::libc::c_int {
    fn exec(self) -> ::variant::Variant {
      let i = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_int(i, &mut object);
        }
        object
      }
    }
  }
  impl VariantNew0Args for () {
    fn exec(self) -> ::variant::Variant {

      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VariantNew0Args for u64 {
    fn exec(self) -> ::variant::Variant {
      let ull = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_qulonglong(ull, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Variant::new1](../struct.Variant.html#method.new1) method.
  pub trait VariantNew1Args {
    unsafe fn exec(self) -> ::variant::Variant;
  }
  impl VariantNew1Args for *const ::libc::c_char {
    unsafe fn exec(self) -> ::variant::Variant {
      let str = self;
      {
        let mut object: ::variant::Variant = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QVariant_constructor_char(str, &mut object);
        object
      }
    }
  }
  impl VariantNew1Args for (::libc::c_int, *const ::libc::c_void) {
    unsafe fn exec(self) -> ::variant::Variant {
      let type_id = self.0;
      let copy = self.1;
      {
        let mut object: ::variant::Variant = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QVariant_constructor_int_void(type_id, copy, &mut object);
        object
      }
    }
  }
  impl VariantNew1Args for (::libc::c_int, *const ::libc::c_void, ::libc::c_uint) {
    unsafe fn exec(self) -> ::variant::Variant {
      let type_id = self.0;
      let copy = self.1;
      let flags = self.2;
      {
        let mut object: ::variant::Variant = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QVariant_constructor_int_void_unsigned_int(type_id, copy, flags, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Variant::new2](../struct.Variant.html#method.new2) method.
  pub trait VariantNew2Args {
    fn exec(self) -> ::variant::Variant;
  }
  impl VariantNew2Args for ::libc::c_float {
    fn exec(self) -> ::variant::Variant {
      let f = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_float(f, &mut object);
        }
        object
      }
    }
  }
  impl VariantNew2Args for i64 {
    fn exec(self) -> ::variant::Variant {
      let ll = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_qlonglong(ll, &mut object);
        }
        object
      }
    }
  }
  impl VariantNew2Args for ::libc::c_uint {
    fn exec(self) -> ::variant::Variant {
      let ui = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_constructor_unsigned_int(ui, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::variant::Variant) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let s = self.0;
      let p = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVariant_G_operator_shl_QDataStream_QVariant(s as *mut ::data_stream::DataStream,
                                                                        p as *const ::variant::Variant)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::variant::Type) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let s = self.0;
      let p = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVariant_G_operator_shl_QDataStream_QVariant_Type(s as *mut ::data_stream::DataStream,
                                                                             p as *const ::variant::Type)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::variant::Type) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVariant_G_operator_shl_to_output(arg1 as *const ::debug::Debug,
                                                             arg2 as *const ::variant::Type,
                                                             &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shr](../fn.op_shr.html) method.
  pub trait OpShrArgs<'largs> {
    fn exec(self) -> &'largs mut ::data_stream::DataStream;
  }
  impl<'largs> OpShrArgs<'largs> for (&'largs mut ::data_stream::DataStream, &'largs mut ::variant::Variant) {
    fn exec(self) -> &'largs mut ::data_stream::DataStream {
      let s = self.0;
      let p = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVariant_G_operator_shr_QDataStream_QVariant(s as *mut ::data_stream::DataStream,
                                                                        p as *mut ::variant::Variant)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> OpShrArgs<'largs> for (&'largs mut ::data_stream::DataStream, &'largs mut ::variant::Type) {
    fn exec(self) -> &'largs mut ::data_stream::DataStream {
      let s = self.0;
      let p = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVariant_G_operator_shr_QDataStream_QVariant_Type(s as *mut ::data_stream::DataStream,
                                                                             p as *mut ::variant::Type)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
}
