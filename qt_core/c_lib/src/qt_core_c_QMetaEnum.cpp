#include "qt_core_c_QMetaEnum.h"

void qt_core_c_QMetaEnum_constructor(QMetaEnum* output) {
  new(output) QMetaEnum();
}

void qt_core_c_QMetaEnum_destructor(QMetaEnum* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

const QMetaObject* qt_core_c_QMetaEnum_enclosingMetaObject(const QMetaEnum* this_ptr) {
  return this_ptr->enclosingMetaObject();
}

bool qt_core_c_QMetaEnum_isFlag(const QMetaEnum* this_ptr) {
  return this_ptr->isFlag();
}

bool qt_core_c_QMetaEnum_isScoped(const QMetaEnum* this_ptr) {
  return this_ptr->isScoped();
}

bool qt_core_c_QMetaEnum_isValid(const QMetaEnum* this_ptr) {
  return this_ptr->isValid();
}

const char* qt_core_c_QMetaEnum_key(const QMetaEnum* this_ptr, int index) {
  return this_ptr->key(index);
}

int qt_core_c_QMetaEnum_keyCount(const QMetaEnum* this_ptr) {
  return this_ptr->keyCount();
}

int qt_core_c_QMetaEnum_keyToValue_key(const QMetaEnum* this_ptr, const char* key) {
  return this_ptr->keyToValue(key);
}

int qt_core_c_QMetaEnum_keyToValue_key_ok(const QMetaEnum* this_ptr, const char* key, bool* ok) {
  return this_ptr->keyToValue(key, ok);
}

int qt_core_c_QMetaEnum_keysToValue_keys(const QMetaEnum* this_ptr, const char* keys) {
  return this_ptr->keysToValue(keys);
}

int qt_core_c_QMetaEnum_keysToValue_keys_ok(const QMetaEnum* this_ptr, const char* keys, bool* ok) {
  return this_ptr->keysToValue(keys, ok);
}

const char* qt_core_c_QMetaEnum_name(const QMetaEnum* this_ptr) {
  return this_ptr->name();
}

const char* qt_core_c_QMetaEnum_scope(const QMetaEnum* this_ptr) {
  return this_ptr->scope();
}

int qt_core_c_QMetaEnum_value(const QMetaEnum* this_ptr, int index) {
  return this_ptr->value(index);
}

const char* qt_core_c_QMetaEnum_valueToKey(const QMetaEnum* this_ptr, int value) {
  return this_ptr->valueToKey(value);
}

void qt_core_c_QMetaEnum_valueToKeys_to_output(const QMetaEnum* this_ptr, int value, QByteArray* output) {
  new(output) QByteArray(this_ptr->valueToKeys(value));
}

