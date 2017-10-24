#include "qt_core_c_QVector.h"

void qt_core_c_QVector_QPair_double_QVariant_append_l(QVector< QPair< double, QVariant > >* this_ptr, const QVector< QPair< double, QVariant > >* l) {
  this_ptr->append(*l);
}

void qt_core_c_QVector_QPair_double_QVariant_append_t(QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  this_ptr->append(*t);
}

const QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_at(const QVector< QPair< double, QVariant > >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_back(QVector< QPair< double, QVariant > >* this_ptr) {
  return &this_ptr->back();
}

const QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_back_const(const QVector< QPair< double, QVariant > >* this_ptr) {
  return &this_ptr->back();
}

int qt_core_c_QVector_QPair_double_QVariant_capacity(const QVector< QPair< double, QVariant > >* this_ptr) {
  return this_ptr->capacity();
}

void qt_core_c_QVector_QPair_double_QVariant_clear(QVector< QPair< double, QVariant > >* this_ptr) {
  this_ptr->clear();
}

const QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_constData(const QVector< QPair< double, QVariant > >* this_ptr) {
  return this_ptr->constData();
}

const QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_constFirst(const QVector< QPair< double, QVariant > >* this_ptr) {
  return &this_ptr->constFirst();
}

const QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_constLast(const QVector< QPair< double, QVariant > >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QVector_QPair_double_QVariant_constructor_no_args(QVector< QPair< double, QVariant > >* output) {
  new(output) QVector< QPair< double, QVariant > >();
}

void qt_core_c_QVector_QPair_double_QVariant_constructor_size(int size, QVector< QPair< double, QVariant > >* output) {
  new(output) QVector< QPair< double, QVariant > >(size);
}

void qt_core_c_QVector_QPair_double_QVariant_constructor_size_t(int size, const QPair< double, QVariant >* t, QVector< QPair< double, QVariant > >* output) {
  new(output) QVector< QPair< double, QVariant > >(size, *t);
}

void qt_core_c_QVector_QPair_double_QVariant_constructor_v(const QVector< QPair< double, QVariant > >* v, QVector< QPair< double, QVariant > >* output) {
  new(output) QVector< QPair< double, QVariant > >(*v);
}

bool qt_core_c_QVector_QPair_double_QVariant_contains(const QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QVector_QPair_double_QVariant_count_no_args(const QVector< QPair< double, QVariant > >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QVector_QPair_double_QVariant_count_t(const QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  return this_ptr->count(*t);
}

QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_data(QVector< QPair< double, QVariant > >* this_ptr) {
  return this_ptr->data();
}

const QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_data_const(const QVector< QPair< double, QVariant > >* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QVector_QPair_double_QVariant_destructor(QVector< QPair< double, QVariant > >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QVector_QPair_double_QVariant_empty(const QVector< QPair< double, QVariant > >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QVector_QPair_double_QVariant_endsWith(const QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  return this_ptr->endsWith(*t);
}

QVector< QPair< double, QVariant > >* qt_core_c_QVector_QPair_double_QVariant_fill_t(QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  return &this_ptr->fill(*t);
}

QVector< QPair< double, QVariant > >* qt_core_c_QVector_QPair_double_QVariant_fill_t_size(QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t, int size) {
  return &this_ptr->fill(*t, size);
}

QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_first(QVector< QPair< double, QVariant > >* this_ptr) {
  return &this_ptr->first();
}

const QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_first_const(const QVector< QPair< double, QVariant > >* this_ptr) {
  return &this_ptr->first();
}

QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_front(QVector< QPair< double, QVariant > >* this_ptr) {
  return &this_ptr->front();
}

const QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_front_const(const QVector< QPair< double, QVariant > >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QVector_QPair_double_QVariant_indexOf_t(const QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QVector_QPair_double_QVariant_indexOf_t_from(const QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QVector_QPair_double_QVariant_insert_i_n_t(QVector< QPair< double, QVariant > >* this_ptr, int i, int n, const QPair< double, QVariant >* t) {
  this_ptr->insert(i, n, *t);
}

void qt_core_c_QVector_QPair_double_QVariant_insert_i_t(QVector< QPair< double, QVariant > >* this_ptr, int i, const QPair< double, QVariant >* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QVector_QPair_double_QVariant_isEmpty(const QVector< QPair< double, QVariant > >* this_ptr) {
  return this_ptr->isEmpty();
}

QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_last(QVector< QPair< double, QVariant > >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QPair_double_QVariant_lastIndexOf_t(const QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QVector_QPair_double_QVariant_lastIndexOf_t_from(const QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_last_const(const QVector< QPair< double, QVariant > >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QPair_double_QVariant_length(const QVector< QPair< double, QVariant > >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QVector_QPair_double_QVariant_mid_to_output_pos(const QVector< QPair< double, QVariant > >* this_ptr, int pos, QVector< QPair< double, QVariant > >* output) {
  new(output) QVector< QPair< double, QVariant > >(this_ptr->mid(pos));
}

void qt_core_c_QVector_QPair_double_QVariant_mid_to_output_pos_len(const QVector< QPair< double, QVariant > >* this_ptr, int pos, int len, QVector< QPair< double, QVariant > >* output) {
  new(output) QVector< QPair< double, QVariant > >(this_ptr->mid(pos, len));
}

void qt_core_c_QVector_QPair_double_QVariant_move(QVector< QPair< double, QVariant > >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QPair< double, QVariant > >* qt_core_c_QVector_QPair_double_QVariant_operator_add_assign_l(QVector< QPair< double, QVariant > >* this_ptr, const QVector< QPair< double, QVariant > >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QPair< double, QVariant > >* qt_core_c_QVector_QPair_double_QVariant_operator_add_assign_t(QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QVector_QPair_double_QVariant_operator_add_to_output(const QVector< QPair< double, QVariant > >* this_ptr, const QVector< QPair< double, QVariant > >* l, QVector< QPair< double, QVariant > >* output) {
  new(output) QVector< QPair< double, QVariant > >(this_ptr->operator+(*l));
}

QVector< QPair< double, QVariant > >* qt_core_c_QVector_QPair_double_QVariant_operator_assign(QVector< QPair< double, QVariant > >* this_ptr, const QVector< QPair< double, QVariant > >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_core_c_QVector_QPair_double_QVariant_operator_eq(const QVector< QPair< double, QVariant > >* this_ptr, const QVector< QPair< double, QVariant > >* v) {
  return this_ptr->operator==(*v);
}

QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_operator_index(QVector< QPair< double, QVariant > >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QPair< double, QVariant >* qt_core_c_QVector_QPair_double_QVariant_operator_index_const(const QVector< QPair< double, QVariant > >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QVector_QPair_double_QVariant_operator_neq(const QVector< QPair< double, QVariant > >* this_ptr, const QVector< QPair< double, QVariant > >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QPair< double, QVariant > >* qt_core_c_QVector_QPair_double_QVariant_operator_shl_l(QVector< QPair< double, QVariant > >* this_ptr, const QVector< QPair< double, QVariant > >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QPair< double, QVariant > >* qt_core_c_QVector_QPair_double_QVariant_operator_shl_t(QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QVector_QPair_double_QVariant_pop_back(QVector< QPair< double, QVariant > >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QVector_QPair_double_QVariant_pop_front(QVector< QPair< double, QVariant > >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QVector_QPair_double_QVariant_prepend(QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QVector_QPair_double_QVariant_push_back(QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QVector_QPair_double_QVariant_push_front(QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QVector_QPair_double_QVariant_removeAll(QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QVector_QPair_double_QVariant_removeAt(QVector< QPair< double, QVariant > >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QVector_QPair_double_QVariant_removeFirst(QVector< QPair< double, QVariant > >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QVector_QPair_double_QVariant_removeLast(QVector< QPair< double, QVariant > >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QVector_QPair_double_QVariant_removeOne(QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QVector_QPair_double_QVariant_remove_i(QVector< QPair< double, QVariant > >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_core_c_QVector_QPair_double_QVariant_remove_i_n(QVector< QPair< double, QVariant > >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_core_c_QVector_QPair_double_QVariant_replace(QVector< QPair< double, QVariant > >* this_ptr, int i, const QPair< double, QVariant >* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QVector_QPair_double_QVariant_reserve(QVector< QPair< double, QVariant > >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_core_c_QVector_QPair_double_QVariant_resize(QVector< QPair< double, QVariant > >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_core_c_QVector_QPair_double_QVariant_size(const QVector< QPair< double, QVariant > >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QVector_QPair_double_QVariant_squeeze(QVector< QPair< double, QVariant > >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_core_c_QVector_QPair_double_QVariant_startsWith(const QVector< QPair< double, QVariant > >* this_ptr, const QPair< double, QVariant >* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QVector_QPair_double_QVariant_swap(QVector< QPair< double, QVariant > >* this_ptr, QVector< QPair< double, QVariant > >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QVector_QPair_double_QVariant_takeAt_to_output(QVector< QPair< double, QVariant > >* this_ptr, int i, QPair< double, QVariant >* output) {
  new(output) QPair< double, QVariant >(this_ptr->takeAt(i));
}

void qt_core_c_QVector_QPair_double_QVariant_takeFirst_to_output(QVector< QPair< double, QVariant > >* this_ptr, QPair< double, QVariant >* output) {
  new(output) QPair< double, QVariant >(this_ptr->takeFirst());
}

void qt_core_c_QVector_QPair_double_QVariant_takeLast_to_output(QVector< QPair< double, QVariant > >* this_ptr, QPair< double, QVariant >* output) {
  new(output) QPair< double, QVariant >(this_ptr->takeLast());
}

void qt_core_c_QVector_QPair_double_QVariant_value_to_output_i(const QVector< QPair< double, QVariant > >* this_ptr, int i, QPair< double, QVariant >* output) {
  new(output) QPair< double, QVariant >(this_ptr->value(i));
}

void qt_core_c_QVector_QPair_double_QVariant_value_to_output_i_defaultValue(const QVector< QPair< double, QVariant > >* this_ptr, int i, const QPair< double, QVariant >* defaultValue, QPair< double, QVariant >* output) {
  new(output) QPair< double, QVariant >(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QVector_QPointF_append_l(QVector< QPointF >* this_ptr, const QVector< QPointF >* l) {
  this_ptr->append(*l);
}

void qt_core_c_QVector_QPointF_append_t(QVector< QPointF >* this_ptr, const QPointF* t) {
  this_ptr->append(*t);
}

const QPointF* qt_core_c_QVector_QPointF_at(const QVector< QPointF >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QPointF* qt_core_c_QVector_QPointF_back(QVector< QPointF >* this_ptr) {
  return &this_ptr->back();
}

const QPointF* qt_core_c_QVector_QPointF_back_const(const QVector< QPointF >* this_ptr) {
  return &this_ptr->back();
}

int qt_core_c_QVector_QPointF_capacity(const QVector< QPointF >* this_ptr) {
  return this_ptr->capacity();
}

void qt_core_c_QVector_QPointF_clear(QVector< QPointF >* this_ptr) {
  this_ptr->clear();
}

const QPointF* qt_core_c_QVector_QPointF_constData(const QVector< QPointF >* this_ptr) {
  return this_ptr->constData();
}

const QPointF* qt_core_c_QVector_QPointF_constFirst(const QVector< QPointF >* this_ptr) {
  return &this_ptr->constFirst();
}

const QPointF* qt_core_c_QVector_QPointF_constLast(const QVector< QPointF >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QVector_QPointF_constructor_no_args(QVector< QPointF >* output) {
  new(output) QVector< QPointF >();
}

void qt_core_c_QVector_QPointF_constructor_size(int size, QVector< QPointF >* output) {
  new(output) QVector< QPointF >(size);
}

void qt_core_c_QVector_QPointF_constructor_size_t(int size, const QPointF* t, QVector< QPointF >* output) {
  new(output) QVector< QPointF >(size, *t);
}

void qt_core_c_QVector_QPointF_constructor_v(const QVector< QPointF >* v, QVector< QPointF >* output) {
  new(output) QVector< QPointF >(*v);
}

bool qt_core_c_QVector_QPointF_contains(const QVector< QPointF >* this_ptr, const QPointF* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QVector_QPointF_count_no_args(const QVector< QPointF >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QVector_QPointF_count_t(const QVector< QPointF >* this_ptr, const QPointF* t) {
  return this_ptr->count(*t);
}

QPointF* qt_core_c_QVector_QPointF_data(QVector< QPointF >* this_ptr) {
  return this_ptr->data();
}

const QPointF* qt_core_c_QVector_QPointF_data_const(const QVector< QPointF >* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QVector_QPointF_destructor(QVector< QPointF >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QVector_QPointF_empty(const QVector< QPointF >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QVector_QPointF_endsWith(const QVector< QPointF >* this_ptr, const QPointF* t) {
  return this_ptr->endsWith(*t);
}

QVector< QPointF >* qt_core_c_QVector_QPointF_fill_t(QVector< QPointF >* this_ptr, const QPointF* t) {
  return &this_ptr->fill(*t);
}

QVector< QPointF >* qt_core_c_QVector_QPointF_fill_t_size(QVector< QPointF >* this_ptr, const QPointF* t, int size) {
  return &this_ptr->fill(*t, size);
}

QPointF* qt_core_c_QVector_QPointF_first(QVector< QPointF >* this_ptr) {
  return &this_ptr->first();
}

const QPointF* qt_core_c_QVector_QPointF_first_const(const QVector< QPointF >* this_ptr) {
  return &this_ptr->first();
}

QPointF* qt_core_c_QVector_QPointF_front(QVector< QPointF >* this_ptr) {
  return &this_ptr->front();
}

const QPointF* qt_core_c_QVector_QPointF_front_const(const QVector< QPointF >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QVector_QPointF_indexOf_t(const QVector< QPointF >* this_ptr, const QPointF* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QVector_QPointF_indexOf_t_from(const QVector< QPointF >* this_ptr, const QPointF* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QVector_QPointF_insert_i_n_t(QVector< QPointF >* this_ptr, int i, int n, const QPointF* t) {
  this_ptr->insert(i, n, *t);
}

void qt_core_c_QVector_QPointF_insert_i_t(QVector< QPointF >* this_ptr, int i, const QPointF* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QVector_QPointF_isEmpty(const QVector< QPointF >* this_ptr) {
  return this_ptr->isEmpty();
}

QPointF* qt_core_c_QVector_QPointF_last(QVector< QPointF >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QPointF_lastIndexOf_t(const QVector< QPointF >* this_ptr, const QPointF* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QVector_QPointF_lastIndexOf_t_from(const QVector< QPointF >* this_ptr, const QPointF* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QPointF* qt_core_c_QVector_QPointF_last_const(const QVector< QPointF >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QPointF_length(const QVector< QPointF >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QVector_QPointF_mid_to_output_pos(const QVector< QPointF >* this_ptr, int pos, QVector< QPointF >* output) {
  new(output) QVector< QPointF >(this_ptr->mid(pos));
}

void qt_core_c_QVector_QPointF_mid_to_output_pos_len(const QVector< QPointF >* this_ptr, int pos, int len, QVector< QPointF >* output) {
  new(output) QVector< QPointF >(this_ptr->mid(pos, len));
}

void qt_core_c_QVector_QPointF_move(QVector< QPointF >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QPointF >* qt_core_c_QVector_QPointF_operator_add_assign_l(QVector< QPointF >* this_ptr, const QVector< QPointF >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QPointF >* qt_core_c_QVector_QPointF_operator_add_assign_t(QVector< QPointF >* this_ptr, const QPointF* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QVector_QPointF_operator_add_to_output(const QVector< QPointF >* this_ptr, const QVector< QPointF >* l, QVector< QPointF >* output) {
  new(output) QVector< QPointF >(this_ptr->operator+(*l));
}

QVector< QPointF >* qt_core_c_QVector_QPointF_operator_assign(QVector< QPointF >* this_ptr, const QVector< QPointF >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_core_c_QVector_QPointF_operator_eq(const QVector< QPointF >* this_ptr, const QVector< QPointF >* v) {
  return this_ptr->operator==(*v);
}

QPointF* qt_core_c_QVector_QPointF_operator_index(QVector< QPointF >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QPointF* qt_core_c_QVector_QPointF_operator_index_const(const QVector< QPointF >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QVector_QPointF_operator_neq(const QVector< QPointF >* this_ptr, const QVector< QPointF >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QPointF >* qt_core_c_QVector_QPointF_operator_shl_l(QVector< QPointF >* this_ptr, const QVector< QPointF >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QPointF >* qt_core_c_QVector_QPointF_operator_shl_t(QVector< QPointF >* this_ptr, const QPointF* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QVector_QPointF_pop_back(QVector< QPointF >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QVector_QPointF_pop_front(QVector< QPointF >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QVector_QPointF_prepend(QVector< QPointF >* this_ptr, const QPointF* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QVector_QPointF_push_back(QVector< QPointF >* this_ptr, const QPointF* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QVector_QPointF_push_front(QVector< QPointF >* this_ptr, const QPointF* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QVector_QPointF_removeAll(QVector< QPointF >* this_ptr, const QPointF* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QVector_QPointF_removeAt(QVector< QPointF >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QVector_QPointF_removeFirst(QVector< QPointF >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QVector_QPointF_removeLast(QVector< QPointF >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QVector_QPointF_removeOne(QVector< QPointF >* this_ptr, const QPointF* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QVector_QPointF_remove_i(QVector< QPointF >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_core_c_QVector_QPointF_remove_i_n(QVector< QPointF >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_core_c_QVector_QPointF_replace(QVector< QPointF >* this_ptr, int i, const QPointF* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QVector_QPointF_reserve(QVector< QPointF >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_core_c_QVector_QPointF_resize(QVector< QPointF >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_core_c_QVector_QPointF_size(const QVector< QPointF >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QVector_QPointF_squeeze(QVector< QPointF >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_core_c_QVector_QPointF_startsWith(const QVector< QPointF >* this_ptr, const QPointF* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QVector_QPointF_swap(QVector< QPointF >* this_ptr, QVector< QPointF >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QVector_QPointF_takeAt_to_output(QVector< QPointF >* this_ptr, int i, QPointF* output) {
  new(output) QPointF(this_ptr->takeAt(i));
}

void qt_core_c_QVector_QPointF_takeFirst_to_output(QVector< QPointF >* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->takeFirst());
}

void qt_core_c_QVector_QPointF_takeLast_to_output(QVector< QPointF >* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->takeLast());
}

void qt_core_c_QVector_QPointF_value_to_output_i(const QVector< QPointF >* this_ptr, int i, QPointF* output) {
  new(output) QPointF(this_ptr->value(i));
}

void qt_core_c_QVector_QPointF_value_to_output_i_defaultValue(const QVector< QPointF >* this_ptr, int i, const QPointF* defaultValue, QPointF* output) {
  new(output) QPointF(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QVector_QStaticPlugin_append_l(QVector< QStaticPlugin >* this_ptr, const QVector< QStaticPlugin >* l) {
  this_ptr->append(*l);
}

void qt_core_c_QVector_QStaticPlugin_append_t(QVector< QStaticPlugin >* this_ptr, const QStaticPlugin* t) {
  this_ptr->append(*t);
}

const QStaticPlugin* qt_core_c_QVector_QStaticPlugin_at(const QVector< QStaticPlugin >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QStaticPlugin* qt_core_c_QVector_QStaticPlugin_back(QVector< QStaticPlugin >* this_ptr) {
  return &this_ptr->back();
}

const QStaticPlugin* qt_core_c_QVector_QStaticPlugin_back_const(const QVector< QStaticPlugin >* this_ptr) {
  return &this_ptr->back();
}

int qt_core_c_QVector_QStaticPlugin_capacity(const QVector< QStaticPlugin >* this_ptr) {
  return this_ptr->capacity();
}

void qt_core_c_QVector_QStaticPlugin_clear(QVector< QStaticPlugin >* this_ptr) {
  this_ptr->clear();
}

const QStaticPlugin* qt_core_c_QVector_QStaticPlugin_constData(const QVector< QStaticPlugin >* this_ptr) {
  return this_ptr->constData();
}

const QStaticPlugin* qt_core_c_QVector_QStaticPlugin_constFirst(const QVector< QStaticPlugin >* this_ptr) {
  return &this_ptr->constFirst();
}

const QStaticPlugin* qt_core_c_QVector_QStaticPlugin_constLast(const QVector< QStaticPlugin >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QVector_QStaticPlugin_constructor_no_args(QVector< QStaticPlugin >* output) {
  new(output) QVector< QStaticPlugin >();
}

void qt_core_c_QVector_QStaticPlugin_constructor_size(int size, QVector< QStaticPlugin >* output) {
  new(output) QVector< QStaticPlugin >(size);
}

void qt_core_c_QVector_QStaticPlugin_constructor_size_t(int size, const QStaticPlugin* t, QVector< QStaticPlugin >* output) {
  new(output) QVector< QStaticPlugin >(size, *t);
}

void qt_core_c_QVector_QStaticPlugin_constructor_v(const QVector< QStaticPlugin >* v, QVector< QStaticPlugin >* output) {
  new(output) QVector< QStaticPlugin >(*v);
}

int qt_core_c_QVector_QStaticPlugin_count(const QVector< QStaticPlugin >* this_ptr) {
  return this_ptr->count();
}

QStaticPlugin* qt_core_c_QVector_QStaticPlugin_data(QVector< QStaticPlugin >* this_ptr) {
  return this_ptr->data();
}

const QStaticPlugin* qt_core_c_QVector_QStaticPlugin_data_const(const QVector< QStaticPlugin >* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QVector_QStaticPlugin_destructor(QVector< QStaticPlugin >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QVector_QStaticPlugin_empty(const QVector< QStaticPlugin >* this_ptr) {
  return this_ptr->empty();
}

QVector< QStaticPlugin >* qt_core_c_QVector_QStaticPlugin_fill_t(QVector< QStaticPlugin >* this_ptr, const QStaticPlugin* t) {
  return &this_ptr->fill(*t);
}

QVector< QStaticPlugin >* qt_core_c_QVector_QStaticPlugin_fill_t_size(QVector< QStaticPlugin >* this_ptr, const QStaticPlugin* t, int size) {
  return &this_ptr->fill(*t, size);
}

QStaticPlugin* qt_core_c_QVector_QStaticPlugin_first(QVector< QStaticPlugin >* this_ptr) {
  return &this_ptr->first();
}

const QStaticPlugin* qt_core_c_QVector_QStaticPlugin_first_const(const QVector< QStaticPlugin >* this_ptr) {
  return &this_ptr->first();
}

QStaticPlugin* qt_core_c_QVector_QStaticPlugin_front(QVector< QStaticPlugin >* this_ptr) {
  return &this_ptr->front();
}

const QStaticPlugin* qt_core_c_QVector_QStaticPlugin_front_const(const QVector< QStaticPlugin >* this_ptr) {
  return &this_ptr->front();
}

void qt_core_c_QVector_QStaticPlugin_insert_i_n_t(QVector< QStaticPlugin >* this_ptr, int i, int n, const QStaticPlugin* t) {
  this_ptr->insert(i, n, *t);
}

void qt_core_c_QVector_QStaticPlugin_insert_i_t(QVector< QStaticPlugin >* this_ptr, int i, const QStaticPlugin* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QVector_QStaticPlugin_isEmpty(const QVector< QStaticPlugin >* this_ptr) {
  return this_ptr->isEmpty();
}

QStaticPlugin* qt_core_c_QVector_QStaticPlugin_last(QVector< QStaticPlugin >* this_ptr) {
  return &this_ptr->last();
}

const QStaticPlugin* qt_core_c_QVector_QStaticPlugin_last_const(const QVector< QStaticPlugin >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QStaticPlugin_length(const QVector< QStaticPlugin >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QVector_QStaticPlugin_mid_to_output_pos(const QVector< QStaticPlugin >* this_ptr, int pos, QVector< QStaticPlugin >* output) {
  new(output) QVector< QStaticPlugin >(this_ptr->mid(pos));
}

void qt_core_c_QVector_QStaticPlugin_mid_to_output_pos_len(const QVector< QStaticPlugin >* this_ptr, int pos, int len, QVector< QStaticPlugin >* output) {
  new(output) QVector< QStaticPlugin >(this_ptr->mid(pos, len));
}

void qt_core_c_QVector_QStaticPlugin_move(QVector< QStaticPlugin >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QStaticPlugin >* qt_core_c_QVector_QStaticPlugin_operator_add_assign_l(QVector< QStaticPlugin >* this_ptr, const QVector< QStaticPlugin >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QStaticPlugin >* qt_core_c_QVector_QStaticPlugin_operator_add_assign_t(QVector< QStaticPlugin >* this_ptr, const QStaticPlugin* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QVector_QStaticPlugin_operator_add_to_output(const QVector< QStaticPlugin >* this_ptr, const QVector< QStaticPlugin >* l, QVector< QStaticPlugin >* output) {
  new(output) QVector< QStaticPlugin >(this_ptr->operator+(*l));
}

QVector< QStaticPlugin >* qt_core_c_QVector_QStaticPlugin_operator_assign(QVector< QStaticPlugin >* this_ptr, const QVector< QStaticPlugin >* v) {
  return &this_ptr->operator=(*v);
}

QStaticPlugin* qt_core_c_QVector_QStaticPlugin_operator_index(QVector< QStaticPlugin >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QStaticPlugin* qt_core_c_QVector_QStaticPlugin_operator_index_const(const QVector< QStaticPlugin >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QVector< QStaticPlugin >* qt_core_c_QVector_QStaticPlugin_operator_shl_l(QVector< QStaticPlugin >* this_ptr, const QVector< QStaticPlugin >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QStaticPlugin >* qt_core_c_QVector_QStaticPlugin_operator_shl_t(QVector< QStaticPlugin >* this_ptr, const QStaticPlugin* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QVector_QStaticPlugin_pop_back(QVector< QStaticPlugin >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QVector_QStaticPlugin_pop_front(QVector< QStaticPlugin >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QVector_QStaticPlugin_prepend(QVector< QStaticPlugin >* this_ptr, const QStaticPlugin* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QVector_QStaticPlugin_push_back(QVector< QStaticPlugin >* this_ptr, const QStaticPlugin* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QVector_QStaticPlugin_push_front(QVector< QStaticPlugin >* this_ptr, const QStaticPlugin* t) {
  this_ptr->push_front(*t);
}

void qt_core_c_QVector_QStaticPlugin_removeAt(QVector< QStaticPlugin >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QVector_QStaticPlugin_removeFirst(QVector< QStaticPlugin >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QVector_QStaticPlugin_removeLast(QVector< QStaticPlugin >* this_ptr) {
  this_ptr->removeLast();
}

void qt_core_c_QVector_QStaticPlugin_remove_i(QVector< QStaticPlugin >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_core_c_QVector_QStaticPlugin_remove_i_n(QVector< QStaticPlugin >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_core_c_QVector_QStaticPlugin_replace(QVector< QStaticPlugin >* this_ptr, int i, const QStaticPlugin* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QVector_QStaticPlugin_reserve(QVector< QStaticPlugin >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_core_c_QVector_QStaticPlugin_resize(QVector< QStaticPlugin >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_core_c_QVector_QStaticPlugin_size(const QVector< QStaticPlugin >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QVector_QStaticPlugin_squeeze(QVector< QStaticPlugin >* this_ptr) {
  this_ptr->squeeze();
}

void qt_core_c_QVector_QStaticPlugin_swap(QVector< QStaticPlugin >* this_ptr, QVector< QStaticPlugin >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QVector_QStaticPlugin_takeAt_to_output(QVector< QStaticPlugin >* this_ptr, int i, QStaticPlugin* output) {
  new(output) QStaticPlugin(this_ptr->takeAt(i));
}

void qt_core_c_QVector_QStaticPlugin_takeFirst_to_output(QVector< QStaticPlugin >* this_ptr, QStaticPlugin* output) {
  new(output) QStaticPlugin(this_ptr->takeFirst());
}

void qt_core_c_QVector_QStaticPlugin_takeLast_to_output(QVector< QStaticPlugin >* this_ptr, QStaticPlugin* output) {
  new(output) QStaticPlugin(this_ptr->takeLast());
}

void qt_core_c_QVector_QStaticPlugin_value_to_output_i(const QVector< QStaticPlugin >* this_ptr, int i, QStaticPlugin* output) {
  new(output) QStaticPlugin(this_ptr->value(i));
}

void qt_core_c_QVector_QStaticPlugin_value_to_output_i_defaultValue(const QVector< QStaticPlugin >* this_ptr, int i, const QStaticPlugin* defaultValue, QStaticPlugin* output) {
  new(output) QStaticPlugin(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QVector_QStringRef_append_l(QVector< QStringRef >* this_ptr, const QVector< QStringRef >* l) {
  this_ptr->append(*l);
}

void qt_core_c_QVector_QStringRef_append_t(QVector< QStringRef >* this_ptr, const QStringRef* t) {
  this_ptr->append(*t);
}

const QStringRef* qt_core_c_QVector_QStringRef_at(const QVector< QStringRef >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QStringRef* qt_core_c_QVector_QStringRef_back(QVector< QStringRef >* this_ptr) {
  return &this_ptr->back();
}

const QStringRef* qt_core_c_QVector_QStringRef_back_const(const QVector< QStringRef >* this_ptr) {
  return &this_ptr->back();
}

int qt_core_c_QVector_QStringRef_capacity(const QVector< QStringRef >* this_ptr) {
  return this_ptr->capacity();
}

void qt_core_c_QVector_QStringRef_clear(QVector< QStringRef >* this_ptr) {
  this_ptr->clear();
}

const QStringRef* qt_core_c_QVector_QStringRef_constData(const QVector< QStringRef >* this_ptr) {
  return this_ptr->constData();
}

const QStringRef* qt_core_c_QVector_QStringRef_constFirst(const QVector< QStringRef >* this_ptr) {
  return &this_ptr->constFirst();
}

const QStringRef* qt_core_c_QVector_QStringRef_constLast(const QVector< QStringRef >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QVector_QStringRef_constructor_no_args(QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >();
}

void qt_core_c_QVector_QStringRef_constructor_size(int size, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(size);
}

void qt_core_c_QVector_QStringRef_constructor_size_t(int size, const QStringRef* t, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(size, *t);
}

void qt_core_c_QVector_QStringRef_constructor_v(const QVector< QStringRef >* v, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(*v);
}

bool qt_core_c_QVector_QStringRef_contains(const QVector< QStringRef >* this_ptr, const QStringRef* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QVector_QStringRef_count_no_args(const QVector< QStringRef >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QVector_QStringRef_count_t(const QVector< QStringRef >* this_ptr, const QStringRef* t) {
  return this_ptr->count(*t);
}

QStringRef* qt_core_c_QVector_QStringRef_data(QVector< QStringRef >* this_ptr) {
  return this_ptr->data();
}

const QStringRef* qt_core_c_QVector_QStringRef_data_const(const QVector< QStringRef >* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QVector_QStringRef_destructor(QVector< QStringRef >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QVector_QStringRef_empty(const QVector< QStringRef >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QVector_QStringRef_endsWith(const QVector< QStringRef >* this_ptr, const QStringRef* t) {
  return this_ptr->endsWith(*t);
}

QVector< QStringRef >* qt_core_c_QVector_QStringRef_fill_t(QVector< QStringRef >* this_ptr, const QStringRef* t) {
  return &this_ptr->fill(*t);
}

QVector< QStringRef >* qt_core_c_QVector_QStringRef_fill_t_size(QVector< QStringRef >* this_ptr, const QStringRef* t, int size) {
  return &this_ptr->fill(*t, size);
}

QStringRef* qt_core_c_QVector_QStringRef_first(QVector< QStringRef >* this_ptr) {
  return &this_ptr->first();
}

const QStringRef* qt_core_c_QVector_QStringRef_first_const(const QVector< QStringRef >* this_ptr) {
  return &this_ptr->first();
}

QStringRef* qt_core_c_QVector_QStringRef_front(QVector< QStringRef >* this_ptr) {
  return &this_ptr->front();
}

const QStringRef* qt_core_c_QVector_QStringRef_front_const(const QVector< QStringRef >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QVector_QStringRef_indexOf_t(const QVector< QStringRef >* this_ptr, const QStringRef* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QVector_QStringRef_indexOf_t_from(const QVector< QStringRef >* this_ptr, const QStringRef* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QVector_QStringRef_insert_i_n_t(QVector< QStringRef >* this_ptr, int i, int n, const QStringRef* t) {
  this_ptr->insert(i, n, *t);
}

void qt_core_c_QVector_QStringRef_insert_i_t(QVector< QStringRef >* this_ptr, int i, const QStringRef* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QVector_QStringRef_isEmpty(const QVector< QStringRef >* this_ptr) {
  return this_ptr->isEmpty();
}

QStringRef* qt_core_c_QVector_QStringRef_last(QVector< QStringRef >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QStringRef_lastIndexOf_t(const QVector< QStringRef >* this_ptr, const QStringRef* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QVector_QStringRef_lastIndexOf_t_from(const QVector< QStringRef >* this_ptr, const QStringRef* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QStringRef* qt_core_c_QVector_QStringRef_last_const(const QVector< QStringRef >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QStringRef_length(const QVector< QStringRef >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QVector_QStringRef_mid_to_output_pos(const QVector< QStringRef >* this_ptr, int pos, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->mid(pos));
}

void qt_core_c_QVector_QStringRef_mid_to_output_pos_len(const QVector< QStringRef >* this_ptr, int pos, int len, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->mid(pos, len));
}

void qt_core_c_QVector_QStringRef_move(QVector< QStringRef >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QStringRef >* qt_core_c_QVector_QStringRef_operator_add_assign_l(QVector< QStringRef >* this_ptr, const QVector< QStringRef >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QStringRef >* qt_core_c_QVector_QStringRef_operator_add_assign_t(QVector< QStringRef >* this_ptr, const QStringRef* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QVector_QStringRef_operator_add_to_output(const QVector< QStringRef >* this_ptr, const QVector< QStringRef >* l, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->operator+(*l));
}

QVector< QStringRef >* qt_core_c_QVector_QStringRef_operator_assign(QVector< QStringRef >* this_ptr, const QVector< QStringRef >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_core_c_QVector_QStringRef_operator_eq(const QVector< QStringRef >* this_ptr, const QVector< QStringRef >* v) {
  return this_ptr->operator==(*v);
}

QStringRef* qt_core_c_QVector_QStringRef_operator_index(QVector< QStringRef >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QStringRef* qt_core_c_QVector_QStringRef_operator_index_const(const QVector< QStringRef >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QVector_QStringRef_operator_neq(const QVector< QStringRef >* this_ptr, const QVector< QStringRef >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QStringRef >* qt_core_c_QVector_QStringRef_operator_shl_l(QVector< QStringRef >* this_ptr, const QVector< QStringRef >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QStringRef >* qt_core_c_QVector_QStringRef_operator_shl_t(QVector< QStringRef >* this_ptr, const QStringRef* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QVector_QStringRef_pop_back(QVector< QStringRef >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QVector_QStringRef_pop_front(QVector< QStringRef >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QVector_QStringRef_prepend(QVector< QStringRef >* this_ptr, const QStringRef* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QVector_QStringRef_push_back(QVector< QStringRef >* this_ptr, const QStringRef* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QVector_QStringRef_push_front(QVector< QStringRef >* this_ptr, const QStringRef* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QVector_QStringRef_removeAll(QVector< QStringRef >* this_ptr, const QStringRef* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QVector_QStringRef_removeAt(QVector< QStringRef >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QVector_QStringRef_removeFirst(QVector< QStringRef >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QVector_QStringRef_removeLast(QVector< QStringRef >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QVector_QStringRef_removeOne(QVector< QStringRef >* this_ptr, const QStringRef* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QVector_QStringRef_remove_i(QVector< QStringRef >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_core_c_QVector_QStringRef_remove_i_n(QVector< QStringRef >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_core_c_QVector_QStringRef_replace(QVector< QStringRef >* this_ptr, int i, const QStringRef* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QVector_QStringRef_reserve(QVector< QStringRef >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_core_c_QVector_QStringRef_resize(QVector< QStringRef >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_core_c_QVector_QStringRef_size(const QVector< QStringRef >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QVector_QStringRef_squeeze(QVector< QStringRef >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_core_c_QVector_QStringRef_startsWith(const QVector< QStringRef >* this_ptr, const QStringRef* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QVector_QStringRef_swap(QVector< QStringRef >* this_ptr, QVector< QStringRef >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QVector_QStringRef_takeAt_to_output(QVector< QStringRef >* this_ptr, int i, QStringRef* output) {
  new(output) QStringRef(this_ptr->takeAt(i));
}

void qt_core_c_QVector_QStringRef_takeFirst_to_output(QVector< QStringRef >* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->takeFirst());
}

void qt_core_c_QVector_QStringRef_takeLast_to_output(QVector< QStringRef >* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->takeLast());
}

void qt_core_c_QVector_QStringRef_value_to_output_i(const QVector< QStringRef >* this_ptr, int i, QStringRef* output) {
  new(output) QStringRef(this_ptr->value(i));
}

void qt_core_c_QVector_QStringRef_value_to_output_i_defaultValue(const QVector< QStringRef >* this_ptr, int i, const QStringRef* defaultValue, QStringRef* output) {
  new(output) QStringRef(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QVector_QTimeZone_OffsetData_append_l(QVector< QTimeZone::OffsetData >* this_ptr, const QVector< QTimeZone::OffsetData >* l) {
  this_ptr->append(*l);
}

void qt_core_c_QVector_QTimeZone_OffsetData_append_t(QVector< QTimeZone::OffsetData >* this_ptr, const QTimeZone::OffsetData* t) {
  this_ptr->append(*t);
}

const QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_at(const QVector< QTimeZone::OffsetData >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_back(QVector< QTimeZone::OffsetData >* this_ptr) {
  return &this_ptr->back();
}

const QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_back_const(const QVector< QTimeZone::OffsetData >* this_ptr) {
  return &this_ptr->back();
}

int qt_core_c_QVector_QTimeZone_OffsetData_capacity(const QVector< QTimeZone::OffsetData >* this_ptr) {
  return this_ptr->capacity();
}

void qt_core_c_QVector_QTimeZone_OffsetData_clear(QVector< QTimeZone::OffsetData >* this_ptr) {
  this_ptr->clear();
}

const QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_constData(const QVector< QTimeZone::OffsetData >* this_ptr) {
  return this_ptr->constData();
}

const QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_constFirst(const QVector< QTimeZone::OffsetData >* this_ptr) {
  return &this_ptr->constFirst();
}

const QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_constLast(const QVector< QTimeZone::OffsetData >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QVector_QTimeZone_OffsetData_constructor_no_args(QVector< QTimeZone::OffsetData >* output) {
  new(output) QVector< QTimeZone::OffsetData >();
}

void qt_core_c_QVector_QTimeZone_OffsetData_constructor_size(int size, QVector< QTimeZone::OffsetData >* output) {
  new(output) QVector< QTimeZone::OffsetData >(size);
}

void qt_core_c_QVector_QTimeZone_OffsetData_constructor_size_t(int size, const QTimeZone::OffsetData* t, QVector< QTimeZone::OffsetData >* output) {
  new(output) QVector< QTimeZone::OffsetData >(size, *t);
}

void qt_core_c_QVector_QTimeZone_OffsetData_constructor_v(const QVector< QTimeZone::OffsetData >* v, QVector< QTimeZone::OffsetData >* output) {
  new(output) QVector< QTimeZone::OffsetData >(*v);
}

int qt_core_c_QVector_QTimeZone_OffsetData_count(const QVector< QTimeZone::OffsetData >* this_ptr) {
  return this_ptr->count();
}

QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_data(QVector< QTimeZone::OffsetData >* this_ptr) {
  return this_ptr->data();
}

const QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_data_const(const QVector< QTimeZone::OffsetData >* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QVector_QTimeZone_OffsetData_destructor(QVector< QTimeZone::OffsetData >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QVector_QTimeZone_OffsetData_empty(const QVector< QTimeZone::OffsetData >* this_ptr) {
  return this_ptr->empty();
}

QVector< QTimeZone::OffsetData >* qt_core_c_QVector_QTimeZone_OffsetData_fill_t(QVector< QTimeZone::OffsetData >* this_ptr, const QTimeZone::OffsetData* t) {
  return &this_ptr->fill(*t);
}

QVector< QTimeZone::OffsetData >* qt_core_c_QVector_QTimeZone_OffsetData_fill_t_size(QVector< QTimeZone::OffsetData >* this_ptr, const QTimeZone::OffsetData* t, int size) {
  return &this_ptr->fill(*t, size);
}

QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_first(QVector< QTimeZone::OffsetData >* this_ptr) {
  return &this_ptr->first();
}

const QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_first_const(const QVector< QTimeZone::OffsetData >* this_ptr) {
  return &this_ptr->first();
}

QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_front(QVector< QTimeZone::OffsetData >* this_ptr) {
  return &this_ptr->front();
}

const QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_front_const(const QVector< QTimeZone::OffsetData >* this_ptr) {
  return &this_ptr->front();
}

void qt_core_c_QVector_QTimeZone_OffsetData_insert_i_n_t(QVector< QTimeZone::OffsetData >* this_ptr, int i, int n, const QTimeZone::OffsetData* t) {
  this_ptr->insert(i, n, *t);
}

void qt_core_c_QVector_QTimeZone_OffsetData_insert_i_t(QVector< QTimeZone::OffsetData >* this_ptr, int i, const QTimeZone::OffsetData* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QVector_QTimeZone_OffsetData_isEmpty(const QVector< QTimeZone::OffsetData >* this_ptr) {
  return this_ptr->isEmpty();
}

QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_last(QVector< QTimeZone::OffsetData >* this_ptr) {
  return &this_ptr->last();
}

const QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_last_const(const QVector< QTimeZone::OffsetData >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QTimeZone_OffsetData_length(const QVector< QTimeZone::OffsetData >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QVector_QTimeZone_OffsetData_mid_to_output_pos(const QVector< QTimeZone::OffsetData >* this_ptr, int pos, QVector< QTimeZone::OffsetData >* output) {
  new(output) QVector< QTimeZone::OffsetData >(this_ptr->mid(pos));
}

void qt_core_c_QVector_QTimeZone_OffsetData_mid_to_output_pos_len(const QVector< QTimeZone::OffsetData >* this_ptr, int pos, int len, QVector< QTimeZone::OffsetData >* output) {
  new(output) QVector< QTimeZone::OffsetData >(this_ptr->mid(pos, len));
}

void qt_core_c_QVector_QTimeZone_OffsetData_move(QVector< QTimeZone::OffsetData >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QTimeZone::OffsetData >* qt_core_c_QVector_QTimeZone_OffsetData_operator_add_assign_l(QVector< QTimeZone::OffsetData >* this_ptr, const QVector< QTimeZone::OffsetData >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QTimeZone::OffsetData >* qt_core_c_QVector_QTimeZone_OffsetData_operator_add_assign_t(QVector< QTimeZone::OffsetData >* this_ptr, const QTimeZone::OffsetData* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QVector_QTimeZone_OffsetData_operator_add_to_output(const QVector< QTimeZone::OffsetData >* this_ptr, const QVector< QTimeZone::OffsetData >* l, QVector< QTimeZone::OffsetData >* output) {
  new(output) QVector< QTimeZone::OffsetData >(this_ptr->operator+(*l));
}

QVector< QTimeZone::OffsetData >* qt_core_c_QVector_QTimeZone_OffsetData_operator_assign(QVector< QTimeZone::OffsetData >* this_ptr, const QVector< QTimeZone::OffsetData >* v) {
  return &this_ptr->operator=(*v);
}

QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_operator_index(QVector< QTimeZone::OffsetData >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QTimeZone::OffsetData* qt_core_c_QVector_QTimeZone_OffsetData_operator_index_const(const QVector< QTimeZone::OffsetData >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QVector< QTimeZone::OffsetData >* qt_core_c_QVector_QTimeZone_OffsetData_operator_shl_l(QVector< QTimeZone::OffsetData >* this_ptr, const QVector< QTimeZone::OffsetData >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QTimeZone::OffsetData >* qt_core_c_QVector_QTimeZone_OffsetData_operator_shl_t(QVector< QTimeZone::OffsetData >* this_ptr, const QTimeZone::OffsetData* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QVector_QTimeZone_OffsetData_pop_back(QVector< QTimeZone::OffsetData >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QVector_QTimeZone_OffsetData_pop_front(QVector< QTimeZone::OffsetData >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QVector_QTimeZone_OffsetData_prepend(QVector< QTimeZone::OffsetData >* this_ptr, const QTimeZone::OffsetData* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QVector_QTimeZone_OffsetData_push_back(QVector< QTimeZone::OffsetData >* this_ptr, const QTimeZone::OffsetData* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QVector_QTimeZone_OffsetData_push_front(QVector< QTimeZone::OffsetData >* this_ptr, const QTimeZone::OffsetData* t) {
  this_ptr->push_front(*t);
}

void qt_core_c_QVector_QTimeZone_OffsetData_removeAt(QVector< QTimeZone::OffsetData >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QVector_QTimeZone_OffsetData_removeFirst(QVector< QTimeZone::OffsetData >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QVector_QTimeZone_OffsetData_removeLast(QVector< QTimeZone::OffsetData >* this_ptr) {
  this_ptr->removeLast();
}

void qt_core_c_QVector_QTimeZone_OffsetData_remove_i(QVector< QTimeZone::OffsetData >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_core_c_QVector_QTimeZone_OffsetData_remove_i_n(QVector< QTimeZone::OffsetData >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_core_c_QVector_QTimeZone_OffsetData_replace(QVector< QTimeZone::OffsetData >* this_ptr, int i, const QTimeZone::OffsetData* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QVector_QTimeZone_OffsetData_reserve(QVector< QTimeZone::OffsetData >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_core_c_QVector_QTimeZone_OffsetData_resize(QVector< QTimeZone::OffsetData >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_core_c_QVector_QTimeZone_OffsetData_size(const QVector< QTimeZone::OffsetData >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QVector_QTimeZone_OffsetData_squeeze(QVector< QTimeZone::OffsetData >* this_ptr) {
  this_ptr->squeeze();
}

void qt_core_c_QVector_QTimeZone_OffsetData_swap(QVector< QTimeZone::OffsetData >* this_ptr, QVector< QTimeZone::OffsetData >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QVector_QTimeZone_OffsetData_takeAt_to_output(QVector< QTimeZone::OffsetData >* this_ptr, int i, QTimeZone::OffsetData* output) {
  new(output) QTimeZone::OffsetData(this_ptr->takeAt(i));
}

void qt_core_c_QVector_QTimeZone_OffsetData_takeFirst_to_output(QVector< QTimeZone::OffsetData >* this_ptr, QTimeZone::OffsetData* output) {
  new(output) QTimeZone::OffsetData(this_ptr->takeFirst());
}

void qt_core_c_QVector_QTimeZone_OffsetData_takeLast_to_output(QVector< QTimeZone::OffsetData >* this_ptr, QTimeZone::OffsetData* output) {
  new(output) QTimeZone::OffsetData(this_ptr->takeLast());
}

void qt_core_c_QVector_QTimeZone_OffsetData_value_to_output_i(const QVector< QTimeZone::OffsetData >* this_ptr, int i, QTimeZone::OffsetData* output) {
  new(output) QTimeZone::OffsetData(this_ptr->value(i));
}

void qt_core_c_QVector_QTimeZone_OffsetData_value_to_output_i_defaultValue(const QVector< QTimeZone::OffsetData >* this_ptr, int i, const QTimeZone::OffsetData* defaultValue, QTimeZone::OffsetData* output) {
  new(output) QTimeZone::OffsetData(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QVector_QXmlStreamAttribute_append_l(QVector< QXmlStreamAttribute >* this_ptr, const QVector< QXmlStreamAttribute >* l) {
  this_ptr->append(*l);
}

void qt_core_c_QVector_QXmlStreamAttribute_append_t(QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  this_ptr->append(*t);
}

const QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_at(const QVector< QXmlStreamAttribute >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_back(QVector< QXmlStreamAttribute >* this_ptr) {
  return &this_ptr->back();
}

const QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_back_const(const QVector< QXmlStreamAttribute >* this_ptr) {
  return &this_ptr->back();
}

int qt_core_c_QVector_QXmlStreamAttribute_capacity(const QVector< QXmlStreamAttribute >* this_ptr) {
  return this_ptr->capacity();
}

void qt_core_c_QVector_QXmlStreamAttribute_clear(QVector< QXmlStreamAttribute >* this_ptr) {
  this_ptr->clear();
}

const QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_constData(const QVector< QXmlStreamAttribute >* this_ptr) {
  return this_ptr->constData();
}

const QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_constFirst(const QVector< QXmlStreamAttribute >* this_ptr) {
  return &this_ptr->constFirst();
}

const QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_constLast(const QVector< QXmlStreamAttribute >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QVector_QXmlStreamAttribute_constructor_no_args(QVector< QXmlStreamAttribute >* output) {
  new(output) QVector< QXmlStreamAttribute >();
}

void qt_core_c_QVector_QXmlStreamAttribute_constructor_size(int size, QVector< QXmlStreamAttribute >* output) {
  new(output) QVector< QXmlStreamAttribute >(size);
}

void qt_core_c_QVector_QXmlStreamAttribute_constructor_size_t(int size, const QXmlStreamAttribute* t, QVector< QXmlStreamAttribute >* output) {
  new(output) QVector< QXmlStreamAttribute >(size, *t);
}

void qt_core_c_QVector_QXmlStreamAttribute_constructor_v(const QVector< QXmlStreamAttribute >* v, QVector< QXmlStreamAttribute >* output) {
  new(output) QVector< QXmlStreamAttribute >(*v);
}

bool qt_core_c_QVector_QXmlStreamAttribute_contains(const QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QVector_QXmlStreamAttribute_count_no_args(const QVector< QXmlStreamAttribute >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QVector_QXmlStreamAttribute_count_t(const QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  return this_ptr->count(*t);
}

QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_data(QVector< QXmlStreamAttribute >* this_ptr) {
  return this_ptr->data();
}

const QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_data_const(const QVector< QXmlStreamAttribute >* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QVector_QXmlStreamAttribute_destructor(QVector< QXmlStreamAttribute >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QVector_QXmlStreamAttribute_empty(const QVector< QXmlStreamAttribute >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QVector_QXmlStreamAttribute_endsWith(const QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  return this_ptr->endsWith(*t);
}

QVector< QXmlStreamAttribute >* qt_core_c_QVector_QXmlStreamAttribute_fill_t(QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  return &this_ptr->fill(*t);
}

QVector< QXmlStreamAttribute >* qt_core_c_QVector_QXmlStreamAttribute_fill_t_size(QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t, int size) {
  return &this_ptr->fill(*t, size);
}

QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_first(QVector< QXmlStreamAttribute >* this_ptr) {
  return &this_ptr->first();
}

const QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_first_const(const QVector< QXmlStreamAttribute >* this_ptr) {
  return &this_ptr->first();
}

QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_front(QVector< QXmlStreamAttribute >* this_ptr) {
  return &this_ptr->front();
}

const QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_front_const(const QVector< QXmlStreamAttribute >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QVector_QXmlStreamAttribute_indexOf_t(const QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QVector_QXmlStreamAttribute_indexOf_t_from(const QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QVector_QXmlStreamAttribute_insert_i_n_t(QVector< QXmlStreamAttribute >* this_ptr, int i, int n, const QXmlStreamAttribute* t) {
  this_ptr->insert(i, n, *t);
}

void qt_core_c_QVector_QXmlStreamAttribute_insert_i_t(QVector< QXmlStreamAttribute >* this_ptr, int i, const QXmlStreamAttribute* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QVector_QXmlStreamAttribute_isEmpty(const QVector< QXmlStreamAttribute >* this_ptr) {
  return this_ptr->isEmpty();
}

QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_last(QVector< QXmlStreamAttribute >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QXmlStreamAttribute_lastIndexOf_t(const QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QVector_QXmlStreamAttribute_lastIndexOf_t_from(const QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_last_const(const QVector< QXmlStreamAttribute >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QXmlStreamAttribute_length(const QVector< QXmlStreamAttribute >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QVector_QXmlStreamAttribute_mid_to_output_pos(const QVector< QXmlStreamAttribute >* this_ptr, int pos, QVector< QXmlStreamAttribute >* output) {
  new(output) QVector< QXmlStreamAttribute >(this_ptr->mid(pos));
}

void qt_core_c_QVector_QXmlStreamAttribute_mid_to_output_pos_len(const QVector< QXmlStreamAttribute >* this_ptr, int pos, int len, QVector< QXmlStreamAttribute >* output) {
  new(output) QVector< QXmlStreamAttribute >(this_ptr->mid(pos, len));
}

void qt_core_c_QVector_QXmlStreamAttribute_move(QVector< QXmlStreamAttribute >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QXmlStreamAttribute >* qt_core_c_QVector_QXmlStreamAttribute_operator_add_assign_l(QVector< QXmlStreamAttribute >* this_ptr, const QVector< QXmlStreamAttribute >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QXmlStreamAttribute >* qt_core_c_QVector_QXmlStreamAttribute_operator_add_assign_t(QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QVector_QXmlStreamAttribute_operator_add_to_output(const QVector< QXmlStreamAttribute >* this_ptr, const QVector< QXmlStreamAttribute >* l, QVector< QXmlStreamAttribute >* output) {
  new(output) QVector< QXmlStreamAttribute >(this_ptr->operator+(*l));
}

QVector< QXmlStreamAttribute >* qt_core_c_QVector_QXmlStreamAttribute_operator_assign(QVector< QXmlStreamAttribute >* this_ptr, const QVector< QXmlStreamAttribute >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_core_c_QVector_QXmlStreamAttribute_operator_eq(const QVector< QXmlStreamAttribute >* this_ptr, const QVector< QXmlStreamAttribute >* v) {
  return this_ptr->operator==(*v);
}

QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_operator_index(QVector< QXmlStreamAttribute >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QXmlStreamAttribute* qt_core_c_QVector_QXmlStreamAttribute_operator_index_const(const QVector< QXmlStreamAttribute >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QVector_QXmlStreamAttribute_operator_neq(const QVector< QXmlStreamAttribute >* this_ptr, const QVector< QXmlStreamAttribute >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QXmlStreamAttribute >* qt_core_c_QVector_QXmlStreamAttribute_operator_shl_l(QVector< QXmlStreamAttribute >* this_ptr, const QVector< QXmlStreamAttribute >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QXmlStreamAttribute >* qt_core_c_QVector_QXmlStreamAttribute_operator_shl_t(QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QVector_QXmlStreamAttribute_pop_back(QVector< QXmlStreamAttribute >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QVector_QXmlStreamAttribute_pop_front(QVector< QXmlStreamAttribute >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QVector_QXmlStreamAttribute_prepend(QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QVector_QXmlStreamAttribute_push_back(QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QVector_QXmlStreamAttribute_push_front(QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QVector_QXmlStreamAttribute_removeAll(QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QVector_QXmlStreamAttribute_removeAt(QVector< QXmlStreamAttribute >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QVector_QXmlStreamAttribute_removeFirst(QVector< QXmlStreamAttribute >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QVector_QXmlStreamAttribute_removeLast(QVector< QXmlStreamAttribute >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QVector_QXmlStreamAttribute_removeOne(QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QVector_QXmlStreamAttribute_remove_i(QVector< QXmlStreamAttribute >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_core_c_QVector_QXmlStreamAttribute_remove_i_n(QVector< QXmlStreamAttribute >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_core_c_QVector_QXmlStreamAttribute_replace(QVector< QXmlStreamAttribute >* this_ptr, int i, const QXmlStreamAttribute* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QVector_QXmlStreamAttribute_reserve(QVector< QXmlStreamAttribute >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_core_c_QVector_QXmlStreamAttribute_resize(QVector< QXmlStreamAttribute >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_core_c_QVector_QXmlStreamAttribute_size(const QVector< QXmlStreamAttribute >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QVector_QXmlStreamAttribute_squeeze(QVector< QXmlStreamAttribute >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_core_c_QVector_QXmlStreamAttribute_startsWith(const QVector< QXmlStreamAttribute >* this_ptr, const QXmlStreamAttribute* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QVector_QXmlStreamAttribute_swap(QVector< QXmlStreamAttribute >* this_ptr, QVector< QXmlStreamAttribute >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QVector_QXmlStreamAttribute_takeAt_to_output(QVector< QXmlStreamAttribute >* this_ptr, int i, QXmlStreamAttribute* output) {
  new(output) QXmlStreamAttribute(this_ptr->takeAt(i));
}

void qt_core_c_QVector_QXmlStreamAttribute_takeFirst_to_output(QVector< QXmlStreamAttribute >* this_ptr, QXmlStreamAttribute* output) {
  new(output) QXmlStreamAttribute(this_ptr->takeFirst());
}

void qt_core_c_QVector_QXmlStreamAttribute_takeLast_to_output(QVector< QXmlStreamAttribute >* this_ptr, QXmlStreamAttribute* output) {
  new(output) QXmlStreamAttribute(this_ptr->takeLast());
}

void qt_core_c_QVector_QXmlStreamAttribute_value_to_output_i(const QVector< QXmlStreamAttribute >* this_ptr, int i, QXmlStreamAttribute* output) {
  new(output) QXmlStreamAttribute(this_ptr->value(i));
}

void qt_core_c_QVector_QXmlStreamAttribute_value_to_output_i_defaultValue(const QVector< QXmlStreamAttribute >* this_ptr, int i, const QXmlStreamAttribute* defaultValue, QXmlStreamAttribute* output) {
  new(output) QXmlStreamAttribute(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_append_l(QVector< QXmlStreamEntityDeclaration >* this_ptr, const QVector< QXmlStreamEntityDeclaration >* l) {
  this_ptr->append(*l);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_append_t(QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  this_ptr->append(*t);
}

const QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_at(const QVector< QXmlStreamEntityDeclaration >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_back(QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return &this_ptr->back();
}

const QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_back_const(const QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return &this_ptr->back();
}

int qt_core_c_QVector_QXmlStreamEntityDeclaration_capacity(const QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return this_ptr->capacity();
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_clear(QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  this_ptr->clear();
}

const QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_constData(const QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return this_ptr->constData();
}

const QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_constFirst(const QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return &this_ptr->constFirst();
}

const QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_constLast(const QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_constructor_no_args(QVector< QXmlStreamEntityDeclaration >* output) {
  new(output) QVector< QXmlStreamEntityDeclaration >();
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_constructor_size(int size, QVector< QXmlStreamEntityDeclaration >* output) {
  new(output) QVector< QXmlStreamEntityDeclaration >(size);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_constructor_size_t(int size, const QXmlStreamEntityDeclaration* t, QVector< QXmlStreamEntityDeclaration >* output) {
  new(output) QVector< QXmlStreamEntityDeclaration >(size, *t);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_constructor_v(const QVector< QXmlStreamEntityDeclaration >* v, QVector< QXmlStreamEntityDeclaration >* output) {
  new(output) QVector< QXmlStreamEntityDeclaration >(*v);
}

bool qt_core_c_QVector_QXmlStreamEntityDeclaration_contains(const QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QVector_QXmlStreamEntityDeclaration_count_no_args(const QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QVector_QXmlStreamEntityDeclaration_count_t(const QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  return this_ptr->count(*t);
}

QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_data(QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return this_ptr->data();
}

const QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_data_const(const QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_destructor(QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QVector_QXmlStreamEntityDeclaration_empty(const QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QVector_QXmlStreamEntityDeclaration_endsWith(const QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  return this_ptr->endsWith(*t);
}

QVector< QXmlStreamEntityDeclaration >* qt_core_c_QVector_QXmlStreamEntityDeclaration_fill_t(QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  return &this_ptr->fill(*t);
}

QVector< QXmlStreamEntityDeclaration >* qt_core_c_QVector_QXmlStreamEntityDeclaration_fill_t_size(QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t, int size) {
  return &this_ptr->fill(*t, size);
}

QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_first(QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return &this_ptr->first();
}

const QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_first_const(const QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return &this_ptr->first();
}

QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_front(QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return &this_ptr->front();
}

const QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_front_const(const QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QVector_QXmlStreamEntityDeclaration_indexOf_t(const QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QVector_QXmlStreamEntityDeclaration_indexOf_t_from(const QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_insert_i_n_t(QVector< QXmlStreamEntityDeclaration >* this_ptr, int i, int n, const QXmlStreamEntityDeclaration* t) {
  this_ptr->insert(i, n, *t);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_insert_i_t(QVector< QXmlStreamEntityDeclaration >* this_ptr, int i, const QXmlStreamEntityDeclaration* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QVector_QXmlStreamEntityDeclaration_isEmpty(const QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return this_ptr->isEmpty();
}

QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_last(QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QXmlStreamEntityDeclaration_lastIndexOf_t(const QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QVector_QXmlStreamEntityDeclaration_lastIndexOf_t_from(const QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_last_const(const QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QXmlStreamEntityDeclaration_length(const QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_mid_to_output_pos(const QVector< QXmlStreamEntityDeclaration >* this_ptr, int pos, QVector< QXmlStreamEntityDeclaration >* output) {
  new(output) QVector< QXmlStreamEntityDeclaration >(this_ptr->mid(pos));
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_mid_to_output_pos_len(const QVector< QXmlStreamEntityDeclaration >* this_ptr, int pos, int len, QVector< QXmlStreamEntityDeclaration >* output) {
  new(output) QVector< QXmlStreamEntityDeclaration >(this_ptr->mid(pos, len));
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_move(QVector< QXmlStreamEntityDeclaration >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QXmlStreamEntityDeclaration >* qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_add_assign_l(QVector< QXmlStreamEntityDeclaration >* this_ptr, const QVector< QXmlStreamEntityDeclaration >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QXmlStreamEntityDeclaration >* qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_add_assign_t(QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_add_to_output(const QVector< QXmlStreamEntityDeclaration >* this_ptr, const QVector< QXmlStreamEntityDeclaration >* l, QVector< QXmlStreamEntityDeclaration >* output) {
  new(output) QVector< QXmlStreamEntityDeclaration >(this_ptr->operator+(*l));
}

QVector< QXmlStreamEntityDeclaration >* qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_assign(QVector< QXmlStreamEntityDeclaration >* this_ptr, const QVector< QXmlStreamEntityDeclaration >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_eq(const QVector< QXmlStreamEntityDeclaration >* this_ptr, const QVector< QXmlStreamEntityDeclaration >* v) {
  return this_ptr->operator==(*v);
}

QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_index(QVector< QXmlStreamEntityDeclaration >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QXmlStreamEntityDeclaration* qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_index_const(const QVector< QXmlStreamEntityDeclaration >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_neq(const QVector< QXmlStreamEntityDeclaration >* this_ptr, const QVector< QXmlStreamEntityDeclaration >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QXmlStreamEntityDeclaration >* qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_shl_l(QVector< QXmlStreamEntityDeclaration >* this_ptr, const QVector< QXmlStreamEntityDeclaration >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QXmlStreamEntityDeclaration >* qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_shl_t(QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_pop_back(QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_pop_front(QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_prepend(QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_push_back(QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_push_front(QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QVector_QXmlStreamEntityDeclaration_removeAll(QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_removeAt(QVector< QXmlStreamEntityDeclaration >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_removeFirst(QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_removeLast(QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QVector_QXmlStreamEntityDeclaration_removeOne(QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_remove_i(QVector< QXmlStreamEntityDeclaration >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_remove_i_n(QVector< QXmlStreamEntityDeclaration >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_replace(QVector< QXmlStreamEntityDeclaration >* this_ptr, int i, const QXmlStreamEntityDeclaration* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_reserve(QVector< QXmlStreamEntityDeclaration >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_resize(QVector< QXmlStreamEntityDeclaration >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_core_c_QVector_QXmlStreamEntityDeclaration_size(const QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_squeeze(QVector< QXmlStreamEntityDeclaration >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_core_c_QVector_QXmlStreamEntityDeclaration_startsWith(const QVector< QXmlStreamEntityDeclaration >* this_ptr, const QXmlStreamEntityDeclaration* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_swap(QVector< QXmlStreamEntityDeclaration >* this_ptr, QVector< QXmlStreamEntityDeclaration >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_takeAt_to_output(QVector< QXmlStreamEntityDeclaration >* this_ptr, int i, QXmlStreamEntityDeclaration* output) {
  new(output) QXmlStreamEntityDeclaration(this_ptr->takeAt(i));
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_takeFirst_to_output(QVector< QXmlStreamEntityDeclaration >* this_ptr, QXmlStreamEntityDeclaration* output) {
  new(output) QXmlStreamEntityDeclaration(this_ptr->takeFirst());
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_takeLast_to_output(QVector< QXmlStreamEntityDeclaration >* this_ptr, QXmlStreamEntityDeclaration* output) {
  new(output) QXmlStreamEntityDeclaration(this_ptr->takeLast());
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_value_to_output_i(const QVector< QXmlStreamEntityDeclaration >* this_ptr, int i, QXmlStreamEntityDeclaration* output) {
  new(output) QXmlStreamEntityDeclaration(this_ptr->value(i));
}

void qt_core_c_QVector_QXmlStreamEntityDeclaration_value_to_output_i_defaultValue(const QVector< QXmlStreamEntityDeclaration >* this_ptr, int i, const QXmlStreamEntityDeclaration* defaultValue, QXmlStreamEntityDeclaration* output) {
  new(output) QXmlStreamEntityDeclaration(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_append_l(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QVector< QXmlStreamNamespaceDeclaration >* l) {
  this_ptr->append(*l);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_append_t(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  this_ptr->append(*t);
}

const QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_at(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_back(QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return &this_ptr->back();
}

const QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_back_const(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return &this_ptr->back();
}

int qt_core_c_QVector_QXmlStreamNamespaceDeclaration_capacity(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return this_ptr->capacity();
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_clear(QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  this_ptr->clear();
}

const QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_constData(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return this_ptr->constData();
}

const QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_constFirst(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return &this_ptr->constFirst();
}

const QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_constLast(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_constructor_no_args(QVector< QXmlStreamNamespaceDeclaration >* output) {
  new(output) QVector< QXmlStreamNamespaceDeclaration >();
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_constructor_size(int size, QVector< QXmlStreamNamespaceDeclaration >* output) {
  new(output) QVector< QXmlStreamNamespaceDeclaration >(size);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_constructor_size_t(int size, const QXmlStreamNamespaceDeclaration* t, QVector< QXmlStreamNamespaceDeclaration >* output) {
  new(output) QVector< QXmlStreamNamespaceDeclaration >(size, *t);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_constructor_v(const QVector< QXmlStreamNamespaceDeclaration >* v, QVector< QXmlStreamNamespaceDeclaration >* output) {
  new(output) QVector< QXmlStreamNamespaceDeclaration >(*v);
}

bool qt_core_c_QVector_QXmlStreamNamespaceDeclaration_contains(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QVector_QXmlStreamNamespaceDeclaration_count_no_args(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QVector_QXmlStreamNamespaceDeclaration_count_t(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  return this_ptr->count(*t);
}

QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_data(QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return this_ptr->data();
}

const QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_data_const(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_destructor(QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QVector_QXmlStreamNamespaceDeclaration_empty(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QVector_QXmlStreamNamespaceDeclaration_endsWith(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  return this_ptr->endsWith(*t);
}

QVector< QXmlStreamNamespaceDeclaration >* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_fill_t(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  return &this_ptr->fill(*t);
}

QVector< QXmlStreamNamespaceDeclaration >* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_fill_t_size(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t, int size) {
  return &this_ptr->fill(*t, size);
}

QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_first(QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return &this_ptr->first();
}

const QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_first_const(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return &this_ptr->first();
}

QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_front(QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return &this_ptr->front();
}

const QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_front_const(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QVector_QXmlStreamNamespaceDeclaration_indexOf_t(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QVector_QXmlStreamNamespaceDeclaration_indexOf_t_from(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_insert_i_n_t(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int i, int n, const QXmlStreamNamespaceDeclaration* t) {
  this_ptr->insert(i, n, *t);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_insert_i_t(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int i, const QXmlStreamNamespaceDeclaration* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QVector_QXmlStreamNamespaceDeclaration_isEmpty(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return this_ptr->isEmpty();
}

QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_last(QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QXmlStreamNamespaceDeclaration_lastIndexOf_t(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QVector_QXmlStreamNamespaceDeclaration_lastIndexOf_t_from(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_last_const(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QXmlStreamNamespaceDeclaration_length(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_mid_to_output_pos(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int pos, QVector< QXmlStreamNamespaceDeclaration >* output) {
  new(output) QVector< QXmlStreamNamespaceDeclaration >(this_ptr->mid(pos));
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_mid_to_output_pos_len(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int pos, int len, QVector< QXmlStreamNamespaceDeclaration >* output) {
  new(output) QVector< QXmlStreamNamespaceDeclaration >(this_ptr->mid(pos, len));
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_move(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QXmlStreamNamespaceDeclaration >* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_add_assign_l(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QVector< QXmlStreamNamespaceDeclaration >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QXmlStreamNamespaceDeclaration >* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_add_assign_t(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_add_to_output(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QVector< QXmlStreamNamespaceDeclaration >* l, QVector< QXmlStreamNamespaceDeclaration >* output) {
  new(output) QVector< QXmlStreamNamespaceDeclaration >(this_ptr->operator+(*l));
}

QVector< QXmlStreamNamespaceDeclaration >* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_assign(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QVector< QXmlStreamNamespaceDeclaration >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_eq(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QVector< QXmlStreamNamespaceDeclaration >* v) {
  return this_ptr->operator==(*v);
}

QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_index(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QXmlStreamNamespaceDeclaration* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_index_const(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_neq(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QVector< QXmlStreamNamespaceDeclaration >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QXmlStreamNamespaceDeclaration >* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_shl_l(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QVector< QXmlStreamNamespaceDeclaration >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QXmlStreamNamespaceDeclaration >* qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_shl_t(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_pop_back(QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_pop_front(QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_prepend(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_push_back(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_push_front(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QVector_QXmlStreamNamespaceDeclaration_removeAll(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_removeAt(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_removeFirst(QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_removeLast(QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QVector_QXmlStreamNamespaceDeclaration_removeOne(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_remove_i(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_remove_i_n(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_replace(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int i, const QXmlStreamNamespaceDeclaration* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_reserve(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_resize(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_core_c_QVector_QXmlStreamNamespaceDeclaration_size(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_squeeze(QVector< QXmlStreamNamespaceDeclaration >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_core_c_QVector_QXmlStreamNamespaceDeclaration_startsWith(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, const QXmlStreamNamespaceDeclaration* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_swap(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, QVector< QXmlStreamNamespaceDeclaration >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_takeAt_to_output(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int i, QXmlStreamNamespaceDeclaration* output) {
  new(output) QXmlStreamNamespaceDeclaration(this_ptr->takeAt(i));
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_takeFirst_to_output(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, QXmlStreamNamespaceDeclaration* output) {
  new(output) QXmlStreamNamespaceDeclaration(this_ptr->takeFirst());
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_takeLast_to_output(QVector< QXmlStreamNamespaceDeclaration >* this_ptr, QXmlStreamNamespaceDeclaration* output) {
  new(output) QXmlStreamNamespaceDeclaration(this_ptr->takeLast());
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_value_to_output_i(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int i, QXmlStreamNamespaceDeclaration* output) {
  new(output) QXmlStreamNamespaceDeclaration(this_ptr->value(i));
}

void qt_core_c_QVector_QXmlStreamNamespaceDeclaration_value_to_output_i_defaultValue(const QVector< QXmlStreamNamespaceDeclaration >* this_ptr, int i, const QXmlStreamNamespaceDeclaration* defaultValue, QXmlStreamNamespaceDeclaration* output) {
  new(output) QXmlStreamNamespaceDeclaration(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_append_l(QVector< QXmlStreamNotationDeclaration >* this_ptr, const QVector< QXmlStreamNotationDeclaration >* l) {
  this_ptr->append(*l);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_append_t(QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  this_ptr->append(*t);
}

const QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_at(const QVector< QXmlStreamNotationDeclaration >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_back(QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return &this_ptr->back();
}

const QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_back_const(const QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return &this_ptr->back();
}

int qt_core_c_QVector_QXmlStreamNotationDeclaration_capacity(const QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return this_ptr->capacity();
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_clear(QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  this_ptr->clear();
}

const QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_constData(const QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return this_ptr->constData();
}

const QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_constFirst(const QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return &this_ptr->constFirst();
}

const QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_constLast(const QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_constructor_no_args(QVector< QXmlStreamNotationDeclaration >* output) {
  new(output) QVector< QXmlStreamNotationDeclaration >();
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_constructor_size(int size, QVector< QXmlStreamNotationDeclaration >* output) {
  new(output) QVector< QXmlStreamNotationDeclaration >(size);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_constructor_size_t(int size, const QXmlStreamNotationDeclaration* t, QVector< QXmlStreamNotationDeclaration >* output) {
  new(output) QVector< QXmlStreamNotationDeclaration >(size, *t);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_constructor_v(const QVector< QXmlStreamNotationDeclaration >* v, QVector< QXmlStreamNotationDeclaration >* output) {
  new(output) QVector< QXmlStreamNotationDeclaration >(*v);
}

bool qt_core_c_QVector_QXmlStreamNotationDeclaration_contains(const QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QVector_QXmlStreamNotationDeclaration_count_no_args(const QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QVector_QXmlStreamNotationDeclaration_count_t(const QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  return this_ptr->count(*t);
}

QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_data(QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return this_ptr->data();
}

const QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_data_const(const QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_destructor(QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QVector_QXmlStreamNotationDeclaration_empty(const QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QVector_QXmlStreamNotationDeclaration_endsWith(const QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  return this_ptr->endsWith(*t);
}

QVector< QXmlStreamNotationDeclaration >* qt_core_c_QVector_QXmlStreamNotationDeclaration_fill_t(QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  return &this_ptr->fill(*t);
}

QVector< QXmlStreamNotationDeclaration >* qt_core_c_QVector_QXmlStreamNotationDeclaration_fill_t_size(QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t, int size) {
  return &this_ptr->fill(*t, size);
}

QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_first(QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return &this_ptr->first();
}

const QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_first_const(const QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return &this_ptr->first();
}

QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_front(QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return &this_ptr->front();
}

const QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_front_const(const QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QVector_QXmlStreamNotationDeclaration_indexOf_t(const QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QVector_QXmlStreamNotationDeclaration_indexOf_t_from(const QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_insert_i_n_t(QVector< QXmlStreamNotationDeclaration >* this_ptr, int i, int n, const QXmlStreamNotationDeclaration* t) {
  this_ptr->insert(i, n, *t);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_insert_i_t(QVector< QXmlStreamNotationDeclaration >* this_ptr, int i, const QXmlStreamNotationDeclaration* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QVector_QXmlStreamNotationDeclaration_isEmpty(const QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return this_ptr->isEmpty();
}

QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_last(QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QXmlStreamNotationDeclaration_lastIndexOf_t(const QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QVector_QXmlStreamNotationDeclaration_lastIndexOf_t_from(const QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_last_const(const QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_QXmlStreamNotationDeclaration_length(const QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_mid_to_output_pos(const QVector< QXmlStreamNotationDeclaration >* this_ptr, int pos, QVector< QXmlStreamNotationDeclaration >* output) {
  new(output) QVector< QXmlStreamNotationDeclaration >(this_ptr->mid(pos));
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_mid_to_output_pos_len(const QVector< QXmlStreamNotationDeclaration >* this_ptr, int pos, int len, QVector< QXmlStreamNotationDeclaration >* output) {
  new(output) QVector< QXmlStreamNotationDeclaration >(this_ptr->mid(pos, len));
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_move(QVector< QXmlStreamNotationDeclaration >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QXmlStreamNotationDeclaration >* qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_add_assign_l(QVector< QXmlStreamNotationDeclaration >* this_ptr, const QVector< QXmlStreamNotationDeclaration >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QXmlStreamNotationDeclaration >* qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_add_assign_t(QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_add_to_output(const QVector< QXmlStreamNotationDeclaration >* this_ptr, const QVector< QXmlStreamNotationDeclaration >* l, QVector< QXmlStreamNotationDeclaration >* output) {
  new(output) QVector< QXmlStreamNotationDeclaration >(this_ptr->operator+(*l));
}

QVector< QXmlStreamNotationDeclaration >* qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_assign(QVector< QXmlStreamNotationDeclaration >* this_ptr, const QVector< QXmlStreamNotationDeclaration >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_eq(const QVector< QXmlStreamNotationDeclaration >* this_ptr, const QVector< QXmlStreamNotationDeclaration >* v) {
  return this_ptr->operator==(*v);
}

QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_index(QVector< QXmlStreamNotationDeclaration >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QXmlStreamNotationDeclaration* qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_index_const(const QVector< QXmlStreamNotationDeclaration >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_neq(const QVector< QXmlStreamNotationDeclaration >* this_ptr, const QVector< QXmlStreamNotationDeclaration >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QXmlStreamNotationDeclaration >* qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_shl_l(QVector< QXmlStreamNotationDeclaration >* this_ptr, const QVector< QXmlStreamNotationDeclaration >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QXmlStreamNotationDeclaration >* qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_shl_t(QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_pop_back(QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_pop_front(QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_prepend(QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_push_back(QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_push_front(QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QVector_QXmlStreamNotationDeclaration_removeAll(QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_removeAt(QVector< QXmlStreamNotationDeclaration >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_removeFirst(QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_removeLast(QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QVector_QXmlStreamNotationDeclaration_removeOne(QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_remove_i(QVector< QXmlStreamNotationDeclaration >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_remove_i_n(QVector< QXmlStreamNotationDeclaration >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_replace(QVector< QXmlStreamNotationDeclaration >* this_ptr, int i, const QXmlStreamNotationDeclaration* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_reserve(QVector< QXmlStreamNotationDeclaration >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_resize(QVector< QXmlStreamNotationDeclaration >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_core_c_QVector_QXmlStreamNotationDeclaration_size(const QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_squeeze(QVector< QXmlStreamNotationDeclaration >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_core_c_QVector_QXmlStreamNotationDeclaration_startsWith(const QVector< QXmlStreamNotationDeclaration >* this_ptr, const QXmlStreamNotationDeclaration* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_swap(QVector< QXmlStreamNotationDeclaration >* this_ptr, QVector< QXmlStreamNotationDeclaration >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_takeAt_to_output(QVector< QXmlStreamNotationDeclaration >* this_ptr, int i, QXmlStreamNotationDeclaration* output) {
  new(output) QXmlStreamNotationDeclaration(this_ptr->takeAt(i));
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_takeFirst_to_output(QVector< QXmlStreamNotationDeclaration >* this_ptr, QXmlStreamNotationDeclaration* output) {
  new(output) QXmlStreamNotationDeclaration(this_ptr->takeFirst());
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_takeLast_to_output(QVector< QXmlStreamNotationDeclaration >* this_ptr, QXmlStreamNotationDeclaration* output) {
  new(output) QXmlStreamNotationDeclaration(this_ptr->takeLast());
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_value_to_output_i(const QVector< QXmlStreamNotationDeclaration >* this_ptr, int i, QXmlStreamNotationDeclaration* output) {
  new(output) QXmlStreamNotationDeclaration(this_ptr->value(i));
}

void qt_core_c_QVector_QXmlStreamNotationDeclaration_value_to_output_i_defaultValue(const QVector< QXmlStreamNotationDeclaration >* this_ptr, int i, const QXmlStreamNotationDeclaration* defaultValue, QXmlStreamNotationDeclaration* output) {
  new(output) QXmlStreamNotationDeclaration(this_ptr->value(i, *defaultValue));
}

void qt_core_c_QVector_int_append_l(QVector< int >* this_ptr, const QVector< int >* l) {
  this_ptr->append(*l);
}

void qt_core_c_QVector_int_append_t(QVector< int >* this_ptr, const int* t) {
  this_ptr->append(*t);
}

const int* qt_core_c_QVector_int_at(const QVector< int >* this_ptr, int i) {
  return &this_ptr->at(i);
}

int* qt_core_c_QVector_int_back(QVector< int >* this_ptr) {
  return &this_ptr->back();
}

const int* qt_core_c_QVector_int_back_const(const QVector< int >* this_ptr) {
  return &this_ptr->back();
}

int qt_core_c_QVector_int_capacity(const QVector< int >* this_ptr) {
  return this_ptr->capacity();
}

void qt_core_c_QVector_int_clear(QVector< int >* this_ptr) {
  this_ptr->clear();
}

const int* qt_core_c_QVector_int_constData(const QVector< int >* this_ptr) {
  return this_ptr->constData();
}

const int* qt_core_c_QVector_int_constFirst(const QVector< int >* this_ptr) {
  return &this_ptr->constFirst();
}

const int* qt_core_c_QVector_int_constLast(const QVector< int >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QVector_int_constructor_no_args(QVector< int >* output) {
  new(output) QVector< int >();
}

void qt_core_c_QVector_int_constructor_size(int size, QVector< int >* output) {
  new(output) QVector< int >(size);
}

void qt_core_c_QVector_int_constructor_size_t(int size, const int* t, QVector< int >* output) {
  new(output) QVector< int >(size, *t);
}

void qt_core_c_QVector_int_constructor_v(const QVector< int >* v, QVector< int >* output) {
  new(output) QVector< int >(*v);
}

bool qt_core_c_QVector_int_contains(const QVector< int >* this_ptr, const int* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QVector_int_count_no_args(const QVector< int >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QVector_int_count_t(const QVector< int >* this_ptr, const int* t) {
  return this_ptr->count(*t);
}

int* qt_core_c_QVector_int_data(QVector< int >* this_ptr) {
  return this_ptr->data();
}

const int* qt_core_c_QVector_int_data_const(const QVector< int >* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QVector_int_destructor(QVector< int >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QVector_int_empty(const QVector< int >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QVector_int_endsWith(const QVector< int >* this_ptr, const int* t) {
  return this_ptr->endsWith(*t);
}

QVector< int >* qt_core_c_QVector_int_fill_t(QVector< int >* this_ptr, const int* t) {
  return &this_ptr->fill(*t);
}

QVector< int >* qt_core_c_QVector_int_fill_t_size(QVector< int >* this_ptr, const int* t, int size) {
  return &this_ptr->fill(*t, size);
}

int* qt_core_c_QVector_int_first(QVector< int >* this_ptr) {
  return &this_ptr->first();
}

const int* qt_core_c_QVector_int_first_const(const QVector< int >* this_ptr) {
  return &this_ptr->first();
}

void qt_core_c_QVector_int_fromList_to_output(const QList< int >* list, QVector< int >* output) {
  new(output) QVector< int >(QVector< int >::fromList(*list));
}

int* qt_core_c_QVector_int_front(QVector< int >* this_ptr) {
  return &this_ptr->front();
}

const int* qt_core_c_QVector_int_front_const(const QVector< int >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QVector_int_indexOf_t(const QVector< int >* this_ptr, const int* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QVector_int_indexOf_t_from(const QVector< int >* this_ptr, const int* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QVector_int_insert_i_n_t(QVector< int >* this_ptr, int i, int n, const int* t) {
  this_ptr->insert(i, n, *t);
}

void qt_core_c_QVector_int_insert_i_t(QVector< int >* this_ptr, int i, const int* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QVector_int_isEmpty(const QVector< int >* this_ptr) {
  return this_ptr->isEmpty();
}

int* qt_core_c_QVector_int_last(QVector< int >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_int_lastIndexOf_t(const QVector< int >* this_ptr, const int* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QVector_int_lastIndexOf_t_from(const QVector< int >* this_ptr, const int* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const int* qt_core_c_QVector_int_last_const(const QVector< int >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_int_length(const QVector< int >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QVector_int_mid_to_output_pos(const QVector< int >* this_ptr, int pos, QVector< int >* output) {
  new(output) QVector< int >(this_ptr->mid(pos));
}

void qt_core_c_QVector_int_mid_to_output_pos_len(const QVector< int >* this_ptr, int pos, int len, QVector< int >* output) {
  new(output) QVector< int >(this_ptr->mid(pos, len));
}

void qt_core_c_QVector_int_move(QVector< int >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< int >* qt_core_c_QVector_int_operator_add_assign_l(QVector< int >* this_ptr, const QVector< int >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< int >* qt_core_c_QVector_int_operator_add_assign_t(QVector< int >* this_ptr, const int* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QVector_int_operator_add_to_output(const QVector< int >* this_ptr, const QVector< int >* l, QVector< int >* output) {
  new(output) QVector< int >(this_ptr->operator+(*l));
}

QVector< int >* qt_core_c_QVector_int_operator_assign(QVector< int >* this_ptr, const QVector< int >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_core_c_QVector_int_operator_eq(const QVector< int >* this_ptr, const QVector< int >* v) {
  return this_ptr->operator==(*v);
}

int* qt_core_c_QVector_int_operator_index(QVector< int >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const int* qt_core_c_QVector_int_operator_index_const(const QVector< int >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QVector_int_operator_neq(const QVector< int >* this_ptr, const QVector< int >* v) {
  return this_ptr->operator!=(*v);
}

QVector< int >* qt_core_c_QVector_int_operator_shl_l(QVector< int >* this_ptr, const QVector< int >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< int >* qt_core_c_QVector_int_operator_shl_t(QVector< int >* this_ptr, const int* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QVector_int_pop_back(QVector< int >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QVector_int_pop_front(QVector< int >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QVector_int_prepend(QVector< int >* this_ptr, const int* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QVector_int_push_back(QVector< int >* this_ptr, const int* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QVector_int_push_front(QVector< int >* this_ptr, const int* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QVector_int_removeAll(QVector< int >* this_ptr, const int* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QVector_int_removeAt(QVector< int >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QVector_int_removeFirst(QVector< int >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QVector_int_removeLast(QVector< int >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QVector_int_removeOne(QVector< int >* this_ptr, const int* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QVector_int_remove_i(QVector< int >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_core_c_QVector_int_remove_i_n(QVector< int >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_core_c_QVector_int_replace(QVector< int >* this_ptr, int i, const int* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QVector_int_reserve(QVector< int >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_core_c_QVector_int_resize(QVector< int >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_core_c_QVector_int_size(const QVector< int >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QVector_int_squeeze(QVector< int >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_core_c_QVector_int_startsWith(const QVector< int >* this_ptr, const int* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QVector_int_swap(QVector< int >* this_ptr, QVector< int >* other) {
  this_ptr->swap(*other);
}

int qt_core_c_QVector_int_takeAt(QVector< int >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

int qt_core_c_QVector_int_takeFirst(QVector< int >* this_ptr) {
  return this_ptr->takeFirst();
}

int qt_core_c_QVector_int_takeLast(QVector< int >* this_ptr) {
  return this_ptr->takeLast();
}

void qt_core_c_QVector_int_toList_to_output(const QVector< int >* this_ptr, QList< int >* output) {
  new(output) QList< int >(this_ptr->toList());
}

int qt_core_c_QVector_int_value_i(const QVector< int >* this_ptr, int i) {
  return this_ptr->value(i);
}

int qt_core_c_QVector_int_value_i_defaultValue(const QVector< int >* this_ptr, int i, const int* defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_core_c_QVector_unsigned_int_append_l(QVector< unsigned int >* this_ptr, const QVector< unsigned int >* l) {
  this_ptr->append(*l);
}

void qt_core_c_QVector_unsigned_int_append_t(QVector< unsigned int >* this_ptr, const unsigned int* t) {
  this_ptr->append(*t);
}

const unsigned int* qt_core_c_QVector_unsigned_int_at(const QVector< unsigned int >* this_ptr, int i) {
  return &this_ptr->at(i);
}

unsigned int* qt_core_c_QVector_unsigned_int_back(QVector< unsigned int >* this_ptr) {
  return &this_ptr->back();
}

const unsigned int* qt_core_c_QVector_unsigned_int_back_const(const QVector< unsigned int >* this_ptr) {
  return &this_ptr->back();
}

int qt_core_c_QVector_unsigned_int_capacity(const QVector< unsigned int >* this_ptr) {
  return this_ptr->capacity();
}

void qt_core_c_QVector_unsigned_int_clear(QVector< unsigned int >* this_ptr) {
  this_ptr->clear();
}

const unsigned int* qt_core_c_QVector_unsigned_int_constData(const QVector< unsigned int >* this_ptr) {
  return this_ptr->constData();
}

const unsigned int* qt_core_c_QVector_unsigned_int_constFirst(const QVector< unsigned int >* this_ptr) {
  return &this_ptr->constFirst();
}

const unsigned int* qt_core_c_QVector_unsigned_int_constLast(const QVector< unsigned int >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_core_c_QVector_unsigned_int_constructor_no_args(QVector< unsigned int >* output) {
  new(output) QVector< unsigned int >();
}

void qt_core_c_QVector_unsigned_int_constructor_size(int size, QVector< unsigned int >* output) {
  new(output) QVector< unsigned int >(size);
}

void qt_core_c_QVector_unsigned_int_constructor_size_t(int size, const unsigned int* t, QVector< unsigned int >* output) {
  new(output) QVector< unsigned int >(size, *t);
}

void qt_core_c_QVector_unsigned_int_constructor_v(const QVector< unsigned int >* v, QVector< unsigned int >* output) {
  new(output) QVector< unsigned int >(*v);
}

bool qt_core_c_QVector_unsigned_int_contains(const QVector< unsigned int >* this_ptr, const unsigned int* t) {
  return this_ptr->contains(*t);
}

int qt_core_c_QVector_unsigned_int_count_no_args(const QVector< unsigned int >* this_ptr) {
  return this_ptr->count();
}

int qt_core_c_QVector_unsigned_int_count_t(const QVector< unsigned int >* this_ptr, const unsigned int* t) {
  return this_ptr->count(*t);
}

unsigned int* qt_core_c_QVector_unsigned_int_data(QVector< unsigned int >* this_ptr) {
  return this_ptr->data();
}

const unsigned int* qt_core_c_QVector_unsigned_int_data_const(const QVector< unsigned int >* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QVector_unsigned_int_destructor(QVector< unsigned int >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QVector_unsigned_int_empty(const QVector< unsigned int >* this_ptr) {
  return this_ptr->empty();
}

bool qt_core_c_QVector_unsigned_int_endsWith(const QVector< unsigned int >* this_ptr, const unsigned int* t) {
  return this_ptr->endsWith(*t);
}

QVector< unsigned int >* qt_core_c_QVector_unsigned_int_fill_t(QVector< unsigned int >* this_ptr, const unsigned int* t) {
  return &this_ptr->fill(*t);
}

QVector< unsigned int >* qt_core_c_QVector_unsigned_int_fill_t_size(QVector< unsigned int >* this_ptr, const unsigned int* t, int size) {
  return &this_ptr->fill(*t, size);
}

unsigned int* qt_core_c_QVector_unsigned_int_first(QVector< unsigned int >* this_ptr) {
  return &this_ptr->first();
}

const unsigned int* qt_core_c_QVector_unsigned_int_first_const(const QVector< unsigned int >* this_ptr) {
  return &this_ptr->first();
}

unsigned int* qt_core_c_QVector_unsigned_int_front(QVector< unsigned int >* this_ptr) {
  return &this_ptr->front();
}

const unsigned int* qt_core_c_QVector_unsigned_int_front_const(const QVector< unsigned int >* this_ptr) {
  return &this_ptr->front();
}

int qt_core_c_QVector_unsigned_int_indexOf_t(const QVector< unsigned int >* this_ptr, const unsigned int* t) {
  return this_ptr->indexOf(*t);
}

int qt_core_c_QVector_unsigned_int_indexOf_t_from(const QVector< unsigned int >* this_ptr, const unsigned int* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_core_c_QVector_unsigned_int_insert_i_n_t(QVector< unsigned int >* this_ptr, int i, int n, const unsigned int* t) {
  this_ptr->insert(i, n, *t);
}

void qt_core_c_QVector_unsigned_int_insert_i_t(QVector< unsigned int >* this_ptr, int i, const unsigned int* t) {
  this_ptr->insert(i, *t);
}

bool qt_core_c_QVector_unsigned_int_isEmpty(const QVector< unsigned int >* this_ptr) {
  return this_ptr->isEmpty();
}

unsigned int* qt_core_c_QVector_unsigned_int_last(QVector< unsigned int >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_unsigned_int_lastIndexOf_t(const QVector< unsigned int >* this_ptr, const unsigned int* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_core_c_QVector_unsigned_int_lastIndexOf_t_from(const QVector< unsigned int >* this_ptr, const unsigned int* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const unsigned int* qt_core_c_QVector_unsigned_int_last_const(const QVector< unsigned int >* this_ptr) {
  return &this_ptr->last();
}

int qt_core_c_QVector_unsigned_int_length(const QVector< unsigned int >* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QVector_unsigned_int_mid_to_output_pos(const QVector< unsigned int >* this_ptr, int pos, QVector< unsigned int >* output) {
  new(output) QVector< unsigned int >(this_ptr->mid(pos));
}

void qt_core_c_QVector_unsigned_int_mid_to_output_pos_len(const QVector< unsigned int >* this_ptr, int pos, int len, QVector< unsigned int >* output) {
  new(output) QVector< unsigned int >(this_ptr->mid(pos, len));
}

void qt_core_c_QVector_unsigned_int_move(QVector< unsigned int >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< unsigned int >* qt_core_c_QVector_unsigned_int_operator_add_assign_l(QVector< unsigned int >* this_ptr, const QVector< unsigned int >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< unsigned int >* qt_core_c_QVector_unsigned_int_operator_add_assign_t(QVector< unsigned int >* this_ptr, const unsigned int* t) {
  return &this_ptr->operator+=(*t);
}

void qt_core_c_QVector_unsigned_int_operator_add_to_output(const QVector< unsigned int >* this_ptr, const QVector< unsigned int >* l, QVector< unsigned int >* output) {
  new(output) QVector< unsigned int >(this_ptr->operator+(*l));
}

QVector< unsigned int >* qt_core_c_QVector_unsigned_int_operator_assign(QVector< unsigned int >* this_ptr, const QVector< unsigned int >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_core_c_QVector_unsigned_int_operator_eq(const QVector< unsigned int >* this_ptr, const QVector< unsigned int >* v) {
  return this_ptr->operator==(*v);
}

unsigned int* qt_core_c_QVector_unsigned_int_operator_index(QVector< unsigned int >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const unsigned int* qt_core_c_QVector_unsigned_int_operator_index_const(const QVector< unsigned int >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_core_c_QVector_unsigned_int_operator_neq(const QVector< unsigned int >* this_ptr, const QVector< unsigned int >* v) {
  return this_ptr->operator!=(*v);
}

QVector< unsigned int >* qt_core_c_QVector_unsigned_int_operator_shl_l(QVector< unsigned int >* this_ptr, const QVector< unsigned int >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< unsigned int >* qt_core_c_QVector_unsigned_int_operator_shl_t(QVector< unsigned int >* this_ptr, const unsigned int* t) {
  return &this_ptr->operator<<(*t);
}

void qt_core_c_QVector_unsigned_int_pop_back(QVector< unsigned int >* this_ptr) {
  this_ptr->pop_back();
}

void qt_core_c_QVector_unsigned_int_pop_front(QVector< unsigned int >* this_ptr) {
  this_ptr->pop_front();
}

void qt_core_c_QVector_unsigned_int_prepend(QVector< unsigned int >* this_ptr, const unsigned int* t) {
  this_ptr->prepend(*t);
}

void qt_core_c_QVector_unsigned_int_push_back(QVector< unsigned int >* this_ptr, const unsigned int* t) {
  this_ptr->push_back(*t);
}

void qt_core_c_QVector_unsigned_int_push_front(QVector< unsigned int >* this_ptr, const unsigned int* t) {
  this_ptr->push_front(*t);
}

int qt_core_c_QVector_unsigned_int_removeAll(QVector< unsigned int >* this_ptr, const unsigned int* t) {
  return this_ptr->removeAll(*t);
}

void qt_core_c_QVector_unsigned_int_removeAt(QVector< unsigned int >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_core_c_QVector_unsigned_int_removeFirst(QVector< unsigned int >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_core_c_QVector_unsigned_int_removeLast(QVector< unsigned int >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_core_c_QVector_unsigned_int_removeOne(QVector< unsigned int >* this_ptr, const unsigned int* t) {
  return this_ptr->removeOne(*t);
}

void qt_core_c_QVector_unsigned_int_remove_i(QVector< unsigned int >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_core_c_QVector_unsigned_int_remove_i_n(QVector< unsigned int >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_core_c_QVector_unsigned_int_replace(QVector< unsigned int >* this_ptr, int i, const unsigned int* t) {
  this_ptr->replace(i, *t);
}

void qt_core_c_QVector_unsigned_int_reserve(QVector< unsigned int >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_core_c_QVector_unsigned_int_resize(QVector< unsigned int >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_core_c_QVector_unsigned_int_size(const QVector< unsigned int >* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QVector_unsigned_int_squeeze(QVector< unsigned int >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_core_c_QVector_unsigned_int_startsWith(const QVector< unsigned int >* this_ptr, const unsigned int* t) {
  return this_ptr->startsWith(*t);
}

void qt_core_c_QVector_unsigned_int_swap(QVector< unsigned int >* this_ptr, QVector< unsigned int >* other) {
  this_ptr->swap(*other);
}

unsigned int qt_core_c_QVector_unsigned_int_takeAt(QVector< unsigned int >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

unsigned int qt_core_c_QVector_unsigned_int_takeFirst(QVector< unsigned int >* this_ptr) {
  return this_ptr->takeFirst();
}

unsigned int qt_core_c_QVector_unsigned_int_takeLast(QVector< unsigned int >* this_ptr) {
  return this_ptr->takeLast();
}

unsigned int qt_core_c_QVector_unsigned_int_value_i(const QVector< unsigned int >* this_ptr, int i) {
  return this_ptr->value(i);
}

unsigned int qt_core_c_QVector_unsigned_int_value_i_defaultValue(const QVector< unsigned int >* this_ptr, int i, const unsigned int* defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

