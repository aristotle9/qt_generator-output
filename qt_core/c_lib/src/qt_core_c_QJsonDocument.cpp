#include "qt_core_c_QJsonDocument.h"

void qt_core_c_QJsonDocument_array_to_output(const QJsonDocument* this_ptr, QJsonArray* output) {
  new(output) QJsonArray(this_ptr->array());
}

void qt_core_c_QJsonDocument_constructor_array(const QJsonArray* array, QJsonDocument* output) {
  new(output) QJsonDocument(*array);
}

void qt_core_c_QJsonDocument_constructor_no_args(QJsonDocument* output) {
  new(output) QJsonDocument();
}

void qt_core_c_QJsonDocument_constructor_object(const QJsonObject* object, QJsonDocument* output) {
  new(output) QJsonDocument(*object);
}

void qt_core_c_QJsonDocument_constructor_other(const QJsonDocument* other, QJsonDocument* output) {
  new(output) QJsonDocument(*other);
}

void qt_core_c_QJsonDocument_destructor(QJsonDocument* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QJsonDocument_fromBinaryData_to_output_data(const QByteArray* data, QJsonDocument* output) {
  new(output) QJsonDocument(QJsonDocument::fromBinaryData(*data));
}

void qt_core_c_QJsonDocument_fromBinaryData_to_output_data_validation(const QByteArray* data, QJsonDocument::DataValidation validation, QJsonDocument* output) {
  new(output) QJsonDocument(QJsonDocument::fromBinaryData(*data, validation));
}

void qt_core_c_QJsonDocument_fromJson_to_output_json(const QByteArray* json, QJsonDocument* output) {
  new(output) QJsonDocument(QJsonDocument::fromJson(*json));
}

void qt_core_c_QJsonDocument_fromJson_to_output_json_error(const QByteArray* json, QJsonParseError* error, QJsonDocument* output) {
  new(output) QJsonDocument(QJsonDocument::fromJson(*json, error));
}

void qt_core_c_QJsonDocument_fromRawData_to_output_data_size(const char* data, int size, QJsonDocument* output) {
  new(output) QJsonDocument(QJsonDocument::fromRawData(data, size));
}

void qt_core_c_QJsonDocument_fromRawData_to_output_data_size_validation(const char* data, int size, QJsonDocument::DataValidation validation, QJsonDocument* output) {
  new(output) QJsonDocument(QJsonDocument::fromRawData(data, size, validation));
}

void qt_core_c_QJsonDocument_fromVariant_to_output(const QVariant* variant, QJsonDocument* output) {
  new(output) QJsonDocument(QJsonDocument::fromVariant(*variant));
}

bool qt_core_c_QJsonDocument_isArray(const QJsonDocument* this_ptr) {
  return this_ptr->isArray();
}

bool qt_core_c_QJsonDocument_isEmpty(const QJsonDocument* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_core_c_QJsonDocument_isNull(const QJsonDocument* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QJsonDocument_isObject(const QJsonDocument* this_ptr) {
  return this_ptr->isObject();
}

void qt_core_c_QJsonDocument_object_to_output(const QJsonDocument* this_ptr, QJsonObject* output) {
  new(output) QJsonObject(this_ptr->object());
}

QJsonDocument* qt_core_c_QJsonDocument_operator_assign(QJsonDocument* this_ptr, const QJsonDocument* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QJsonDocument_operator_eq(const QJsonDocument* this_ptr, const QJsonDocument* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QJsonDocument_operator_neq(const QJsonDocument* this_ptr, const QJsonDocument* other) {
  return this_ptr->operator!=(*other);
}

const char* qt_core_c_QJsonDocument_rawData(const QJsonDocument* this_ptr, int* size) {
  return this_ptr->rawData(size);
}

void qt_core_c_QJsonDocument_setArray(QJsonDocument* this_ptr, const QJsonArray* array) {
  this_ptr->setArray(*array);
}

void qt_core_c_QJsonDocument_setObject(QJsonDocument* this_ptr, const QJsonObject* object) {
  this_ptr->setObject(*object);
}

void qt_core_c_QJsonDocument_toBinaryData_to_output(const QJsonDocument* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toBinaryData());
}

void qt_core_c_QJsonDocument_toJson_to_output_format(const QJsonDocument* this_ptr, QJsonDocument::JsonFormat format, QByteArray* output) {
  new(output) QByteArray(this_ptr->toJson(format));
}

void qt_core_c_QJsonDocument_toJson_to_output_no_args(const QJsonDocument* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toJson());
}

void qt_core_c_QJsonDocument_toVariant_to_output(const QJsonDocument* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->toVariant());
}

