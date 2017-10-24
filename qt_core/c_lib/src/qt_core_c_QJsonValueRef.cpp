#include "qt_core_c_QJsonValueRef.h"

void qt_core_c_QJsonValueRef_constructor_array_idx(QJsonArray* array, int idx, QJsonValueRef* output) {
  new(output) QJsonValueRef(array, idx);
}

void qt_core_c_QJsonValueRef_constructor_object_idx(QJsonObject* object, int idx, QJsonValueRef* output) {
  new(output) QJsonValueRef(object, idx);
}

void qt_core_c_QJsonValueRef_convert_to_QJsonValue_to_output(const QJsonValueRef* this_ptr, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->operator QJsonValue());
}

void qt_core_c_QJsonValueRef_destructor(QJsonValueRef* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QJsonValueRef_isArray(const QJsonValueRef* this_ptr) {
  return this_ptr->isArray();
}

bool qt_core_c_QJsonValueRef_isBool(const QJsonValueRef* this_ptr) {
  return this_ptr->isBool();
}

bool qt_core_c_QJsonValueRef_isDouble(const QJsonValueRef* this_ptr) {
  return this_ptr->isDouble();
}

bool qt_core_c_QJsonValueRef_isNull(const QJsonValueRef* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QJsonValueRef_isObject(const QJsonValueRef* this_ptr) {
  return this_ptr->isObject();
}

bool qt_core_c_QJsonValueRef_isString(const QJsonValueRef* this_ptr) {
  return this_ptr->isString();
}

bool qt_core_c_QJsonValueRef_isUndefined(const QJsonValueRef* this_ptr) {
  return this_ptr->isUndefined();
}

QJsonValueRef* qt_core_c_QJsonValueRef_operator_assign_QJsonValue(QJsonValueRef* this_ptr, const QJsonValue* val) {
  return &this_ptr->operator=(*val);
}

QJsonValueRef* qt_core_c_QJsonValueRef_operator_assign_QJsonValueRef(QJsonValueRef* this_ptr, const QJsonValueRef* val) {
  return &this_ptr->operator=(*val);
}

bool qt_core_c_QJsonValueRef_operator_eq(const QJsonValueRef* this_ptr, const QJsonValue* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QJsonValueRef_operator_neq(const QJsonValueRef* this_ptr, const QJsonValue* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QJsonValueRef_toArray_to_output(const QJsonValueRef* this_ptr, QJsonArray* output) {
  new(output) QJsonArray(this_ptr->toArray());
}

bool qt_core_c_QJsonValueRef_toBool_defaultValue(const QJsonValueRef* this_ptr, bool defaultValue) {
  return this_ptr->toBool(defaultValue);
}

bool qt_core_c_QJsonValueRef_toBool_no_args(const QJsonValueRef* this_ptr) {
  return this_ptr->toBool();
}

double qt_core_c_QJsonValueRef_toDouble_defaultValue(const QJsonValueRef* this_ptr, double defaultValue) {
  return this_ptr->toDouble(defaultValue);
}

double qt_core_c_QJsonValueRef_toDouble_no_args(const QJsonValueRef* this_ptr) {
  return this_ptr->toDouble();
}

int qt_core_c_QJsonValueRef_toInt_defaultValue(const QJsonValueRef* this_ptr, int defaultValue) {
  return this_ptr->toInt(defaultValue);
}

int qt_core_c_QJsonValueRef_toInt_no_args(const QJsonValueRef* this_ptr) {
  return this_ptr->toInt();
}

void qt_core_c_QJsonValueRef_toObject_to_output(const QJsonValueRef* this_ptr, QJsonObject* output) {
  new(output) QJsonObject(this_ptr->toObject());
}

void qt_core_c_QJsonValueRef_toString_to_output_defaultValue(const QJsonValueRef* this_ptr, const QString* defaultValue, QString* output) {
  new(output) QString(this_ptr->toString(*defaultValue));
}

void qt_core_c_QJsonValueRef_toString_to_output_no_args(const QJsonValueRef* this_ptr, QString* output) {
  new(output) QString(this_ptr->toString());
}

void qt_core_c_QJsonValueRef_toVariant_to_output(const QJsonValueRef* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->toVariant());
}

