#include "qt_widgets_c_QMap.h"

void qt_widgets_c_QMap_QDate_QTextCharFormat_clear(QMap< QDate, QTextCharFormat >* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QMap_QDate_QTextCharFormat_constructor_no_args(QMap< QDate, QTextCharFormat >* output) {
  new(output) QMap< QDate, QTextCharFormat >();
}

void qt_widgets_c_QMap_QDate_QTextCharFormat_constructor_other(const QMap< QDate, QTextCharFormat >* other, QMap< QDate, QTextCharFormat >* output) {
  new(output) QMap< QDate, QTextCharFormat >(*other);
}

bool qt_widgets_c_QMap_QDate_QTextCharFormat_contains(const QMap< QDate, QTextCharFormat >* this_ptr, const QDate* key) {
  return this_ptr->contains(*key);
}

int qt_widgets_c_QMap_QDate_QTextCharFormat_count_key(const QMap< QDate, QTextCharFormat >* this_ptr, const QDate* key) {
  return this_ptr->count(*key);
}

int qt_widgets_c_QMap_QDate_QTextCharFormat_count_no_args(const QMap< QDate, QTextCharFormat >* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QMap_QDate_QTextCharFormat_destructor(QMap< QDate, QTextCharFormat >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QMap_QDate_QTextCharFormat_empty(const QMap< QDate, QTextCharFormat >* this_ptr) {
  return this_ptr->empty();
}

QTextCharFormat* qt_widgets_c_QMap_QDate_QTextCharFormat_first(QMap< QDate, QTextCharFormat >* this_ptr) {
  return &this_ptr->first();
}

const QDate* qt_widgets_c_QMap_QDate_QTextCharFormat_firstKey(const QMap< QDate, QTextCharFormat >* this_ptr) {
  return &this_ptr->firstKey();
}

const QTextCharFormat* qt_widgets_c_QMap_QDate_QTextCharFormat_first_const(const QMap< QDate, QTextCharFormat >* this_ptr) {
  return &this_ptr->first();
}

bool qt_widgets_c_QMap_QDate_QTextCharFormat_isEmpty(const QMap< QDate, QTextCharFormat >* this_ptr) {
  return this_ptr->isEmpty();
}

void qt_widgets_c_QMap_QDate_QTextCharFormat_key_to_output_value(const QMap< QDate, QTextCharFormat >* this_ptr, const QTextCharFormat* value, QDate* output) {
  new(output) QDate(this_ptr->key(*value));
}

void qt_widgets_c_QMap_QDate_QTextCharFormat_key_to_output_value_defaultKey(const QMap< QDate, QTextCharFormat >* this_ptr, const QTextCharFormat* value, const QDate* defaultKey, QDate* output) {
  new(output) QDate(this_ptr->key(*value, *defaultKey));
}

QTextCharFormat* qt_widgets_c_QMap_QDate_QTextCharFormat_last(QMap< QDate, QTextCharFormat >* this_ptr) {
  return &this_ptr->last();
}

const QDate* qt_widgets_c_QMap_QDate_QTextCharFormat_lastKey(const QMap< QDate, QTextCharFormat >* this_ptr) {
  return &this_ptr->lastKey();
}

const QTextCharFormat* qt_widgets_c_QMap_QDate_QTextCharFormat_last_const(const QMap< QDate, QTextCharFormat >* this_ptr) {
  return &this_ptr->last();
}

QMap< QDate, QTextCharFormat >* qt_widgets_c_QMap_QDate_QTextCharFormat_operator_assign(QMap< QDate, QTextCharFormat >* this_ptr, const QMap< QDate, QTextCharFormat >* other) {
  return &this_ptr->operator=(*other);
}

bool qt_widgets_c_QMap_QDate_QTextCharFormat_operator_eq(const QMap< QDate, QTextCharFormat >* this_ptr, const QMap< QDate, QTextCharFormat >* other) {
  return this_ptr->operator==(*other);
}

QTextCharFormat* qt_widgets_c_QMap_QDate_QTextCharFormat_operator_index(QMap< QDate, QTextCharFormat >* this_ptr, const QDate* key) {
  return &this_ptr->operator[](*key);
}

void qt_widgets_c_QMap_QDate_QTextCharFormat_operator_index_to_output(const QMap< QDate, QTextCharFormat >* this_ptr, const QDate* key, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->operator[](*key));
}

bool qt_widgets_c_QMap_QDate_QTextCharFormat_operator_neq(const QMap< QDate, QTextCharFormat >* this_ptr, const QMap< QDate, QTextCharFormat >* other) {
  return this_ptr->operator!=(*other);
}

int qt_widgets_c_QMap_QDate_QTextCharFormat_remove(QMap< QDate, QTextCharFormat >* this_ptr, const QDate* key) {
  return this_ptr->remove(*key);
}

int qt_widgets_c_QMap_QDate_QTextCharFormat_size(const QMap< QDate, QTextCharFormat >* this_ptr) {
  return this_ptr->size();
}

void qt_widgets_c_QMap_QDate_QTextCharFormat_swap(QMap< QDate, QTextCharFormat >* this_ptr, QMap< QDate, QTextCharFormat >* other) {
  this_ptr->swap(*other);
}

void qt_widgets_c_QMap_QDate_QTextCharFormat_take_to_output(QMap< QDate, QTextCharFormat >* this_ptr, const QDate* key, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->take(*key));
}

QMap< QDate, QTextCharFormat >* qt_widgets_c_QMap_QDate_QTextCharFormat_unite(QMap< QDate, QTextCharFormat >* this_ptr, const QMap< QDate, QTextCharFormat >* other) {
  return &this_ptr->unite(*other);
}

void qt_widgets_c_QMap_QDate_QTextCharFormat_value_to_output_key(const QMap< QDate, QTextCharFormat >* this_ptr, const QDate* key, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->value(*key));
}

void qt_widgets_c_QMap_QDate_QTextCharFormat_value_to_output_key_defaultValue(const QMap< QDate, QTextCharFormat >* this_ptr, const QDate* key, const QTextCharFormat* defaultValue, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->value(*key, *defaultValue));
}

void qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_clear(QMap< Qt::GestureType, QWidget* >* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_constructor_no_args(QMap< Qt::GestureType, QWidget* >* output) {
  new(output) QMap< Qt::GestureType, QWidget* >();
}

void qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_constructor_other(const QMap< Qt::GestureType, QWidget* >* other, QMap< Qt::GestureType, QWidget* >* output) {
  new(output) QMap< Qt::GestureType, QWidget* >(*other);
}

bool qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_contains(const QMap< Qt::GestureType, QWidget* >* this_ptr, const Qt::GestureType* key) {
  return this_ptr->contains(*key);
}

int qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_count_key(const QMap< Qt::GestureType, QWidget* >* this_ptr, const Qt::GestureType* key) {
  return this_ptr->count(*key);
}

int qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_count_no_args(const QMap< Qt::GestureType, QWidget* >* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_destructor(QMap< Qt::GestureType, QWidget* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_empty(const QMap< Qt::GestureType, QWidget* >* this_ptr) {
  return this_ptr->empty();
}

QWidget** qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_first(QMap< Qt::GestureType, QWidget* >* this_ptr) {
  return &this_ptr->first();
}

const Qt::GestureType* qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_firstKey(const QMap< Qt::GestureType, QWidget* >* this_ptr) {
  return &this_ptr->firstKey();
}

QWidget* const * qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_first_const(const QMap< Qt::GestureType, QWidget* >* this_ptr) {
  return &this_ptr->first();
}

bool qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_isEmpty(const QMap< Qt::GestureType, QWidget* >* this_ptr) {
  return this_ptr->isEmpty();
}

const Qt::GestureType qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_key_value(const QMap< Qt::GestureType, QWidget* >* this_ptr, QWidget* const * value) {
  return this_ptr->key(*value);
}

const Qt::GestureType qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_key_value_defaultKey(const QMap< Qt::GestureType, QWidget* >* this_ptr, QWidget* const * value, const Qt::GestureType* defaultKey) {
  return this_ptr->key(*value, *defaultKey);
}

QWidget** qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_last(QMap< Qt::GestureType, QWidget* >* this_ptr) {
  return &this_ptr->last();
}

const Qt::GestureType* qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_lastKey(const QMap< Qt::GestureType, QWidget* >* this_ptr) {
  return &this_ptr->lastKey();
}

QWidget* const * qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_last_const(const QMap< Qt::GestureType, QWidget* >* this_ptr) {
  return &this_ptr->last();
}

QMap< Qt::GestureType, QWidget* >* qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_operator_assign(QMap< Qt::GestureType, QWidget* >* this_ptr, const QMap< Qt::GestureType, QWidget* >* other) {
  return &this_ptr->operator=(*other);
}

bool qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_operator_eq(const QMap< Qt::GestureType, QWidget* >* this_ptr, const QMap< Qt::GestureType, QWidget* >* other) {
  return this_ptr->operator==(*other);
}

QWidget** qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_operator_index(QMap< Qt::GestureType, QWidget* >* this_ptr, const Qt::GestureType* key) {
  return &this_ptr->operator[](*key);
}

QWidget* qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_operator_index_const(const QMap< Qt::GestureType, QWidget* >* this_ptr, const Qt::GestureType* key) {
  return this_ptr->operator[](*key);
}

bool qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_operator_neq(const QMap< Qt::GestureType, QWidget* >* this_ptr, const QMap< Qt::GestureType, QWidget* >* other) {
  return this_ptr->operator!=(*other);
}

int qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_remove(QMap< Qt::GestureType, QWidget* >* this_ptr, const Qt::GestureType* key) {
  return this_ptr->remove(*key);
}

int qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_size(const QMap< Qt::GestureType, QWidget* >* this_ptr) {
  return this_ptr->size();
}

void qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_swap(QMap< Qt::GestureType, QWidget* >* this_ptr, QMap< Qt::GestureType, QWidget* >* other) {
  this_ptr->swap(*other);
}

QWidget* qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_take(QMap< Qt::GestureType, QWidget* >* this_ptr, const Qt::GestureType* key) {
  return this_ptr->take(*key);
}

QMap< Qt::GestureType, QWidget* >* qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_unite(QMap< Qt::GestureType, QWidget* >* this_ptr, const QMap< Qt::GestureType, QWidget* >* other) {
  return &this_ptr->unite(*other);
}

QWidget* qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_value_key(const QMap< Qt::GestureType, QWidget* >* this_ptr, const Qt::GestureType* key) {
  return this_ptr->value(*key);
}

QWidget* qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_value_key_defaultValue(const QMap< Qt::GestureType, QWidget* >* this_ptr, const Qt::GestureType* key, QWidget* const * defaultValue) {
  return this_ptr->value(*key, *defaultValue);
}

void qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_values_to_output_key(const QMap< Qt::GestureType, QWidget* >* this_ptr, const Qt::GestureType* key, QList< QWidget* >* output) {
  new(output) QList< QWidget* >(this_ptr->values(*key));
}

void qt_widgets_c_QMap_Qt_GestureType_QWidget_ptr_values_to_output_no_args(const QMap< Qt::GestureType, QWidget* >* this_ptr, QList< QWidget* >* output) {
  new(output) QList< QWidget* >(this_ptr->values());
}

void qt_widgets_c_QMap_Qt_GestureType_bool_clear(QMap< Qt::GestureType, bool >* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QMap_Qt_GestureType_bool_constructor_no_args(QMap< Qt::GestureType, bool >* output) {
  new(output) QMap< Qt::GestureType, bool >();
}

void qt_widgets_c_QMap_Qt_GestureType_bool_constructor_other(const QMap< Qt::GestureType, bool >* other, QMap< Qt::GestureType, bool >* output) {
  new(output) QMap< Qt::GestureType, bool >(*other);
}

bool qt_widgets_c_QMap_Qt_GestureType_bool_contains(const QMap< Qt::GestureType, bool >* this_ptr, const Qt::GestureType* key) {
  return this_ptr->contains(*key);
}

int qt_widgets_c_QMap_Qt_GestureType_bool_count_key(const QMap< Qt::GestureType, bool >* this_ptr, const Qt::GestureType* key) {
  return this_ptr->count(*key);
}

int qt_widgets_c_QMap_Qt_GestureType_bool_count_no_args(const QMap< Qt::GestureType, bool >* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QMap_Qt_GestureType_bool_destructor(QMap< Qt::GestureType, bool >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QMap_Qt_GestureType_bool_empty(const QMap< Qt::GestureType, bool >* this_ptr) {
  return this_ptr->empty();
}

bool* qt_widgets_c_QMap_Qt_GestureType_bool_first(QMap< Qt::GestureType, bool >* this_ptr) {
  return &this_ptr->first();
}

const Qt::GestureType* qt_widgets_c_QMap_Qt_GestureType_bool_firstKey(const QMap< Qt::GestureType, bool >* this_ptr) {
  return &this_ptr->firstKey();
}

const bool* qt_widgets_c_QMap_Qt_GestureType_bool_first_const(const QMap< Qt::GestureType, bool >* this_ptr) {
  return &this_ptr->first();
}

bool qt_widgets_c_QMap_Qt_GestureType_bool_isEmpty(const QMap< Qt::GestureType, bool >* this_ptr) {
  return this_ptr->isEmpty();
}

const Qt::GestureType qt_widgets_c_QMap_Qt_GestureType_bool_key_value(const QMap< Qt::GestureType, bool >* this_ptr, const bool* value) {
  return this_ptr->key(*value);
}

const Qt::GestureType qt_widgets_c_QMap_Qt_GestureType_bool_key_value_defaultKey(const QMap< Qt::GestureType, bool >* this_ptr, const bool* value, const Qt::GestureType* defaultKey) {
  return this_ptr->key(*value, *defaultKey);
}

bool* qt_widgets_c_QMap_Qt_GestureType_bool_last(QMap< Qt::GestureType, bool >* this_ptr) {
  return &this_ptr->last();
}

const Qt::GestureType* qt_widgets_c_QMap_Qt_GestureType_bool_lastKey(const QMap< Qt::GestureType, bool >* this_ptr) {
  return &this_ptr->lastKey();
}

const bool* qt_widgets_c_QMap_Qt_GestureType_bool_last_const(const QMap< Qt::GestureType, bool >* this_ptr) {
  return &this_ptr->last();
}

QMap< Qt::GestureType, bool >* qt_widgets_c_QMap_Qt_GestureType_bool_operator_assign(QMap< Qt::GestureType, bool >* this_ptr, const QMap< Qt::GestureType, bool >* other) {
  return &this_ptr->operator=(*other);
}

bool qt_widgets_c_QMap_Qt_GestureType_bool_operator_eq(const QMap< Qt::GestureType, bool >* this_ptr, const QMap< Qt::GestureType, bool >* other) {
  return this_ptr->operator==(*other);
}

bool* qt_widgets_c_QMap_Qt_GestureType_bool_operator_index(QMap< Qt::GestureType, bool >* this_ptr, const Qt::GestureType* key) {
  return &this_ptr->operator[](*key);
}

const bool qt_widgets_c_QMap_Qt_GestureType_bool_operator_index_const(const QMap< Qt::GestureType, bool >* this_ptr, const Qt::GestureType* key) {
  return this_ptr->operator[](*key);
}

bool qt_widgets_c_QMap_Qt_GestureType_bool_operator_neq(const QMap< Qt::GestureType, bool >* this_ptr, const QMap< Qt::GestureType, bool >* other) {
  return this_ptr->operator!=(*other);
}

int qt_widgets_c_QMap_Qt_GestureType_bool_remove(QMap< Qt::GestureType, bool >* this_ptr, const Qt::GestureType* key) {
  return this_ptr->remove(*key);
}

int qt_widgets_c_QMap_Qt_GestureType_bool_size(const QMap< Qt::GestureType, bool >* this_ptr) {
  return this_ptr->size();
}

void qt_widgets_c_QMap_Qt_GestureType_bool_swap(QMap< Qt::GestureType, bool >* this_ptr, QMap< Qt::GestureType, bool >* other) {
  this_ptr->swap(*other);
}

bool qt_widgets_c_QMap_Qt_GestureType_bool_take(QMap< Qt::GestureType, bool >* this_ptr, const Qt::GestureType* key) {
  return this_ptr->take(*key);
}

QMap< Qt::GestureType, bool >* qt_widgets_c_QMap_Qt_GestureType_bool_unite(QMap< Qt::GestureType, bool >* this_ptr, const QMap< Qt::GestureType, bool >* other) {
  return &this_ptr->unite(*other);
}

const bool qt_widgets_c_QMap_Qt_GestureType_bool_value_key(const QMap< Qt::GestureType, bool >* this_ptr, const Qt::GestureType* key) {
  return this_ptr->value(*key);
}

const bool qt_widgets_c_QMap_Qt_GestureType_bool_value_key_defaultValue(const QMap< Qt::GestureType, bool >* this_ptr, const Qt::GestureType* key, const bool* defaultValue) {
  return this_ptr->value(*key, *defaultValue);
}

