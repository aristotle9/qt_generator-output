#include "qt_widgets_c_QVector.h"

void qt_widgets_c_QVector_QColor_append_l(QVector< QColor >* this_ptr, const QVector< QColor >* l) {
  this_ptr->append(*l);
}

void qt_widgets_c_QVector_QColor_append_t(QVector< QColor >* this_ptr, const QColor* t) {
  this_ptr->append(*t);
}

const QColor* qt_widgets_c_QVector_QColor_at(const QVector< QColor >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QColor* qt_widgets_c_QVector_QColor_back(QVector< QColor >* this_ptr) {
  return &this_ptr->back();
}

const QColor* qt_widgets_c_QVector_QColor_back_const(const QVector< QColor >* this_ptr) {
  return &this_ptr->back();
}

int qt_widgets_c_QVector_QColor_capacity(const QVector< QColor >* this_ptr) {
  return this_ptr->capacity();
}

void qt_widgets_c_QVector_QColor_clear(QVector< QColor >* this_ptr) {
  this_ptr->clear();
}

const QColor* qt_widgets_c_QVector_QColor_constData(const QVector< QColor >* this_ptr) {
  return this_ptr->constData();
}

const QColor* qt_widgets_c_QVector_QColor_constFirst(const QVector< QColor >* this_ptr) {
  return &this_ptr->constFirst();
}

const QColor* qt_widgets_c_QVector_QColor_constLast(const QVector< QColor >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QVector_QColor_constructor_no_args(QVector< QColor >* output) {
  new(output) QVector< QColor >();
}

void qt_widgets_c_QVector_QColor_constructor_size(int size, QVector< QColor >* output) {
  new(output) QVector< QColor >(size);
}

void qt_widgets_c_QVector_QColor_constructor_size_t(int size, const QColor* t, QVector< QColor >* output) {
  new(output) QVector< QColor >(size, *t);
}

void qt_widgets_c_QVector_QColor_constructor_v(const QVector< QColor >* v, QVector< QColor >* output) {
  new(output) QVector< QColor >(*v);
}

bool qt_widgets_c_QVector_QColor_contains(const QVector< QColor >* this_ptr, const QColor* t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QVector_QColor_count_no_args(const QVector< QColor >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QVector_QColor_count_t(const QVector< QColor >* this_ptr, const QColor* t) {
  return this_ptr->count(*t);
}

QColor* qt_widgets_c_QVector_QColor_data(QVector< QColor >* this_ptr) {
  return this_ptr->data();
}

const QColor* qt_widgets_c_QVector_QColor_data_const(const QVector< QColor >* this_ptr) {
  return this_ptr->data();
}

void qt_widgets_c_QVector_QColor_destructor(QVector< QColor >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QVector_QColor_empty(const QVector< QColor >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QVector_QColor_endsWith(const QVector< QColor >* this_ptr, const QColor* t) {
  return this_ptr->endsWith(*t);
}

QVector< QColor >* qt_widgets_c_QVector_QColor_fill_t(QVector< QColor >* this_ptr, const QColor* t) {
  return &this_ptr->fill(*t);
}

QVector< QColor >* qt_widgets_c_QVector_QColor_fill_t_size(QVector< QColor >* this_ptr, const QColor* t, int size) {
  return &this_ptr->fill(*t, size);
}

QColor* qt_widgets_c_QVector_QColor_first(QVector< QColor >* this_ptr) {
  return &this_ptr->first();
}

const QColor* qt_widgets_c_QVector_QColor_first_const(const QVector< QColor >* this_ptr) {
  return &this_ptr->first();
}

QColor* qt_widgets_c_QVector_QColor_front(QVector< QColor >* this_ptr) {
  return &this_ptr->front();
}

const QColor* qt_widgets_c_QVector_QColor_front_const(const QVector< QColor >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QVector_QColor_indexOf_t(const QVector< QColor >* this_ptr, const QColor* t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QVector_QColor_indexOf_t_from(const QVector< QColor >* this_ptr, const QColor* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QVector_QColor_insert_i_n_t(QVector< QColor >* this_ptr, int i, int n, const QColor* t) {
  this_ptr->insert(i, n, *t);
}

void qt_widgets_c_QVector_QColor_insert_i_t(QVector< QColor >* this_ptr, int i, const QColor* t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QVector_QColor_isEmpty(const QVector< QColor >* this_ptr) {
  return this_ptr->isEmpty();
}

QColor* qt_widgets_c_QVector_QColor_last(QVector< QColor >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QVector_QColor_lastIndexOf_t(const QVector< QColor >* this_ptr, const QColor* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QVector_QColor_lastIndexOf_t_from(const QVector< QColor >* this_ptr, const QColor* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QColor* qt_widgets_c_QVector_QColor_last_const(const QVector< QColor >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QVector_QColor_length(const QVector< QColor >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QVector_QColor_mid_to_output_pos(const QVector< QColor >* this_ptr, int pos, QVector< QColor >* output) {
  new(output) QVector< QColor >(this_ptr->mid(pos));
}

void qt_widgets_c_QVector_QColor_mid_to_output_pos_len(const QVector< QColor >* this_ptr, int pos, int len, QVector< QColor >* output) {
  new(output) QVector< QColor >(this_ptr->mid(pos, len));
}

void qt_widgets_c_QVector_QColor_move(QVector< QColor >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QColor >* qt_widgets_c_QVector_QColor_operator_add_assign_l(QVector< QColor >* this_ptr, const QVector< QColor >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QColor >* qt_widgets_c_QVector_QColor_operator_add_assign_t(QVector< QColor >* this_ptr, const QColor* t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QVector_QColor_operator_add_to_output(const QVector< QColor >* this_ptr, const QVector< QColor >* l, QVector< QColor >* output) {
  new(output) QVector< QColor >(this_ptr->operator+(*l));
}

QVector< QColor >* qt_widgets_c_QVector_QColor_operator_assign(QVector< QColor >* this_ptr, const QVector< QColor >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_widgets_c_QVector_QColor_operator_eq(const QVector< QColor >* this_ptr, const QVector< QColor >* v) {
  return this_ptr->operator==(*v);
}

QColor* qt_widgets_c_QVector_QColor_operator_index(QVector< QColor >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QColor* qt_widgets_c_QVector_QColor_operator_index_const(const QVector< QColor >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QVector_QColor_operator_neq(const QVector< QColor >* this_ptr, const QVector< QColor >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QColor >* qt_widgets_c_QVector_QColor_operator_shl_l(QVector< QColor >* this_ptr, const QVector< QColor >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QColor >* qt_widgets_c_QVector_QColor_operator_shl_t(QVector< QColor >* this_ptr, const QColor* t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QVector_QColor_pop_back(QVector< QColor >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QVector_QColor_pop_front(QVector< QColor >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QVector_QColor_prepend(QVector< QColor >* this_ptr, const QColor* t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QVector_QColor_push_back(QVector< QColor >* this_ptr, const QColor* t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QVector_QColor_push_front(QVector< QColor >* this_ptr, const QColor* t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QVector_QColor_removeAll(QVector< QColor >* this_ptr, const QColor* t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QVector_QColor_removeAt(QVector< QColor >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QVector_QColor_removeFirst(QVector< QColor >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QVector_QColor_removeLast(QVector< QColor >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QVector_QColor_removeOne(QVector< QColor >* this_ptr, const QColor* t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QVector_QColor_remove_i(QVector< QColor >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_widgets_c_QVector_QColor_remove_i_n(QVector< QColor >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_widgets_c_QVector_QColor_replace(QVector< QColor >* this_ptr, int i, const QColor* t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QVector_QColor_reserve(QVector< QColor >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_widgets_c_QVector_QColor_resize(QVector< QColor >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_widgets_c_QVector_QColor_size(const QVector< QColor >* this_ptr) {
  return this_ptr->size();
}

void qt_widgets_c_QVector_QColor_squeeze(QVector< QColor >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_widgets_c_QVector_QColor_startsWith(const QVector< QColor >* this_ptr, const QColor* t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QVector_QColor_swap(QVector< QColor >* this_ptr, QVector< QColor >* other) {
  this_ptr->swap(*other);
}

void qt_widgets_c_QVector_QColor_takeAt_to_output(QVector< QColor >* this_ptr, int i, QColor* output) {
  new(output) QColor(this_ptr->takeAt(i));
}

void qt_widgets_c_QVector_QColor_takeFirst_to_output(QVector< QColor >* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->takeFirst());
}

void qt_widgets_c_QVector_QColor_takeLast_to_output(QVector< QColor >* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->takeLast());
}

void qt_widgets_c_QVector_QColor_value_to_output_i(const QVector< QColor >* this_ptr, int i, QColor* output) {
  new(output) QColor(this_ptr->value(i));
}

void qt_widgets_c_QVector_QColor_value_to_output_i_defaultValue(const QVector< QColor >* this_ptr, int i, const QColor* defaultValue, QColor* output) {
  new(output) QColor(this_ptr->value(i, *defaultValue));
}

void qt_widgets_c_QVector_void_ptr_append_l(QVector< void* >* this_ptr, const QVector< void* >* l) {
  this_ptr->append(*l);
}

void qt_widgets_c_QVector_void_ptr_append_t(QVector< void* >* this_ptr, void* const * t) {
  this_ptr->append(*t);
}

void* const * qt_widgets_c_QVector_void_ptr_at(const QVector< void* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

void** qt_widgets_c_QVector_void_ptr_back(QVector< void* >* this_ptr) {
  return &this_ptr->back();
}

void* const * qt_widgets_c_QVector_void_ptr_back_const(const QVector< void* >* this_ptr) {
  return &this_ptr->back();
}

int qt_widgets_c_QVector_void_ptr_capacity(const QVector< void* >* this_ptr) {
  return this_ptr->capacity();
}

void qt_widgets_c_QVector_void_ptr_clear(QVector< void* >* this_ptr) {
  this_ptr->clear();
}

void* const * qt_widgets_c_QVector_void_ptr_constData(const QVector< void* >* this_ptr) {
  return this_ptr->constData();
}

void* const * qt_widgets_c_QVector_void_ptr_constFirst(const QVector< void* >* this_ptr) {
  return &this_ptr->constFirst();
}

void* const * qt_widgets_c_QVector_void_ptr_constLast(const QVector< void* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QVector_void_ptr_constructor_no_args(QVector< void* >* output) {
  new(output) QVector< void* >();
}

void qt_widgets_c_QVector_void_ptr_constructor_size(int size, QVector< void* >* output) {
  new(output) QVector< void* >(size);
}

void qt_widgets_c_QVector_void_ptr_constructor_size_t(int size, void* const * t, QVector< void* >* output) {
  new(output) QVector< void* >(size, *t);
}

void qt_widgets_c_QVector_void_ptr_constructor_v(const QVector< void* >* v, QVector< void* >* output) {
  new(output) QVector< void* >(*v);
}

bool qt_widgets_c_QVector_void_ptr_contains(const QVector< void* >* this_ptr, void* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QVector_void_ptr_count_no_args(const QVector< void* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QVector_void_ptr_count_t(const QVector< void* >* this_ptr, void* const * t) {
  return this_ptr->count(*t);
}

void** qt_widgets_c_QVector_void_ptr_data(QVector< void* >* this_ptr) {
  return this_ptr->data();
}

void* const * qt_widgets_c_QVector_void_ptr_data_const(const QVector< void* >* this_ptr) {
  return this_ptr->data();
}

void qt_widgets_c_QVector_void_ptr_destructor(QVector< void* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QVector_void_ptr_empty(const QVector< void* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QVector_void_ptr_endsWith(const QVector< void* >* this_ptr, void* const * t) {
  return this_ptr->endsWith(*t);
}

QVector< void* >* qt_widgets_c_QVector_void_ptr_fill_t(QVector< void* >* this_ptr, void* const * t) {
  return &this_ptr->fill(*t);
}

QVector< void* >* qt_widgets_c_QVector_void_ptr_fill_t_size(QVector< void* >* this_ptr, void* const * t, int size) {
  return &this_ptr->fill(*t, size);
}

void** qt_widgets_c_QVector_void_ptr_first(QVector< void* >* this_ptr) {
  return &this_ptr->first();
}

void* const * qt_widgets_c_QVector_void_ptr_first_const(const QVector< void* >* this_ptr) {
  return &this_ptr->first();
}

void** qt_widgets_c_QVector_void_ptr_front(QVector< void* >* this_ptr) {
  return &this_ptr->front();
}

void* const * qt_widgets_c_QVector_void_ptr_front_const(const QVector< void* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QVector_void_ptr_indexOf_t(const QVector< void* >* this_ptr, void* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QVector_void_ptr_indexOf_t_from(const QVector< void* >* this_ptr, void* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QVector_void_ptr_insert_i_n_t(QVector< void* >* this_ptr, int i, int n, void* const * t) {
  this_ptr->insert(i, n, *t);
}

void qt_widgets_c_QVector_void_ptr_insert_i_t(QVector< void* >* this_ptr, int i, void* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QVector_void_ptr_isEmpty(const QVector< void* >* this_ptr) {
  return this_ptr->isEmpty();
}

void** qt_widgets_c_QVector_void_ptr_last(QVector< void* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QVector_void_ptr_lastIndexOf_t(const QVector< void* >* this_ptr, void* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QVector_void_ptr_lastIndexOf_t_from(const QVector< void* >* this_ptr, void* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

void* const * qt_widgets_c_QVector_void_ptr_last_const(const QVector< void* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QVector_void_ptr_length(const QVector< void* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QVector_void_ptr_mid_to_output_pos(const QVector< void* >* this_ptr, int pos, QVector< void* >* output) {
  new(output) QVector< void* >(this_ptr->mid(pos));
}

void qt_widgets_c_QVector_void_ptr_mid_to_output_pos_len(const QVector< void* >* this_ptr, int pos, int len, QVector< void* >* output) {
  new(output) QVector< void* >(this_ptr->mid(pos, len));
}

void qt_widgets_c_QVector_void_ptr_move(QVector< void* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< void* >* qt_widgets_c_QVector_void_ptr_operator_add_assign_l(QVector< void* >* this_ptr, const QVector< void* >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< void* >* qt_widgets_c_QVector_void_ptr_operator_add_assign_t(QVector< void* >* this_ptr, void* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QVector_void_ptr_operator_add_to_output(const QVector< void* >* this_ptr, const QVector< void* >* l, QVector< void* >* output) {
  new(output) QVector< void* >(this_ptr->operator+(*l));
}

QVector< void* >* qt_widgets_c_QVector_void_ptr_operator_assign(QVector< void* >* this_ptr, const QVector< void* >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_widgets_c_QVector_void_ptr_operator_eq(const QVector< void* >* this_ptr, const QVector< void* >* v) {
  return this_ptr->operator==(*v);
}

void** qt_widgets_c_QVector_void_ptr_operator_index(QVector< void* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

void* const * qt_widgets_c_QVector_void_ptr_operator_index_const(const QVector< void* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QVector_void_ptr_operator_neq(const QVector< void* >* this_ptr, const QVector< void* >* v) {
  return this_ptr->operator!=(*v);
}

QVector< void* >* qt_widgets_c_QVector_void_ptr_operator_shl_l(QVector< void* >* this_ptr, const QVector< void* >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< void* >* qt_widgets_c_QVector_void_ptr_operator_shl_t(QVector< void* >* this_ptr, void* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QVector_void_ptr_pop_back(QVector< void* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QVector_void_ptr_pop_front(QVector< void* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QVector_void_ptr_prepend(QVector< void* >* this_ptr, void* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QVector_void_ptr_push_back(QVector< void* >* this_ptr, void* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QVector_void_ptr_push_front(QVector< void* >* this_ptr, void* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QVector_void_ptr_removeAll(QVector< void* >* this_ptr, void* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QVector_void_ptr_removeAt(QVector< void* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QVector_void_ptr_removeFirst(QVector< void* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QVector_void_ptr_removeLast(QVector< void* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QVector_void_ptr_removeOne(QVector< void* >* this_ptr, void* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QVector_void_ptr_remove_i(QVector< void* >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_widgets_c_QVector_void_ptr_remove_i_n(QVector< void* >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_widgets_c_QVector_void_ptr_replace(QVector< void* >* this_ptr, int i, void* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QVector_void_ptr_reserve(QVector< void* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_widgets_c_QVector_void_ptr_resize(QVector< void* >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_widgets_c_QVector_void_ptr_size(const QVector< void* >* this_ptr) {
  return this_ptr->size();
}

void qt_widgets_c_QVector_void_ptr_squeeze(QVector< void* >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_widgets_c_QVector_void_ptr_startsWith(const QVector< void* >* this_ptr, void* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QVector_void_ptr_swap(QVector< void* >* this_ptr, QVector< void* >* other) {
  this_ptr->swap(*other);
}

void* qt_widgets_c_QVector_void_ptr_takeAt(QVector< void* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

void* qt_widgets_c_QVector_void_ptr_takeFirst(QVector< void* >* this_ptr) {
  return this_ptr->takeFirst();
}

void* qt_widgets_c_QVector_void_ptr_takeLast(QVector< void* >* this_ptr) {
  return this_ptr->takeLast();
}

void* qt_widgets_c_QVector_void_ptr_value_i(const QVector< void* >* this_ptr, int i) {
  return this_ptr->value(i);
}

void* qt_widgets_c_QVector_void_ptr_value_i_defaultValue(const QVector< void* >* this_ptr, int i, void* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

