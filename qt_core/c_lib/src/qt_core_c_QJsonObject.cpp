#include "qt_core_c_QJsonObject.h"

void qt_core_c_QJsonObject_begin_to_output(QJsonObject* this_ptr, QJsonObject::iterator* output) {
  new(output) QJsonObject::iterator(this_ptr->begin());
}

void qt_core_c_QJsonObject_begin_to_output_const(const QJsonObject* this_ptr, QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator(this_ptr->begin());
}

void qt_core_c_QJsonObject_constBegin_to_output(const QJsonObject* this_ptr, QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator(this_ptr->constBegin());
}

void qt_core_c_QJsonObject_constEnd_to_output(const QJsonObject* this_ptr, QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator(this_ptr->constEnd());
}

void qt_core_c_QJsonObject_constFind_to_output_QLatin1String(const QJsonObject* this_ptr, const QLatin1String* key, QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator(this_ptr->constFind(*key));
}

void qt_core_c_QJsonObject_constFind_to_output_QString(const QJsonObject* this_ptr, const QString* key, QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator(this_ptr->constFind(*key));
}

void qt_core_c_QJsonObject_const_iterator_constructor_no_args(QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator();
}

void qt_core_c_QJsonObject_const_iterator_constructor_obj_index(const QJsonObject* obj, int index, QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator(obj, index);
}

void qt_core_c_QJsonObject_const_iterator_constructor_other(const QJsonObject::iterator* other, QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator(*other);
}

void qt_core_c_QJsonObject_const_iterator_destructor(QJsonObject::const_iterator* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QJsonObject_const_iterator_key_to_output(const QJsonObject::const_iterator* this_ptr, QString* output) {
  new(output) QString(this_ptr->key());
}

QJsonObject::const_iterator* qt_core_c_QJsonObject_const_iterator_operator_add_assign(QJsonObject::const_iterator* this_ptr, int j) {
  return &this_ptr->operator+=(j);
}

void qt_core_c_QJsonObject_const_iterator_operator_add_to_output(const QJsonObject::const_iterator* this_ptr, int j, QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator(this_ptr->operator+(j));
}

QJsonObject::const_iterator* qt_core_c_QJsonObject_const_iterator_operator_dec(QJsonObject::const_iterator* this_ptr) {
  return &this_ptr->operator--();
}

void qt_core_c_QJsonObject_const_iterator_operator_dec_postfix_to_output(QJsonObject::const_iterator* this_ptr, int arg1, QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator(this_ptr->operator--(arg1));
}

bool qt_core_c_QJsonObject_const_iterator_operator_eq_QJsonObject_const_iterator(const QJsonObject::const_iterator* this_ptr, const QJsonObject::const_iterator* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QJsonObject_const_iterator_operator_eq_QJsonObject_iterator(const QJsonObject::const_iterator* this_ptr, const QJsonObject::iterator* other) {
  return this_ptr->operator==(*other);
}

QJsonObject::const_iterator* qt_core_c_QJsonObject_const_iterator_operator_inc(QJsonObject::const_iterator* this_ptr) {
  return &this_ptr->operator++();
}

void qt_core_c_QJsonObject_const_iterator_operator_inc_postfix_to_output(QJsonObject::const_iterator* this_ptr, int arg1, QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator(this_ptr->operator++(arg1));
}

void qt_core_c_QJsonObject_const_iterator_operator_indirection_to_output(const QJsonObject::const_iterator* this_ptr, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->operator*());
}

bool qt_core_c_QJsonObject_const_iterator_operator_neq_QJsonObject_const_iterator(const QJsonObject::const_iterator* this_ptr, const QJsonObject::const_iterator* other) {
  return this_ptr->operator!=(*other);
}

bool qt_core_c_QJsonObject_const_iterator_operator_neq_QJsonObject_iterator(const QJsonObject::const_iterator* this_ptr, const QJsonObject::iterator* other) {
  return this_ptr->operator!=(*other);
}

QJsonObject::const_iterator* qt_core_c_QJsonObject_const_iterator_operator_sub_assign(QJsonObject::const_iterator* this_ptr, int j) {
  return &this_ptr->operator-=(j);
}

void qt_core_c_QJsonObject_const_iterator_operator_sub_to_output(const QJsonObject::const_iterator* this_ptr, int j, QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator(this_ptr->operator-(j));
}

void qt_core_c_QJsonObject_const_iterator_value_to_output(const QJsonObject::const_iterator* this_ptr, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->value());
}

void qt_core_c_QJsonObject_constructor_no_args(QJsonObject* output) {
  new(output) QJsonObject();
}

void qt_core_c_QJsonObject_constructor_other(const QJsonObject* other, QJsonObject* output) {
  new(output) QJsonObject(*other);
}

bool qt_core_c_QJsonObject_contains_QLatin1String(const QJsonObject* this_ptr, const QLatin1String* key) {
  return this_ptr->contains(*key);
}

bool qt_core_c_QJsonObject_contains_QString(const QJsonObject* this_ptr, const QString* key) {
  return this_ptr->contains(*key);
}

int qt_core_c_QJsonObject_count(const QJsonObject* this_ptr) {
  return this_ptr->count();
}

void qt_core_c_QJsonObject_destructor(QJsonObject* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QJsonObject_empty(const QJsonObject* this_ptr) {
  return this_ptr->empty();
}

void qt_core_c_QJsonObject_end_to_output(QJsonObject* this_ptr, QJsonObject::iterator* output) {
  new(output) QJsonObject::iterator(this_ptr->end());
}

void qt_core_c_QJsonObject_end_to_output_const(const QJsonObject* this_ptr, QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator(this_ptr->end());
}

void qt_core_c_QJsonObject_erase_to_output(QJsonObject* this_ptr, const QJsonObject::iterator* it, QJsonObject::iterator* output) {
  new(output) QJsonObject::iterator(this_ptr->erase(*it));
}

void qt_core_c_QJsonObject_find_to_output_QLatin1String(QJsonObject* this_ptr, const QLatin1String* key, QJsonObject::iterator* output) {
  new(output) QJsonObject::iterator(this_ptr->find(*key));
}

void qt_core_c_QJsonObject_find_to_output_QString(QJsonObject* this_ptr, const QString* key, QJsonObject::iterator* output) {
  new(output) QJsonObject::iterator(this_ptr->find(*key));
}

void qt_core_c_QJsonObject_find_to_output_const_QLatin1String(const QJsonObject* this_ptr, const QLatin1String* key, QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator(this_ptr->find(*key));
}

void qt_core_c_QJsonObject_find_to_output_const_QString(const QJsonObject* this_ptr, const QString* key, QJsonObject::const_iterator* output) {
  new(output) QJsonObject::const_iterator(this_ptr->find(*key));
}

void qt_core_c_QJsonObject_fromVariantHash_to_output(const QHash< QString, QVariant >* map, QJsonObject* output) {
  new(output) QJsonObject(QJsonObject::fromVariantHash(*map));
}

void qt_core_c_QJsonObject_fromVariantMap_to_output(const QMap< QString, QVariant >* map, QJsonObject* output) {
  new(output) QJsonObject(QJsonObject::fromVariantMap(*map));
}

void qt_core_c_QJsonObject_insert_to_output(QJsonObject* this_ptr, const QString* key, const QJsonValue* value, QJsonObject::iterator* output) {
  new(output) QJsonObject::iterator(this_ptr->insert(*key, *value));
}

bool qt_core_c_QJsonObject_isEmpty(const QJsonObject* this_ptr) {
  return this_ptr->isEmpty();
}

void qt_core_c_QJsonObject_iterator_constructor_no_args(QJsonObject::iterator* output) {
  new(output) QJsonObject::iterator();
}

void qt_core_c_QJsonObject_iterator_constructor_obj_index(QJsonObject* obj, int index, QJsonObject::iterator* output) {
  new(output) QJsonObject::iterator(obj, index);
}

void qt_core_c_QJsonObject_iterator_destructor(QJsonObject::iterator* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QJsonObject_iterator_key_to_output(const QJsonObject::iterator* this_ptr, QString* output) {
  new(output) QString(this_ptr->key());
}

QJsonObject::iterator* qt_core_c_QJsonObject_iterator_operator_add_assign(QJsonObject::iterator* this_ptr, int j) {
  return &this_ptr->operator+=(j);
}

void qt_core_c_QJsonObject_iterator_operator_add_to_output(const QJsonObject::iterator* this_ptr, int j, QJsonObject::iterator* output) {
  new(output) QJsonObject::iterator(this_ptr->operator+(j));
}

QJsonObject::iterator* qt_core_c_QJsonObject_iterator_operator_dec(QJsonObject::iterator* this_ptr) {
  return &this_ptr->operator--();
}

void qt_core_c_QJsonObject_iterator_operator_dec_postfix_to_output(QJsonObject::iterator* this_ptr, int arg1, QJsonObject::iterator* output) {
  new(output) QJsonObject::iterator(this_ptr->operator--(arg1));
}

bool qt_core_c_QJsonObject_iterator_operator_eq_QJsonObject_const_iterator(const QJsonObject::iterator* this_ptr, const QJsonObject::const_iterator* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QJsonObject_iterator_operator_eq_QJsonObject_iterator(const QJsonObject::iterator* this_ptr, const QJsonObject::iterator* other) {
  return this_ptr->operator==(*other);
}

QJsonObject::iterator* qt_core_c_QJsonObject_iterator_operator_inc(QJsonObject::iterator* this_ptr) {
  return &this_ptr->operator++();
}

void qt_core_c_QJsonObject_iterator_operator_inc_postfix_to_output(QJsonObject::iterator* this_ptr, int arg1, QJsonObject::iterator* output) {
  new(output) QJsonObject::iterator(this_ptr->operator++(arg1));
}

void qt_core_c_QJsonObject_iterator_operator_indirection_to_output(const QJsonObject::iterator* this_ptr, QJsonValueRef* output) {
  new(output) QJsonValueRef(this_ptr->operator*());
}

bool qt_core_c_QJsonObject_iterator_operator_neq_QJsonObject_const_iterator(const QJsonObject::iterator* this_ptr, const QJsonObject::const_iterator* other) {
  return this_ptr->operator!=(*other);
}

bool qt_core_c_QJsonObject_iterator_operator_neq_QJsonObject_iterator(const QJsonObject::iterator* this_ptr, const QJsonObject::iterator* other) {
  return this_ptr->operator!=(*other);
}

QJsonObject::iterator* qt_core_c_QJsonObject_iterator_operator_sub_assign(QJsonObject::iterator* this_ptr, int j) {
  return &this_ptr->operator-=(j);
}

void qt_core_c_QJsonObject_iterator_operator_sub_to_output(const QJsonObject::iterator* this_ptr, int j, QJsonObject::iterator* output) {
  new(output) QJsonObject::iterator(this_ptr->operator-(j));
}

void qt_core_c_QJsonObject_iterator_value_to_output(const QJsonObject::iterator* this_ptr, QJsonValueRef* output) {
  new(output) QJsonValueRef(this_ptr->value());
}

void qt_core_c_QJsonObject_keys_to_output(const QJsonObject* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->keys());
}

int qt_core_c_QJsonObject_length(const QJsonObject* this_ptr) {
  return this_ptr->length();
}

QJsonObject* qt_core_c_QJsonObject_operator_assign(QJsonObject* this_ptr, const QJsonObject* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QJsonObject_operator_eq(const QJsonObject* this_ptr, const QJsonObject* other) {
  return this_ptr->operator==(*other);
}

void qt_core_c_QJsonObject_operator_index_to_output_QLatin1String(QJsonObject* this_ptr, const QLatin1String* key, QJsonValueRef* output) {
  new(output) QJsonValueRef(this_ptr->operator[](*key));
}

void qt_core_c_QJsonObject_operator_index_to_output_QString(QJsonObject* this_ptr, const QString* key, QJsonValueRef* output) {
  new(output) QJsonValueRef(this_ptr->operator[](*key));
}

void qt_core_c_QJsonObject_operator_index_to_output_const_QLatin1String(const QJsonObject* this_ptr, const QLatin1String* key, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->operator[](*key));
}

void qt_core_c_QJsonObject_operator_index_to_output_const_QString(const QJsonObject* this_ptr, const QString* key, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->operator[](*key));
}

bool qt_core_c_QJsonObject_operator_neq(const QJsonObject* this_ptr, const QJsonObject* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QJsonObject_remove(QJsonObject* this_ptr, const QString* key) {
  this_ptr->remove(*key);
}

int qt_core_c_QJsonObject_size(const QJsonObject* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QJsonObject_take_to_output(QJsonObject* this_ptr, const QString* key, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->take(*key));
}

void qt_core_c_QJsonObject_toVariantHash_to_output(const QJsonObject* this_ptr, QHash< QString, QVariant >* output) {
  new(output) QHash< QString, QVariant >(this_ptr->toVariantHash());
}

void qt_core_c_QJsonObject_toVariantMap_to_output(const QJsonObject* this_ptr, QMap< QString, QVariant >* output) {
  new(output) QMap< QString, QVariant >(this_ptr->toVariantMap());
}

void qt_core_c_QJsonObject_value_to_output_QLatin1String(const QJsonObject* this_ptr, const QLatin1String* key, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->value(*key));
}

void qt_core_c_QJsonObject_value_to_output_QString(const QJsonObject* this_ptr, const QString* key, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->value(*key));
}

