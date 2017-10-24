#include "qt_widgets_c_QHash.h"

int qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_capacity(const QHash< int, QItemEditorCreatorBase* >* this_ptr) {
  return this_ptr->capacity();
}

void qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_clear(QHash< int, QItemEditorCreatorBase* >* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_constructor(QHash< int, QItemEditorCreatorBase* >* output) {
  new(output) QHash< int, QItemEditorCreatorBase* >();
}

bool qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_contains(const QHash< int, QItemEditorCreatorBase* >* this_ptr, const int* key) {
  return this_ptr->contains(*key);
}

int qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_count_key(const QHash< int, QItemEditorCreatorBase* >* this_ptr, const int* key) {
  return this_ptr->count(*key);
}

int qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_count_no_args(const QHash< int, QItemEditorCreatorBase* >* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_destructor(QHash< int, QItemEditorCreatorBase* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_empty(const QHash< int, QItemEditorCreatorBase* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_isEmpty(const QHash< int, QItemEditorCreatorBase* >* this_ptr) {
  return this_ptr->isEmpty();
}

const int qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_key_value(const QHash< int, QItemEditorCreatorBase* >* this_ptr, QItemEditorCreatorBase* const * value) {
  return this_ptr->key(*value);
}

const int qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_key_value_defaultKey(const QHash< int, QItemEditorCreatorBase* >* this_ptr, QItemEditorCreatorBase* const * value, const int* defaultKey) {
  return this_ptr->key(*value, *defaultKey);
}

void qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_keys_to_output_no_args(const QHash< int, QItemEditorCreatorBase* >* this_ptr, QList< int >* output) {
  new(output) QList< int >(this_ptr->keys());
}

void qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_keys_to_output_value(const QHash< int, QItemEditorCreatorBase* >* this_ptr, QItemEditorCreatorBase* const * value, QList< int >* output) {
  new(output) QList< int >(this_ptr->keys(*value));
}

QItemEditorCreatorBase** qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_operator_index(QHash< int, QItemEditorCreatorBase* >* this_ptr, const int* key) {
  return &this_ptr->operator[](*key);
}

QItemEditorCreatorBase* qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_operator_index_const(const QHash< int, QItemEditorCreatorBase* >* this_ptr, const int* key) {
  return this_ptr->operator[](*key);
}

int qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_remove(QHash< int, QItemEditorCreatorBase* >* this_ptr, const int* key) {
  return this_ptr->remove(*key);
}

void qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_reserve(QHash< int, QItemEditorCreatorBase* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_size(const QHash< int, QItemEditorCreatorBase* >* this_ptr) {
  return this_ptr->size();
}

void qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_squeeze(QHash< int, QItemEditorCreatorBase* >* this_ptr) {
  this_ptr->squeeze();
}

QItemEditorCreatorBase* qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_take(QHash< int, QItemEditorCreatorBase* >* this_ptr, const int* key) {
  return this_ptr->take(*key);
}

void qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_uniqueKeys_to_output(const QHash< int, QItemEditorCreatorBase* >* this_ptr, QList< int >* output) {
  new(output) QList< int >(this_ptr->uniqueKeys());
}

QItemEditorCreatorBase* qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_value_key(const QHash< int, QItemEditorCreatorBase* >* this_ptr, const int* key) {
  return this_ptr->value(*key);
}

QItemEditorCreatorBase* qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_value_key_defaultValue(const QHash< int, QItemEditorCreatorBase* >* this_ptr, const int* key, QItemEditorCreatorBase* const * defaultValue) {
  return this_ptr->value(*key, *defaultValue);
}

