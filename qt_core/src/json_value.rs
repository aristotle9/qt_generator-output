/// C++ type: <span style='color: green;'>```QJsonValue```</span>
#[repr(C)]
pub struct JsonValue([u8; ::type_sizes::QT_CORE_JSON_VALUE_JSON_VALUE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for JsonValue {
  unsafe fn new_uninitialized() -> JsonValue {
    JsonValue(::std::mem::uninitialized())
  }
}

impl JsonValue {
  /// C++ method: <span style='color: green;'>```static QJsonValue QJsonValue::fromVariant(const QVariant& variant)```</span>
  ///
  ///
  pub fn from_variant(variant: &::variant::Variant) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonValue_fromVariant_to_output(variant as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValue::isArray() const```</span>
  ///
  ///
  pub fn is_array(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonValue_isArray(self as *const ::json_value::JsonValue) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValue::isBool() const```</span>
  ///
  ///
  pub fn is_bool(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonValue_isBool(self as *const ::json_value::JsonValue) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValue::isDouble() const```</span>
  ///
  ///
  pub fn is_double(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonValue_isDouble(self as *const ::json_value::JsonValue) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValue::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonValue_isNull(self as *const ::json_value::JsonValue) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValue::isObject() const```</span>
  ///
  ///
  pub fn is_object(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonValue_isObject(self as *const ::json_value::JsonValue) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValue::isString() const```</span>
  ///
  ///
  pub fn is_string(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonValue_isString(self as *const ::json_value::JsonValue) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValue::isUndefined() const```</span>
  ///
  ///
  pub fn is_undefined(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonValue_isUndefined(self as *const ::json_value::JsonValue) }
  }

  /// C++ method: <span style='color: green;'>```QJsonValue::QJsonValue```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new0(()) -> ::json_value::JsonValue```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonValue::QJsonValue()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new0(::json_value::Type) -> ::json_value::JsonValue```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonValue::QJsonValue(QJsonValue::Type arg1 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new0(&::latin1_string::Latin1String) -> ::json_value::JsonValue```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonValue::QJsonValue(QLatin1String s)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new0(bool) -> ::json_value::JsonValue```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonValue::QJsonValue(bool b)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new0(&::json_array::JsonArray) -> ::json_value::JsonValue```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonValue::QJsonValue(const QJsonArray& a)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new0(&::json_object::JsonObject) -> ::json_value::JsonValue```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonValue::QJsonValue(const QJsonObject& o)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new0(&::json_value::JsonValue) -> ::json_value::JsonValue```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonValue::QJsonValue(const QJsonValue& other)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new0(&::string::String) -> ::json_value::JsonValue```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonValue::QJsonValue(const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn new0(::libc::c_double) -> ::json_value::JsonValue```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonValue::QJsonValue(double n)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn new0(::libc::c_int) -> ::json_value::JsonValue```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonValue::QJsonValue(int n)```</span>
  ///
  ///
  pub fn new0<Args>(args: Args) -> ::json_value::JsonValue
    where Args: overloading::JsonValueNew0Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonValue::QJsonValue(const char* s)```</span>
  ///
  ///
  pub unsafe fn new1(s: *const ::libc::c_char) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QJsonValue_constructor_char(s, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QJsonValue::QJsonValue(qint64 n)```</span>
  ///
  ///
  pub fn new2(n: i64) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonValue_constructor_qint64(n, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValue& QJsonValue::operator=(const QJsonValue& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::json_value::JsonValue) -> &'l0 mut ::json_value::JsonValue {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QJsonValue_operator_assign(self as *mut ::json_value::JsonValue,
                                                  other as *const ::json_value::JsonValue)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValue::operator==(const QJsonValue& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::json_value::JsonValue) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonValue_operator_eq(self as *const ::json_value::JsonValue,
                                              other as *const ::json_value::JsonValue)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValue::operator!=(const QJsonValue& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::json_value::JsonValue) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonValue_operator_neq(self as *const ::json_value::JsonValue,
                                               other as *const ::json_value::JsonValue)
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValue::toArray```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_array(&self, ()) -> ::json_array::JsonArray```<br>
  /// C++ method: <span style='color: green;'>```QJsonArray QJsonValue::toArray() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_array(&self, &::json_array::JsonArray) -> ::json_array::JsonArray```<br>
  /// C++ method: <span style='color: green;'>```QJsonArray QJsonValue::toArray(const QJsonArray& defaultValue) const```</span>
  ///
  ///
  pub fn to_array<'largs, Args>(&'largs self, args: Args) -> ::json_array::JsonArray
    where Args: overloading::JsonValueToArrayArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonValue::toBool```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_bool(&self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonValue::toBool() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_bool(&self, bool) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonValue::toBool(bool defaultValue = ?) const```</span>
  ///
  ///
  pub fn to_bool<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::JsonValueToBoolArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonValue::toDouble```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_double(&self, ()) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QJsonValue::toDouble() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_double(&self, ::libc::c_double) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QJsonValue::toDouble(double defaultValue = ?) const```</span>
  ///
  ///
  pub fn to_double<'largs, Args>(&'largs self, args: Args) -> ::libc::c_double
    where Args: overloading::JsonValueToDoubleArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonValue::toInt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_int(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QJsonValue::toInt() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_int(&self, ::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QJsonValue::toInt(int defaultValue = ?) const```</span>
  ///
  ///
  pub fn to_int<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::JsonValueToIntArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonValue::toObject```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_object(&self, ()) -> ::json_object::JsonObject```<br>
  /// C++ method: <span style='color: green;'>```QJsonObject QJsonValue::toObject() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_object(&self, &::json_object::JsonObject) -> ::json_object::JsonObject```<br>
  /// C++ method: <span style='color: green;'>```QJsonObject QJsonValue::toObject(const QJsonObject& defaultValue) const```</span>
  ///
  ///
  pub fn to_object<'largs, Args>(&'largs self, args: Args) -> ::json_object::JsonObject
    where Args: overloading::JsonValueToObjectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonValue::toString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_string(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QJsonValue::toString() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_string(&self, &::string::String) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QJsonValue::toString(const QString& defaultValue) const```</span>
  ///
  ///
  pub fn to_string<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::JsonValueToStringArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVariant QJsonValue::toVariant() const```</span>
  ///
  ///
  pub fn to_variant(&self) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonValue_toVariant_to_output(self as *const ::json_value::JsonValue, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValue::Type QJsonValue::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::json_value::Type {
    unsafe { ::ffi::qt_core_c_QJsonValue_type(self as *const ::json_value::JsonValue) }
  }
}

impl Drop for ::json_value::JsonValue {
  /// C++ method: <span style='color: green;'>```[destructor] void QJsonValue::~QJsonValue()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QJsonValue_destructor(self as *mut ::json_value::JsonValue) }
  }
}

/// C++ type: <span style='color: green;'>```QJsonValue::Type```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Type {
  /// C++ enum variant: <span style='color: green;'>```Null = 0```</span>
  Null = 0,
  /// C++ enum variant: <span style='color: green;'>```Bool = 1```</span>
  Bool = 1,
  /// C++ enum variant: <span style='color: green;'>```Double = 2```</span>
  Double = 2,
  /// C++ enum variant: <span style='color: green;'>```String = 3```</span>
  String = 3,
  /// C++ enum variant: <span style='color: green;'>```Array = 4```</span>
  Array = 4,
  /// C++ enum variant: <span style='color: green;'>```Object = 5```</span>
  Object = 5,
  /// C++ enum variant: <span style='color: green;'>```Undefined = 128```</span>
  Undefined = 128,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [JsonValue::new0](../struct.JsonValue.html#method.new0) method.
  pub trait JsonValueNew0Args {
    fn exec(self) -> ::json_value::JsonValue;
  }
  impl<'a> JsonValueNew0Args for &'a ::json_array::JsonArray {
    fn exec(self) -> ::json_value::JsonValue {
      let a = self;
      {
        let mut object: ::json_value::JsonValue =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_constructor_QJsonArray(a as *const ::json_array::JsonArray, &mut object);
        }
        object
      }
    }
  }
  impl<'a> JsonValueNew0Args for &'a ::json_object::JsonObject {
    fn exec(self) -> ::json_value::JsonValue {
      let o = self;
      {
        let mut object: ::json_value::JsonValue =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_constructor_QJsonObject(o as *const ::json_object::JsonObject, &mut object);
        }
        object
      }
    }
  }
  impl<'a> JsonValueNew0Args for &'a ::json_value::JsonValue {
    fn exec(self) -> ::json_value::JsonValue {
      let other = self;
      {
        let mut object: ::json_value::JsonValue =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_constructor_QJsonValue(other as *const ::json_value::JsonValue, &mut object);
        }
        object
      }
    }
  }
  impl JsonValueNew0Args for ::json_value::Type {
    fn exec(self) -> ::json_value::JsonValue {
      let arg1 = self;
      {
        let mut object: ::json_value::JsonValue =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_constructor_QJsonValue_Type(arg1, &mut object);
        }
        object
      }
    }
  }
  impl<'a> JsonValueNew0Args for &'a ::latin1_string::Latin1String {
    fn exec(self) -> ::json_value::JsonValue {
      let s = self;
      {
        let mut object: ::json_value::JsonValue =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_constructor_QLatin1String(s as *const ::latin1_string::Latin1String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> JsonValueNew0Args for &'a ::string::String {
    fn exec(self) -> ::json_value::JsonValue {
      let s = self;
      {
        let mut object: ::json_value::JsonValue =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_constructor_QString(s as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl JsonValueNew0Args for bool {
    fn exec(self) -> ::json_value::JsonValue {
      let b = self;
      {
        let mut object: ::json_value::JsonValue =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_constructor_bool(b, &mut object);
        }
        object
      }
    }
  }
  impl JsonValueNew0Args for ::libc::c_double {
    fn exec(self) -> ::json_value::JsonValue {
      let n = self;
      {
        let mut object: ::json_value::JsonValue =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_constructor_double(n, &mut object);
        }
        object
      }
    }
  }
  impl JsonValueNew0Args for ::libc::c_int {
    fn exec(self) -> ::json_value::JsonValue {
      let n = self;
      {
        let mut object: ::json_value::JsonValue =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_constructor_int(n, &mut object);
        }
        object
      }
    }
  }
  impl JsonValueNew0Args for () {
    fn exec(self) -> ::json_value::JsonValue {

      {
        let mut object: ::json_value::JsonValue =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonValue::to_array](../struct.JsonValue.html#method.to_array) method.
  pub trait JsonValueToArrayArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::json_array::JsonArray;
  }
  impl<'largs> JsonValueToArrayArgs<'largs> for &'largs ::json_array::JsonArray {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::json_array::JsonArray {
      let default_value = self;
      {
        let mut object: ::json_array::JsonArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_toArray_to_output_defaultValue(original_self as *const ::json_value::JsonValue,
                                                                     default_value as *const ::json_array::JsonArray,
                                                                     &mut object);
        }
        object
      }
    }
  }
  impl<'largs> JsonValueToArrayArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::json_array::JsonArray {

      {
        let mut object: ::json_array::JsonArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_toArray_to_output_no_args(original_self as *const ::json_value::JsonValue,
                                                                &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonValue::to_bool](../struct.JsonValue.html#method.to_bool) method.
  pub trait JsonValueToBoolArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> bool;
  }
  impl<'largs> JsonValueToBoolArgs<'largs> for bool {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> bool {
      let default_value = self;
      unsafe {
        ::ffi::qt_core_c_QJsonValue_toBool_defaultValue(original_self as *const ::json_value::JsonValue,
                                                        default_value)
      }
    }
  }
  impl<'largs> JsonValueToBoolArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> bool {

      unsafe { ::ffi::qt_core_c_QJsonValue_toBool_no_args(original_self as *const ::json_value::JsonValue) }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonValue::to_double](../struct.JsonValue.html#method.to_double) method.
  pub trait JsonValueToDoubleArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::libc::c_double;
  }
  impl<'largs> JsonValueToDoubleArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::libc::c_double {
      let default_value = self;
      unsafe {
        ::ffi::qt_core_c_QJsonValue_toDouble_defaultValue(original_self as *const ::json_value::JsonValue,
                                                          default_value)
      }
    }
  }
  impl<'largs> JsonValueToDoubleArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::libc::c_double {

      unsafe { ::ffi::qt_core_c_QJsonValue_toDouble_no_args(original_self as *const ::json_value::JsonValue) }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonValue::to_int](../struct.JsonValue.html#method.to_int) method.
  pub trait JsonValueToIntArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::libc::c_int;
  }
  impl<'largs> JsonValueToIntArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::libc::c_int {
      let default_value = self;
      unsafe {
        ::ffi::qt_core_c_QJsonValue_toInt_defaultValue(original_self as *const ::json_value::JsonValue,
                                                       default_value)
      }
    }
  }
  impl<'largs> JsonValueToIntArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QJsonValue_toInt_no_args(original_self as *const ::json_value::JsonValue) }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonValue::to_object](../struct.JsonValue.html#method.to_object) method.
  pub trait JsonValueToObjectArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::json_object::JsonObject;
  }
  impl<'largs> JsonValueToObjectArgs<'largs> for &'largs ::json_object::JsonObject {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::json_object::JsonObject {
      let default_value = self;
      {
        let mut object: ::json_object::JsonObject =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_toObject_to_output_defaultValue(original_self as *const ::json_value::JsonValue, default_value as *const ::json_object::JsonObject, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> JsonValueToObjectArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::json_object::JsonObject {

      {
        let mut object: ::json_object::JsonObject =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_toObject_to_output_no_args(original_self as *const ::json_value::JsonValue,
                                                                 &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonValue::to_string](../struct.JsonValue.html#method.to_string) method.
  pub trait JsonValueToStringArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::string::String;
  }
  impl<'largs> JsonValueToStringArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::string::String {
      let default_value = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_toString_to_output_defaultValue(original_self as *const ::json_value::JsonValue, default_value as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> JsonValueToStringArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::json_value::JsonValue) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValue_toString_to_output_no_args(original_self as *const ::json_value::JsonValue,
                                                                 &mut object);
        }
        object
      }
    }
  }
}
