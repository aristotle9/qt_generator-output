/// C++ type: <span style='color: green;'>```QJsonValueRef```</span>
#[repr(C)]
pub struct JsonValueRef([u8; ::type_sizes::QT_CORE_JSON_VALUE_REF_JSON_VALUE_REF]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for JsonValueRef {
  unsafe fn new_uninitialized() -> JsonValueRef {
    JsonValueRef(::std::mem::uninitialized())
  }
}

impl JsonValueRef {
  /// C++ method: <span style='color: green;'>```QJsonValue QJsonValueRef::operator QJsonValue() const```</span>
  ///
  ///
  pub fn as_q_json_value(&self) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonValueRef_convert_to_QJsonValue_to_output(self as *const ::json_value_ref::JsonValueRef,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValueRef::isArray() const```</span>
  ///
  ///
  pub fn is_array(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonValueRef_isArray(self as *const ::json_value_ref::JsonValueRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValueRef::isBool() const```</span>
  ///
  ///
  pub fn is_bool(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonValueRef_isBool(self as *const ::json_value_ref::JsonValueRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValueRef::isDouble() const```</span>
  ///
  ///
  pub fn is_double(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonValueRef_isDouble(self as *const ::json_value_ref::JsonValueRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValueRef::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonValueRef_isNull(self as *const ::json_value_ref::JsonValueRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValueRef::isObject() const```</span>
  ///
  ///
  pub fn is_object(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonValueRef_isObject(self as *const ::json_value_ref::JsonValueRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValueRef::isString() const```</span>
  ///
  ///
  pub fn is_string(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonValueRef_isString(self as *const ::json_value_ref::JsonValueRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValueRef::isUndefined() const```</span>
  ///
  ///
  pub fn is_undefined(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonValueRef_isUndefined(self as *const ::json_value_ref::JsonValueRef) }
  }

  /// C++ method: <span style='color: green;'>```QJsonValueRef::QJsonValueRef```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((*mut ::json_array::JsonArray, ::libc::c_int)) -> ::json_value_ref::JsonValueRef```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonValueRef::QJsonValueRef(QJsonArray* array, int idx)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((*mut ::json_object::JsonObject, ::libc::c_int)) -> ::json_value_ref::JsonValueRef```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonValueRef::QJsonValueRef(QJsonObject* object, int idx)```</span>
  ///
  ///
  pub unsafe fn new<Args>(args: Args) -> ::json_value_ref::JsonValueRef
    where Args: overloading::JsonValueRefNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QJsonValueRef::operator=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::json_value::JsonValue) -> &'l0 mut ::json_value_ref::JsonValueRef```<br>
  /// C++ method: <span style='color: green;'>```QJsonValueRef& QJsonValueRef::operator=(const QJsonValue& val)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::json_value_ref::JsonValueRef) -> &'l0 mut ::json_value_ref::JsonValueRef```<br>
  /// C++ method: <span style='color: green;'>```QJsonValueRef& QJsonValueRef::operator=(const QJsonValueRef& val)```</span>
  ///
  ///
  pub fn op_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::json_value_ref::JsonValueRef
    where Args: overloading::JsonValueRefOpAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QJsonValueRef::operator==(const QJsonValue& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::json_value::JsonValue) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonValueRef_operator_eq(self as *const ::json_value_ref::JsonValueRef,
                                                 other as *const ::json_value::JsonValue)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonValueRef::operator!=(const QJsonValue& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::json_value::JsonValue) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonValueRef_operator_neq(self as *const ::json_value_ref::JsonValueRef,
                                                  other as *const ::json_value::JsonValue)
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray QJsonValueRef::toArray() const```</span>
  ///
  ///
  pub fn to_array(&self) -> ::json_array::JsonArray {
    {
      let mut object: ::json_array::JsonArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonValueRef_toArray_to_output(self as *const ::json_value_ref::JsonValueRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValueRef::toBool```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_bool(&self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonValueRef::toBool() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_bool(&self, bool) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonValueRef::toBool(bool defaultValue) const```</span>
  ///
  ///
  pub fn to_bool<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::JsonValueRefToBoolArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonValueRef::toDouble```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_double(&self, ()) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QJsonValueRef::toDouble() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_double(&self, ::libc::c_double) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QJsonValueRef::toDouble(double defaultValue) const```</span>
  ///
  ///
  pub fn to_double<'largs, Args>(&'largs self, args: Args) -> ::libc::c_double
    where Args: overloading::JsonValueRefToDoubleArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonValueRef::toInt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_int(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QJsonValueRef::toInt() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_int(&self, ::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QJsonValueRef::toInt(int defaultValue) const```</span>
  ///
  ///
  pub fn to_int<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::JsonValueRefToIntArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonObject QJsonValueRef::toObject() const```</span>
  ///
  ///
  pub fn to_object(&self) -> ::json_object::JsonObject {
    {
      let mut object: ::json_object::JsonObject =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonValueRef_toObject_to_output(self as *const ::json_value_ref::JsonValueRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValueRef::toString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_string(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QJsonValueRef::toString() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_string(&self, &::string::String) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QJsonValueRef::toString(const QString& defaultValue) const```</span>
  ///
  ///
  pub fn to_string<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::JsonValueRefToStringArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVariant QJsonValueRef::toVariant() const```</span>
  ///
  ///
  pub fn to_variant(&self) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonValueRef_toVariant_to_output(self as *const ::json_value_ref::JsonValueRef, &mut object);
      }
      object
    }
  }
}

impl Drop for ::json_value_ref::JsonValueRef {
  /// C++ method: <span style='color: green;'>```[destructor] void QJsonValueRef::~QJsonValueRef()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QJsonValueRef_destructor(self as *mut ::json_value_ref::JsonValueRef) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [JsonValueRef::new](../struct.JsonValueRef.html#method.new) method.
  pub trait JsonValueRefNewArgs {
    unsafe fn exec(self) -> ::json_value_ref::JsonValueRef;
  }
  impl JsonValueRefNewArgs for (*mut ::json_array::JsonArray, ::libc::c_int) {
    unsafe fn exec(self) -> ::json_value_ref::JsonValueRef {
      let array = self.0;
      let idx = self.1;
      {
        let mut object: ::json_value_ref::JsonValueRef =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QJsonValueRef_constructor_array_idx(array, idx, &mut object);
        object
      }
    }
  }
  impl JsonValueRefNewArgs for (*mut ::json_object::JsonObject, ::libc::c_int) {
    unsafe fn exec(self) -> ::json_value_ref::JsonValueRef {
      let object = self.0;
      let idx = self.1;
      {
        let mut object2: ::json_value_ref::JsonValueRef =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QJsonValueRef_constructor_object_idx(object, idx, &mut object2);
        object2
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonValueRef::op_assign](../struct.JsonValueRef.html#method.op_assign) method.
  pub trait JsonValueRefOpAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::json_value_ref::JsonValueRef)
            -> &'largs mut ::json_value_ref::JsonValueRef;
  }
  impl<'largs> JsonValueRefOpAssignArgs<'largs> for &'largs ::json_value::JsonValue {
    fn exec(self,
            original_self: &'largs mut ::json_value_ref::JsonValueRef)
            -> &'largs mut ::json_value_ref::JsonValueRef {
      let val = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QJsonValueRef_operator_assign_QJsonValue(original_self as *mut ::json_value_ref::JsonValueRef, val as *const ::json_value::JsonValue) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> JsonValueRefOpAssignArgs<'largs> for &'largs ::json_value_ref::JsonValueRef {
    fn exec(self,
            original_self: &'largs mut ::json_value_ref::JsonValueRef)
            -> &'largs mut ::json_value_ref::JsonValueRef {
      let val = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QJsonValueRef_operator_assign_QJsonValueRef(original_self as *mut ::json_value_ref::JsonValueRef, val as *const ::json_value_ref::JsonValueRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [JsonValueRef::to_bool](../struct.JsonValueRef.html#method.to_bool) method.
  pub trait JsonValueRefToBoolArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_value_ref::JsonValueRef) -> bool;
  }
  impl<'largs> JsonValueRefToBoolArgs<'largs> for bool {
    fn exec(self, original_self: &'largs ::json_value_ref::JsonValueRef) -> bool {
      let default_value = self;
      unsafe {
        ::ffi::qt_core_c_QJsonValueRef_toBool_defaultValue(original_self as *const ::json_value_ref::JsonValueRef,
                                                           default_value)
      }
    }
  }
  impl<'largs> JsonValueRefToBoolArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::json_value_ref::JsonValueRef) -> bool {

      unsafe { ::ffi::qt_core_c_QJsonValueRef_toBool_no_args(original_self as *const ::json_value_ref::JsonValueRef) }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonValueRef::to_double](../struct.JsonValueRef.html#method.to_double) method.
  pub trait JsonValueRefToDoubleArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_value_ref::JsonValueRef) -> ::libc::c_double;
  }
  impl<'largs> JsonValueRefToDoubleArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs ::json_value_ref::JsonValueRef) -> ::libc::c_double {
      let default_value = self;
      unsafe {
        ::ffi::qt_core_c_QJsonValueRef_toDouble_defaultValue(original_self as *const ::json_value_ref::JsonValueRef,
                                                             default_value)
      }
    }
  }
  impl<'largs> JsonValueRefToDoubleArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::json_value_ref::JsonValueRef) -> ::libc::c_double {

      unsafe { ::ffi::qt_core_c_QJsonValueRef_toDouble_no_args(original_self as *const ::json_value_ref::JsonValueRef) }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonValueRef::to_int](../struct.JsonValueRef.html#method.to_int) method.
  pub trait JsonValueRefToIntArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_value_ref::JsonValueRef) -> ::libc::c_int;
  }
  impl<'largs> JsonValueRefToIntArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::json_value_ref::JsonValueRef) -> ::libc::c_int {
      let default_value = self;
      unsafe {
        ::ffi::qt_core_c_QJsonValueRef_toInt_defaultValue(original_self as *const ::json_value_ref::JsonValueRef,
                                                          default_value)
      }
    }
  }
  impl<'largs> JsonValueRefToIntArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::json_value_ref::JsonValueRef) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QJsonValueRef_toInt_no_args(original_self as *const ::json_value_ref::JsonValueRef) }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonValueRef::to_string](../struct.JsonValueRef.html#method.to_string) method.
  pub trait JsonValueRefToStringArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_value_ref::JsonValueRef) -> ::string::String;
  }
  impl<'largs> JsonValueRefToStringArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::json_value_ref::JsonValueRef) -> ::string::String {
      let default_value = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValueRef_toString_to_output_defaultValue(original_self as *const ::json_value_ref::JsonValueRef, default_value as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> JsonValueRefToStringArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::json_value_ref::JsonValueRef) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonValueRef_toString_to_output_no_args(original_self as *const ::json_value_ref::JsonValueRef, &mut object);
        }
        object
      }
    }
  }
}
