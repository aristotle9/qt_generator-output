#include "qt_core_c_QHash.h"

int qt_core_c_QHash_QString_QVariant_capacity(const QHash< QString, QVariant >* this_ptr) {
  return this_ptr->capacity();
}

void qt_core_c_QHash_QString_QVariant_clear(QHash< QString, QVariant >* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QHash_QString_QVariant_constructor(QHash< QString, QVariant >* output) {
  new(output) QHash< QString, QVariant >();
}

bool qt_core_c_QHash_QString_QVariant_contains(const QHash< QString, QVariant >* this_ptr, const QString* key) {
  return this_ptr->contains(*key);
}

int qt_core_c_QHash_QString_QVariant_count_key(const QHash< QString, QVariant >* this_ptr, const QString* key) {
  return this_ptr->count(*key);
}

int qt_core_c_QHash_QString_QVariant_count_no_args(const QHash< QString, QVariant >* this_ptr) {
  return this_ptr->count();
}

void qt_core_c_QHash_QString_QVariant_destructor(QHash< QString, QVariant >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QHash_QString_QVariant_empty(const QHash< QString, QVariant >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QHash_QString_QVariant_isEmpty(const QHash< QString, QVariant >* this_ptr) {
  return this_ptr->isEmpty();
}

void qt_core_c_QHash_QString_QVariant_key_to_output_value(const QHash< QString, QVariant >* this_ptr, const QVariant* value, QString* output) {
  new(output) QString(this_ptr->key(*value));
}

void qt_core_c_QHash_QString_QVariant_key_to_output_value_defaultKey(const QHash< QString, QVariant >* this_ptr, const QVariant* value, const QString* defaultKey, QString* output) {
  new(output) QString(this_ptr->key(*value, *defaultKey));
}

void qt_core_c_QHash_QString_QVariant_keys_to_output_no_args(const QHash< QString, QVariant >* this_ptr, QList< QString >* output) {
  new(output) QList< QString >(this_ptr->keys());
}

void qt_core_c_QHash_QString_QVariant_keys_to_output_value(const QHash< QString, QVariant >* this_ptr, const QVariant* value, QList< QString >* output) {
  new(output) QList< QString >(this_ptr->keys(*value));
}

QVariant* qt_core_c_QHash_QString_QVariant_operator_index(QHash< QString, QVariant >* this_ptr, const QString* key) {
  return &this_ptr->operator[](*key);
}

void qt_core_c_QHash_QString_QVariant_operator_index_to_output(const QHash< QString, QVariant >* this_ptr, const QString* key, QVariant* output) {
  new(output) QVariant(this_ptr->operator[](*key));
}

int qt_core_c_QHash_QString_QVariant_remove(QHash< QString, QVariant >* this_ptr, const QString* key) {
  return this_ptr->remove(*key);
}

void qt_core_c_QHash_QString_QVariant_reserve(QHash< QString, QVariant >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QHash_QString_QVariant_size(const QHash< QString, QVariant >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QHash_QString_QVariant_squeeze(QHash< QString, QVariant >* this_ptr) {
  this_ptr->squeeze();
}

void qt_core_c_QHash_QString_QVariant_take_to_output(QHash< QString, QVariant >* this_ptr, const QString* key, QVariant* output) {
  new(output) QVariant(this_ptr->take(*key));
}

void qt_core_c_QHash_QString_QVariant_uniqueKeys_to_output(const QHash< QString, QVariant >* this_ptr, QList< QString >* output) {
  new(output) QList< QString >(this_ptr->uniqueKeys());
}

void qt_core_c_QHash_QString_QVariant_value_to_output_key(const QHash< QString, QVariant >* this_ptr, const QString* key, QVariant* output) {
  new(output) QVariant(this_ptr->value(*key));
}

void qt_core_c_QHash_QString_QVariant_value_to_output_key_defaultValue(const QHash< QString, QVariant >* this_ptr, const QString* key, const QVariant* defaultValue, QVariant* output) {
  new(output) QVariant(this_ptr->value(*key, *defaultValue));
}

void qt_core_c_QHash_QString_QVariant_values_to_output_key(const QHash< QString, QVariant >* this_ptr, const QString* key, QList< QVariant >* output) {
  new(output) QList< QVariant >(this_ptr->values(*key));
}

void qt_core_c_QHash_QString_QVariant_values_to_output_no_args(const QHash< QString, QVariant >* this_ptr, QList< QVariant >* output) {
  new(output) QList< QVariant >(this_ptr->values());
}

int qt_core_c_QHash_int_QByteArray_capacity(const QHash< int, QByteArray >* this_ptr) {
  return this_ptr->capacity();
}

void qt_core_c_QHash_int_QByteArray_clear(QHash< int, QByteArray >* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QHash_int_QByteArray_constructor(QHash< int, QByteArray >* output) {
  new(output) QHash< int, QByteArray >();
}

bool qt_core_c_QHash_int_QByteArray_contains(const QHash< int, QByteArray >* this_ptr, const int* key) {
  return this_ptr->contains(*key);
}

int qt_core_c_QHash_int_QByteArray_count_key(const QHash< int, QByteArray >* this_ptr, const int* key) {
  return this_ptr->count(*key);
}

int qt_core_c_QHash_int_QByteArray_count_no_args(const QHash< int, QByteArray >* this_ptr) {
  return this_ptr->count();
}

void qt_core_c_QHash_int_QByteArray_destructor(QHash< int, QByteArray >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QHash_int_QByteArray_empty(const QHash< int, QByteArray >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QHash_int_QByteArray_isEmpty(const QHash< int, QByteArray >* this_ptr) {
  return this_ptr->isEmpty();
}

const int qt_core_c_QHash_int_QByteArray_key_value(const QHash< int, QByteArray >* this_ptr, const QByteArray* value) {
  return this_ptr->key(*value);
}

const int qt_core_c_QHash_int_QByteArray_key_value_defaultKey(const QHash< int, QByteArray >* this_ptr, const QByteArray* value, const int* defaultKey) {
  return this_ptr->key(*value, *defaultKey);
}

void qt_core_c_QHash_int_QByteArray_keys_to_output_no_args(const QHash< int, QByteArray >* this_ptr, QList< int >* output) {
  new(output) QList< int >(this_ptr->keys());
}

void qt_core_c_QHash_int_QByteArray_keys_to_output_value(const QHash< int, QByteArray >* this_ptr, const QByteArray* value, QList< int >* output) {
  new(output) QList< int >(this_ptr->keys(*value));
}

QByteArray* qt_core_c_QHash_int_QByteArray_operator_index(QHash< int, QByteArray >* this_ptr, const int* key) {
  return &this_ptr->operator[](*key);
}

void qt_core_c_QHash_int_QByteArray_operator_index_to_output(const QHash< int, QByteArray >* this_ptr, const int* key, QByteArray* output) {
  new(output) QByteArray(this_ptr->operator[](*key));
}

int qt_core_c_QHash_int_QByteArray_remove(QHash< int, QByteArray >* this_ptr, const int* key) {
  return this_ptr->remove(*key);
}

void qt_core_c_QHash_int_QByteArray_reserve(QHash< int, QByteArray >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QHash_int_QByteArray_size(const QHash< int, QByteArray >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QHash_int_QByteArray_squeeze(QHash< int, QByteArray >* this_ptr) {
  this_ptr->squeeze();
}

void qt_core_c_QHash_int_QByteArray_take_to_output(QHash< int, QByteArray >* this_ptr, const int* key, QByteArray* output) {
  new(output) QByteArray(this_ptr->take(*key));
}

void qt_core_c_QHash_int_QByteArray_uniqueKeys_to_output(const QHash< int, QByteArray >* this_ptr, QList< int >* output) {
  new(output) QList< int >(this_ptr->uniqueKeys());
}

void qt_core_c_QHash_int_QByteArray_value_to_output_key(const QHash< int, QByteArray >* this_ptr, const int* key, QByteArray* output) {
  new(output) QByteArray(this_ptr->value(*key));
}

void qt_core_c_QHash_int_QByteArray_value_to_output_key_defaultValue(const QHash< int, QByteArray >* this_ptr, const int* key, const QByteArray* defaultValue, QByteArray* output) {
  new(output) QByteArray(this_ptr->value(*key, *defaultValue));
}

void qt_core_c_QHash_int_QByteArray_values_to_output_key(const QHash< int, QByteArray >* this_ptr, const int* key, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(this_ptr->values(*key));
}

void qt_core_c_QHash_int_QByteArray_values_to_output_no_args(const QHash< int, QByteArray >* this_ptr, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(this_ptr->values());
}

