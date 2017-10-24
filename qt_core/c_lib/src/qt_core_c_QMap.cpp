#include "qt_core_c_QMap.h"

void qt_core_c_QMap_QString_QVariant_clear(QMap< QString, QVariant >* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QMap_QString_QVariant_constructor_no_args(QMap< QString, QVariant >* output) {
  new(output) QMap< QString, QVariant >();
}

void qt_core_c_QMap_QString_QVariant_constructor_other(const QMap< QString, QVariant >* other, QMap< QString, QVariant >* output) {
  new(output) QMap< QString, QVariant >(*other);
}

bool qt_core_c_QMap_QString_QVariant_contains(const QMap< QString, QVariant >* this_ptr, const QString* key) {
  return this_ptr->contains(*key);
}

int qt_core_c_QMap_QString_QVariant_count_key(const QMap< QString, QVariant >* this_ptr, const QString* key) {
  return this_ptr->count(*key);
}

int qt_core_c_QMap_QString_QVariant_count_no_args(const QMap< QString, QVariant >* this_ptr) {
  return this_ptr->count();
}

void qt_core_c_QMap_QString_QVariant_destructor(QMap< QString, QVariant >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QMap_QString_QVariant_empty(const QMap< QString, QVariant >* this_ptr) {
  return this_ptr->empty();
}

QVariant* qt_core_c_QMap_QString_QVariant_first(QMap< QString, QVariant >* this_ptr) {
  return &this_ptr->first();
}

const QString* qt_core_c_QMap_QString_QVariant_firstKey(const QMap< QString, QVariant >* this_ptr) {
  return &this_ptr->firstKey();
}

const QVariant* qt_core_c_QMap_QString_QVariant_first_const(const QMap< QString, QVariant >* this_ptr) {
  return &this_ptr->first();
}

bool qt_core_c_QMap_QString_QVariant_isEmpty(const QMap< QString, QVariant >* this_ptr) {
  return this_ptr->isEmpty();
}

void qt_core_c_QMap_QString_QVariant_key_to_output_value(const QMap< QString, QVariant >* this_ptr, const QVariant* value, QString* output) {
  new(output) QString(this_ptr->key(*value));
}

void qt_core_c_QMap_QString_QVariant_key_to_output_value_defaultKey(const QMap< QString, QVariant >* this_ptr, const QVariant* value, const QString* defaultKey, QString* output) {
  new(output) QString(this_ptr->key(*value, *defaultKey));
}

void qt_core_c_QMap_QString_QVariant_keys_to_output_no_args(const QMap< QString, QVariant >* this_ptr, QList< QString >* output) {
  new(output) QList< QString >(this_ptr->keys());
}

void qt_core_c_QMap_QString_QVariant_keys_to_output_value(const QMap< QString, QVariant >* this_ptr, const QVariant* value, QList< QString >* output) {
  new(output) QList< QString >(this_ptr->keys(*value));
}

QVariant* qt_core_c_QMap_QString_QVariant_last(QMap< QString, QVariant >* this_ptr) {
  return &this_ptr->last();
}

const QString* qt_core_c_QMap_QString_QVariant_lastKey(const QMap< QString, QVariant >* this_ptr) {
  return &this_ptr->lastKey();
}

const QVariant* qt_core_c_QMap_QString_QVariant_last_const(const QMap< QString, QVariant >* this_ptr) {
  return &this_ptr->last();
}

QMap< QString, QVariant >* qt_core_c_QMap_QString_QVariant_operator_assign(QMap< QString, QVariant >* this_ptr, const QMap< QString, QVariant >* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QMap_QString_QVariant_operator_eq(const QMap< QString, QVariant >* this_ptr, const QMap< QString, QVariant >* other) {
  return this_ptr->operator==(*other);
}

QVariant* qt_core_c_QMap_QString_QVariant_operator_index(QMap< QString, QVariant >* this_ptr, const QString* key) {
  return &this_ptr->operator[](*key);
}

void qt_core_c_QMap_QString_QVariant_operator_index_to_output(const QMap< QString, QVariant >* this_ptr, const QString* key, QVariant* output) {
  new(output) QVariant(this_ptr->operator[](*key));
}

bool qt_core_c_QMap_QString_QVariant_operator_neq(const QMap< QString, QVariant >* this_ptr, const QMap< QString, QVariant >* other) {
  return this_ptr->operator!=(*other);
}

int qt_core_c_QMap_QString_QVariant_remove(QMap< QString, QVariant >* this_ptr, const QString* key) {
  return this_ptr->remove(*key);
}

int qt_core_c_QMap_QString_QVariant_size(const QMap< QString, QVariant >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QMap_QString_QVariant_swap(QMap< QString, QVariant >* this_ptr, QMap< QString, QVariant >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QMap_QString_QVariant_take_to_output(QMap< QString, QVariant >* this_ptr, const QString* key, QVariant* output) {
  new(output) QVariant(this_ptr->take(*key));
}

void qt_core_c_QMap_QString_QVariant_uniqueKeys_to_output(const QMap< QString, QVariant >* this_ptr, QList< QString >* output) {
  new(output) QList< QString >(this_ptr->uniqueKeys());
}

QMap< QString, QVariant >* qt_core_c_QMap_QString_QVariant_unite(QMap< QString, QVariant >* this_ptr, const QMap< QString, QVariant >* other) {
  return &this_ptr->unite(*other);
}

void qt_core_c_QMap_QString_QVariant_value_to_output_key(const QMap< QString, QVariant >* this_ptr, const QString* key, QVariant* output) {
  new(output) QVariant(this_ptr->value(*key));
}

void qt_core_c_QMap_QString_QVariant_value_to_output_key_defaultValue(const QMap< QString, QVariant >* this_ptr, const QString* key, const QVariant* defaultValue, QVariant* output) {
  new(output) QVariant(this_ptr->value(*key, *defaultValue));
}

void qt_core_c_QMap_QString_QVariant_values_to_output_key(const QMap< QString, QVariant >* this_ptr, const QString* key, QList< QVariant >* output) {
  new(output) QList< QVariant >(this_ptr->values(*key));
}

void qt_core_c_QMap_QString_QVariant_values_to_output_no_args(const QMap< QString, QVariant >* this_ptr, QList< QVariant >* output) {
  new(output) QList< QVariant >(this_ptr->values());
}

void qt_core_c_QMap_int_QVariant_clear(QMap< int, QVariant >* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QMap_int_QVariant_constructor_no_args(QMap< int, QVariant >* output) {
  new(output) QMap< int, QVariant >();
}

void qt_core_c_QMap_int_QVariant_constructor_other(const QMap< int, QVariant >* other, QMap< int, QVariant >* output) {
  new(output) QMap< int, QVariant >(*other);
}

bool qt_core_c_QMap_int_QVariant_contains(const QMap< int, QVariant >* this_ptr, const int* key) {
  return this_ptr->contains(*key);
}

int qt_core_c_QMap_int_QVariant_count_key(const QMap< int, QVariant >* this_ptr, const int* key) {
  return this_ptr->count(*key);
}

int qt_core_c_QMap_int_QVariant_count_no_args(const QMap< int, QVariant >* this_ptr) {
  return this_ptr->count();
}

void qt_core_c_QMap_int_QVariant_destructor(QMap< int, QVariant >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QMap_int_QVariant_empty(const QMap< int, QVariant >* this_ptr) {
  return this_ptr->empty();
}

QVariant* qt_core_c_QMap_int_QVariant_first(QMap< int, QVariant >* this_ptr) {
  return &this_ptr->first();
}

const int* qt_core_c_QMap_int_QVariant_firstKey(const QMap< int, QVariant >* this_ptr) {
  return &this_ptr->firstKey();
}

const QVariant* qt_core_c_QMap_int_QVariant_first_const(const QMap< int, QVariant >* this_ptr) {
  return &this_ptr->first();
}

bool qt_core_c_QMap_int_QVariant_isEmpty(const QMap< int, QVariant >* this_ptr) {
  return this_ptr->isEmpty();
}

const int qt_core_c_QMap_int_QVariant_key_value(const QMap< int, QVariant >* this_ptr, const QVariant* value) {
  return this_ptr->key(*value);
}

const int qt_core_c_QMap_int_QVariant_key_value_defaultKey(const QMap< int, QVariant >* this_ptr, const QVariant* value, const int* defaultKey) {
  return this_ptr->key(*value, *defaultKey);
}

void qt_core_c_QMap_int_QVariant_keys_to_output_no_args(const QMap< int, QVariant >* this_ptr, QList< int >* output) {
  new(output) QList< int >(this_ptr->keys());
}

void qt_core_c_QMap_int_QVariant_keys_to_output_value(const QMap< int, QVariant >* this_ptr, const QVariant* value, QList< int >* output) {
  new(output) QList< int >(this_ptr->keys(*value));
}

QVariant* qt_core_c_QMap_int_QVariant_last(QMap< int, QVariant >* this_ptr) {
  return &this_ptr->last();
}

const int* qt_core_c_QMap_int_QVariant_lastKey(const QMap< int, QVariant >* this_ptr) {
  return &this_ptr->lastKey();
}

const QVariant* qt_core_c_QMap_int_QVariant_last_const(const QMap< int, QVariant >* this_ptr) {
  return &this_ptr->last();
}

QMap< int, QVariant >* qt_core_c_QMap_int_QVariant_operator_assign(QMap< int, QVariant >* this_ptr, const QMap< int, QVariant >* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QMap_int_QVariant_operator_eq(const QMap< int, QVariant >* this_ptr, const QMap< int, QVariant >* other) {
  return this_ptr->operator==(*other);
}

QVariant* qt_core_c_QMap_int_QVariant_operator_index(QMap< int, QVariant >* this_ptr, const int* key) {
  return &this_ptr->operator[](*key);
}

void qt_core_c_QMap_int_QVariant_operator_index_to_output(const QMap< int, QVariant >* this_ptr, const int* key, QVariant* output) {
  new(output) QVariant(this_ptr->operator[](*key));
}

bool qt_core_c_QMap_int_QVariant_operator_neq(const QMap< int, QVariant >* this_ptr, const QMap< int, QVariant >* other) {
  return this_ptr->operator!=(*other);
}

int qt_core_c_QMap_int_QVariant_remove(QMap< int, QVariant >* this_ptr, const int* key) {
  return this_ptr->remove(*key);
}

int qt_core_c_QMap_int_QVariant_size(const QMap< int, QVariant >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QMap_int_QVariant_swap(QMap< int, QVariant >* this_ptr, QMap< int, QVariant >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QMap_int_QVariant_take_to_output(QMap< int, QVariant >* this_ptr, const int* key, QVariant* output) {
  new(output) QVariant(this_ptr->take(*key));
}

void qt_core_c_QMap_int_QVariant_uniqueKeys_to_output(const QMap< int, QVariant >* this_ptr, QList< int >* output) {
  new(output) QList< int >(this_ptr->uniqueKeys());
}

QMap< int, QVariant >* qt_core_c_QMap_int_QVariant_unite(QMap< int, QVariant >* this_ptr, const QMap< int, QVariant >* other) {
  return &this_ptr->unite(*other);
}

void qt_core_c_QMap_int_QVariant_value_to_output_key(const QMap< int, QVariant >* this_ptr, const int* key, QVariant* output) {
  new(output) QVariant(this_ptr->value(*key));
}

void qt_core_c_QMap_int_QVariant_value_to_output_key_defaultValue(const QMap< int, QVariant >* this_ptr, const int* key, const QVariant* defaultValue, QVariant* output) {
  new(output) QVariant(this_ptr->value(*key, *defaultValue));
}

void qt_core_c_QMap_int_QVariant_values_to_output_key(const QMap< int, QVariant >* this_ptr, const int* key, QList< QVariant >* output) {
  new(output) QList< QVariant >(this_ptr->values(*key));
}

void qt_core_c_QMap_int_QVariant_values_to_output_no_args(const QMap< int, QVariant >* this_ptr, QList< QVariant >* output) {
  new(output) QList< QVariant >(this_ptr->values());
}

