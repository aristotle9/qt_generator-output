#include "qt_core_c_QJsonValue.h"

void qt_core_c_QJsonValue_constructor_QJsonArray(const QJsonArray* a, QJsonValue* output) {
  new(output) QJsonValue(*a);
}

void qt_core_c_QJsonValue_constructor_QJsonObject(const QJsonObject* o, QJsonValue* output) {
  new(output) QJsonValue(*o);
}

void qt_core_c_QJsonValue_constructor_QJsonValue(const QJsonValue* other, QJsonValue* output) {
  new(output) QJsonValue(*other);
}

void qt_core_c_QJsonValue_constructor_QJsonValue_Type(QJsonValue::Type arg1, QJsonValue* output) {
  new(output) QJsonValue(arg1);
}

void qt_core_c_QJsonValue_constructor_QLatin1String(const QLatin1String* s, QJsonValue* output) {
  new(output) QJsonValue(*s);
}

void qt_core_c_QJsonValue_constructor_QString(const QString* s, QJsonValue* output) {
  new(output) QJsonValue(*s);
}

void qt_core_c_QJsonValue_constructor_bool(bool b, QJsonValue* output) {
  new(output) QJsonValue(b);
}

void qt_core_c_QJsonValue_constructor_char(const char* s, QJsonValue* output) {
  new(output) QJsonValue(s);
}

void qt_core_c_QJsonValue_constructor_double(double n, QJsonValue* output) {
  new(output) QJsonValue(n);
}

void qt_core_c_QJsonValue_constructor_int(int n, QJsonValue* output) {
  new(output) QJsonValue(n);
}

void qt_core_c_QJsonValue_constructor_no_args(QJsonValue* output) {
  new(output) QJsonValue();
}

void qt_core_c_QJsonValue_constructor_qint64(qint64 n, QJsonValue* output) {
  new(output) QJsonValue(n);
}

void qt_core_c_QJsonValue_destructor(QJsonValue* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QJsonValue_fromVariant_to_output(const QVariant* variant, QJsonValue* output) {
  new(output) QJsonValue(QJsonValue::fromVariant(*variant));
}

bool qt_core_c_QJsonValue_isArray(const QJsonValue* this_ptr) {
  return this_ptr->isArray();
}

bool qt_core_c_QJsonValue_isBool(const QJsonValue* this_ptr) {
  return this_ptr->isBool();
}

bool qt_core_c_QJsonValue_isDouble(const QJsonValue* this_ptr) {
  return this_ptr->isDouble();
}

bool qt_core_c_QJsonValue_isNull(const QJsonValue* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QJsonValue_isObject(const QJsonValue* this_ptr) {
  return this_ptr->isObject();
}

bool qt_core_c_QJsonValue_isString(const QJsonValue* this_ptr) {
  return this_ptr->isString();
}

bool qt_core_c_QJsonValue_isUndefined(const QJsonValue* this_ptr) {
  return this_ptr->isUndefined();
}

QJsonValue* qt_core_c_QJsonValue_operator_assign(QJsonValue* this_ptr, const QJsonValue* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QJsonValue_operator_eq(const QJsonValue* this_ptr, const QJsonValue* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QJsonValue_operator_neq(const QJsonValue* this_ptr, const QJsonValue* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QJsonValue_toArray_to_output_defaultValue(const QJsonValue* this_ptr, const QJsonArray* defaultValue, QJsonArray* output) {
  new(output) QJsonArray(this_ptr->toArray(*defaultValue));
}

void qt_core_c_QJsonValue_toArray_to_output_no_args(const QJsonValue* this_ptr, QJsonArray* output) {
  new(output) QJsonArray(this_ptr->toArray());
}

bool qt_core_c_QJsonValue_toBool_defaultValue(const QJsonValue* this_ptr, bool defaultValue) {
  return this_ptr->toBool(defaultValue);
}

bool qt_core_c_QJsonValue_toBool_no_args(const QJsonValue* this_ptr) {
  return this_ptr->toBool();
}

double qt_core_c_QJsonValue_toDouble_defaultValue(const QJsonValue* this_ptr, double defaultValue) {
  return this_ptr->toDouble(defaultValue);
}

double qt_core_c_QJsonValue_toDouble_no_args(const QJsonValue* this_ptr) {
  return this_ptr->toDouble();
}

int qt_core_c_QJsonValue_toInt_defaultValue(const QJsonValue* this_ptr, int defaultValue) {
  return this_ptr->toInt(defaultValue);
}

int qt_core_c_QJsonValue_toInt_no_args(const QJsonValue* this_ptr) {
  return this_ptr->toInt();
}

void qt_core_c_QJsonValue_toObject_to_output_defaultValue(const QJsonValue* this_ptr, const QJsonObject* defaultValue, QJsonObject* output) {
  new(output) QJsonObject(this_ptr->toObject(*defaultValue));
}

void qt_core_c_QJsonValue_toObject_to_output_no_args(const QJsonValue* this_ptr, QJsonObject* output) {
  new(output) QJsonObject(this_ptr->toObject());
}

void qt_core_c_QJsonValue_toString_to_output_defaultValue(const QJsonValue* this_ptr, const QString* defaultValue, QString* output) {
  new(output) QString(this_ptr->toString(*defaultValue));
}

void qt_core_c_QJsonValue_toString_to_output_no_args(const QJsonValue* this_ptr, QString* output) {
  new(output) QString(this_ptr->toString());
}

void qt_core_c_QJsonValue_toVariant_to_output(const QJsonValue* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->toVariant());
}

QJsonValue::Type qt_core_c_QJsonValue_type(const QJsonValue* this_ptr) {
  return this_ptr->type();
}

