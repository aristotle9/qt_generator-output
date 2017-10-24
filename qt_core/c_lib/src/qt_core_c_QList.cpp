#include "qt_core_c_QList.h"

void qt_core_c_QList_QAbstractAnimation_ptr_append_QAbstractAnimation(QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QAbstractAnimation_ptr_append_QList_QAbstractAnimation_ptr(QList< QAbstractAnimation* >* this_ptr, const QList< QAbstractAnimation* >* t) {
  this_ptr->append(*t);
}

QAbstractAnimation* const * qt_core_c_QList_QAbstractAnimation_ptr_at(const QList< QAbstractAnimation* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QAbstractAnimation** qt_core_c_QList_QAbstractAnimation_ptr_back(QList< QAbstractAnimation* >* this_ptr) {
  return &this_ptr->back();
}

QAbstractAnimation* const * qt_core_c_QList_QAbstractAnimation_ptr_back_const(const QList< QAbstractAnimation* >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QAbstractAnimation_ptr_clear(QList< QAbstractAnimation* >* this_ptr) {
  this_ptr->clear();
}

QAbstractAnimation* const * qt_core_c_QList_QAbstractAnimation_ptr_constFirst(const QList< QAbstractAnimation* >* this_ptr) {
  return &this_ptr->constFirst();
}

QAbstractAnimation* const * qt_core_c_QList_QAbstractAnimation_ptr_constLast(const QList< QAbstractAnimation* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QAbstractAnimation_ptr_constructor_l(const QList< QAbstractAnimation* >* l, QList< QAbstractAnimation* >* output) {
  new(output) QList< QAbstractAnimation* >(*l);
}

void qt_core_c_QList_QAbstractAnimation_ptr_constructor_no_args(QList< QAbstractAnimation* >* output) {
  new(output) QList< QAbstractAnimation* >();
}

bool qt_core_c_QList_QAbstractAnimation_ptr_contains(const QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QAbstractAnimation_ptr_count_no_args(const QList< QAbstractAnimation* >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QAbstractAnimation_ptr_count_t(const QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QAbstractAnimation_ptr_destructor(QList< QAbstractAnimation* >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QAbstractAnimation_ptr_empty(const QList< QAbstractAnimation* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QAbstractAnimation_ptr_endsWith(const QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t) {
  return this_ptr->endsWith(*t);
}

QAbstractAnimation** qt_core_c_QList_QAbstractAnimation_ptr_first(QList< QAbstractAnimation* >* this_ptr) {
  return &this_ptr->first();
}

QAbstractAnimation* const * qt_core_c_QList_QAbstractAnimation_ptr_first_const(const QList< QAbstractAnimation* >* this_ptr) {
  return &this_ptr->first();
}

QAbstractAnimation** qt_core_c_QList_QAbstractAnimation_ptr_front(QList< QAbstractAnimation* >* this_ptr) {
  return &this_ptr->front();
}

QAbstractAnimation* const * qt_core_c_QList_QAbstractAnimation_ptr_front_const(const QList< QAbstractAnimation* >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QAbstractAnimation_ptr_indexOf_t(const QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QAbstractAnimation_ptr_indexOf_t_from(const QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QAbstractAnimation_ptr_insert(QList< QAbstractAnimation* >* this_ptr, int i, QAbstractAnimation* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QAbstractAnimation_ptr_isEmpty(const QList< QAbstractAnimation* >* this_ptr) {
  return this_ptr->isEmpty();
}

QAbstractAnimation** qt_core_c_QList_QAbstractAnimation_ptr_last(QList< QAbstractAnimation* >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QAbstractAnimation_ptr_lastIndexOf_t(const QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QAbstractAnimation_ptr_lastIndexOf_t_from(const QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QAbstractAnimation* const * qt_core_c_QList_QAbstractAnimation_ptr_last_const(const QList< QAbstractAnimation* >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QAbstractAnimation_ptr_length(const QList< QAbstractAnimation* >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QAbstractAnimation_ptr_mid_to_output_pos(const QList< QAbstractAnimation* >* this_ptr, int pos, QList< QAbstractAnimation* >* output) {
  new(output) QList< QAbstractAnimation* >(this_ptr->mid(pos));
}

void qt_core_c_QList_QAbstractAnimation_ptr_mid_to_output_pos_length(const QList< QAbstractAnimation* >* this_ptr, int pos, int length, QList< QAbstractAnimation* >* output) {
  new(output) QList< QAbstractAnimation* >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QAbstractAnimation_ptr_move(QList< QAbstractAnimation* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QAbstractAnimation* >* qt_core_c_QList_QAbstractAnimation_ptr_operator_add_assign_l(QList< QAbstractAnimation* >* this_ptr, const QList< QAbstractAnimation* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QAbstractAnimation* >* qt_core_c_QList_QAbstractAnimation_ptr_operator_add_assign_t(QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QAbstractAnimation_ptr_operator_add_to_output(const QList< QAbstractAnimation* >* this_ptr, const QList< QAbstractAnimation* >* l, QList< QAbstractAnimation* >* output) {
  new(output) QList< QAbstractAnimation* >(this_ptr->operator+(*l));
}

QList< QAbstractAnimation* >* qt_core_c_QList_QAbstractAnimation_ptr_operator_assign(QList< QAbstractAnimation* >* this_ptr, const QList< QAbstractAnimation* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QAbstractAnimation_ptr_operator_eq(const QList< QAbstractAnimation* >* this_ptr, const QList< QAbstractAnimation* >* l) {
  return this_ptr->operator==(*l);
}

QAbstractAnimation** qt_core_c_QList_QAbstractAnimation_ptr_operator_index(QList< QAbstractAnimation* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QAbstractAnimation* const * qt_core_c_QList_QAbstractAnimation_ptr_operator_index_const(const QList< QAbstractAnimation* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QAbstractAnimation_ptr_operator_neq(const QList< QAbstractAnimation* >* this_ptr, const QList< QAbstractAnimation* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QAbstractAnimation* >* qt_core_c_QList_QAbstractAnimation_ptr_operator_shl_l(QList< QAbstractAnimation* >* this_ptr, const QList< QAbstractAnimation* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QAbstractAnimation* >* qt_core_c_QList_QAbstractAnimation_ptr_operator_shl_t(QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QAbstractAnimation_ptr_pop_back(QList< QAbstractAnimation* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QAbstractAnimation_ptr_pop_front(QList< QAbstractAnimation* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QAbstractAnimation_ptr_prepend(QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QAbstractAnimation_ptr_push_back(QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QAbstractAnimation_ptr_push_front(QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QAbstractAnimation_ptr_removeAll(QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QAbstractAnimation_ptr_removeAt(QList< QAbstractAnimation* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QAbstractAnimation_ptr_removeFirst(QList< QAbstractAnimation* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QAbstractAnimation_ptr_removeLast(QList< QAbstractAnimation* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QAbstractAnimation_ptr_removeOne(QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QAbstractAnimation_ptr_replace(QList< QAbstractAnimation* >* this_ptr, int i, QAbstractAnimation* const * t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QAbstractAnimation_ptr_reserve(QList< QAbstractAnimation* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QAbstractAnimation_ptr_size(const QList< QAbstractAnimation* >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QAbstractAnimation_ptr_startsWith(const QList< QAbstractAnimation* >* this_ptr, QAbstractAnimation* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QAbstractAnimation_ptr_swap_i_j(QList< QAbstractAnimation* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QAbstractAnimation_ptr_swap_other(QList< QAbstractAnimation* >* this_ptr, QList< QAbstractAnimation* >* other) {
  this_ptr->swap(*other);
}

QAbstractAnimation* qt_core_c_QList_QAbstractAnimation_ptr_takeAt(QList< QAbstractAnimation* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QAbstractAnimation* qt_core_c_QList_QAbstractAnimation_ptr_takeFirst(QList< QAbstractAnimation* >* this_ptr) {
  return this_ptr->takeFirst();
}

QAbstractAnimation* qt_core_c_QList_QAbstractAnimation_ptr_takeLast(QList< QAbstractAnimation* >* this_ptr) {
  return this_ptr->takeLast();
}

QAbstractAnimation* qt_core_c_QList_QAbstractAnimation_ptr_value_i(const QList< QAbstractAnimation* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QAbstractAnimation* qt_core_c_QList_QAbstractAnimation_ptr_value_i_defaultValue(const QList< QAbstractAnimation* >* this_ptr, int i, QAbstractAnimation* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_append_QAbstractEventDispatcher_TimerInfo(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, const QAbstractEventDispatcher::TimerInfo* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_append_QList_QAbstractEventDispatcher_TimerInfo(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, const QList< QAbstractEventDispatcher::TimerInfo >* t) {
  this_ptr->append(*t);
}

const QAbstractEventDispatcher::TimerInfo* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_at(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QAbstractEventDispatcher::TimerInfo* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_back(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return &this_ptr->back();
}

const QAbstractEventDispatcher::TimerInfo* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_back_const(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_clear(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  this_ptr->clear();
}

const QAbstractEventDispatcher::TimerInfo* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_constFirst(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return &this_ptr->constFirst();
}

const QAbstractEventDispatcher::TimerInfo* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_constLast(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_constructor_l(const QList< QAbstractEventDispatcher::TimerInfo >* l, QList< QAbstractEventDispatcher::TimerInfo >* output) {
  new(output) QList< QAbstractEventDispatcher::TimerInfo >(*l);
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_constructor_no_args(QList< QAbstractEventDispatcher::TimerInfo >* output) {
  new(output) QList< QAbstractEventDispatcher::TimerInfo >();
}

int qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_count(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return this_ptr->count();
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_destructor(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_empty(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return this_ptr->empty();
}

QAbstractEventDispatcher::TimerInfo* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_first(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return &this_ptr->first();
}

const QAbstractEventDispatcher::TimerInfo* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_first_const(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return &this_ptr->first();
}

QAbstractEventDispatcher::TimerInfo* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_front(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return &this_ptr->front();
}

const QAbstractEventDispatcher::TimerInfo* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_front_const(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return &this_ptr->front();
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_insert(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, int i, const QAbstractEventDispatcher::TimerInfo* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_isEmpty(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return this_ptr->isEmpty();
}

QAbstractEventDispatcher::TimerInfo* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_last(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return &this_ptr->last();
}

const QAbstractEventDispatcher::TimerInfo* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_last_const(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_length(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_mid_to_output_pos(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, int pos, QList< QAbstractEventDispatcher::TimerInfo >* output) {
  new(output) QList< QAbstractEventDispatcher::TimerInfo >(this_ptr->mid(pos));
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_mid_to_output_pos_length(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, int pos, int length, QList< QAbstractEventDispatcher::TimerInfo >* output) {
  new(output) QList< QAbstractEventDispatcher::TimerInfo >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_move(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QAbstractEventDispatcher::TimerInfo >* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_add_assign_l(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, const QList< QAbstractEventDispatcher::TimerInfo >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QAbstractEventDispatcher::TimerInfo >* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_add_assign_t(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, const QAbstractEventDispatcher::TimerInfo* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_add_to_output(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, const QList< QAbstractEventDispatcher::TimerInfo >* l, QList< QAbstractEventDispatcher::TimerInfo >* output) {
  new(output) QList< QAbstractEventDispatcher::TimerInfo >(this_ptr->operator+(*l));
}

QList< QAbstractEventDispatcher::TimerInfo >* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_assign(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, const QList< QAbstractEventDispatcher::TimerInfo >* l) {
  return &this_ptr->operator=(*l);
}

QAbstractEventDispatcher::TimerInfo* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_index(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QAbstractEventDispatcher::TimerInfo* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_index_const(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QList< QAbstractEventDispatcher::TimerInfo >* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_shl_l(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, const QList< QAbstractEventDispatcher::TimerInfo >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QAbstractEventDispatcher::TimerInfo >* qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_shl_t(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, const QAbstractEventDispatcher::TimerInfo* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_pop_back(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_pop_front(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_prepend(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, const QAbstractEventDispatcher::TimerInfo* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_push_back(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, const QAbstractEventDispatcher::TimerInfo* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_push_front(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, const QAbstractEventDispatcher::TimerInfo* t) {
  this_ptr->push_front(*t);
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_removeAt(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_removeFirst(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_removeLast(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  this_ptr->removeLast();
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_replace(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, int i, const QAbstractEventDispatcher::TimerInfo* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_reserve(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_size(const QList< QAbstractEventDispatcher::TimerInfo >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_swap_i_j(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_swap_other(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, QList< QAbstractEventDispatcher::TimerInfo >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_takeAt_to_output(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, int i, QAbstractEventDispatcher::TimerInfo* output) {
  new(output) QAbstractEventDispatcher::TimerInfo(this_ptr->takeAt(i));
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_takeFirst_to_output(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, QAbstractEventDispatcher::TimerInfo* output) {
  new(output) QAbstractEventDispatcher::TimerInfo(this_ptr->takeFirst());
}

void qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_takeLast_to_output(QList< QAbstractEventDispatcher::TimerInfo >* this_ptr, QAbstractEventDispatcher::TimerInfo* output) {
  new(output) QAbstractEventDispatcher::TimerInfo(this_ptr->takeLast());
}

void qt_core_c_QList_QAbstractState_ptr_append_QAbstractState(QList< QAbstractState* >* this_ptr, QAbstractState* const * t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QAbstractState_ptr_append_QList_QAbstractState_ptr(QList< QAbstractState* >* this_ptr, const QList< QAbstractState* >* t) {
  this_ptr->append(*t);
}

QAbstractState* const * qt_core_c_QList_QAbstractState_ptr_at(const QList< QAbstractState* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QAbstractState** qt_core_c_QList_QAbstractState_ptr_back(QList< QAbstractState* >* this_ptr) {
  return &this_ptr->back();
}

QAbstractState* const * qt_core_c_QList_QAbstractState_ptr_back_const(const QList< QAbstractState* >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QAbstractState_ptr_clear(QList< QAbstractState* >* this_ptr) {
  this_ptr->clear();
}

QAbstractState* const * qt_core_c_QList_QAbstractState_ptr_constFirst(const QList< QAbstractState* >* this_ptr) {
  return &this_ptr->constFirst();
}

QAbstractState* const * qt_core_c_QList_QAbstractState_ptr_constLast(const QList< QAbstractState* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QAbstractState_ptr_constructor_l(const QList< QAbstractState* >* l, QList< QAbstractState* >* output) {
  new(output) QList< QAbstractState* >(*l);
}

void qt_core_c_QList_QAbstractState_ptr_constructor_no_args(QList< QAbstractState* >* output) {
  new(output) QList< QAbstractState* >();
}

bool qt_core_c_QList_QAbstractState_ptr_contains(const QList< QAbstractState* >* this_ptr, QAbstractState* const * t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QAbstractState_ptr_count_no_args(const QList< QAbstractState* >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QAbstractState_ptr_count_t(const QList< QAbstractState* >* this_ptr, QAbstractState* const * t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QAbstractState_ptr_destructor(QList< QAbstractState* >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QAbstractState_ptr_empty(const QList< QAbstractState* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QAbstractState_ptr_endsWith(const QList< QAbstractState* >* this_ptr, QAbstractState* const * t) {
  return this_ptr->endsWith(*t);
}

QAbstractState** qt_core_c_QList_QAbstractState_ptr_first(QList< QAbstractState* >* this_ptr) {
  return &this_ptr->first();
}

QAbstractState* const * qt_core_c_QList_QAbstractState_ptr_first_const(const QList< QAbstractState* >* this_ptr) {
  return &this_ptr->first();
}

void qt_core_c_QList_QAbstractState_ptr_fromSet_to_output(const QSet< QAbstractState* >* set, QList< QAbstractState* >* output) {
  new(output) QList< QAbstractState* >(QList< QAbstractState* >::fromSet(*set));
}

QAbstractState** qt_core_c_QList_QAbstractState_ptr_front(QList< QAbstractState* >* this_ptr) {
  return &this_ptr->front();
}

QAbstractState* const * qt_core_c_QList_QAbstractState_ptr_front_const(const QList< QAbstractState* >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QAbstractState_ptr_indexOf_t(const QList< QAbstractState* >* this_ptr, QAbstractState* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QAbstractState_ptr_indexOf_t_from(const QList< QAbstractState* >* this_ptr, QAbstractState* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QAbstractState_ptr_insert(QList< QAbstractState* >* this_ptr, int i, QAbstractState* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QAbstractState_ptr_isEmpty(const QList< QAbstractState* >* this_ptr) {
  return this_ptr->isEmpty();
}

QAbstractState** qt_core_c_QList_QAbstractState_ptr_last(QList< QAbstractState* >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QAbstractState_ptr_lastIndexOf_t(const QList< QAbstractState* >* this_ptr, QAbstractState* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QAbstractState_ptr_lastIndexOf_t_from(const QList< QAbstractState* >* this_ptr, QAbstractState* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QAbstractState* const * qt_core_c_QList_QAbstractState_ptr_last_const(const QList< QAbstractState* >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QAbstractState_ptr_length(const QList< QAbstractState* >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QAbstractState_ptr_mid_to_output_pos(const QList< QAbstractState* >* this_ptr, int pos, QList< QAbstractState* >* output) {
  new(output) QList< QAbstractState* >(this_ptr->mid(pos));
}

void qt_core_c_QList_QAbstractState_ptr_mid_to_output_pos_length(const QList< QAbstractState* >* this_ptr, int pos, int length, QList< QAbstractState* >* output) {
  new(output) QList< QAbstractState* >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QAbstractState_ptr_move(QList< QAbstractState* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QAbstractState* >* qt_core_c_QList_QAbstractState_ptr_operator_add_assign_l(QList< QAbstractState* >* this_ptr, const QList< QAbstractState* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QAbstractState* >* qt_core_c_QList_QAbstractState_ptr_operator_add_assign_t(QList< QAbstractState* >* this_ptr, QAbstractState* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QAbstractState_ptr_operator_add_to_output(const QList< QAbstractState* >* this_ptr, const QList< QAbstractState* >* l, QList< QAbstractState* >* output) {
  new(output) QList< QAbstractState* >(this_ptr->operator+(*l));
}

QList< QAbstractState* >* qt_core_c_QList_QAbstractState_ptr_operator_assign(QList< QAbstractState* >* this_ptr, const QList< QAbstractState* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QAbstractState_ptr_operator_eq(const QList< QAbstractState* >* this_ptr, const QList< QAbstractState* >* l) {
  return this_ptr->operator==(*l);
}

QAbstractState** qt_core_c_QList_QAbstractState_ptr_operator_index(QList< QAbstractState* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QAbstractState* const * qt_core_c_QList_QAbstractState_ptr_operator_index_const(const QList< QAbstractState* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QAbstractState_ptr_operator_neq(const QList< QAbstractState* >* this_ptr, const QList< QAbstractState* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QAbstractState* >* qt_core_c_QList_QAbstractState_ptr_operator_shl_l(QList< QAbstractState* >* this_ptr, const QList< QAbstractState* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QAbstractState* >* qt_core_c_QList_QAbstractState_ptr_operator_shl_t(QList< QAbstractState* >* this_ptr, QAbstractState* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QAbstractState_ptr_pop_back(QList< QAbstractState* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QAbstractState_ptr_pop_front(QList< QAbstractState* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QAbstractState_ptr_prepend(QList< QAbstractState* >* this_ptr, QAbstractState* const * t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QAbstractState_ptr_push_back(QList< QAbstractState* >* this_ptr, QAbstractState* const * t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QAbstractState_ptr_push_front(QList< QAbstractState* >* this_ptr, QAbstractState* const * t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QAbstractState_ptr_removeAll(QList< QAbstractState* >* this_ptr, QAbstractState* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QAbstractState_ptr_removeAt(QList< QAbstractState* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QAbstractState_ptr_removeFirst(QList< QAbstractState* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QAbstractState_ptr_removeLast(QList< QAbstractState* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QAbstractState_ptr_removeOne(QList< QAbstractState* >* this_ptr, QAbstractState* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QAbstractState_ptr_replace(QList< QAbstractState* >* this_ptr, int i, QAbstractState* const * t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QAbstractState_ptr_reserve(QList< QAbstractState* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QAbstractState_ptr_size(const QList< QAbstractState* >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QAbstractState_ptr_startsWith(const QList< QAbstractState* >* this_ptr, QAbstractState* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QAbstractState_ptr_swap_i_j(QList< QAbstractState* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QAbstractState_ptr_swap_other(QList< QAbstractState* >* this_ptr, QList< QAbstractState* >* other) {
  this_ptr->swap(*other);
}

QAbstractState* qt_core_c_QList_QAbstractState_ptr_takeAt(QList< QAbstractState* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QAbstractState* qt_core_c_QList_QAbstractState_ptr_takeFirst(QList< QAbstractState* >* this_ptr) {
  return this_ptr->takeFirst();
}

QAbstractState* qt_core_c_QList_QAbstractState_ptr_takeLast(QList< QAbstractState* >* this_ptr) {
  return this_ptr->takeLast();
}

void qt_core_c_QList_QAbstractState_ptr_toSet_to_output(const QList< QAbstractState* >* this_ptr, QSet< QAbstractState* >* output) {
  new(output) QSet< QAbstractState* >(this_ptr->toSet());
}

QAbstractState* qt_core_c_QList_QAbstractState_ptr_value_i(const QList< QAbstractState* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QAbstractState* qt_core_c_QList_QAbstractState_ptr_value_i_defaultValue(const QList< QAbstractState* >* this_ptr, int i, QAbstractState* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_core_c_QList_QAbstractTransition_ptr_append_QAbstractTransition(QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QAbstractTransition_ptr_append_QList_QAbstractTransition_ptr(QList< QAbstractTransition* >* this_ptr, const QList< QAbstractTransition* >* t) {
  this_ptr->append(*t);
}

QAbstractTransition* const * qt_core_c_QList_QAbstractTransition_ptr_at(const QList< QAbstractTransition* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QAbstractTransition** qt_core_c_QList_QAbstractTransition_ptr_back(QList< QAbstractTransition* >* this_ptr) {
  return &this_ptr->back();
}

QAbstractTransition* const * qt_core_c_QList_QAbstractTransition_ptr_back_const(const QList< QAbstractTransition* >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QAbstractTransition_ptr_clear(QList< QAbstractTransition* >* this_ptr) {
  this_ptr->clear();
}

QAbstractTransition* const * qt_core_c_QList_QAbstractTransition_ptr_constFirst(const QList< QAbstractTransition* >* this_ptr) {
  return &this_ptr->constFirst();
}

QAbstractTransition* const * qt_core_c_QList_QAbstractTransition_ptr_constLast(const QList< QAbstractTransition* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QAbstractTransition_ptr_constructor_l(const QList< QAbstractTransition* >* l, QList< QAbstractTransition* >* output) {
  new(output) QList< QAbstractTransition* >(*l);
}

void qt_core_c_QList_QAbstractTransition_ptr_constructor_no_args(QList< QAbstractTransition* >* output) {
  new(output) QList< QAbstractTransition* >();
}

bool qt_core_c_QList_QAbstractTransition_ptr_contains(const QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QAbstractTransition_ptr_count_no_args(const QList< QAbstractTransition* >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QAbstractTransition_ptr_count_t(const QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QAbstractTransition_ptr_destructor(QList< QAbstractTransition* >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QAbstractTransition_ptr_empty(const QList< QAbstractTransition* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QAbstractTransition_ptr_endsWith(const QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t) {
  return this_ptr->endsWith(*t);
}

QAbstractTransition** qt_core_c_QList_QAbstractTransition_ptr_first(QList< QAbstractTransition* >* this_ptr) {
  return &this_ptr->first();
}

QAbstractTransition* const * qt_core_c_QList_QAbstractTransition_ptr_first_const(const QList< QAbstractTransition* >* this_ptr) {
  return &this_ptr->first();
}

QAbstractTransition** qt_core_c_QList_QAbstractTransition_ptr_front(QList< QAbstractTransition* >* this_ptr) {
  return &this_ptr->front();
}

QAbstractTransition* const * qt_core_c_QList_QAbstractTransition_ptr_front_const(const QList< QAbstractTransition* >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QAbstractTransition_ptr_indexOf_t(const QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QAbstractTransition_ptr_indexOf_t_from(const QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QAbstractTransition_ptr_insert(QList< QAbstractTransition* >* this_ptr, int i, QAbstractTransition* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QAbstractTransition_ptr_isEmpty(const QList< QAbstractTransition* >* this_ptr) {
  return this_ptr->isEmpty();
}

QAbstractTransition** qt_core_c_QList_QAbstractTransition_ptr_last(QList< QAbstractTransition* >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QAbstractTransition_ptr_lastIndexOf_t(const QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QAbstractTransition_ptr_lastIndexOf_t_from(const QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QAbstractTransition* const * qt_core_c_QList_QAbstractTransition_ptr_last_const(const QList< QAbstractTransition* >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QAbstractTransition_ptr_length(const QList< QAbstractTransition* >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QAbstractTransition_ptr_mid_to_output_pos(const QList< QAbstractTransition* >* this_ptr, int pos, QList< QAbstractTransition* >* output) {
  new(output) QList< QAbstractTransition* >(this_ptr->mid(pos));
}

void qt_core_c_QList_QAbstractTransition_ptr_mid_to_output_pos_length(const QList< QAbstractTransition* >* this_ptr, int pos, int length, QList< QAbstractTransition* >* output) {
  new(output) QList< QAbstractTransition* >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QAbstractTransition_ptr_move(QList< QAbstractTransition* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QAbstractTransition* >* qt_core_c_QList_QAbstractTransition_ptr_operator_add_assign_l(QList< QAbstractTransition* >* this_ptr, const QList< QAbstractTransition* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QAbstractTransition* >* qt_core_c_QList_QAbstractTransition_ptr_operator_add_assign_t(QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QAbstractTransition_ptr_operator_add_to_output(const QList< QAbstractTransition* >* this_ptr, const QList< QAbstractTransition* >* l, QList< QAbstractTransition* >* output) {
  new(output) QList< QAbstractTransition* >(this_ptr->operator+(*l));
}

QList< QAbstractTransition* >* qt_core_c_QList_QAbstractTransition_ptr_operator_assign(QList< QAbstractTransition* >* this_ptr, const QList< QAbstractTransition* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QAbstractTransition_ptr_operator_eq(const QList< QAbstractTransition* >* this_ptr, const QList< QAbstractTransition* >* l) {
  return this_ptr->operator==(*l);
}

QAbstractTransition** qt_core_c_QList_QAbstractTransition_ptr_operator_index(QList< QAbstractTransition* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QAbstractTransition* const * qt_core_c_QList_QAbstractTransition_ptr_operator_index_const(const QList< QAbstractTransition* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QAbstractTransition_ptr_operator_neq(const QList< QAbstractTransition* >* this_ptr, const QList< QAbstractTransition* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QAbstractTransition* >* qt_core_c_QList_QAbstractTransition_ptr_operator_shl_l(QList< QAbstractTransition* >* this_ptr, const QList< QAbstractTransition* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QAbstractTransition* >* qt_core_c_QList_QAbstractTransition_ptr_operator_shl_t(QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QAbstractTransition_ptr_pop_back(QList< QAbstractTransition* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QAbstractTransition_ptr_pop_front(QList< QAbstractTransition* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QAbstractTransition_ptr_prepend(QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QAbstractTransition_ptr_push_back(QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QAbstractTransition_ptr_push_front(QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QAbstractTransition_ptr_removeAll(QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QAbstractTransition_ptr_removeAt(QList< QAbstractTransition* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QAbstractTransition_ptr_removeFirst(QList< QAbstractTransition* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QAbstractTransition_ptr_removeLast(QList< QAbstractTransition* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QAbstractTransition_ptr_removeOne(QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QAbstractTransition_ptr_replace(QList< QAbstractTransition* >* this_ptr, int i, QAbstractTransition* const * t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QAbstractTransition_ptr_reserve(QList< QAbstractTransition* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QAbstractTransition_ptr_size(const QList< QAbstractTransition* >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QAbstractTransition_ptr_startsWith(const QList< QAbstractTransition* >* this_ptr, QAbstractTransition* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QAbstractTransition_ptr_swap_i_j(QList< QAbstractTransition* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QAbstractTransition_ptr_swap_other(QList< QAbstractTransition* >* this_ptr, QList< QAbstractTransition* >* other) {
  this_ptr->swap(*other);
}

QAbstractTransition* qt_core_c_QList_QAbstractTransition_ptr_takeAt(QList< QAbstractTransition* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QAbstractTransition* qt_core_c_QList_QAbstractTransition_ptr_takeFirst(QList< QAbstractTransition* >* this_ptr) {
  return this_ptr->takeFirst();
}

QAbstractTransition* qt_core_c_QList_QAbstractTransition_ptr_takeLast(QList< QAbstractTransition* >* this_ptr) {
  return this_ptr->takeLast();
}

QAbstractTransition* qt_core_c_QList_QAbstractTransition_ptr_value_i(const QList< QAbstractTransition* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QAbstractTransition* qt_core_c_QList_QAbstractTransition_ptr_value_i_defaultValue(const QList< QAbstractTransition* >* this_ptr, int i, QAbstractTransition* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_core_c_QList_QByteArray_append_QByteArray(QList< QByteArray >* this_ptr, const QByteArray* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QByteArray_append_QList_QByteArray(QList< QByteArray >* this_ptr, const QList< QByteArray >* t) {
  this_ptr->append(*t);
}

const QByteArray* qt_core_c_QList_QByteArray_at(const QList< QByteArray >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QByteArray* qt_core_c_QList_QByteArray_back(QList< QByteArray >* this_ptr) {
  return &this_ptr->back();
}

const QByteArray* qt_core_c_QList_QByteArray_back_const(const QList< QByteArray >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QByteArray_clear(QList< QByteArray >* this_ptr) {
  this_ptr->clear();
}

const QByteArray* qt_core_c_QList_QByteArray_constFirst(const QList< QByteArray >* this_ptr) {
  return &this_ptr->constFirst();
}

const QByteArray* qt_core_c_QList_QByteArray_constLast(const QList< QByteArray >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QByteArray_constructor_l(const QList< QByteArray >* l, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(*l);
}

void qt_core_c_QList_QByteArray_constructor_no_args(QList< QByteArray >* output) {
  new(output) QList< QByteArray >();
}

bool qt_core_c_QList_QByteArray_contains(const QList< QByteArray >* this_ptr, const QByteArray* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QByteArray_count_no_args(const QList< QByteArray >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QByteArray_count_t(const QList< QByteArray >* this_ptr, const QByteArray* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QByteArray_destructor(QList< QByteArray >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QByteArray_empty(const QList< QByteArray >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QByteArray_endsWith(const QList< QByteArray >* this_ptr, const QByteArray* t) {
  return this_ptr->endsWith(*t);
}

QByteArray* qt_core_c_QList_QByteArray_first(QList< QByteArray >* this_ptr) {
  return &this_ptr->first();
}

const QByteArray* qt_core_c_QList_QByteArray_first_const(const QList< QByteArray >* this_ptr) {
  return &this_ptr->first();
}

QByteArray* qt_core_c_QList_QByteArray_front(QList< QByteArray >* this_ptr) {
  return &this_ptr->front();
}

const QByteArray* qt_core_c_QList_QByteArray_front_const(const QList< QByteArray >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QByteArray_indexOf_t(const QList< QByteArray >* this_ptr, const QByteArray* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QByteArray_indexOf_t_from(const QList< QByteArray >* this_ptr, const QByteArray* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QByteArray_insert(QList< QByteArray >* this_ptr, int i, const QByteArray* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QByteArray_isEmpty(const QList< QByteArray >* this_ptr) {
  return this_ptr->isEmpty();
}

QByteArray* qt_core_c_QList_QByteArray_last(QList< QByteArray >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QByteArray_lastIndexOf_t(const QList< QByteArray >* this_ptr, const QByteArray* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QByteArray_lastIndexOf_t_from(const QList< QByteArray >* this_ptr, const QByteArray* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QByteArray* qt_core_c_QList_QByteArray_last_const(const QList< QByteArray >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QByteArray_length(const QList< QByteArray >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QByteArray_mid_to_output_pos(const QList< QByteArray >* this_ptr, int pos, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(this_ptr->mid(pos));
}

void qt_core_c_QList_QByteArray_mid_to_output_pos_length(const QList< QByteArray >* this_ptr, int pos, int length, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QByteArray_move(QList< QByteArray >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QByteArray >* qt_core_c_QList_QByteArray_operator_add_assign_l(QList< QByteArray >* this_ptr, const QList< QByteArray >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QByteArray >* qt_core_c_QList_QByteArray_operator_add_assign_t(QList< QByteArray >* this_ptr, const QByteArray* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QByteArray_operator_add_to_output(const QList< QByteArray >* this_ptr, const QList< QByteArray >* l, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(this_ptr->operator+(*l));
}

QList< QByteArray >* qt_core_c_QList_QByteArray_operator_assign(QList< QByteArray >* this_ptr, const QList< QByteArray >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QByteArray_operator_eq(const QList< QByteArray >* this_ptr, const QList< QByteArray >* l) {
  return this_ptr->operator==(*l);
}

QByteArray* qt_core_c_QList_QByteArray_operator_index(QList< QByteArray >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QByteArray* qt_core_c_QList_QByteArray_operator_index_const(const QList< QByteArray >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QByteArray_operator_neq(const QList< QByteArray >* this_ptr, const QList< QByteArray >* l) {
  return this_ptr->operator!=(*l);
}

QList< QByteArray >* qt_core_c_QList_QByteArray_operator_shl_l(QList< QByteArray >* this_ptr, const QList< QByteArray >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QByteArray >* qt_core_c_QList_QByteArray_operator_shl_t(QList< QByteArray >* this_ptr, const QByteArray* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QByteArray_pop_back(QList< QByteArray >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QByteArray_pop_front(QList< QByteArray >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QByteArray_prepend(QList< QByteArray >* this_ptr, const QByteArray* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QByteArray_push_back(QList< QByteArray >* this_ptr, const QByteArray* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QByteArray_push_front(QList< QByteArray >* this_ptr, const QByteArray* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QByteArray_removeAll(QList< QByteArray >* this_ptr, const QByteArray* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QByteArray_removeAt(QList< QByteArray >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QByteArray_removeFirst(QList< QByteArray >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QByteArray_removeLast(QList< QByteArray >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QByteArray_removeOne(QList< QByteArray >* this_ptr, const QByteArray* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QByteArray_replace(QList< QByteArray >* this_ptr, int i, const QByteArray* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QByteArray_reserve(QList< QByteArray >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QByteArray_size(const QList< QByteArray >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QByteArray_startsWith(const QList< QByteArray >* this_ptr, const QByteArray* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QByteArray_swap_i_j(QList< QByteArray >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QByteArray_swap_other(QList< QByteArray >* this_ptr, QList< QByteArray >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QList_QByteArray_takeAt_to_output(QList< QByteArray >* this_ptr, int i, QByteArray* output) {
  new(output) QByteArray(this_ptr->takeAt(i));
}

void qt_core_c_QList_QByteArray_takeFirst_to_output(QList< QByteArray >* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->takeFirst());
}

void qt_core_c_QList_QByteArray_takeLast_to_output(QList< QByteArray >* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->takeLast());
}

void qt_core_c_QList_QByteArray_value_to_output_i(const QList< QByteArray >* this_ptr, int i, QByteArray* output) {
  new(output) QByteArray(this_ptr->value(i));
}

void qt_core_c_QList_QByteArray_value_to_output_i_defaultValue(const QList< QByteArray >* this_ptr, int i, const QByteArray* defaultValue, QByteArray* output) {
  new(output) QByteArray(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QList_QCommandLineOption_append_QCommandLineOption(QList< QCommandLineOption >* this_ptr, const QCommandLineOption* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QCommandLineOption_append_QList_QCommandLineOption(QList< QCommandLineOption >* this_ptr, const QList< QCommandLineOption >* t) {
  this_ptr->append(*t);
}

const QCommandLineOption* qt_core_c_QList_QCommandLineOption_at(const QList< QCommandLineOption >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QCommandLineOption* qt_core_c_QList_QCommandLineOption_back(QList< QCommandLineOption >* this_ptr) {
  return &this_ptr->back();
}

const QCommandLineOption* qt_core_c_QList_QCommandLineOption_back_const(const QList< QCommandLineOption >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QCommandLineOption_clear(QList< QCommandLineOption >* this_ptr) {
  this_ptr->clear();
}

const QCommandLineOption* qt_core_c_QList_QCommandLineOption_constFirst(const QList< QCommandLineOption >* this_ptr) {
  return &this_ptr->constFirst();
}

const QCommandLineOption* qt_core_c_QList_QCommandLineOption_constLast(const QList< QCommandLineOption >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QCommandLineOption_constructor_l(const QList< QCommandLineOption >* l, QList< QCommandLineOption >* output) {
  new(output) QList< QCommandLineOption >(*l);
}

void qt_core_c_QList_QCommandLineOption_constructor_no_args(QList< QCommandLineOption >* output) {
  new(output) QList< QCommandLineOption >();
}

int qt_core_c_QList_QCommandLineOption_count(const QList< QCommandLineOption >* this_ptr) {
  return this_ptr->count();
}

void qt_core_c_QList_QCommandLineOption_destructor(QList< QCommandLineOption >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QCommandLineOption_empty(const QList< QCommandLineOption >* this_ptr) {
  return this_ptr->empty();
}

QCommandLineOption* qt_core_c_QList_QCommandLineOption_first(QList< QCommandLineOption >* this_ptr) {
  return &this_ptr->first();
}

const QCommandLineOption* qt_core_c_QList_QCommandLineOption_first_const(const QList< QCommandLineOption >* this_ptr) {
  return &this_ptr->first();
}

QCommandLineOption* qt_core_c_QList_QCommandLineOption_front(QList< QCommandLineOption >* this_ptr) {
  return &this_ptr->front();
}

const QCommandLineOption* qt_core_c_QList_QCommandLineOption_front_const(const QList< QCommandLineOption >* this_ptr) {
  return &this_ptr->front();
}

void qt_core_c_QList_QCommandLineOption_insert(QList< QCommandLineOption >* this_ptr, int i, const QCommandLineOption* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QCommandLineOption_isEmpty(const QList< QCommandLineOption >* this_ptr) {
  return this_ptr->isEmpty();
}

QCommandLineOption* qt_core_c_QList_QCommandLineOption_last(QList< QCommandLineOption >* this_ptr) {
  return &this_ptr->last();
}

const QCommandLineOption* qt_core_c_QList_QCommandLineOption_last_const(const QList< QCommandLineOption >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QCommandLineOption_length(const QList< QCommandLineOption >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QCommandLineOption_mid_to_output_pos(const QList< QCommandLineOption >* this_ptr, int pos, QList< QCommandLineOption >* output) {
  new(output) QList< QCommandLineOption >(this_ptr->mid(pos));
}

void qt_core_c_QList_QCommandLineOption_mid_to_output_pos_length(const QList< QCommandLineOption >* this_ptr, int pos, int length, QList< QCommandLineOption >* output) {
  new(output) QList< QCommandLineOption >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QCommandLineOption_move(QList< QCommandLineOption >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QCommandLineOption >* qt_core_c_QList_QCommandLineOption_operator_add_assign_l(QList< QCommandLineOption >* this_ptr, const QList< QCommandLineOption >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QCommandLineOption >* qt_core_c_QList_QCommandLineOption_operator_add_assign_t(QList< QCommandLineOption >* this_ptr, const QCommandLineOption* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QCommandLineOption_operator_add_to_output(const QList< QCommandLineOption >* this_ptr, const QList< QCommandLineOption >* l, QList< QCommandLineOption >* output) {
  new(output) QList< QCommandLineOption >(this_ptr->operator+(*l));
}

QList< QCommandLineOption >* qt_core_c_QList_QCommandLineOption_operator_assign(QList< QCommandLineOption >* this_ptr, const QList< QCommandLineOption >* l) {
  return &this_ptr->operator=(*l);
}

QCommandLineOption* qt_core_c_QList_QCommandLineOption_operator_index(QList< QCommandLineOption >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QCommandLineOption* qt_core_c_QList_QCommandLineOption_operator_index_const(const QList< QCommandLineOption >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QList< QCommandLineOption >* qt_core_c_QList_QCommandLineOption_operator_shl_l(QList< QCommandLineOption >* this_ptr, const QList< QCommandLineOption >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QCommandLineOption >* qt_core_c_QList_QCommandLineOption_operator_shl_t(QList< QCommandLineOption >* this_ptr, const QCommandLineOption* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QCommandLineOption_pop_back(QList< QCommandLineOption >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QCommandLineOption_pop_front(QList< QCommandLineOption >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QCommandLineOption_prepend(QList< QCommandLineOption >* this_ptr, const QCommandLineOption* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QCommandLineOption_push_back(QList< QCommandLineOption >* this_ptr, const QCommandLineOption* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QCommandLineOption_push_front(QList< QCommandLineOption >* this_ptr, const QCommandLineOption* t) {
  this_ptr->push_front(*t);
}

void qt_core_c_QList_QCommandLineOption_removeAt(QList< QCommandLineOption >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QCommandLineOption_removeFirst(QList< QCommandLineOption >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QCommandLineOption_removeLast(QList< QCommandLineOption >* this_ptr) {
  this_ptr->removeLast();
}

void qt_core_c_QList_QCommandLineOption_replace(QList< QCommandLineOption >* this_ptr, int i, const QCommandLineOption* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QCommandLineOption_reserve(QList< QCommandLineOption >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QCommandLineOption_size(const QList< QCommandLineOption >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QList_QCommandLineOption_swap_i_j(QList< QCommandLineOption >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QCommandLineOption_swap_other(QList< QCommandLineOption >* this_ptr, QList< QCommandLineOption >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QList_QCommandLineOption_takeAt_to_output(QList< QCommandLineOption >* this_ptr, int i, QCommandLineOption* output) {
  new(output) QCommandLineOption(this_ptr->takeAt(i));
}

void qt_core_c_QList_QCommandLineOption_takeFirst_to_output(QList< QCommandLineOption >* this_ptr, QCommandLineOption* output) {
  new(output) QCommandLineOption(this_ptr->takeFirst());
}

void qt_core_c_QList_QCommandLineOption_takeLast_to_output(QList< QCommandLineOption >* this_ptr, QCommandLineOption* output) {
  new(output) QCommandLineOption(this_ptr->takeLast());
}

void qt_core_c_QList_QFileInfo_append_QFileInfo(QList< QFileInfo >* this_ptr, const QFileInfo* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QFileInfo_append_QList_QFileInfo(QList< QFileInfo >* this_ptr, const QList< QFileInfo >* t) {
  this_ptr->append(*t);
}

const QFileInfo* qt_core_c_QList_QFileInfo_at(const QList< QFileInfo >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QFileInfo* qt_core_c_QList_QFileInfo_back(QList< QFileInfo >* this_ptr) {
  return &this_ptr->back();
}

const QFileInfo* qt_core_c_QList_QFileInfo_back_const(const QList< QFileInfo >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QFileInfo_clear(QList< QFileInfo >* this_ptr) {
  this_ptr->clear();
}

const QFileInfo* qt_core_c_QList_QFileInfo_constFirst(const QList< QFileInfo >* this_ptr) {
  return &this_ptr->constFirst();
}

const QFileInfo* qt_core_c_QList_QFileInfo_constLast(const QList< QFileInfo >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QFileInfo_constructor_l(const QList< QFileInfo >* l, QList< QFileInfo >* output) {
  new(output) QList< QFileInfo >(*l);
}

void qt_core_c_QList_QFileInfo_constructor_no_args(QList< QFileInfo >* output) {
  new(output) QList< QFileInfo >();
}

bool qt_core_c_QList_QFileInfo_contains(const QList< QFileInfo >* this_ptr, const QFileInfo* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QFileInfo_count_no_args(const QList< QFileInfo >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QFileInfo_count_t(const QList< QFileInfo >* this_ptr, const QFileInfo* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QFileInfo_destructor(QList< QFileInfo >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QFileInfo_empty(const QList< QFileInfo >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QFileInfo_endsWith(const QList< QFileInfo >* this_ptr, const QFileInfo* t) {
  return this_ptr->endsWith(*t);
}

QFileInfo* qt_core_c_QList_QFileInfo_first(QList< QFileInfo >* this_ptr) {
  return &this_ptr->first();
}

const QFileInfo* qt_core_c_QList_QFileInfo_first_const(const QList< QFileInfo >* this_ptr) {
  return &this_ptr->first();
}

QFileInfo* qt_core_c_QList_QFileInfo_front(QList< QFileInfo >* this_ptr) {
  return &this_ptr->front();
}

const QFileInfo* qt_core_c_QList_QFileInfo_front_const(const QList< QFileInfo >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QFileInfo_indexOf_t(const QList< QFileInfo >* this_ptr, const QFileInfo* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QFileInfo_indexOf_t_from(const QList< QFileInfo >* this_ptr, const QFileInfo* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QFileInfo_insert(QList< QFileInfo >* this_ptr, int i, const QFileInfo* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QFileInfo_isEmpty(const QList< QFileInfo >* this_ptr) {
  return this_ptr->isEmpty();
}

QFileInfo* qt_core_c_QList_QFileInfo_last(QList< QFileInfo >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QFileInfo_lastIndexOf_t(const QList< QFileInfo >* this_ptr, const QFileInfo* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QFileInfo_lastIndexOf_t_from(const QList< QFileInfo >* this_ptr, const QFileInfo* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QFileInfo* qt_core_c_QList_QFileInfo_last_const(const QList< QFileInfo >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QFileInfo_length(const QList< QFileInfo >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QFileInfo_mid_to_output_pos(const QList< QFileInfo >* this_ptr, int pos, QList< QFileInfo >* output) {
  new(output) QList< QFileInfo >(this_ptr->mid(pos));
}

void qt_core_c_QList_QFileInfo_mid_to_output_pos_length(const QList< QFileInfo >* this_ptr, int pos, int length, QList< QFileInfo >* output) {
  new(output) QList< QFileInfo >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QFileInfo_move(QList< QFileInfo >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QFileInfo >* qt_core_c_QList_QFileInfo_operator_add_assign_l(QList< QFileInfo >* this_ptr, const QList< QFileInfo >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QFileInfo >* qt_core_c_QList_QFileInfo_operator_add_assign_t(QList< QFileInfo >* this_ptr, const QFileInfo* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QFileInfo_operator_add_to_output(const QList< QFileInfo >* this_ptr, const QList< QFileInfo >* l, QList< QFileInfo >* output) {
  new(output) QList< QFileInfo >(this_ptr->operator+(*l));
}

QList< QFileInfo >* qt_core_c_QList_QFileInfo_operator_assign(QList< QFileInfo >* this_ptr, const QList< QFileInfo >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QFileInfo_operator_eq(const QList< QFileInfo >* this_ptr, const QList< QFileInfo >* l) {
  return this_ptr->operator==(*l);
}

QFileInfo* qt_core_c_QList_QFileInfo_operator_index(QList< QFileInfo >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QFileInfo* qt_core_c_QList_QFileInfo_operator_index_const(const QList< QFileInfo >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QFileInfo_operator_neq(const QList< QFileInfo >* this_ptr, const QList< QFileInfo >* l) {
  return this_ptr->operator!=(*l);
}

QList< QFileInfo >* qt_core_c_QList_QFileInfo_operator_shl_l(QList< QFileInfo >* this_ptr, const QList< QFileInfo >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QFileInfo >* qt_core_c_QList_QFileInfo_operator_shl_t(QList< QFileInfo >* this_ptr, const QFileInfo* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QFileInfo_pop_back(QList< QFileInfo >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QFileInfo_pop_front(QList< QFileInfo >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QFileInfo_prepend(QList< QFileInfo >* this_ptr, const QFileInfo* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QFileInfo_push_back(QList< QFileInfo >* this_ptr, const QFileInfo* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QFileInfo_push_front(QList< QFileInfo >* this_ptr, const QFileInfo* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QFileInfo_removeAll(QList< QFileInfo >* this_ptr, const QFileInfo* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QFileInfo_removeAt(QList< QFileInfo >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QFileInfo_removeFirst(QList< QFileInfo >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QFileInfo_removeLast(QList< QFileInfo >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QFileInfo_removeOne(QList< QFileInfo >* this_ptr, const QFileInfo* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QFileInfo_replace(QList< QFileInfo >* this_ptr, int i, const QFileInfo* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QFileInfo_reserve(QList< QFileInfo >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QFileInfo_size(const QList< QFileInfo >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QFileInfo_startsWith(const QList< QFileInfo >* this_ptr, const QFileInfo* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QFileInfo_swap_i_j(QList< QFileInfo >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QFileInfo_swap_other(QList< QFileInfo >* this_ptr, QList< QFileInfo >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QList_QFileInfo_takeAt_to_output(QList< QFileInfo >* this_ptr, int i, QFileInfo* output) {
  new(output) QFileInfo(this_ptr->takeAt(i));
}

void qt_core_c_QList_QFileInfo_takeFirst_to_output(QList< QFileInfo >* this_ptr, QFileInfo* output) {
  new(output) QFileInfo(this_ptr->takeFirst());
}

void qt_core_c_QList_QFileInfo_takeLast_to_output(QList< QFileInfo >* this_ptr, QFileInfo* output) {
  new(output) QFileInfo(this_ptr->takeLast());
}

void qt_core_c_QList_QFileInfo_value_to_output_i(const QList< QFileInfo >* this_ptr, int i, QFileInfo* output) {
  new(output) QFileInfo(this_ptr->value(i));
}

void qt_core_c_QList_QFileInfo_value_to_output_i_defaultValue(const QList< QFileInfo >* this_ptr, int i, const QFileInfo* defaultValue, QFileInfo* output) {
  new(output) QFileInfo(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QList_QItemSelectionRange_append_QItemSelectionRange(QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QItemSelectionRange_append_QList_QItemSelectionRange(QList< QItemSelectionRange >* this_ptr, const QList< QItemSelectionRange >* t) {
  this_ptr->append(*t);
}

const QItemSelectionRange* qt_core_c_QList_QItemSelectionRange_at(const QList< QItemSelectionRange >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QItemSelectionRange* qt_core_c_QList_QItemSelectionRange_back(QList< QItemSelectionRange >* this_ptr) {
  return &this_ptr->back();
}

const QItemSelectionRange* qt_core_c_QList_QItemSelectionRange_back_const(const QList< QItemSelectionRange >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QItemSelectionRange_clear(QList< QItemSelectionRange >* this_ptr) {
  this_ptr->clear();
}

const QItemSelectionRange* qt_core_c_QList_QItemSelectionRange_constFirst(const QList< QItemSelectionRange >* this_ptr) {
  return &this_ptr->constFirst();
}

const QItemSelectionRange* qt_core_c_QList_QItemSelectionRange_constLast(const QList< QItemSelectionRange >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QItemSelectionRange_constructor_l(const QList< QItemSelectionRange >* l, QList< QItemSelectionRange >* output) {
  new(output) QList< QItemSelectionRange >(*l);
}

void qt_core_c_QList_QItemSelectionRange_constructor_no_args(QList< QItemSelectionRange >* output) {
  new(output) QList< QItemSelectionRange >();
}

bool qt_core_c_QList_QItemSelectionRange_contains(const QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QItemSelectionRange_count_no_args(const QList< QItemSelectionRange >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QItemSelectionRange_count_t(const QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QItemSelectionRange_destructor(QList< QItemSelectionRange >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QItemSelectionRange_empty(const QList< QItemSelectionRange >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QItemSelectionRange_endsWith(const QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t) {
  return this_ptr->endsWith(*t);
}

QItemSelectionRange* qt_core_c_QList_QItemSelectionRange_first(QList< QItemSelectionRange >* this_ptr) {
  return &this_ptr->first();
}

const QItemSelectionRange* qt_core_c_QList_QItemSelectionRange_first_const(const QList< QItemSelectionRange >* this_ptr) {
  return &this_ptr->first();
}

QItemSelectionRange* qt_core_c_QList_QItemSelectionRange_front(QList< QItemSelectionRange >* this_ptr) {
  return &this_ptr->front();
}

const QItemSelectionRange* qt_core_c_QList_QItemSelectionRange_front_const(const QList< QItemSelectionRange >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QItemSelectionRange_indexOf_t(const QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QItemSelectionRange_indexOf_t_from(const QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QItemSelectionRange_insert(QList< QItemSelectionRange >* this_ptr, int i, const QItemSelectionRange* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QItemSelectionRange_isEmpty(const QList< QItemSelectionRange >* this_ptr) {
  return this_ptr->isEmpty();
}

QItemSelectionRange* qt_core_c_QList_QItemSelectionRange_last(QList< QItemSelectionRange >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QItemSelectionRange_lastIndexOf_t(const QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QItemSelectionRange_lastIndexOf_t_from(const QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QItemSelectionRange* qt_core_c_QList_QItemSelectionRange_last_const(const QList< QItemSelectionRange >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QItemSelectionRange_length(const QList< QItemSelectionRange >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QItemSelectionRange_mid_to_output_pos(const QList< QItemSelectionRange >* this_ptr, int pos, QList< QItemSelectionRange >* output) {
  new(output) QList< QItemSelectionRange >(this_ptr->mid(pos));
}

void qt_core_c_QList_QItemSelectionRange_mid_to_output_pos_length(const QList< QItemSelectionRange >* this_ptr, int pos, int length, QList< QItemSelectionRange >* output) {
  new(output) QList< QItemSelectionRange >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QItemSelectionRange_move(QList< QItemSelectionRange >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QItemSelectionRange >* qt_core_c_QList_QItemSelectionRange_operator_add_assign_l(QList< QItemSelectionRange >* this_ptr, const QList< QItemSelectionRange >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QItemSelectionRange >* qt_core_c_QList_QItemSelectionRange_operator_add_assign_t(QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QItemSelectionRange_operator_add_to_output(const QList< QItemSelectionRange >* this_ptr, const QList< QItemSelectionRange >* l, QList< QItemSelectionRange >* output) {
  new(output) QList< QItemSelectionRange >(this_ptr->operator+(*l));
}

QList< QItemSelectionRange >* qt_core_c_QList_QItemSelectionRange_operator_assign(QList< QItemSelectionRange >* this_ptr, const QList< QItemSelectionRange >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QItemSelectionRange_operator_eq(const QList< QItemSelectionRange >* this_ptr, const QList< QItemSelectionRange >* l) {
  return this_ptr->operator==(*l);
}

QItemSelectionRange* qt_core_c_QList_QItemSelectionRange_operator_index(QList< QItemSelectionRange >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QItemSelectionRange* qt_core_c_QList_QItemSelectionRange_operator_index_const(const QList< QItemSelectionRange >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QItemSelectionRange_operator_neq(const QList< QItemSelectionRange >* this_ptr, const QList< QItemSelectionRange >* l) {
  return this_ptr->operator!=(*l);
}

QList< QItemSelectionRange >* qt_core_c_QList_QItemSelectionRange_operator_shl_l(QList< QItemSelectionRange >* this_ptr, const QList< QItemSelectionRange >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QItemSelectionRange >* qt_core_c_QList_QItemSelectionRange_operator_shl_t(QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QItemSelectionRange_pop_back(QList< QItemSelectionRange >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QItemSelectionRange_pop_front(QList< QItemSelectionRange >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QItemSelectionRange_prepend(QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QItemSelectionRange_push_back(QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QItemSelectionRange_push_front(QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QItemSelectionRange_removeAll(QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QItemSelectionRange_removeAt(QList< QItemSelectionRange >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QItemSelectionRange_removeFirst(QList< QItemSelectionRange >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QItemSelectionRange_removeLast(QList< QItemSelectionRange >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QItemSelectionRange_removeOne(QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QItemSelectionRange_replace(QList< QItemSelectionRange >* this_ptr, int i, const QItemSelectionRange* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QItemSelectionRange_reserve(QList< QItemSelectionRange >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QItemSelectionRange_size(const QList< QItemSelectionRange >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QItemSelectionRange_startsWith(const QList< QItemSelectionRange >* this_ptr, const QItemSelectionRange* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QItemSelectionRange_swap_i_j(QList< QItemSelectionRange >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QItemSelectionRange_swap_other(QList< QItemSelectionRange >* this_ptr, QList< QItemSelectionRange >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QList_QItemSelectionRange_takeAt_to_output(QList< QItemSelectionRange >* this_ptr, int i, QItemSelectionRange* output) {
  new(output) QItemSelectionRange(this_ptr->takeAt(i));
}

void qt_core_c_QList_QItemSelectionRange_takeFirst_to_output(QList< QItemSelectionRange >* this_ptr, QItemSelectionRange* output) {
  new(output) QItemSelectionRange(this_ptr->takeFirst());
}

void qt_core_c_QList_QItemSelectionRange_takeLast_to_output(QList< QItemSelectionRange >* this_ptr, QItemSelectionRange* output) {
  new(output) QItemSelectionRange(this_ptr->takeLast());
}

void qt_core_c_QList_QItemSelectionRange_value_to_output_i(const QList< QItemSelectionRange >* this_ptr, int i, QItemSelectionRange* output) {
  new(output) QItemSelectionRange(this_ptr->value(i));
}

void qt_core_c_QList_QItemSelectionRange_value_to_output_i_defaultValue(const QList< QItemSelectionRange >* this_ptr, int i, const QItemSelectionRange* defaultValue, QItemSelectionRange* output) {
  new(output) QItemSelectionRange(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QList_QLocale_Country_append_QList_QLocale_Country(QList< QLocale::Country >* this_ptr, const QList< QLocale::Country >* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QLocale_Country_append_QLocale_Country(QList< QLocale::Country >* this_ptr, const QLocale::Country* t) {
  this_ptr->append(*t);
}

const QLocale::Country* qt_core_c_QList_QLocale_Country_at(const QList< QLocale::Country >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QLocale::Country* qt_core_c_QList_QLocale_Country_back(QList< QLocale::Country >* this_ptr) {
  return &this_ptr->back();
}

const QLocale::Country* qt_core_c_QList_QLocale_Country_back_const(const QList< QLocale::Country >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QLocale_Country_clear(QList< QLocale::Country >* this_ptr) {
  this_ptr->clear();
}

const QLocale::Country* qt_core_c_QList_QLocale_Country_constFirst(const QList< QLocale::Country >* this_ptr) {
  return &this_ptr->constFirst();
}

const QLocale::Country* qt_core_c_QList_QLocale_Country_constLast(const QList< QLocale::Country >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QLocale_Country_constructor_l(const QList< QLocale::Country >* l, QList< QLocale::Country >* output) {
  new(output) QList< QLocale::Country >(*l);
}

void qt_core_c_QList_QLocale_Country_constructor_no_args(QList< QLocale::Country >* output) {
  new(output) QList< QLocale::Country >();
}

bool qt_core_c_QList_QLocale_Country_contains(const QList< QLocale::Country >* this_ptr, const QLocale::Country* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QLocale_Country_count_no_args(const QList< QLocale::Country >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QLocale_Country_count_t(const QList< QLocale::Country >* this_ptr, const QLocale::Country* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QLocale_Country_destructor(QList< QLocale::Country >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QLocale_Country_empty(const QList< QLocale::Country >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QLocale_Country_endsWith(const QList< QLocale::Country >* this_ptr, const QLocale::Country* t) {
  return this_ptr->endsWith(*t);
}

QLocale::Country* qt_core_c_QList_QLocale_Country_first(QList< QLocale::Country >* this_ptr) {
  return &this_ptr->first();
}

const QLocale::Country* qt_core_c_QList_QLocale_Country_first_const(const QList< QLocale::Country >* this_ptr) {
  return &this_ptr->first();
}

QLocale::Country* qt_core_c_QList_QLocale_Country_front(QList< QLocale::Country >* this_ptr) {
  return &this_ptr->front();
}

const QLocale::Country* qt_core_c_QList_QLocale_Country_front_const(const QList< QLocale::Country >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QLocale_Country_indexOf_t(const QList< QLocale::Country >* this_ptr, const QLocale::Country* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QLocale_Country_indexOf_t_from(const QList< QLocale::Country >* this_ptr, const QLocale::Country* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QLocale_Country_insert(QList< QLocale::Country >* this_ptr, int i, const QLocale::Country* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QLocale_Country_isEmpty(const QList< QLocale::Country >* this_ptr) {
  return this_ptr->isEmpty();
}

QLocale::Country* qt_core_c_QList_QLocale_Country_last(QList< QLocale::Country >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QLocale_Country_lastIndexOf_t(const QList< QLocale::Country >* this_ptr, const QLocale::Country* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QLocale_Country_lastIndexOf_t_from(const QList< QLocale::Country >* this_ptr, const QLocale::Country* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QLocale::Country* qt_core_c_QList_QLocale_Country_last_const(const QList< QLocale::Country >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QLocale_Country_length(const QList< QLocale::Country >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QLocale_Country_mid_to_output_pos(const QList< QLocale::Country >* this_ptr, int pos, QList< QLocale::Country >* output) {
  new(output) QList< QLocale::Country >(this_ptr->mid(pos));
}

void qt_core_c_QList_QLocale_Country_mid_to_output_pos_length(const QList< QLocale::Country >* this_ptr, int pos, int length, QList< QLocale::Country >* output) {
  new(output) QList< QLocale::Country >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QLocale_Country_move(QList< QLocale::Country >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QLocale::Country >* qt_core_c_QList_QLocale_Country_operator_add_assign_l(QList< QLocale::Country >* this_ptr, const QList< QLocale::Country >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QLocale::Country >* qt_core_c_QList_QLocale_Country_operator_add_assign_t(QList< QLocale::Country >* this_ptr, const QLocale::Country* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QLocale_Country_operator_add_to_output(const QList< QLocale::Country >* this_ptr, const QList< QLocale::Country >* l, QList< QLocale::Country >* output) {
  new(output) QList< QLocale::Country >(this_ptr->operator+(*l));
}

QList< QLocale::Country >* qt_core_c_QList_QLocale_Country_operator_assign(QList< QLocale::Country >* this_ptr, const QList< QLocale::Country >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QLocale_Country_operator_eq(const QList< QLocale::Country >* this_ptr, const QList< QLocale::Country >* l) {
  return this_ptr->operator==(*l);
}

QLocale::Country* qt_core_c_QList_QLocale_Country_operator_index(QList< QLocale::Country >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QLocale::Country* qt_core_c_QList_QLocale_Country_operator_index_const(const QList< QLocale::Country >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QLocale_Country_operator_neq(const QList< QLocale::Country >* this_ptr, const QList< QLocale::Country >* l) {
  return this_ptr->operator!=(*l);
}

QList< QLocale::Country >* qt_core_c_QList_QLocale_Country_operator_shl_l(QList< QLocale::Country >* this_ptr, const QList< QLocale::Country >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QLocale::Country >* qt_core_c_QList_QLocale_Country_operator_shl_t(QList< QLocale::Country >* this_ptr, const QLocale::Country* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QLocale_Country_pop_back(QList< QLocale::Country >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QLocale_Country_pop_front(QList< QLocale::Country >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QLocale_Country_prepend(QList< QLocale::Country >* this_ptr, const QLocale::Country* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QLocale_Country_push_back(QList< QLocale::Country >* this_ptr, const QLocale::Country* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QLocale_Country_push_front(QList< QLocale::Country >* this_ptr, const QLocale::Country* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QLocale_Country_removeAll(QList< QLocale::Country >* this_ptr, const QLocale::Country* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QLocale_Country_removeAt(QList< QLocale::Country >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QLocale_Country_removeFirst(QList< QLocale::Country >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QLocale_Country_removeLast(QList< QLocale::Country >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QLocale_Country_removeOne(QList< QLocale::Country >* this_ptr, const QLocale::Country* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QLocale_Country_replace(QList< QLocale::Country >* this_ptr, int i, const QLocale::Country* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QLocale_Country_reserve(QList< QLocale::Country >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QLocale_Country_size(const QList< QLocale::Country >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QLocale_Country_startsWith(const QList< QLocale::Country >* this_ptr, const QLocale::Country* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QLocale_Country_swap_i_j(QList< QLocale::Country >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QLocale_Country_swap_other(QList< QLocale::Country >* this_ptr, QList< QLocale::Country >* other) {
  this_ptr->swap(*other);
}

QLocale::Country qt_core_c_QList_QLocale_Country_takeAt(QList< QLocale::Country >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QLocale::Country qt_core_c_QList_QLocale_Country_takeFirst(QList< QLocale::Country >* this_ptr) {
  return this_ptr->takeFirst();
}

QLocale::Country qt_core_c_QList_QLocale_Country_takeLast(QList< QLocale::Country >* this_ptr) {
  return this_ptr->takeLast();
}

QLocale::Country qt_core_c_QList_QLocale_Country_value_i(const QList< QLocale::Country >* this_ptr, int i) {
  return this_ptr->value(i);
}

QLocale::Country qt_core_c_QList_QLocale_Country_value_i_defaultValue(const QList< QLocale::Country >* this_ptr, int i, const QLocale::Country* defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_core_c_QList_QLocale_append_QList_QLocale(QList< QLocale >* this_ptr, const QList< QLocale >* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QLocale_append_QLocale(QList< QLocale >* this_ptr, const QLocale* t) {
  this_ptr->append(*t);
}

const QLocale* qt_core_c_QList_QLocale_at(const QList< QLocale >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QLocale* qt_core_c_QList_QLocale_back(QList< QLocale >* this_ptr) {
  return &this_ptr->back();
}

const QLocale* qt_core_c_QList_QLocale_back_const(const QList< QLocale >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QLocale_clear(QList< QLocale >* this_ptr) {
  this_ptr->clear();
}

const QLocale* qt_core_c_QList_QLocale_constFirst(const QList< QLocale >* this_ptr) {
  return &this_ptr->constFirst();
}

const QLocale* qt_core_c_QList_QLocale_constLast(const QList< QLocale >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QLocale_constructor_l(const QList< QLocale >* l, QList< QLocale >* output) {
  new(output) QList< QLocale >(*l);
}

void qt_core_c_QList_QLocale_constructor_no_args(QList< QLocale >* output) {
  new(output) QList< QLocale >();
}

bool qt_core_c_QList_QLocale_contains(const QList< QLocale >* this_ptr, const QLocale* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QLocale_count_no_args(const QList< QLocale >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QLocale_count_t(const QList< QLocale >* this_ptr, const QLocale* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QLocale_destructor(QList< QLocale >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QLocale_empty(const QList< QLocale >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QLocale_endsWith(const QList< QLocale >* this_ptr, const QLocale* t) {
  return this_ptr->endsWith(*t);
}

QLocale* qt_core_c_QList_QLocale_first(QList< QLocale >* this_ptr) {
  return &this_ptr->first();
}

const QLocale* qt_core_c_QList_QLocale_first_const(const QList< QLocale >* this_ptr) {
  return &this_ptr->first();
}

QLocale* qt_core_c_QList_QLocale_front(QList< QLocale >* this_ptr) {
  return &this_ptr->front();
}

const QLocale* qt_core_c_QList_QLocale_front_const(const QList< QLocale >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QLocale_indexOf_t(const QList< QLocale >* this_ptr, const QLocale* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QLocale_indexOf_t_from(const QList< QLocale >* this_ptr, const QLocale* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QLocale_insert(QList< QLocale >* this_ptr, int i, const QLocale* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QLocale_isEmpty(const QList< QLocale >* this_ptr) {
  return this_ptr->isEmpty();
}

QLocale* qt_core_c_QList_QLocale_last(QList< QLocale >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QLocale_lastIndexOf_t(const QList< QLocale >* this_ptr, const QLocale* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QLocale_lastIndexOf_t_from(const QList< QLocale >* this_ptr, const QLocale* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QLocale* qt_core_c_QList_QLocale_last_const(const QList< QLocale >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QLocale_length(const QList< QLocale >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QLocale_mid_to_output_pos(const QList< QLocale >* this_ptr, int pos, QList< QLocale >* output) {
  new(output) QList< QLocale >(this_ptr->mid(pos));
}

void qt_core_c_QList_QLocale_mid_to_output_pos_length(const QList< QLocale >* this_ptr, int pos, int length, QList< QLocale >* output) {
  new(output) QList< QLocale >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QLocale_move(QList< QLocale >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QLocale >* qt_core_c_QList_QLocale_operator_add_assign_l(QList< QLocale >* this_ptr, const QList< QLocale >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QLocale >* qt_core_c_QList_QLocale_operator_add_assign_t(QList< QLocale >* this_ptr, const QLocale* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QLocale_operator_add_to_output(const QList< QLocale >* this_ptr, const QList< QLocale >* l, QList< QLocale >* output) {
  new(output) QList< QLocale >(this_ptr->operator+(*l));
}

QList< QLocale >* qt_core_c_QList_QLocale_operator_assign(QList< QLocale >* this_ptr, const QList< QLocale >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QLocale_operator_eq(const QList< QLocale >* this_ptr, const QList< QLocale >* l) {
  return this_ptr->operator==(*l);
}

QLocale* qt_core_c_QList_QLocale_operator_index(QList< QLocale >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QLocale* qt_core_c_QList_QLocale_operator_index_const(const QList< QLocale >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QLocale_operator_neq(const QList< QLocale >* this_ptr, const QList< QLocale >* l) {
  return this_ptr->operator!=(*l);
}

QList< QLocale >* qt_core_c_QList_QLocale_operator_shl_l(QList< QLocale >* this_ptr, const QList< QLocale >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QLocale >* qt_core_c_QList_QLocale_operator_shl_t(QList< QLocale >* this_ptr, const QLocale* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QLocale_pop_back(QList< QLocale >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QLocale_pop_front(QList< QLocale >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QLocale_prepend(QList< QLocale >* this_ptr, const QLocale* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QLocale_push_back(QList< QLocale >* this_ptr, const QLocale* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QLocale_push_front(QList< QLocale >* this_ptr, const QLocale* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QLocale_removeAll(QList< QLocale >* this_ptr, const QLocale* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QLocale_removeAt(QList< QLocale >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QLocale_removeFirst(QList< QLocale >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QLocale_removeLast(QList< QLocale >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QLocale_removeOne(QList< QLocale >* this_ptr, const QLocale* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QLocale_replace(QList< QLocale >* this_ptr, int i, const QLocale* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QLocale_reserve(QList< QLocale >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QLocale_size(const QList< QLocale >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QLocale_startsWith(const QList< QLocale >* this_ptr, const QLocale* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QLocale_swap_i_j(QList< QLocale >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QLocale_swap_other(QList< QLocale >* this_ptr, QList< QLocale >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QList_QLocale_takeAt_to_output(QList< QLocale >* this_ptr, int i, QLocale* output) {
  new(output) QLocale(this_ptr->takeAt(i));
}

void qt_core_c_QList_QLocale_takeFirst_to_output(QList< QLocale >* this_ptr, QLocale* output) {
  new(output) QLocale(this_ptr->takeFirst());
}

void qt_core_c_QList_QLocale_takeLast_to_output(QList< QLocale >* this_ptr, QLocale* output) {
  new(output) QLocale(this_ptr->takeLast());
}

void qt_core_c_QList_QLocale_value_to_output_i(const QList< QLocale >* this_ptr, int i, QLocale* output) {
  new(output) QLocale(this_ptr->value(i));
}

void qt_core_c_QList_QLocale_value_to_output_i_defaultValue(const QList< QLocale >* this_ptr, int i, const QLocale* defaultValue, QLocale* output) {
  new(output) QLocale(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QList_QMimeType_append_QList_QMimeType(QList< QMimeType >* this_ptr, const QList< QMimeType >* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QMimeType_append_QMimeType(QList< QMimeType >* this_ptr, const QMimeType* t) {
  this_ptr->append(*t);
}

const QMimeType* qt_core_c_QList_QMimeType_at(const QList< QMimeType >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QMimeType* qt_core_c_QList_QMimeType_back(QList< QMimeType >* this_ptr) {
  return &this_ptr->back();
}

const QMimeType* qt_core_c_QList_QMimeType_back_const(const QList< QMimeType >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QMimeType_clear(QList< QMimeType >* this_ptr) {
  this_ptr->clear();
}

const QMimeType* qt_core_c_QList_QMimeType_constFirst(const QList< QMimeType >* this_ptr) {
  return &this_ptr->constFirst();
}

const QMimeType* qt_core_c_QList_QMimeType_constLast(const QList< QMimeType >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QMimeType_constructor_l(const QList< QMimeType >* l, QList< QMimeType >* output) {
  new(output) QList< QMimeType >(*l);
}

void qt_core_c_QList_QMimeType_constructor_no_args(QList< QMimeType >* output) {
  new(output) QList< QMimeType >();
}

bool qt_core_c_QList_QMimeType_contains(const QList< QMimeType >* this_ptr, const QMimeType* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QMimeType_count_no_args(const QList< QMimeType >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QMimeType_count_t(const QList< QMimeType >* this_ptr, const QMimeType* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QMimeType_destructor(QList< QMimeType >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QMimeType_empty(const QList< QMimeType >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QMimeType_endsWith(const QList< QMimeType >* this_ptr, const QMimeType* t) {
  return this_ptr->endsWith(*t);
}

QMimeType* qt_core_c_QList_QMimeType_first(QList< QMimeType >* this_ptr) {
  return &this_ptr->first();
}

const QMimeType* qt_core_c_QList_QMimeType_first_const(const QList< QMimeType >* this_ptr) {
  return &this_ptr->first();
}

QMimeType* qt_core_c_QList_QMimeType_front(QList< QMimeType >* this_ptr) {
  return &this_ptr->front();
}

const QMimeType* qt_core_c_QList_QMimeType_front_const(const QList< QMimeType >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QMimeType_indexOf_t(const QList< QMimeType >* this_ptr, const QMimeType* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QMimeType_indexOf_t_from(const QList< QMimeType >* this_ptr, const QMimeType* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QMimeType_insert(QList< QMimeType >* this_ptr, int i, const QMimeType* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QMimeType_isEmpty(const QList< QMimeType >* this_ptr) {
  return this_ptr->isEmpty();
}

QMimeType* qt_core_c_QList_QMimeType_last(QList< QMimeType >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QMimeType_lastIndexOf_t(const QList< QMimeType >* this_ptr, const QMimeType* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QMimeType_lastIndexOf_t_from(const QList< QMimeType >* this_ptr, const QMimeType* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QMimeType* qt_core_c_QList_QMimeType_last_const(const QList< QMimeType >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QMimeType_length(const QList< QMimeType >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QMimeType_mid_to_output_pos(const QList< QMimeType >* this_ptr, int pos, QList< QMimeType >* output) {
  new(output) QList< QMimeType >(this_ptr->mid(pos));
}

void qt_core_c_QList_QMimeType_mid_to_output_pos_length(const QList< QMimeType >* this_ptr, int pos, int length, QList< QMimeType >* output) {
  new(output) QList< QMimeType >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QMimeType_move(QList< QMimeType >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QMimeType >* qt_core_c_QList_QMimeType_operator_add_assign_l(QList< QMimeType >* this_ptr, const QList< QMimeType >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QMimeType >* qt_core_c_QList_QMimeType_operator_add_assign_t(QList< QMimeType >* this_ptr, const QMimeType* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QMimeType_operator_add_to_output(const QList< QMimeType >* this_ptr, const QList< QMimeType >* l, QList< QMimeType >* output) {
  new(output) QList< QMimeType >(this_ptr->operator+(*l));
}

QList< QMimeType >* qt_core_c_QList_QMimeType_operator_assign(QList< QMimeType >* this_ptr, const QList< QMimeType >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QMimeType_operator_eq(const QList< QMimeType >* this_ptr, const QList< QMimeType >* l) {
  return this_ptr->operator==(*l);
}

QMimeType* qt_core_c_QList_QMimeType_operator_index(QList< QMimeType >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QMimeType* qt_core_c_QList_QMimeType_operator_index_const(const QList< QMimeType >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QMimeType_operator_neq(const QList< QMimeType >* this_ptr, const QList< QMimeType >* l) {
  return this_ptr->operator!=(*l);
}

QList< QMimeType >* qt_core_c_QList_QMimeType_operator_shl_l(QList< QMimeType >* this_ptr, const QList< QMimeType >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QMimeType >* qt_core_c_QList_QMimeType_operator_shl_t(QList< QMimeType >* this_ptr, const QMimeType* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QMimeType_pop_back(QList< QMimeType >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QMimeType_pop_front(QList< QMimeType >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QMimeType_prepend(QList< QMimeType >* this_ptr, const QMimeType* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QMimeType_push_back(QList< QMimeType >* this_ptr, const QMimeType* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QMimeType_push_front(QList< QMimeType >* this_ptr, const QMimeType* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QMimeType_removeAll(QList< QMimeType >* this_ptr, const QMimeType* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QMimeType_removeAt(QList< QMimeType >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QMimeType_removeFirst(QList< QMimeType >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QMimeType_removeLast(QList< QMimeType >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QMimeType_removeOne(QList< QMimeType >* this_ptr, const QMimeType* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QMimeType_replace(QList< QMimeType >* this_ptr, int i, const QMimeType* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QMimeType_reserve(QList< QMimeType >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QMimeType_size(const QList< QMimeType >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QMimeType_startsWith(const QList< QMimeType >* this_ptr, const QMimeType* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QMimeType_swap_i_j(QList< QMimeType >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QMimeType_swap_other(QList< QMimeType >* this_ptr, QList< QMimeType >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QList_QMimeType_takeAt_to_output(QList< QMimeType >* this_ptr, int i, QMimeType* output) {
  new(output) QMimeType(this_ptr->takeAt(i));
}

void qt_core_c_QList_QMimeType_takeFirst_to_output(QList< QMimeType >* this_ptr, QMimeType* output) {
  new(output) QMimeType(this_ptr->takeFirst());
}

void qt_core_c_QList_QMimeType_takeLast_to_output(QList< QMimeType >* this_ptr, QMimeType* output) {
  new(output) QMimeType(this_ptr->takeLast());
}

void qt_core_c_QList_QMimeType_value_to_output_i(const QList< QMimeType >* this_ptr, int i, QMimeType* output) {
  new(output) QMimeType(this_ptr->value(i));
}

void qt_core_c_QList_QMimeType_value_to_output_i_defaultValue(const QList< QMimeType >* this_ptr, int i, const QMimeType* defaultValue, QMimeType* output) {
  new(output) QMimeType(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QList_QModelIndex_append_QList_QModelIndex(QList< QModelIndex >* this_ptr, const QList< QModelIndex >* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QModelIndex_append_QModelIndex(QList< QModelIndex >* this_ptr, const QModelIndex* t) {
  this_ptr->append(*t);
}

const QModelIndex* qt_core_c_QList_QModelIndex_at(const QList< QModelIndex >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QModelIndex* qt_core_c_QList_QModelIndex_back(QList< QModelIndex >* this_ptr) {
  return &this_ptr->back();
}

const QModelIndex* qt_core_c_QList_QModelIndex_back_const(const QList< QModelIndex >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QModelIndex_clear(QList< QModelIndex >* this_ptr) {
  this_ptr->clear();
}

const QModelIndex* qt_core_c_QList_QModelIndex_constFirst(const QList< QModelIndex >* this_ptr) {
  return &this_ptr->constFirst();
}

const QModelIndex* qt_core_c_QList_QModelIndex_constLast(const QList< QModelIndex >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QModelIndex_constructor_l(const QList< QModelIndex >* l, QList< QModelIndex >* output) {
  new(output) QList< QModelIndex >(*l);
}

void qt_core_c_QList_QModelIndex_constructor_no_args(QList< QModelIndex >* output) {
  new(output) QList< QModelIndex >();
}

bool qt_core_c_QList_QModelIndex_contains(const QList< QModelIndex >* this_ptr, const QModelIndex* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QModelIndex_count_no_args(const QList< QModelIndex >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QModelIndex_count_t(const QList< QModelIndex >* this_ptr, const QModelIndex* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QModelIndex_destructor(QList< QModelIndex >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QModelIndex_empty(const QList< QModelIndex >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QModelIndex_endsWith(const QList< QModelIndex >* this_ptr, const QModelIndex* t) {
  return this_ptr->endsWith(*t);
}

QModelIndex* qt_core_c_QList_QModelIndex_first(QList< QModelIndex >* this_ptr) {
  return &this_ptr->first();
}

const QModelIndex* qt_core_c_QList_QModelIndex_first_const(const QList< QModelIndex >* this_ptr) {
  return &this_ptr->first();
}

QModelIndex* qt_core_c_QList_QModelIndex_front(QList< QModelIndex >* this_ptr) {
  return &this_ptr->front();
}

const QModelIndex* qt_core_c_QList_QModelIndex_front_const(const QList< QModelIndex >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QModelIndex_indexOf_t(const QList< QModelIndex >* this_ptr, const QModelIndex* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QModelIndex_indexOf_t_from(const QList< QModelIndex >* this_ptr, const QModelIndex* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QModelIndex_insert(QList< QModelIndex >* this_ptr, int i, const QModelIndex* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QModelIndex_isEmpty(const QList< QModelIndex >* this_ptr) {
  return this_ptr->isEmpty();
}

QModelIndex* qt_core_c_QList_QModelIndex_last(QList< QModelIndex >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QModelIndex_lastIndexOf_t(const QList< QModelIndex >* this_ptr, const QModelIndex* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QModelIndex_lastIndexOf_t_from(const QList< QModelIndex >* this_ptr, const QModelIndex* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QModelIndex* qt_core_c_QList_QModelIndex_last_const(const QList< QModelIndex >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QModelIndex_length(const QList< QModelIndex >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QModelIndex_mid_to_output_pos(const QList< QModelIndex >* this_ptr, int pos, QList< QModelIndex >* output) {
  new(output) QList< QModelIndex >(this_ptr->mid(pos));
}

void qt_core_c_QList_QModelIndex_mid_to_output_pos_length(const QList< QModelIndex >* this_ptr, int pos, int length, QList< QModelIndex >* output) {
  new(output) QList< QModelIndex >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QModelIndex_move(QList< QModelIndex >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QModelIndex >* qt_core_c_QList_QModelIndex_operator_add_assign_l(QList< QModelIndex >* this_ptr, const QList< QModelIndex >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QModelIndex >* qt_core_c_QList_QModelIndex_operator_add_assign_t(QList< QModelIndex >* this_ptr, const QModelIndex* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QModelIndex_operator_add_to_output(const QList< QModelIndex >* this_ptr, const QList< QModelIndex >* l, QList< QModelIndex >* output) {
  new(output) QList< QModelIndex >(this_ptr->operator+(*l));
}

QList< QModelIndex >* qt_core_c_QList_QModelIndex_operator_assign(QList< QModelIndex >* this_ptr, const QList< QModelIndex >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QModelIndex_operator_eq(const QList< QModelIndex >* this_ptr, const QList< QModelIndex >* l) {
  return this_ptr->operator==(*l);
}

QModelIndex* qt_core_c_QList_QModelIndex_operator_index(QList< QModelIndex >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QModelIndex* qt_core_c_QList_QModelIndex_operator_index_const(const QList< QModelIndex >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QModelIndex_operator_neq(const QList< QModelIndex >* this_ptr, const QList< QModelIndex >* l) {
  return this_ptr->operator!=(*l);
}

QList< QModelIndex >* qt_core_c_QList_QModelIndex_operator_shl_l(QList< QModelIndex >* this_ptr, const QList< QModelIndex >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QModelIndex >* qt_core_c_QList_QModelIndex_operator_shl_t(QList< QModelIndex >* this_ptr, const QModelIndex* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QModelIndex_pop_back(QList< QModelIndex >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QModelIndex_pop_front(QList< QModelIndex >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QModelIndex_prepend(QList< QModelIndex >* this_ptr, const QModelIndex* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QModelIndex_push_back(QList< QModelIndex >* this_ptr, const QModelIndex* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QModelIndex_push_front(QList< QModelIndex >* this_ptr, const QModelIndex* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QModelIndex_removeAll(QList< QModelIndex >* this_ptr, const QModelIndex* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QModelIndex_removeAt(QList< QModelIndex >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QModelIndex_removeFirst(QList< QModelIndex >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QModelIndex_removeLast(QList< QModelIndex >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QModelIndex_removeOne(QList< QModelIndex >* this_ptr, const QModelIndex* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QModelIndex_replace(QList< QModelIndex >* this_ptr, int i, const QModelIndex* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QModelIndex_reserve(QList< QModelIndex >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QModelIndex_size(const QList< QModelIndex >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QModelIndex_startsWith(const QList< QModelIndex >* this_ptr, const QModelIndex* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QModelIndex_swap_i_j(QList< QModelIndex >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QModelIndex_swap_other(QList< QModelIndex >* this_ptr, QList< QModelIndex >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QList_QModelIndex_takeAt_to_output(QList< QModelIndex >* this_ptr, int i, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->takeAt(i));
}

void qt_core_c_QList_QModelIndex_takeFirst_to_output(QList< QModelIndex >* this_ptr, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->takeFirst());
}

void qt_core_c_QList_QModelIndex_takeLast_to_output(QList< QModelIndex >* this_ptr, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->takeLast());
}

void qt_core_c_QList_QModelIndex_value_to_output_i(const QList< QModelIndex >* this_ptr, int i, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->value(i));
}

void qt_core_c_QList_QModelIndex_value_to_output_i_defaultValue(const QList< QModelIndex >* this_ptr, int i, const QModelIndex* defaultValue, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QList_QObject_ptr_append_QList_QObject_ptr(QList< QObject* >* this_ptr, const QList< QObject* >* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QObject_ptr_append_QObject(QList< QObject* >* this_ptr, QObject* const * t) {
  this_ptr->append(*t);
}

QObject* const * qt_core_c_QList_QObject_ptr_at(const QList< QObject* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QObject** qt_core_c_QList_QObject_ptr_back(QList< QObject* >* this_ptr) {
  return &this_ptr->back();
}

QObject* const * qt_core_c_QList_QObject_ptr_back_const(const QList< QObject* >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QObject_ptr_clear(QList< QObject* >* this_ptr) {
  this_ptr->clear();
}

QObject* const * qt_core_c_QList_QObject_ptr_constFirst(const QList< QObject* >* this_ptr) {
  return &this_ptr->constFirst();
}

QObject* const * qt_core_c_QList_QObject_ptr_constLast(const QList< QObject* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QObject_ptr_constructor_l(const QList< QObject* >* l, QList< QObject* >* output) {
  new(output) QList< QObject* >(*l);
}

void qt_core_c_QList_QObject_ptr_constructor_no_args(QList< QObject* >* output) {
  new(output) QList< QObject* >();
}

bool qt_core_c_QList_QObject_ptr_contains(const QList< QObject* >* this_ptr, QObject* const * t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QObject_ptr_count_no_args(const QList< QObject* >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QObject_ptr_count_t(const QList< QObject* >* this_ptr, QObject* const * t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QObject_ptr_destructor(QList< QObject* >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QObject_ptr_empty(const QList< QObject* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QObject_ptr_endsWith(const QList< QObject* >* this_ptr, QObject* const * t) {
  return this_ptr->endsWith(*t);
}

QObject** qt_core_c_QList_QObject_ptr_first(QList< QObject* >* this_ptr) {
  return &this_ptr->first();
}

QObject* const * qt_core_c_QList_QObject_ptr_first_const(const QList< QObject* >* this_ptr) {
  return &this_ptr->first();
}

QObject** qt_core_c_QList_QObject_ptr_front(QList< QObject* >* this_ptr) {
  return &this_ptr->front();
}

QObject* const * qt_core_c_QList_QObject_ptr_front_const(const QList< QObject* >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QObject_ptr_indexOf_t(const QList< QObject* >* this_ptr, QObject* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QObject_ptr_indexOf_t_from(const QList< QObject* >* this_ptr, QObject* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QObject_ptr_insert(QList< QObject* >* this_ptr, int i, QObject* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QObject_ptr_isEmpty(const QList< QObject* >* this_ptr) {
  return this_ptr->isEmpty();
}

QObject** qt_core_c_QList_QObject_ptr_last(QList< QObject* >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QObject_ptr_lastIndexOf_t(const QList< QObject* >* this_ptr, QObject* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QObject_ptr_lastIndexOf_t_from(const QList< QObject* >* this_ptr, QObject* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QObject* const * qt_core_c_QList_QObject_ptr_last_const(const QList< QObject* >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QObject_ptr_length(const QList< QObject* >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QObject_ptr_mid_to_output_pos(const QList< QObject* >* this_ptr, int pos, QList< QObject* >* output) {
  new(output) QList< QObject* >(this_ptr->mid(pos));
}

void qt_core_c_QList_QObject_ptr_mid_to_output_pos_length(const QList< QObject* >* this_ptr, int pos, int length, QList< QObject* >* output) {
  new(output) QList< QObject* >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QObject_ptr_move(QList< QObject* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QObject* >* qt_core_c_QList_QObject_ptr_operator_add_assign_l(QList< QObject* >* this_ptr, const QList< QObject* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QObject* >* qt_core_c_QList_QObject_ptr_operator_add_assign_t(QList< QObject* >* this_ptr, QObject* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QObject_ptr_operator_add_to_output(const QList< QObject* >* this_ptr, const QList< QObject* >* l, QList< QObject* >* output) {
  new(output) QList< QObject* >(this_ptr->operator+(*l));
}

QList< QObject* >* qt_core_c_QList_QObject_ptr_operator_assign(QList< QObject* >* this_ptr, const QList< QObject* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QObject_ptr_operator_eq(const QList< QObject* >* this_ptr, const QList< QObject* >* l) {
  return this_ptr->operator==(*l);
}

QObject** qt_core_c_QList_QObject_ptr_operator_index(QList< QObject* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QObject* const * qt_core_c_QList_QObject_ptr_operator_index_const(const QList< QObject* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QObject_ptr_operator_neq(const QList< QObject* >* this_ptr, const QList< QObject* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QObject* >* qt_core_c_QList_QObject_ptr_operator_shl_l(QList< QObject* >* this_ptr, const QList< QObject* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QObject* >* qt_core_c_QList_QObject_ptr_operator_shl_t(QList< QObject* >* this_ptr, QObject* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QObject_ptr_pop_back(QList< QObject* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QObject_ptr_pop_front(QList< QObject* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QObject_ptr_prepend(QList< QObject* >* this_ptr, QObject* const * t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QObject_ptr_push_back(QList< QObject* >* this_ptr, QObject* const * t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QObject_ptr_push_front(QList< QObject* >* this_ptr, QObject* const * t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QObject_ptr_removeAll(QList< QObject* >* this_ptr, QObject* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QObject_ptr_removeAt(QList< QObject* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QObject_ptr_removeFirst(QList< QObject* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QObject_ptr_removeLast(QList< QObject* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QObject_ptr_removeOne(QList< QObject* >* this_ptr, QObject* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QObject_ptr_replace(QList< QObject* >* this_ptr, int i, QObject* const * t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QObject_ptr_reserve(QList< QObject* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QObject_ptr_size(const QList< QObject* >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QObject_ptr_startsWith(const QList< QObject* >* this_ptr, QObject* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QObject_ptr_swap_i_j(QList< QObject* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QObject_ptr_swap_other(QList< QObject* >* this_ptr, QList< QObject* >* other) {
  this_ptr->swap(*other);
}

QObject* qt_core_c_QList_QObject_ptr_takeAt(QList< QObject* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QObject* qt_core_c_QList_QObject_ptr_takeFirst(QList< QObject* >* this_ptr) {
  return this_ptr->takeFirst();
}

QObject* qt_core_c_QList_QObject_ptr_takeLast(QList< QObject* >* this_ptr) {
  return this_ptr->takeLast();
}

QObject* qt_core_c_QList_QObject_ptr_value_i(const QList< QObject* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QObject* qt_core_c_QList_QObject_ptr_value_i_defaultValue(const QList< QObject* >* this_ptr, int i, QObject* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_core_c_QList_QPair_QString_QString_append_QList_QPair_QString_QString(QList< QPair< QString, QString > >* this_ptr, const QList< QPair< QString, QString > >* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QPair_QString_QString_append_QPair_QString_QString(QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t) {
  this_ptr->append(*t);
}

const QPair< QString, QString >* qt_core_c_QList_QPair_QString_QString_at(const QList< QPair< QString, QString > >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QPair< QString, QString >* qt_core_c_QList_QPair_QString_QString_back(QList< QPair< QString, QString > >* this_ptr) {
  return &this_ptr->back();
}

const QPair< QString, QString >* qt_core_c_QList_QPair_QString_QString_back_const(const QList< QPair< QString, QString > >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QPair_QString_QString_clear(QList< QPair< QString, QString > >* this_ptr) {
  this_ptr->clear();
}

const QPair< QString, QString >* qt_core_c_QList_QPair_QString_QString_constFirst(const QList< QPair< QString, QString > >* this_ptr) {
  return &this_ptr->constFirst();
}

const QPair< QString, QString >* qt_core_c_QList_QPair_QString_QString_constLast(const QList< QPair< QString, QString > >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QPair_QString_QString_constructor_l(const QList< QPair< QString, QString > >* l, QList< QPair< QString, QString > >* output) {
  new(output) QList< QPair< QString, QString > >(*l);
}

void qt_core_c_QList_QPair_QString_QString_constructor_no_args(QList< QPair< QString, QString > >* output) {
  new(output) QList< QPair< QString, QString > >();
}

bool qt_core_c_QList_QPair_QString_QString_contains(const QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QPair_QString_QString_count_no_args(const QList< QPair< QString, QString > >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QPair_QString_QString_count_t(const QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QPair_QString_QString_destructor(QList< QPair< QString, QString > >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QPair_QString_QString_empty(const QList< QPair< QString, QString > >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QPair_QString_QString_endsWith(const QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t) {
  return this_ptr->endsWith(*t);
}

QPair< QString, QString >* qt_core_c_QList_QPair_QString_QString_first(QList< QPair< QString, QString > >* this_ptr) {
  return &this_ptr->first();
}

const QPair< QString, QString >* qt_core_c_QList_QPair_QString_QString_first_const(const QList< QPair< QString, QString > >* this_ptr) {
  return &this_ptr->first();
}

QPair< QString, QString >* qt_core_c_QList_QPair_QString_QString_front(QList< QPair< QString, QString > >* this_ptr) {
  return &this_ptr->front();
}

const QPair< QString, QString >* qt_core_c_QList_QPair_QString_QString_front_const(const QList< QPair< QString, QString > >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QPair_QString_QString_indexOf_t(const QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QPair_QString_QString_indexOf_t_from(const QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QPair_QString_QString_insert(QList< QPair< QString, QString > >* this_ptr, int i, const QPair< QString, QString >* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QPair_QString_QString_isEmpty(const QList< QPair< QString, QString > >* this_ptr) {
  return this_ptr->isEmpty();
}

QPair< QString, QString >* qt_core_c_QList_QPair_QString_QString_last(QList< QPair< QString, QString > >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QPair_QString_QString_lastIndexOf_t(const QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QPair_QString_QString_lastIndexOf_t_from(const QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QPair< QString, QString >* qt_core_c_QList_QPair_QString_QString_last_const(const QList< QPair< QString, QString > >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QPair_QString_QString_length(const QList< QPair< QString, QString > >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QPair_QString_QString_mid_to_output_pos(const QList< QPair< QString, QString > >* this_ptr, int pos, QList< QPair< QString, QString > >* output) {
  new(output) QList< QPair< QString, QString > >(this_ptr->mid(pos));
}

void qt_core_c_QList_QPair_QString_QString_mid_to_output_pos_length(const QList< QPair< QString, QString > >* this_ptr, int pos, int length, QList< QPair< QString, QString > >* output) {
  new(output) QList< QPair< QString, QString > >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QPair_QString_QString_move(QList< QPair< QString, QString > >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QPair< QString, QString > >* qt_core_c_QList_QPair_QString_QString_operator_add_assign_l(QList< QPair< QString, QString > >* this_ptr, const QList< QPair< QString, QString > >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QPair< QString, QString > >* qt_core_c_QList_QPair_QString_QString_operator_add_assign_t(QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QPair_QString_QString_operator_add_to_output(const QList< QPair< QString, QString > >* this_ptr, const QList< QPair< QString, QString > >* l, QList< QPair< QString, QString > >* output) {
  new(output) QList< QPair< QString, QString > >(this_ptr->operator+(*l));
}

QList< QPair< QString, QString > >* qt_core_c_QList_QPair_QString_QString_operator_assign(QList< QPair< QString, QString > >* this_ptr, const QList< QPair< QString, QString > >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QPair_QString_QString_operator_eq(const QList< QPair< QString, QString > >* this_ptr, const QList< QPair< QString, QString > >* l) {
  return this_ptr->operator==(*l);
}

QPair< QString, QString >* qt_core_c_QList_QPair_QString_QString_operator_index(QList< QPair< QString, QString > >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QPair< QString, QString >* qt_core_c_QList_QPair_QString_QString_operator_index_const(const QList< QPair< QString, QString > >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QPair_QString_QString_operator_neq(const QList< QPair< QString, QString > >* this_ptr, const QList< QPair< QString, QString > >* l) {
  return this_ptr->operator!=(*l);
}

QList< QPair< QString, QString > >* qt_core_c_QList_QPair_QString_QString_operator_shl_l(QList< QPair< QString, QString > >* this_ptr, const QList< QPair< QString, QString > >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QPair< QString, QString > >* qt_core_c_QList_QPair_QString_QString_operator_shl_t(QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QPair_QString_QString_pop_back(QList< QPair< QString, QString > >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QPair_QString_QString_pop_front(QList< QPair< QString, QString > >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QPair_QString_QString_prepend(QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QPair_QString_QString_push_back(QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QPair_QString_QString_push_front(QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QPair_QString_QString_removeAll(QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QPair_QString_QString_removeAt(QList< QPair< QString, QString > >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QPair_QString_QString_removeFirst(QList< QPair< QString, QString > >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QPair_QString_QString_removeLast(QList< QPair< QString, QString > >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QPair_QString_QString_removeOne(QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QPair_QString_QString_replace(QList< QPair< QString, QString > >* this_ptr, int i, const QPair< QString, QString >* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QPair_QString_QString_reserve(QList< QPair< QString, QString > >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QPair_QString_QString_size(const QList< QPair< QString, QString > >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QPair_QString_QString_startsWith(const QList< QPair< QString, QString > >* this_ptr, const QPair< QString, QString >* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QPair_QString_QString_swap_i_j(QList< QPair< QString, QString > >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QPair_QString_QString_swap_other(QList< QPair< QString, QString > >* this_ptr, QList< QPair< QString, QString > >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QList_QPair_QString_QString_takeAt_to_output(QList< QPair< QString, QString > >* this_ptr, int i, QPair< QString, QString >* output) {
  new(output) QPair< QString, QString >(this_ptr->takeAt(i));
}

void qt_core_c_QList_QPair_QString_QString_takeFirst_to_output(QList< QPair< QString, QString > >* this_ptr, QPair< QString, QString >* output) {
  new(output) QPair< QString, QString >(this_ptr->takeFirst());
}

void qt_core_c_QList_QPair_QString_QString_takeLast_to_output(QList< QPair< QString, QString > >* this_ptr, QPair< QString, QString >* output) {
  new(output) QPair< QString, QString >(this_ptr->takeLast());
}

void qt_core_c_QList_QPair_QString_QString_value_to_output_i(const QList< QPair< QString, QString > >* this_ptr, int i, QPair< QString, QString >* output) {
  new(output) QPair< QString, QString >(this_ptr->value(i));
}

void qt_core_c_QList_QPair_QString_QString_value_to_output_i_defaultValue(const QList< QPair< QString, QString > >* this_ptr, int i, const QPair< QString, QString >* defaultValue, QPair< QString, QString >* output) {
  new(output) QPair< QString, QString >(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QList_QPersistentModelIndex_append_QList_QPersistentModelIndex(QList< QPersistentModelIndex >* this_ptr, const QList< QPersistentModelIndex >* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QPersistentModelIndex_append_QPersistentModelIndex(QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t) {
  this_ptr->append(*t);
}

const QPersistentModelIndex* qt_core_c_QList_QPersistentModelIndex_at(const QList< QPersistentModelIndex >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QPersistentModelIndex* qt_core_c_QList_QPersistentModelIndex_back(QList< QPersistentModelIndex >* this_ptr) {
  return &this_ptr->back();
}

const QPersistentModelIndex* qt_core_c_QList_QPersistentModelIndex_back_const(const QList< QPersistentModelIndex >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QPersistentModelIndex_clear(QList< QPersistentModelIndex >* this_ptr) {
  this_ptr->clear();
}

const QPersistentModelIndex* qt_core_c_QList_QPersistentModelIndex_constFirst(const QList< QPersistentModelIndex >* this_ptr) {
  return &this_ptr->constFirst();
}

const QPersistentModelIndex* qt_core_c_QList_QPersistentModelIndex_constLast(const QList< QPersistentModelIndex >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QPersistentModelIndex_constructor_l(const QList< QPersistentModelIndex >* l, QList< QPersistentModelIndex >* output) {
  new(output) QList< QPersistentModelIndex >(*l);
}

void qt_core_c_QList_QPersistentModelIndex_constructor_no_args(QList< QPersistentModelIndex >* output) {
  new(output) QList< QPersistentModelIndex >();
}

bool qt_core_c_QList_QPersistentModelIndex_contains(const QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QPersistentModelIndex_count_no_args(const QList< QPersistentModelIndex >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QPersistentModelIndex_count_t(const QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QPersistentModelIndex_destructor(QList< QPersistentModelIndex >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QPersistentModelIndex_empty(const QList< QPersistentModelIndex >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QPersistentModelIndex_endsWith(const QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t) {
  return this_ptr->endsWith(*t);
}

QPersistentModelIndex* qt_core_c_QList_QPersistentModelIndex_first(QList< QPersistentModelIndex >* this_ptr) {
  return &this_ptr->first();
}

const QPersistentModelIndex* qt_core_c_QList_QPersistentModelIndex_first_const(const QList< QPersistentModelIndex >* this_ptr) {
  return &this_ptr->first();
}

QPersistentModelIndex* qt_core_c_QList_QPersistentModelIndex_front(QList< QPersistentModelIndex >* this_ptr) {
  return &this_ptr->front();
}

const QPersistentModelIndex* qt_core_c_QList_QPersistentModelIndex_front_const(const QList< QPersistentModelIndex >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QPersistentModelIndex_indexOf_t(const QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QPersistentModelIndex_indexOf_t_from(const QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QPersistentModelIndex_insert(QList< QPersistentModelIndex >* this_ptr, int i, const QPersistentModelIndex* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QPersistentModelIndex_isEmpty(const QList< QPersistentModelIndex >* this_ptr) {
  return this_ptr->isEmpty();
}

QPersistentModelIndex* qt_core_c_QList_QPersistentModelIndex_last(QList< QPersistentModelIndex >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QPersistentModelIndex_lastIndexOf_t(const QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QPersistentModelIndex_lastIndexOf_t_from(const QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QPersistentModelIndex* qt_core_c_QList_QPersistentModelIndex_last_const(const QList< QPersistentModelIndex >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QPersistentModelIndex_length(const QList< QPersistentModelIndex >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QPersistentModelIndex_mid_to_output_pos(const QList< QPersistentModelIndex >* this_ptr, int pos, QList< QPersistentModelIndex >* output) {
  new(output) QList< QPersistentModelIndex >(this_ptr->mid(pos));
}

void qt_core_c_QList_QPersistentModelIndex_mid_to_output_pos_length(const QList< QPersistentModelIndex >* this_ptr, int pos, int length, QList< QPersistentModelIndex >* output) {
  new(output) QList< QPersistentModelIndex >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QPersistentModelIndex_move(QList< QPersistentModelIndex >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QPersistentModelIndex >* qt_core_c_QList_QPersistentModelIndex_operator_add_assign_l(QList< QPersistentModelIndex >* this_ptr, const QList< QPersistentModelIndex >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QPersistentModelIndex >* qt_core_c_QList_QPersistentModelIndex_operator_add_assign_t(QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QPersistentModelIndex_operator_add_to_output(const QList< QPersistentModelIndex >* this_ptr, const QList< QPersistentModelIndex >* l, QList< QPersistentModelIndex >* output) {
  new(output) QList< QPersistentModelIndex >(this_ptr->operator+(*l));
}

QList< QPersistentModelIndex >* qt_core_c_QList_QPersistentModelIndex_operator_assign(QList< QPersistentModelIndex >* this_ptr, const QList< QPersistentModelIndex >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QPersistentModelIndex_operator_eq(const QList< QPersistentModelIndex >* this_ptr, const QList< QPersistentModelIndex >* l) {
  return this_ptr->operator==(*l);
}

QPersistentModelIndex* qt_core_c_QList_QPersistentModelIndex_operator_index(QList< QPersistentModelIndex >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QPersistentModelIndex* qt_core_c_QList_QPersistentModelIndex_operator_index_const(const QList< QPersistentModelIndex >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QPersistentModelIndex_operator_neq(const QList< QPersistentModelIndex >* this_ptr, const QList< QPersistentModelIndex >* l) {
  return this_ptr->operator!=(*l);
}

QList< QPersistentModelIndex >* qt_core_c_QList_QPersistentModelIndex_operator_shl_l(QList< QPersistentModelIndex >* this_ptr, const QList< QPersistentModelIndex >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QPersistentModelIndex >* qt_core_c_QList_QPersistentModelIndex_operator_shl_t(QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QPersistentModelIndex_pop_back(QList< QPersistentModelIndex >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QPersistentModelIndex_pop_front(QList< QPersistentModelIndex >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QPersistentModelIndex_prepend(QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QPersistentModelIndex_push_back(QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QPersistentModelIndex_push_front(QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QPersistentModelIndex_removeAll(QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QPersistentModelIndex_removeAt(QList< QPersistentModelIndex >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QPersistentModelIndex_removeFirst(QList< QPersistentModelIndex >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QPersistentModelIndex_removeLast(QList< QPersistentModelIndex >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QPersistentModelIndex_removeOne(QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QPersistentModelIndex_replace(QList< QPersistentModelIndex >* this_ptr, int i, const QPersistentModelIndex* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QPersistentModelIndex_reserve(QList< QPersistentModelIndex >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QPersistentModelIndex_size(const QList< QPersistentModelIndex >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QPersistentModelIndex_startsWith(const QList< QPersistentModelIndex >* this_ptr, const QPersistentModelIndex* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QPersistentModelIndex_swap_i_j(QList< QPersistentModelIndex >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QPersistentModelIndex_swap_other(QList< QPersistentModelIndex >* this_ptr, QList< QPersistentModelIndex >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QList_QPersistentModelIndex_takeAt_to_output(QList< QPersistentModelIndex >* this_ptr, int i, QPersistentModelIndex* output) {
  new(output) QPersistentModelIndex(this_ptr->takeAt(i));
}

void qt_core_c_QList_QPersistentModelIndex_takeFirst_to_output(QList< QPersistentModelIndex >* this_ptr, QPersistentModelIndex* output) {
  new(output) QPersistentModelIndex(this_ptr->takeFirst());
}

void qt_core_c_QList_QPersistentModelIndex_takeLast_to_output(QList< QPersistentModelIndex >* this_ptr, QPersistentModelIndex* output) {
  new(output) QPersistentModelIndex(this_ptr->takeLast());
}

void qt_core_c_QList_QPersistentModelIndex_value_to_output_i(const QList< QPersistentModelIndex >* this_ptr, int i, QPersistentModelIndex* output) {
  new(output) QPersistentModelIndex(this_ptr->value(i));
}

void qt_core_c_QList_QPersistentModelIndex_value_to_output_i_defaultValue(const QList< QPersistentModelIndex >* this_ptr, int i, const QPersistentModelIndex* defaultValue, QPersistentModelIndex* output) {
  new(output) QPersistentModelIndex(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QList_QStorageInfo_append_QList_QStorageInfo(QList< QStorageInfo >* this_ptr, const QList< QStorageInfo >* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QStorageInfo_append_QStorageInfo(QList< QStorageInfo >* this_ptr, const QStorageInfo* t) {
  this_ptr->append(*t);
}

const QStorageInfo* qt_core_c_QList_QStorageInfo_at(const QList< QStorageInfo >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QStorageInfo* qt_core_c_QList_QStorageInfo_back(QList< QStorageInfo >* this_ptr) {
  return &this_ptr->back();
}

const QStorageInfo* qt_core_c_QList_QStorageInfo_back_const(const QList< QStorageInfo >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QStorageInfo_clear(QList< QStorageInfo >* this_ptr) {
  this_ptr->clear();
}

const QStorageInfo* qt_core_c_QList_QStorageInfo_constFirst(const QList< QStorageInfo >* this_ptr) {
  return &this_ptr->constFirst();
}

const QStorageInfo* qt_core_c_QList_QStorageInfo_constLast(const QList< QStorageInfo >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QStorageInfo_constructor_l(const QList< QStorageInfo >* l, QList< QStorageInfo >* output) {
  new(output) QList< QStorageInfo >(*l);
}

void qt_core_c_QList_QStorageInfo_constructor_no_args(QList< QStorageInfo >* output) {
  new(output) QList< QStorageInfo >();
}

bool qt_core_c_QList_QStorageInfo_contains(const QList< QStorageInfo >* this_ptr, const QStorageInfo* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QStorageInfo_count_no_args(const QList< QStorageInfo >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QStorageInfo_count_t(const QList< QStorageInfo >* this_ptr, const QStorageInfo* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QStorageInfo_destructor(QList< QStorageInfo >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QStorageInfo_empty(const QList< QStorageInfo >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QStorageInfo_endsWith(const QList< QStorageInfo >* this_ptr, const QStorageInfo* t) {
  return this_ptr->endsWith(*t);
}

QStorageInfo* qt_core_c_QList_QStorageInfo_first(QList< QStorageInfo >* this_ptr) {
  return &this_ptr->first();
}

const QStorageInfo* qt_core_c_QList_QStorageInfo_first_const(const QList< QStorageInfo >* this_ptr) {
  return &this_ptr->first();
}

QStorageInfo* qt_core_c_QList_QStorageInfo_front(QList< QStorageInfo >* this_ptr) {
  return &this_ptr->front();
}

const QStorageInfo* qt_core_c_QList_QStorageInfo_front_const(const QList< QStorageInfo >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QStorageInfo_indexOf_t(const QList< QStorageInfo >* this_ptr, const QStorageInfo* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QStorageInfo_indexOf_t_from(const QList< QStorageInfo >* this_ptr, const QStorageInfo* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QStorageInfo_insert(QList< QStorageInfo >* this_ptr, int i, const QStorageInfo* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QStorageInfo_isEmpty(const QList< QStorageInfo >* this_ptr) {
  return this_ptr->isEmpty();
}

QStorageInfo* qt_core_c_QList_QStorageInfo_last(QList< QStorageInfo >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QStorageInfo_lastIndexOf_t(const QList< QStorageInfo >* this_ptr, const QStorageInfo* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QStorageInfo_lastIndexOf_t_from(const QList< QStorageInfo >* this_ptr, const QStorageInfo* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QStorageInfo* qt_core_c_QList_QStorageInfo_last_const(const QList< QStorageInfo >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QStorageInfo_length(const QList< QStorageInfo >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QStorageInfo_mid_to_output_pos(const QList< QStorageInfo >* this_ptr, int pos, QList< QStorageInfo >* output) {
  new(output) QList< QStorageInfo >(this_ptr->mid(pos));
}

void qt_core_c_QList_QStorageInfo_mid_to_output_pos_length(const QList< QStorageInfo >* this_ptr, int pos, int length, QList< QStorageInfo >* output) {
  new(output) QList< QStorageInfo >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QStorageInfo_move(QList< QStorageInfo >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QStorageInfo >* qt_core_c_QList_QStorageInfo_operator_add_assign_l(QList< QStorageInfo >* this_ptr, const QList< QStorageInfo >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QStorageInfo >* qt_core_c_QList_QStorageInfo_operator_add_assign_t(QList< QStorageInfo >* this_ptr, const QStorageInfo* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QStorageInfo_operator_add_to_output(const QList< QStorageInfo >* this_ptr, const QList< QStorageInfo >* l, QList< QStorageInfo >* output) {
  new(output) QList< QStorageInfo >(this_ptr->operator+(*l));
}

QList< QStorageInfo >* qt_core_c_QList_QStorageInfo_operator_assign(QList< QStorageInfo >* this_ptr, const QList< QStorageInfo >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QStorageInfo_operator_eq(const QList< QStorageInfo >* this_ptr, const QList< QStorageInfo >* l) {
  return this_ptr->operator==(*l);
}

QStorageInfo* qt_core_c_QList_QStorageInfo_operator_index(QList< QStorageInfo >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QStorageInfo* qt_core_c_QList_QStorageInfo_operator_index_const(const QList< QStorageInfo >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QStorageInfo_operator_neq(const QList< QStorageInfo >* this_ptr, const QList< QStorageInfo >* l) {
  return this_ptr->operator!=(*l);
}

QList< QStorageInfo >* qt_core_c_QList_QStorageInfo_operator_shl_l(QList< QStorageInfo >* this_ptr, const QList< QStorageInfo >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QStorageInfo >* qt_core_c_QList_QStorageInfo_operator_shl_t(QList< QStorageInfo >* this_ptr, const QStorageInfo* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QStorageInfo_pop_back(QList< QStorageInfo >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QStorageInfo_pop_front(QList< QStorageInfo >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QStorageInfo_prepend(QList< QStorageInfo >* this_ptr, const QStorageInfo* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QStorageInfo_push_back(QList< QStorageInfo >* this_ptr, const QStorageInfo* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QStorageInfo_push_front(QList< QStorageInfo >* this_ptr, const QStorageInfo* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QStorageInfo_removeAll(QList< QStorageInfo >* this_ptr, const QStorageInfo* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QStorageInfo_removeAt(QList< QStorageInfo >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QStorageInfo_removeFirst(QList< QStorageInfo >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QStorageInfo_removeLast(QList< QStorageInfo >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QStorageInfo_removeOne(QList< QStorageInfo >* this_ptr, const QStorageInfo* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QStorageInfo_replace(QList< QStorageInfo >* this_ptr, int i, const QStorageInfo* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QStorageInfo_reserve(QList< QStorageInfo >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QStorageInfo_size(const QList< QStorageInfo >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QStorageInfo_startsWith(const QList< QStorageInfo >* this_ptr, const QStorageInfo* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QStorageInfo_swap_i_j(QList< QStorageInfo >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QStorageInfo_swap_other(QList< QStorageInfo >* this_ptr, QList< QStorageInfo >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QList_QStorageInfo_takeAt_to_output(QList< QStorageInfo >* this_ptr, int i, QStorageInfo* output) {
  new(output) QStorageInfo(this_ptr->takeAt(i));
}

void qt_core_c_QList_QStorageInfo_takeFirst_to_output(QList< QStorageInfo >* this_ptr, QStorageInfo* output) {
  new(output) QStorageInfo(this_ptr->takeFirst());
}

void qt_core_c_QList_QStorageInfo_takeLast_to_output(QList< QStorageInfo >* this_ptr, QStorageInfo* output) {
  new(output) QStorageInfo(this_ptr->takeLast());
}

void qt_core_c_QList_QStorageInfo_value_to_output_i(const QList< QStorageInfo >* this_ptr, int i, QStorageInfo* output) {
  new(output) QStorageInfo(this_ptr->value(i));
}

void qt_core_c_QList_QStorageInfo_value_to_output_i_defaultValue(const QList< QStorageInfo >* this_ptr, int i, const QStorageInfo* defaultValue, QStorageInfo* output) {
  new(output) QStorageInfo(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QList_QString_append_QList_QString(QList< QString >* this_ptr, const QList< QString >* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QString_append_QString(QList< QString >* this_ptr, const QString* t) {
  this_ptr->append(*t);
}

const QString* qt_core_c_QList_QString_at(const QList< QString >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QString* qt_core_c_QList_QString_back(QList< QString >* this_ptr) {
  return &this_ptr->back();
}

const QString* qt_core_c_QList_QString_back_const(const QList< QString >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QString_clear(QList< QString >* this_ptr) {
  this_ptr->clear();
}

const QString* qt_core_c_QList_QString_constFirst(const QList< QString >* this_ptr) {
  return &this_ptr->constFirst();
}

const QString* qt_core_c_QList_QString_constLast(const QList< QString >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QString_constructor_l(const QList< QString >* l, QList< QString >* output) {
  new(output) QList< QString >(*l);
}

void qt_core_c_QList_QString_constructor_no_args(QList< QString >* output) {
  new(output) QList< QString >();
}

bool qt_core_c_QList_QString_contains(const QList< QString >* this_ptr, const QString* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QString_count_no_args(const QList< QString >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QString_count_t(const QList< QString >* this_ptr, const QString* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QString_destructor(QList< QString >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QString_empty(const QList< QString >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QString_endsWith(const QList< QString >* this_ptr, const QString* t) {
  return this_ptr->endsWith(*t);
}

QString* qt_core_c_QList_QString_first(QList< QString >* this_ptr) {
  return &this_ptr->first();
}

const QString* qt_core_c_QList_QString_first_const(const QList< QString >* this_ptr) {
  return &this_ptr->first();
}

QString* qt_core_c_QList_QString_front(QList< QString >* this_ptr) {
  return &this_ptr->front();
}

const QString* qt_core_c_QList_QString_front_const(const QList< QString >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QString_indexOf_t(const QList< QString >* this_ptr, const QString* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QString_indexOf_t_from(const QList< QString >* this_ptr, const QString* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QString_insert(QList< QString >* this_ptr, int i, const QString* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QString_isEmpty(const QList< QString >* this_ptr) {
  return this_ptr->isEmpty();
}

QString* qt_core_c_QList_QString_last(QList< QString >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QString_lastIndexOf_t(const QList< QString >* this_ptr, const QString* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QString_lastIndexOf_t_from(const QList< QString >* this_ptr, const QString* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QString* qt_core_c_QList_QString_last_const(const QList< QString >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QString_length(const QList< QString >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QString_mid_to_output_pos(const QList< QString >* this_ptr, int pos, QList< QString >* output) {
  new(output) QList< QString >(this_ptr->mid(pos));
}

void qt_core_c_QList_QString_mid_to_output_pos_length(const QList< QString >* this_ptr, int pos, int length, QList< QString >* output) {
  new(output) QList< QString >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QString_move(QList< QString >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QString >* qt_core_c_QList_QString_operator_add_assign_l(QList< QString >* this_ptr, const QList< QString >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QString >* qt_core_c_QList_QString_operator_add_assign_t(QList< QString >* this_ptr, const QString* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QString_operator_add_to_output(const QList< QString >* this_ptr, const QList< QString >* l, QList< QString >* output) {
  new(output) QList< QString >(this_ptr->operator+(*l));
}

QList< QString >* qt_core_c_QList_QString_operator_assign(QList< QString >* this_ptr, const QList< QString >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QString_operator_eq(const QList< QString >* this_ptr, const QList< QString >* l) {
  return this_ptr->operator==(*l);
}

QString* qt_core_c_QList_QString_operator_index(QList< QString >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QString* qt_core_c_QList_QString_operator_index_const(const QList< QString >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QString_operator_neq(const QList< QString >* this_ptr, const QList< QString >* l) {
  return this_ptr->operator!=(*l);
}

QList< QString >* qt_core_c_QList_QString_operator_shl_l(QList< QString >* this_ptr, const QList< QString >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QString >* qt_core_c_QList_QString_operator_shl_t(QList< QString >* this_ptr, const QString* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QString_pop_back(QList< QString >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QString_pop_front(QList< QString >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QString_prepend(QList< QString >* this_ptr, const QString* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QString_push_back(QList< QString >* this_ptr, const QString* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QString_push_front(QList< QString >* this_ptr, const QString* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QString_removeAll(QList< QString >* this_ptr, const QString* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QString_removeAt(QList< QString >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QString_removeFirst(QList< QString >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QString_removeLast(QList< QString >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QString_removeOne(QList< QString >* this_ptr, const QString* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QString_replace(QList< QString >* this_ptr, int i, const QString* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QString_reserve(QList< QString >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QString_size(const QList< QString >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QString_startsWith(const QList< QString >* this_ptr, const QString* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QString_swap_i_j(QList< QString >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QString_swap_other(QList< QString >* this_ptr, QList< QString >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QList_QString_takeAt_to_output(QList< QString >* this_ptr, int i, QString* output) {
  new(output) QString(this_ptr->takeAt(i));
}

void qt_core_c_QList_QString_takeFirst_to_output(QList< QString >* this_ptr, QString* output) {
  new(output) QString(this_ptr->takeFirst());
}

void qt_core_c_QList_QString_takeLast_to_output(QList< QString >* this_ptr, QString* output) {
  new(output) QString(this_ptr->takeLast());
}

void qt_core_c_QList_QString_value_to_output_i(const QList< QString >* this_ptr, int i, QString* output) {
  new(output) QString(this_ptr->value(i));
}

void qt_core_c_QList_QString_value_to_output_i_defaultValue(const QList< QString >* this_ptr, int i, const QString* defaultValue, QString* output) {
  new(output) QString(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QList_QUrl_append_QList_QUrl(QList< QUrl >* this_ptr, const QList< QUrl >* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QUrl_append_QUrl(QList< QUrl >* this_ptr, const QUrl* t) {
  this_ptr->append(*t);
}

const QUrl* qt_core_c_QList_QUrl_at(const QList< QUrl >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QUrl* qt_core_c_QList_QUrl_back(QList< QUrl >* this_ptr) {
  return &this_ptr->back();
}

const QUrl* qt_core_c_QList_QUrl_back_const(const QList< QUrl >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QUrl_clear(QList< QUrl >* this_ptr) {
  this_ptr->clear();
}

const QUrl* qt_core_c_QList_QUrl_constFirst(const QList< QUrl >* this_ptr) {
  return &this_ptr->constFirst();
}

const QUrl* qt_core_c_QList_QUrl_constLast(const QList< QUrl >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QUrl_constructor_l(const QList< QUrl >* l, QList< QUrl >* output) {
  new(output) QList< QUrl >(*l);
}

void qt_core_c_QList_QUrl_constructor_no_args(QList< QUrl >* output) {
  new(output) QList< QUrl >();
}

bool qt_core_c_QList_QUrl_contains(const QList< QUrl >* this_ptr, const QUrl* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QUrl_count_no_args(const QList< QUrl >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QUrl_count_t(const QList< QUrl >* this_ptr, const QUrl* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QUrl_destructor(QList< QUrl >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QUrl_empty(const QList< QUrl >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QUrl_endsWith(const QList< QUrl >* this_ptr, const QUrl* t) {
  return this_ptr->endsWith(*t);
}

QUrl* qt_core_c_QList_QUrl_first(QList< QUrl >* this_ptr) {
  return &this_ptr->first();
}

const QUrl* qt_core_c_QList_QUrl_first_const(const QList< QUrl >* this_ptr) {
  return &this_ptr->first();
}

QUrl* qt_core_c_QList_QUrl_front(QList< QUrl >* this_ptr) {
  return &this_ptr->front();
}

const QUrl* qt_core_c_QList_QUrl_front_const(const QList< QUrl >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QUrl_indexOf_t(const QList< QUrl >* this_ptr, const QUrl* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QUrl_indexOf_t_from(const QList< QUrl >* this_ptr, const QUrl* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QUrl_insert(QList< QUrl >* this_ptr, int i, const QUrl* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QUrl_isEmpty(const QList< QUrl >* this_ptr) {
  return this_ptr->isEmpty();
}

QUrl* qt_core_c_QList_QUrl_last(QList< QUrl >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QUrl_lastIndexOf_t(const QList< QUrl >* this_ptr, const QUrl* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QUrl_lastIndexOf_t_from(const QList< QUrl >* this_ptr, const QUrl* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QUrl* qt_core_c_QList_QUrl_last_const(const QList< QUrl >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QUrl_length(const QList< QUrl >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QUrl_mid_to_output_pos(const QList< QUrl >* this_ptr, int pos, QList< QUrl >* output) {
  new(output) QList< QUrl >(this_ptr->mid(pos));
}

void qt_core_c_QList_QUrl_mid_to_output_pos_length(const QList< QUrl >* this_ptr, int pos, int length, QList< QUrl >* output) {
  new(output) QList< QUrl >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QUrl_move(QList< QUrl >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QUrl >* qt_core_c_QList_QUrl_operator_add_assign_l(QList< QUrl >* this_ptr, const QList< QUrl >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QUrl >* qt_core_c_QList_QUrl_operator_add_assign_t(QList< QUrl >* this_ptr, const QUrl* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QUrl_operator_add_to_output(const QList< QUrl >* this_ptr, const QList< QUrl >* l, QList< QUrl >* output) {
  new(output) QList< QUrl >(this_ptr->operator+(*l));
}

QList< QUrl >* qt_core_c_QList_QUrl_operator_assign(QList< QUrl >* this_ptr, const QList< QUrl >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QUrl_operator_eq(const QList< QUrl >* this_ptr, const QList< QUrl >* l) {
  return this_ptr->operator==(*l);
}

QUrl* qt_core_c_QList_QUrl_operator_index(QList< QUrl >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QUrl* qt_core_c_QList_QUrl_operator_index_const(const QList< QUrl >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QUrl_operator_neq(const QList< QUrl >* this_ptr, const QList< QUrl >* l) {
  return this_ptr->operator!=(*l);
}

QList< QUrl >* qt_core_c_QList_QUrl_operator_shl_l(QList< QUrl >* this_ptr, const QList< QUrl >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QUrl >* qt_core_c_QList_QUrl_operator_shl_t(QList< QUrl >* this_ptr, const QUrl* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QUrl_pop_back(QList< QUrl >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QUrl_pop_front(QList< QUrl >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QUrl_prepend(QList< QUrl >* this_ptr, const QUrl* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QUrl_push_back(QList< QUrl >* this_ptr, const QUrl* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QUrl_push_front(QList< QUrl >* this_ptr, const QUrl* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QUrl_removeAll(QList< QUrl >* this_ptr, const QUrl* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QUrl_removeAt(QList< QUrl >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QUrl_removeFirst(QList< QUrl >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QUrl_removeLast(QList< QUrl >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QUrl_removeOne(QList< QUrl >* this_ptr, const QUrl* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QUrl_replace(QList< QUrl >* this_ptr, int i, const QUrl* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QUrl_reserve(QList< QUrl >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QUrl_size(const QList< QUrl >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QUrl_startsWith(const QList< QUrl >* this_ptr, const QUrl* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QUrl_swap_i_j(QList< QUrl >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QUrl_swap_other(QList< QUrl >* this_ptr, QList< QUrl >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QList_QUrl_takeAt_to_output(QList< QUrl >* this_ptr, int i, QUrl* output) {
  new(output) QUrl(this_ptr->takeAt(i));
}

void qt_core_c_QList_QUrl_takeFirst_to_output(QList< QUrl >* this_ptr, QUrl* output) {
  new(output) QUrl(this_ptr->takeFirst());
}

void qt_core_c_QList_QUrl_takeLast_to_output(QList< QUrl >* this_ptr, QUrl* output) {
  new(output) QUrl(this_ptr->takeLast());
}

void qt_core_c_QList_QUrl_value_to_output_i(const QList< QUrl >* this_ptr, int i, QUrl* output) {
  new(output) QUrl(this_ptr->value(i));
}

void qt_core_c_QList_QUrl_value_to_output_i_defaultValue(const QList< QUrl >* this_ptr, int i, const QUrl* defaultValue, QUrl* output) {
  new(output) QUrl(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QList_QVariant_append_QList_QVariant(QList< QVariant >* this_ptr, const QList< QVariant >* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_QVariant_append_QVariant(QList< QVariant >* this_ptr, const QVariant* t) {
  this_ptr->append(*t);
}

const QVariant* qt_core_c_QList_QVariant_at(const QList< QVariant >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QVariant* qt_core_c_QList_QVariant_back(QList< QVariant >* this_ptr) {
  return &this_ptr->back();
}

const QVariant* qt_core_c_QList_QVariant_back_const(const QList< QVariant >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_QVariant_clear(QList< QVariant >* this_ptr) {
  this_ptr->clear();
}

const QVariant* qt_core_c_QList_QVariant_constFirst(const QList< QVariant >* this_ptr) {
  return &this_ptr->constFirst();
}

const QVariant* qt_core_c_QList_QVariant_constLast(const QList< QVariant >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_QVariant_constructor_l(const QList< QVariant >* l, QList< QVariant >* output) {
  new(output) QList< QVariant >(*l);
}

void qt_core_c_QList_QVariant_constructor_no_args(QList< QVariant >* output) {
  new(output) QList< QVariant >();
}

bool qt_core_c_QList_QVariant_contains(const QList< QVariant >* this_ptr, const QVariant* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_QVariant_count_no_args(const QList< QVariant >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_QVariant_count_t(const QList< QVariant >* this_ptr, const QVariant* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_QVariant_destructor(QList< QVariant >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_QVariant_empty(const QList< QVariant >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_QVariant_endsWith(const QList< QVariant >* this_ptr, const QVariant* t) {
  return this_ptr->endsWith(*t);
}

QVariant* qt_core_c_QList_QVariant_first(QList< QVariant >* this_ptr) {
  return &this_ptr->first();
}

const QVariant* qt_core_c_QList_QVariant_first_const(const QList< QVariant >* this_ptr) {
  return &this_ptr->first();
}

QVariant* qt_core_c_QList_QVariant_front(QList< QVariant >* this_ptr) {
  return &this_ptr->front();
}

const QVariant* qt_core_c_QList_QVariant_front_const(const QList< QVariant >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_QVariant_indexOf_t(const QList< QVariant >* this_ptr, const QVariant* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_QVariant_indexOf_t_from(const QList< QVariant >* this_ptr, const QVariant* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_QVariant_insert(QList< QVariant >* this_ptr, int i, const QVariant* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_QVariant_isEmpty(const QList< QVariant >* this_ptr) {
  return this_ptr->isEmpty();
}

QVariant* qt_core_c_QList_QVariant_last(QList< QVariant >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QVariant_lastIndexOf_t(const QList< QVariant >* this_ptr, const QVariant* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_QVariant_lastIndexOf_t_from(const QList< QVariant >* this_ptr, const QVariant* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QVariant* qt_core_c_QList_QVariant_last_const(const QList< QVariant >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_QVariant_length(const QList< QVariant >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_QVariant_mid_to_output_pos(const QList< QVariant >* this_ptr, int pos, QList< QVariant >* output) {
  new(output) QList< QVariant >(this_ptr->mid(pos));
}

void qt_core_c_QList_QVariant_mid_to_output_pos_length(const QList< QVariant >* this_ptr, int pos, int length, QList< QVariant >* output) {
  new(output) QList< QVariant >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_QVariant_move(QList< QVariant >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QVariant >* qt_core_c_QList_QVariant_operator_add_assign_l(QList< QVariant >* this_ptr, const QList< QVariant >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QVariant >* qt_core_c_QList_QVariant_operator_add_assign_t(QList< QVariant >* this_ptr, const QVariant* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_QVariant_operator_add_to_output(const QList< QVariant >* this_ptr, const QList< QVariant >* l, QList< QVariant >* output) {
  new(output) QList< QVariant >(this_ptr->operator+(*l));
}

QList< QVariant >* qt_core_c_QList_QVariant_operator_assign(QList< QVariant >* this_ptr, const QList< QVariant >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_QVariant_operator_eq(const QList< QVariant >* this_ptr, const QList< QVariant >* l) {
  return this_ptr->operator==(*l);
}

QVariant* qt_core_c_QList_QVariant_operator_index(QList< QVariant >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QVariant* qt_core_c_QList_QVariant_operator_index_const(const QList< QVariant >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_QVariant_operator_neq(const QList< QVariant >* this_ptr, const QList< QVariant >* l) {
  return this_ptr->operator!=(*l);
}

QList< QVariant >* qt_core_c_QList_QVariant_operator_shl_l(QList< QVariant >* this_ptr, const QList< QVariant >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QVariant >* qt_core_c_QList_QVariant_operator_shl_t(QList< QVariant >* this_ptr, const QVariant* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_QVariant_pop_back(QList< QVariant >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_QVariant_pop_front(QList< QVariant >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_QVariant_prepend(QList< QVariant >* this_ptr, const QVariant* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_QVariant_push_back(QList< QVariant >* this_ptr, const QVariant* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_QVariant_push_front(QList< QVariant >* this_ptr, const QVariant* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_QVariant_removeAll(QList< QVariant >* this_ptr, const QVariant* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_QVariant_removeAt(QList< QVariant >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_QVariant_removeFirst(QList< QVariant >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_QVariant_removeLast(QList< QVariant >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_QVariant_removeOne(QList< QVariant >* this_ptr, const QVariant* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_QVariant_replace(QList< QVariant >* this_ptr, int i, const QVariant* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_QVariant_reserve(QList< QVariant >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_QVariant_size(const QList< QVariant >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_QVariant_startsWith(const QList< QVariant >* this_ptr, const QVariant* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_QVariant_swap_i_j(QList< QVariant >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_QVariant_swap_other(QList< QVariant >* this_ptr, QList< QVariant >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QList_QVariant_takeAt_to_output(QList< QVariant >* this_ptr, int i, QVariant* output) {
  new(output) QVariant(this_ptr->takeAt(i));
}

void qt_core_c_QList_QVariant_takeFirst_to_output(QList< QVariant >* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->takeFirst());
}

void qt_core_c_QList_QVariant_takeLast_to_output(QList< QVariant >* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->takeLast());
}

void qt_core_c_QList_QVariant_value_to_output_i(const QList< QVariant >* this_ptr, int i, QVariant* output) {
  new(output) QVariant(this_ptr->value(i));
}

void qt_core_c_QList_QVariant_value_to_output_i_defaultValue(const QList< QVariant >* this_ptr, int i, const QVariant* defaultValue, QVariant* output) {
  new(output) QVariant(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QList_Qt_DayOfWeek_append_QList_Qt_DayOfWeek(QList< Qt::DayOfWeek >* this_ptr, const QList< Qt::DayOfWeek >* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_Qt_DayOfWeek_append_Qt_DayOfWeek(QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t) {
  this_ptr->append(*t);
}

const Qt::DayOfWeek* qt_core_c_QList_Qt_DayOfWeek_at(const QList< Qt::DayOfWeek >* this_ptr, int i) {
  return &this_ptr->at(i);
}

Qt::DayOfWeek* qt_core_c_QList_Qt_DayOfWeek_back(QList< Qt::DayOfWeek >* this_ptr) {
  return &this_ptr->back();
}

const Qt::DayOfWeek* qt_core_c_QList_Qt_DayOfWeek_back_const(const QList< Qt::DayOfWeek >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_Qt_DayOfWeek_clear(QList< Qt::DayOfWeek >* this_ptr) {
  this_ptr->clear();
}

const Qt::DayOfWeek* qt_core_c_QList_Qt_DayOfWeek_constFirst(const QList< Qt::DayOfWeek >* this_ptr) {
  return &this_ptr->constFirst();
}

const Qt::DayOfWeek* qt_core_c_QList_Qt_DayOfWeek_constLast(const QList< Qt::DayOfWeek >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_Qt_DayOfWeek_constructor_l(const QList< Qt::DayOfWeek >* l, QList< Qt::DayOfWeek >* output) {
  new(output) QList< Qt::DayOfWeek >(*l);
}

void qt_core_c_QList_Qt_DayOfWeek_constructor_no_args(QList< Qt::DayOfWeek >* output) {
  new(output) QList< Qt::DayOfWeek >();
}

bool qt_core_c_QList_Qt_DayOfWeek_contains(const QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_Qt_DayOfWeek_count_no_args(const QList< Qt::DayOfWeek >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_Qt_DayOfWeek_count_t(const QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_Qt_DayOfWeek_destructor(QList< Qt::DayOfWeek >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_Qt_DayOfWeek_empty(const QList< Qt::DayOfWeek >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_Qt_DayOfWeek_endsWith(const QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t) {
  return this_ptr->endsWith(*t);
}

Qt::DayOfWeek* qt_core_c_QList_Qt_DayOfWeek_first(QList< Qt::DayOfWeek >* this_ptr) {
  return &this_ptr->first();
}

const Qt::DayOfWeek* qt_core_c_QList_Qt_DayOfWeek_first_const(const QList< Qt::DayOfWeek >* this_ptr) {
  return &this_ptr->first();
}

Qt::DayOfWeek* qt_core_c_QList_Qt_DayOfWeek_front(QList< Qt::DayOfWeek >* this_ptr) {
  return &this_ptr->front();
}

const Qt::DayOfWeek* qt_core_c_QList_Qt_DayOfWeek_front_const(const QList< Qt::DayOfWeek >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_Qt_DayOfWeek_indexOf_t(const QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_Qt_DayOfWeek_indexOf_t_from(const QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_Qt_DayOfWeek_insert(QList< Qt::DayOfWeek >* this_ptr, int i, const Qt::DayOfWeek* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_Qt_DayOfWeek_isEmpty(const QList< Qt::DayOfWeek >* this_ptr) {
  return this_ptr->isEmpty();
}

Qt::DayOfWeek* qt_core_c_QList_Qt_DayOfWeek_last(QList< Qt::DayOfWeek >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_Qt_DayOfWeek_lastIndexOf_t(const QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_Qt_DayOfWeek_lastIndexOf_t_from(const QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const Qt::DayOfWeek* qt_core_c_QList_Qt_DayOfWeek_last_const(const QList< Qt::DayOfWeek >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_Qt_DayOfWeek_length(const QList< Qt::DayOfWeek >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_Qt_DayOfWeek_mid_to_output_pos(const QList< Qt::DayOfWeek >* this_ptr, int pos, QList< Qt::DayOfWeek >* output) {
  new(output) QList< Qt::DayOfWeek >(this_ptr->mid(pos));
}

void qt_core_c_QList_Qt_DayOfWeek_mid_to_output_pos_length(const QList< Qt::DayOfWeek >* this_ptr, int pos, int length, QList< Qt::DayOfWeek >* output) {
  new(output) QList< Qt::DayOfWeek >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_Qt_DayOfWeek_move(QList< Qt::DayOfWeek >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< Qt::DayOfWeek >* qt_core_c_QList_Qt_DayOfWeek_operator_add_assign_l(QList< Qt::DayOfWeek >* this_ptr, const QList< Qt::DayOfWeek >* l) {
  return &this_ptr->operator+=(*l);
}

QList< Qt::DayOfWeek >* qt_core_c_QList_Qt_DayOfWeek_operator_add_assign_t(QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_Qt_DayOfWeek_operator_add_to_output(const QList< Qt::DayOfWeek >* this_ptr, const QList< Qt::DayOfWeek >* l, QList< Qt::DayOfWeek >* output) {
  new(output) QList< Qt::DayOfWeek >(this_ptr->operator+(*l));
}

QList< Qt::DayOfWeek >* qt_core_c_QList_Qt_DayOfWeek_operator_assign(QList< Qt::DayOfWeek >* this_ptr, const QList< Qt::DayOfWeek >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_Qt_DayOfWeek_operator_eq(const QList< Qt::DayOfWeek >* this_ptr, const QList< Qt::DayOfWeek >* l) {
  return this_ptr->operator==(*l);
}

Qt::DayOfWeek* qt_core_c_QList_Qt_DayOfWeek_operator_index(QList< Qt::DayOfWeek >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const Qt::DayOfWeek* qt_core_c_QList_Qt_DayOfWeek_operator_index_const(const QList< Qt::DayOfWeek >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_Qt_DayOfWeek_operator_neq(const QList< Qt::DayOfWeek >* this_ptr, const QList< Qt::DayOfWeek >* l) {
  return this_ptr->operator!=(*l);
}

QList< Qt::DayOfWeek >* qt_core_c_QList_Qt_DayOfWeek_operator_shl_l(QList< Qt::DayOfWeek >* this_ptr, const QList< Qt::DayOfWeek >* l) {
  return &this_ptr->operator<<(*l);
}

QList< Qt::DayOfWeek >* qt_core_c_QList_Qt_DayOfWeek_operator_shl_t(QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_Qt_DayOfWeek_pop_back(QList< Qt::DayOfWeek >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_Qt_DayOfWeek_pop_front(QList< Qt::DayOfWeek >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_Qt_DayOfWeek_prepend(QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_Qt_DayOfWeek_push_back(QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_Qt_DayOfWeek_push_front(QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_Qt_DayOfWeek_removeAll(QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_Qt_DayOfWeek_removeAt(QList< Qt::DayOfWeek >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_Qt_DayOfWeek_removeFirst(QList< Qt::DayOfWeek >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_Qt_DayOfWeek_removeLast(QList< Qt::DayOfWeek >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_Qt_DayOfWeek_removeOne(QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_Qt_DayOfWeek_replace(QList< Qt::DayOfWeek >* this_ptr, int i, const Qt::DayOfWeek* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_Qt_DayOfWeek_reserve(QList< Qt::DayOfWeek >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_Qt_DayOfWeek_size(const QList< Qt::DayOfWeek >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_Qt_DayOfWeek_startsWith(const QList< Qt::DayOfWeek >* this_ptr, const Qt::DayOfWeek* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_Qt_DayOfWeek_swap_i_j(QList< Qt::DayOfWeek >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_Qt_DayOfWeek_swap_other(QList< Qt::DayOfWeek >* this_ptr, QList< Qt::DayOfWeek >* other) {
  this_ptr->swap(*other);
}

Qt::DayOfWeek qt_core_c_QList_Qt_DayOfWeek_takeAt(QList< Qt::DayOfWeek >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

Qt::DayOfWeek qt_core_c_QList_Qt_DayOfWeek_takeFirst(QList< Qt::DayOfWeek >* this_ptr) {
  return this_ptr->takeFirst();
}

Qt::DayOfWeek qt_core_c_QList_Qt_DayOfWeek_takeLast(QList< Qt::DayOfWeek >* this_ptr) {
  return this_ptr->takeLast();
}

Qt::DayOfWeek qt_core_c_QList_Qt_DayOfWeek_value_i(const QList< Qt::DayOfWeek >* this_ptr, int i) {
  return this_ptr->value(i);
}

Qt::DayOfWeek qt_core_c_QList_Qt_DayOfWeek_value_i_defaultValue(const QList< Qt::DayOfWeek >* this_ptr, int i, const Qt::DayOfWeek* defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_core_c_QList_int_append_QList_int(QList< int >* this_ptr, const QList< int >* t) {
  this_ptr->append(*t);
}

void qt_core_c_QList_int_append_int(QList< int >* this_ptr, const int* t) {
  this_ptr->append(*t);
}

const int* qt_core_c_QList_int_at(const QList< int >* this_ptr, int i) {
  return &this_ptr->at(i);
}

int* qt_core_c_QList_int_back(QList< int >* this_ptr) {
  return &this_ptr->back();
}

const int* qt_core_c_QList_int_back_const(const QList< int >* this_ptr) {
  return &this_ptr->back();
}

void qt_core_c_QList_int_clear(QList< int >* this_ptr) {
  this_ptr->clear();
}

const int* qt_core_c_QList_int_constFirst(const QList< int >* this_ptr) {
  return &this_ptr->constFirst();
}

const int* qt_core_c_QList_int_constLast(const QList< int >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QList_int_constructor_l(const QList< int >* l, QList< int >* output) {
  new(output) QList< int >(*l);
}

void qt_core_c_QList_int_constructor_no_args(QList< int >* output) {
  new(output) QList< int >();
}

bool qt_core_c_QList_int_contains(const QList< int >* this_ptr, const int* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QList_int_count_no_args(const QList< int >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QList_int_count_t(const QList< int >* this_ptr, const int* t) {
  return this_ptr->count(*t);
}

void qt_core_c_QList_int_destructor(QList< int >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QList_int_empty(const QList< int >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QList_int_endsWith(const QList< int >* this_ptr, const int* t) {
  return this_ptr->endsWith(*t);
}

int* qt_core_c_QList_int_first(QList< int >* this_ptr) {
  return &this_ptr->first();
}

const int* qt_core_c_QList_int_first_const(const QList< int >* this_ptr) {
  return &this_ptr->first();
}

void qt_core_c_QList_int_fromVector_to_output(const QVector< int >* vector, QList< int >* output) {
  new(output) QList< int >(QList< int >::fromVector(*vector));
}

int* qt_core_c_QList_int_front(QList< int >* this_ptr) {
  return &this_ptr->front();
}

const int* qt_core_c_QList_int_front_const(const QList< int >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QList_int_indexOf_t(const QList< int >* this_ptr, const int* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QList_int_indexOf_t_from(const QList< int >* this_ptr, const int* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QList_int_insert(QList< int >* this_ptr, int i, const int* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QList_int_isEmpty(const QList< int >* this_ptr) {
  return this_ptr->isEmpty();
}

int* qt_core_c_QList_int_last(QList< int >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_int_lastIndexOf_t(const QList< int >* this_ptr, const int* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QList_int_lastIndexOf_t_from(const QList< int >* this_ptr, const int* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const int* qt_core_c_QList_int_last_const(const QList< int >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QList_int_length(const QList< int >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QList_int_mid_to_output_pos(const QList< int >* this_ptr, int pos, QList< int >* output) {
  new(output) QList< int >(this_ptr->mid(pos));
}

void qt_core_c_QList_int_mid_to_output_pos_length(const QList< int >* this_ptr, int pos, int length, QList< int >* output) {
  new(output) QList< int >(this_ptr->mid(pos, length));
}

void qt_core_c_QList_int_move(QList< int >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< int >* qt_core_c_QList_int_operator_add_assign_l(QList< int >* this_ptr, const QList< int >* l) {
  return &this_ptr->operator+=(*l);
}

QList< int >* qt_core_c_QList_int_operator_add_assign_t(QList< int >* this_ptr, const int* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QList_int_operator_add_to_output(const QList< int >* this_ptr, const QList< int >* l, QList< int >* output) {
  new(output) QList< int >(this_ptr->operator+(*l));
}

QList< int >* qt_core_c_QList_int_operator_assign(QList< int >* this_ptr, const QList< int >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_core_c_QList_int_operator_eq(const QList< int >* this_ptr, const QList< int >* l) {
  return this_ptr->operator==(*l);
}

int* qt_core_c_QList_int_operator_index(QList< int >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const int* qt_core_c_QList_int_operator_index_const(const QList< int >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QList_int_operator_neq(const QList< int >* this_ptr, const QList< int >* l) {
  return this_ptr->operator!=(*l);
}

QList< int >* qt_core_c_QList_int_operator_shl_l(QList< int >* this_ptr, const QList< int >* l) {
  return &this_ptr->operator<<(*l);
}

QList< int >* qt_core_c_QList_int_operator_shl_t(QList< int >* this_ptr, const int* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QList_int_pop_back(QList< int >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QList_int_pop_front(QList< int >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QList_int_prepend(QList< int >* this_ptr, const int* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QList_int_push_back(QList< int >* this_ptr, const int* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QList_int_push_front(QList< int >* this_ptr, const int* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QList_int_removeAll(QList< int >* this_ptr, const int* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QList_int_removeAt(QList< int >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QList_int_removeFirst(QList< int >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QList_int_removeLast(QList< int >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QList_int_removeOne(QList< int >* this_ptr, const int* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QList_int_replace(QList< int >* this_ptr, int i, const int* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QList_int_reserve(QList< int >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_core_c_QList_int_size(const QList< int >* this_ptr) {
  return this_ptr->size();
}

bool qt_core_c_QList_int_startsWith(const QList< int >* this_ptr, const int* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QList_int_swap_i_j(QList< int >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_core_c_QList_int_swap_other(QList< int >* this_ptr, QList< int >* other) {
  this_ptr->swap(*other);
}

int qt_core_c_QList_int_takeAt(QList< int >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

int qt_core_c_QList_int_takeFirst(QList< int >* this_ptr) {
  return this_ptr->takeFirst();
}

int qt_core_c_QList_int_takeLast(QList< int >* this_ptr) {
  return this_ptr->takeLast();
}

void qt_core_c_QList_int_toVector_to_output(const QList< int >* this_ptr, QVector< int >* output) {
  new(output) QVector< int >(this_ptr->toVector());
}

int qt_core_c_QList_int_value_i(const QList< int >* this_ptr, int i) {
  return this_ptr->value(i);
}

int qt_core_c_QList_int_value_i_defaultValue(const QList< int >* this_ptr, int i, const int* defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

