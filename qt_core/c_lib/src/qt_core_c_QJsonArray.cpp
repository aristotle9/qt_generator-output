#include "qt_core_c_QJsonArray.h"

void qt_core_c_QJsonArray_append(QJsonArray* this_ptr, const QJsonValue* value) {
  this_ptr->append(*value);
}

void qt_core_c_QJsonArray_at_to_output(const QJsonArray* this_ptr, int i, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->at(i));
}

void qt_core_c_QJsonArray_begin_to_output(QJsonArray* this_ptr, QJsonArray::iterator* output) {
  new(output) QJsonArray::iterator(this_ptr->begin());
}

void qt_core_c_QJsonArray_begin_to_output_const(const QJsonArray* this_ptr, QJsonArray::const_iterator* output) {
  new(output) QJsonArray::const_iterator(this_ptr->begin());
}

void qt_core_c_QJsonArray_constBegin_to_output(const QJsonArray* this_ptr, QJsonArray::const_iterator* output) {
  new(output) QJsonArray::const_iterator(this_ptr->constBegin());
}

void qt_core_c_QJsonArray_constEnd_to_output(const QJsonArray* this_ptr, QJsonArray::const_iterator* output) {
  new(output) QJsonArray::const_iterator(this_ptr->constEnd());
}

const QJsonArray* qt_core_c_QJsonArray_const_iterator_a(const QJsonArray::const_iterator* this_ptr) {
  return this_ptr->a;
}

void qt_core_c_QJsonArray_const_iterator_constructor_QJsonArray_const_iterator(const QJsonArray::const_iterator* o, QJsonArray::const_iterator* output) {
  new(output) QJsonArray::const_iterator(*o);
}

void qt_core_c_QJsonArray_const_iterator_constructor_QJsonArray_int(const QJsonArray* array, int index, QJsonArray::const_iterator* output) {
  new(output) QJsonArray::const_iterator(array, index);
}

void qt_core_c_QJsonArray_const_iterator_constructor_QJsonArray_iterator(const QJsonArray::iterator* o, QJsonArray::const_iterator* output) {
  new(output) QJsonArray::const_iterator(*o);
}

void qt_core_c_QJsonArray_const_iterator_constructor_no_args(QJsonArray::const_iterator* output) {
  new(output) QJsonArray::const_iterator();
}

void qt_core_c_QJsonArray_const_iterator_destructor(QJsonArray::const_iterator* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

int qt_core_c_QJsonArray_const_iterator_i(const QJsonArray::const_iterator* this_ptr) {
  return this_ptr->i;
}

QJsonArray::const_iterator* qt_core_c_QJsonArray_const_iterator_operator_add_assign(QJsonArray::const_iterator* this_ptr, int j) {
  return &this_ptr->operator+=(j);
}

void qt_core_c_QJsonArray_const_iterator_operator_add_to_output(const QJsonArray::const_iterator* this_ptr, int j, QJsonArray::const_iterator* output) {
  new(output) QJsonArray::const_iterator(this_ptr->operator+(j));
}

QJsonArray::const_iterator* qt_core_c_QJsonArray_const_iterator_operator_dec(QJsonArray::const_iterator* this_ptr) {
  return &this_ptr->operator--();
}

void qt_core_c_QJsonArray_const_iterator_operator_dec_postfix_to_output(QJsonArray::const_iterator* this_ptr, int arg1, QJsonArray::const_iterator* output) {
  new(output) QJsonArray::const_iterator(this_ptr->operator--(arg1));
}

bool qt_core_c_QJsonArray_const_iterator_operator_eq(const QJsonArray::const_iterator* this_ptr, const QJsonArray::const_iterator* o) {
  return this_ptr->operator==(*o);
}

bool qt_core_c_QJsonArray_const_iterator_operator_ge(const QJsonArray::const_iterator* this_ptr, const QJsonArray::const_iterator* other) {
  return this_ptr->operator>=(*other);
}

bool qt_core_c_QJsonArray_const_iterator_operator_gt(const QJsonArray::const_iterator* this_ptr, const QJsonArray::const_iterator* other) {
  return this_ptr->operator>(*other);
}

QJsonArray::const_iterator* qt_core_c_QJsonArray_const_iterator_operator_inc(QJsonArray::const_iterator* this_ptr) {
  return &this_ptr->operator++();
}

void qt_core_c_QJsonArray_const_iterator_operator_inc_postfix_to_output(QJsonArray::const_iterator* this_ptr, int arg1, QJsonArray::const_iterator* output) {
  new(output) QJsonArray::const_iterator(this_ptr->operator++(arg1));
}

void qt_core_c_QJsonArray_const_iterator_operator_index_to_output(const QJsonArray::const_iterator* this_ptr, int j, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->operator[](j));
}

void qt_core_c_QJsonArray_const_iterator_operator_indirection_to_output(const QJsonArray::const_iterator* this_ptr, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->operator*());
}

bool qt_core_c_QJsonArray_const_iterator_operator_le(const QJsonArray::const_iterator* this_ptr, const QJsonArray::const_iterator* other) {
  return this_ptr->operator<=(*other);
}

bool qt_core_c_QJsonArray_const_iterator_operator_lt(const QJsonArray::const_iterator* this_ptr, const QJsonArray::const_iterator* other) {
  return this_ptr->operator<(*other);
}

bool qt_core_c_QJsonArray_const_iterator_operator_neq(const QJsonArray::const_iterator* this_ptr, const QJsonArray::const_iterator* o) {
  return this_ptr->operator!=(*o);
}

int qt_core_c_QJsonArray_const_iterator_operator_sub(const QJsonArray::const_iterator* this_ptr, const QJsonArray::const_iterator* j) {
  return this_ptr->operator-(*j);
}

QJsonArray::const_iterator* qt_core_c_QJsonArray_const_iterator_operator_sub_assign(QJsonArray::const_iterator* this_ptr, int j) {
  return &this_ptr->operator-=(j);
}

void qt_core_c_QJsonArray_const_iterator_operator_sub_to_output(const QJsonArray::const_iterator* this_ptr, int j, QJsonArray::const_iterator* output) {
  new(output) QJsonArray::const_iterator(this_ptr->operator-(j));
}

void qt_core_c_QJsonArray_const_iterator_set_a(QJsonArray::const_iterator* this_ptr, const QJsonArray* value) {
  this_ptr->a = value;
}

void qt_core_c_QJsonArray_const_iterator_set_i(QJsonArray::const_iterator* this_ptr, int value) {
  this_ptr->i = value;
}

void qt_core_c_QJsonArray_constructor_no_args(QJsonArray* output) {
  new(output) QJsonArray();
}

void qt_core_c_QJsonArray_constructor_other(const QJsonArray* other, QJsonArray* output) {
  new(output) QJsonArray(*other);
}

bool qt_core_c_QJsonArray_contains(const QJsonArray* this_ptr, const QJsonValue* element) {
  return this_ptr->contains(*element);
}

int qt_core_c_QJsonArray_count(const QJsonArray* this_ptr) {
  return this_ptr->count();
}

void qt_core_c_QJsonArray_destructor(QJsonArray* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QJsonArray_empty(const QJsonArray* this_ptr) {
  return this_ptr->empty();
}

void qt_core_c_QJsonArray_end_to_output(QJsonArray* this_ptr, QJsonArray::iterator* output) {
  new(output) QJsonArray::iterator(this_ptr->end());
}

void qt_core_c_QJsonArray_end_to_output_const(const QJsonArray* this_ptr, QJsonArray::const_iterator* output) {
  new(output) QJsonArray::const_iterator(this_ptr->end());
}

void qt_core_c_QJsonArray_erase_to_output(QJsonArray* this_ptr, const QJsonArray::iterator* it, QJsonArray::iterator* output) {
  new(output) QJsonArray::iterator(this_ptr->erase(*it));
}

void qt_core_c_QJsonArray_first_to_output(const QJsonArray* this_ptr, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->first());
}

void qt_core_c_QJsonArray_fromStringList_to_output(const QStringList* list, QJsonArray* output) {
  new(output) QJsonArray(QJsonArray::fromStringList(*list));
}

void qt_core_c_QJsonArray_fromVariantList_to_output(const QList< QVariant >* list, QJsonArray* output) {
  new(output) QJsonArray(QJsonArray::fromVariantList(*list));
}

void qt_core_c_QJsonArray_insert(QJsonArray* this_ptr, int i, const QJsonValue* value) {
  this_ptr->insert(i, *value);
}

void qt_core_c_QJsonArray_insert_to_output(QJsonArray* this_ptr, const QJsonArray::iterator* before, const QJsonValue* value, QJsonArray::iterator* output) {
  new(output) QJsonArray::iterator(this_ptr->insert(*before, *value));
}

bool qt_core_c_QJsonArray_isEmpty(const QJsonArray* this_ptr) {
  return this_ptr->isEmpty();
}

QJsonArray* qt_core_c_QJsonArray_iterator_a(const QJsonArray::iterator* this_ptr) {
  return this_ptr->a;
}

void qt_core_c_QJsonArray_iterator_constructor_array_index(QJsonArray* array, int index, QJsonArray::iterator* output) {
  new(output) QJsonArray::iterator(array, index);
}

void qt_core_c_QJsonArray_iterator_constructor_no_args(QJsonArray::iterator* output) {
  new(output) QJsonArray::iterator();
}

void qt_core_c_QJsonArray_iterator_destructor(QJsonArray::iterator* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

int qt_core_c_QJsonArray_iterator_i(const QJsonArray::iterator* this_ptr) {
  return this_ptr->i;
}

QJsonArray::iterator* qt_core_c_QJsonArray_iterator_operator_add_assign(QJsonArray::iterator* this_ptr, int j) {
  return &this_ptr->operator+=(j);
}

void qt_core_c_QJsonArray_iterator_operator_add_to_output(const QJsonArray::iterator* this_ptr, int j, QJsonArray::iterator* output) {
  new(output) QJsonArray::iterator(this_ptr->operator+(j));
}

QJsonArray::iterator* qt_core_c_QJsonArray_iterator_operator_dec(QJsonArray::iterator* this_ptr) {
  return &this_ptr->operator--();
}

void qt_core_c_QJsonArray_iterator_operator_dec_postfix_to_output(QJsonArray::iterator* this_ptr, int arg1, QJsonArray::iterator* output) {
  new(output) QJsonArray::iterator(this_ptr->operator--(arg1));
}

bool qt_core_c_QJsonArray_iterator_operator_eq_QJsonArray_const_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::const_iterator* o) {
  return this_ptr->operator==(*o);
}

bool qt_core_c_QJsonArray_iterator_operator_eq_QJsonArray_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::iterator* o) {
  return this_ptr->operator==(*o);
}

bool qt_core_c_QJsonArray_iterator_operator_ge_QJsonArray_const_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::const_iterator* other) {
  return this_ptr->operator>=(*other);
}

bool qt_core_c_QJsonArray_iterator_operator_ge_QJsonArray_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::iterator* other) {
  return this_ptr->operator>=(*other);
}

bool qt_core_c_QJsonArray_iterator_operator_gt_QJsonArray_const_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::const_iterator* other) {
  return this_ptr->operator>(*other);
}

bool qt_core_c_QJsonArray_iterator_operator_gt_QJsonArray_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::iterator* other) {
  return this_ptr->operator>(*other);
}

QJsonArray::iterator* qt_core_c_QJsonArray_iterator_operator_inc(QJsonArray::iterator* this_ptr) {
  return &this_ptr->operator++();
}

void qt_core_c_QJsonArray_iterator_operator_inc_postfix_to_output(QJsonArray::iterator* this_ptr, int arg1, QJsonArray::iterator* output) {
  new(output) QJsonArray::iterator(this_ptr->operator++(arg1));
}

void qt_core_c_QJsonArray_iterator_operator_index_to_output(const QJsonArray::iterator* this_ptr, int j, QJsonValueRef* output) {
  new(output) QJsonValueRef(this_ptr->operator[](j));
}

void qt_core_c_QJsonArray_iterator_operator_indirection_to_output(const QJsonArray::iterator* this_ptr, QJsonValueRef* output) {
  new(output) QJsonValueRef(this_ptr->operator*());
}

bool qt_core_c_QJsonArray_iterator_operator_le_QJsonArray_const_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::const_iterator* other) {
  return this_ptr->operator<=(*other);
}

bool qt_core_c_QJsonArray_iterator_operator_le_QJsonArray_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::iterator* other) {
  return this_ptr->operator<=(*other);
}

bool qt_core_c_QJsonArray_iterator_operator_lt_QJsonArray_const_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::const_iterator* other) {
  return this_ptr->operator<(*other);
}

bool qt_core_c_QJsonArray_iterator_operator_lt_QJsonArray_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::iterator* other) {
  return this_ptr->operator<(*other);
}

bool qt_core_c_QJsonArray_iterator_operator_neq_QJsonArray_const_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::const_iterator* o) {
  return this_ptr->operator!=(*o);
}

bool qt_core_c_QJsonArray_iterator_operator_neq_QJsonArray_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::iterator* o) {
  return this_ptr->operator!=(*o);
}

int qt_core_c_QJsonArray_iterator_operator_sub(const QJsonArray::iterator* this_ptr, const QJsonArray::iterator* j) {
  return this_ptr->operator-(*j);
}

QJsonArray::iterator* qt_core_c_QJsonArray_iterator_operator_sub_assign(QJsonArray::iterator* this_ptr, int j) {
  return &this_ptr->operator-=(j);
}

void qt_core_c_QJsonArray_iterator_operator_sub_to_output(const QJsonArray::iterator* this_ptr, int j, QJsonArray::iterator* output) {
  new(output) QJsonArray::iterator(this_ptr->operator-(j));
}

void qt_core_c_QJsonArray_iterator_set_a(QJsonArray::iterator* this_ptr, QJsonArray* value) {
  this_ptr->a = value;
}

void qt_core_c_QJsonArray_iterator_set_i(QJsonArray::iterator* this_ptr, int value) {
  this_ptr->i = value;
}

void qt_core_c_QJsonArray_last_to_output(const QJsonArray* this_ptr, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->last());
}

QJsonArray* qt_core_c_QJsonArray_operator_add_assign(QJsonArray* this_ptr, const QJsonValue* v) {
  return &this_ptr->operator+=(*v);
}

void qt_core_c_QJsonArray_operator_add_to_output(const QJsonArray* this_ptr, const QJsonValue* v, QJsonArray* output) {
  new(output) QJsonArray(this_ptr->operator+(*v));
}

QJsonArray* qt_core_c_QJsonArray_operator_assign(QJsonArray* this_ptr, const QJsonArray* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QJsonArray_operator_eq(const QJsonArray* this_ptr, const QJsonArray* other) {
  return this_ptr->operator==(*other);
}

void qt_core_c_QJsonArray_operator_index_to_output(QJsonArray* this_ptr, int i, QJsonValueRef* output) {
  new(output) QJsonValueRef(this_ptr->operator[](i));
}

void qt_core_c_QJsonArray_operator_index_to_output_const(const QJsonArray* this_ptr, int i, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->operator[](i));
}

bool qt_core_c_QJsonArray_operator_neq(const QJsonArray* this_ptr, const QJsonArray* other) {
  return this_ptr->operator!=(*other);
}

QJsonArray* qt_core_c_QJsonArray_operator_shl(QJsonArray* this_ptr, const QJsonValue* v) {
  return &this_ptr->operator<<(*v);
}

void qt_core_c_QJsonArray_pop_back(QJsonArray* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QJsonArray_pop_front(QJsonArray* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QJsonArray_prepend(QJsonArray* this_ptr, const QJsonValue* value) {
  this_ptr->prepend(*value);
}

void qt_core_c_QJsonArray_push_back(QJsonArray* this_ptr, const QJsonValue* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QJsonArray_push_front(QJsonArray* this_ptr, const QJsonValue* t) {
  this_ptr->push_front(*t);
}

void qt_core_c_QJsonArray_removeAt(QJsonArray* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QJsonArray_removeFirst(QJsonArray* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QJsonArray_removeLast(QJsonArray* this_ptr) {
  this_ptr->removeLast();
}

void qt_core_c_QJsonArray_replace(QJsonArray* this_ptr, int i, const QJsonValue* value) {
  this_ptr->replace(i, *value);
}

int qt_core_c_QJsonArray_size(const QJsonArray* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QJsonArray_takeAt_to_output(QJsonArray* this_ptr, int i, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->takeAt(i));
}

void qt_core_c_QJsonArray_toVariantList_to_output(const QJsonArray* this_ptr, QList< QVariant >* output) {
  new(output) QList< QVariant >(this_ptr->toVariantList());
}

