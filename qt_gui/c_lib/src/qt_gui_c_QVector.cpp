#include "qt_gui_c_QVector.h"

void qt_gui_c_QVector_GLuint64_append_l(QVector< GLuint64 >* this_ptr, const QVector< GLuint64 >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_GLuint64_append_t(QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  this_ptr->append(*t);
}

const GLuint64* qt_gui_c_QVector_GLuint64_at(const QVector< GLuint64 >* this_ptr, int i) {
  return &this_ptr->at(i);
}

GLuint64* qt_gui_c_QVector_GLuint64_back(QVector< GLuint64 >* this_ptr) {
  return &this_ptr->back();
}

const GLuint64* qt_gui_c_QVector_GLuint64_back_const(const QVector< GLuint64 >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_GLuint64_capacity(const QVector< GLuint64 >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_GLuint64_clear(QVector< GLuint64 >* this_ptr) {
  this_ptr->clear();
}

const GLuint64* qt_gui_c_QVector_GLuint64_constData(const QVector< GLuint64 >* this_ptr) {
  return this_ptr->constData();
}

const GLuint64* qt_gui_c_QVector_GLuint64_constFirst(const QVector< GLuint64 >* this_ptr) {
  return &this_ptr->constFirst();
}

const GLuint64* qt_gui_c_QVector_GLuint64_constLast(const QVector< GLuint64 >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_GLuint64_constructor_no_args(QVector< GLuint64 >* output) {
  new(output) QVector< GLuint64 >();
}

void qt_gui_c_QVector_GLuint64_constructor_size(int size, QVector< GLuint64 >* output) {
  new(output) QVector< GLuint64 >(size);
}

void qt_gui_c_QVector_GLuint64_constructor_size_t(int size, const GLuint64* t, QVector< GLuint64 >* output) {
  new(output) QVector< GLuint64 >(size, *t);
}

void qt_gui_c_QVector_GLuint64_constructor_v(const QVector< GLuint64 >* v, QVector< GLuint64 >* output) {
  new(output) QVector< GLuint64 >(*v);
}

bool qt_gui_c_QVector_GLuint64_contains(const QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QVector_GLuint64_count_no_args(const QVector< GLuint64 >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QVector_GLuint64_count_t(const QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  return this_ptr->count(*t);
}

GLuint64* qt_gui_c_QVector_GLuint64_data(QVector< GLuint64 >* this_ptr) {
  return this_ptr->data();
}

const GLuint64* qt_gui_c_QVector_GLuint64_data_const(const QVector< GLuint64 >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_GLuint64_destructor(QVector< GLuint64 >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_GLuint64_empty(const QVector< GLuint64 >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QVector_GLuint64_endsWith(const QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  return this_ptr->endsWith(*t);
}

QVector< GLuint64 >* qt_gui_c_QVector_GLuint64_fill_t(QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  return &this_ptr->fill(*t);
}

QVector< GLuint64 >* qt_gui_c_QVector_GLuint64_fill_t_size(QVector< GLuint64 >* this_ptr, const GLuint64* t, int size) {
  return &this_ptr->fill(*t, size);
}

GLuint64* qt_gui_c_QVector_GLuint64_first(QVector< GLuint64 >* this_ptr) {
  return &this_ptr->first();
}

const GLuint64* qt_gui_c_QVector_GLuint64_first_const(const QVector< GLuint64 >* this_ptr) {
  return &this_ptr->first();
}

GLuint64* qt_gui_c_QVector_GLuint64_front(QVector< GLuint64 >* this_ptr) {
  return &this_ptr->front();
}

const GLuint64* qt_gui_c_QVector_GLuint64_front_const(const QVector< GLuint64 >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QVector_GLuint64_indexOf_t(const QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QVector_GLuint64_indexOf_t_from(const QVector< GLuint64 >* this_ptr, const GLuint64* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QVector_GLuint64_insert_i_n_t(QVector< GLuint64 >* this_ptr, int i, int n, const GLuint64* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_GLuint64_insert_i_t(QVector< GLuint64 >* this_ptr, int i, const GLuint64* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_GLuint64_isEmpty(const QVector< GLuint64 >* this_ptr) {
  return this_ptr->isEmpty();
}

GLuint64* qt_gui_c_QVector_GLuint64_last(QVector< GLuint64 >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_GLuint64_lastIndexOf_t(const QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QVector_GLuint64_lastIndexOf_t_from(const QVector< GLuint64 >* this_ptr, const GLuint64* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const GLuint64* qt_gui_c_QVector_GLuint64_last_const(const QVector< GLuint64 >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_GLuint64_length(const QVector< GLuint64 >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_GLuint64_mid_to_output_pos(const QVector< GLuint64 >* this_ptr, int pos, QVector< GLuint64 >* output) {
  new(output) QVector< GLuint64 >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_GLuint64_mid_to_output_pos_len(const QVector< GLuint64 >* this_ptr, int pos, int len, QVector< GLuint64 >* output) {
  new(output) QVector< GLuint64 >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_GLuint64_move(QVector< GLuint64 >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< GLuint64 >* qt_gui_c_QVector_GLuint64_operator_add_assign_l(QVector< GLuint64 >* this_ptr, const QVector< GLuint64 >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< GLuint64 >* qt_gui_c_QVector_GLuint64_operator_add_assign_t(QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_GLuint64_operator_add_to_output(const QVector< GLuint64 >* this_ptr, const QVector< GLuint64 >* l, QVector< GLuint64 >* output) {
  new(output) QVector< GLuint64 >(this_ptr->operator+(*l));
}

QVector< GLuint64 >* qt_gui_c_QVector_GLuint64_operator_assign(QVector< GLuint64 >* this_ptr, const QVector< GLuint64 >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_gui_c_QVector_GLuint64_operator_eq(const QVector< GLuint64 >* this_ptr, const QVector< GLuint64 >* v) {
  return this_ptr->operator==(*v);
}

GLuint64* qt_gui_c_QVector_GLuint64_operator_index(QVector< GLuint64 >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const GLuint64* qt_gui_c_QVector_GLuint64_operator_index_const(const QVector< GLuint64 >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QVector_GLuint64_operator_neq(const QVector< GLuint64 >* this_ptr, const QVector< GLuint64 >* v) {
  return this_ptr->operator!=(*v);
}

QVector< GLuint64 >* qt_gui_c_QVector_GLuint64_operator_shl_l(QVector< GLuint64 >* this_ptr, const QVector< GLuint64 >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< GLuint64 >* qt_gui_c_QVector_GLuint64_operator_shl_t(QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_GLuint64_pop_back(QVector< GLuint64 >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_GLuint64_pop_front(QVector< GLuint64 >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_GLuint64_prepend(QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_GLuint64_push_back(QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_GLuint64_push_front(QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QVector_GLuint64_removeAll(QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QVector_GLuint64_removeAt(QVector< GLuint64 >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_GLuint64_removeFirst(QVector< GLuint64 >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_GLuint64_removeLast(QVector< GLuint64 >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QVector_GLuint64_removeOne(QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QVector_GLuint64_remove_i(QVector< GLuint64 >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_GLuint64_remove_i_n(QVector< GLuint64 >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_GLuint64_replace(QVector< GLuint64 >* this_ptr, int i, const GLuint64* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_GLuint64_reserve(QVector< GLuint64 >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_GLuint64_resize(QVector< GLuint64 >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_GLuint64_size(const QVector< GLuint64 >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_GLuint64_squeeze(QVector< GLuint64 >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_gui_c_QVector_GLuint64_startsWith(const QVector< GLuint64 >* this_ptr, const GLuint64* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QVector_GLuint64_swap(QVector< GLuint64 >* this_ptr, QVector< GLuint64 >* other) {
  this_ptr->swap(*other);
}

GLuint64 qt_gui_c_QVector_GLuint64_takeAt(QVector< GLuint64 >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

GLuint64 qt_gui_c_QVector_GLuint64_takeFirst(QVector< GLuint64 >* this_ptr) {
  return this_ptr->takeFirst();
}

GLuint64 qt_gui_c_QVector_GLuint64_takeLast(QVector< GLuint64 >* this_ptr) {
  return this_ptr->takeLast();
}

GLuint64 qt_gui_c_QVector_GLuint64_value_i(const QVector< GLuint64 >* this_ptr, int i) {
  return this_ptr->value(i);
}

GLuint64 qt_gui_c_QVector_GLuint64_value_i_defaultValue(const QVector< GLuint64 >* this_ptr, int i, const GLuint64* defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_append_l(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, const QVector< QAbstractTextDocumentLayout::Selection >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_append_t(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, const QAbstractTextDocumentLayout::Selection* t) {
  this_ptr->append(*t);
}

const QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_at(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_back(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return &this_ptr->back();
}

const QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_back_const(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_capacity(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_clear(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  this_ptr->clear();
}

const QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_constData(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return this_ptr->constData();
}

const QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_constFirst(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return &this_ptr->constFirst();
}

const QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_constLast(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_constructor_no_args(QVector< QAbstractTextDocumentLayout::Selection >* output) {
  new(output) QVector< QAbstractTextDocumentLayout::Selection >();
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_constructor_size(int size, QVector< QAbstractTextDocumentLayout::Selection >* output) {
  new(output) QVector< QAbstractTextDocumentLayout::Selection >(size);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_constructor_size_t(int size, const QAbstractTextDocumentLayout::Selection* t, QVector< QAbstractTextDocumentLayout::Selection >* output) {
  new(output) QVector< QAbstractTextDocumentLayout::Selection >(size, *t);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_constructor_v(const QVector< QAbstractTextDocumentLayout::Selection >* v, QVector< QAbstractTextDocumentLayout::Selection >* output) {
  new(output) QVector< QAbstractTextDocumentLayout::Selection >(*v);
}

int qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_count(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return this_ptr->count();
}

QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_data(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return this_ptr->data();
}

const QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_data_const(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_destructor(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_empty(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return this_ptr->empty();
}

QVector< QAbstractTextDocumentLayout::Selection >* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_fill_t(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, const QAbstractTextDocumentLayout::Selection* t) {
  return &this_ptr->fill(*t);
}

QVector< QAbstractTextDocumentLayout::Selection >* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_fill_t_size(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, const QAbstractTextDocumentLayout::Selection* t, int size) {
  return &this_ptr->fill(*t, size);
}

QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_first(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return &this_ptr->first();
}

const QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_first_const(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return &this_ptr->first();
}

QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_front(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return &this_ptr->front();
}

const QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_front_const(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return &this_ptr->front();
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_insert_i_n_t(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int i, int n, const QAbstractTextDocumentLayout::Selection* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_insert_i_t(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int i, const QAbstractTextDocumentLayout::Selection* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_isEmpty(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return this_ptr->isEmpty();
}

QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_last(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return &this_ptr->last();
}

const QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_last_const(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_length(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_mid_to_output_pos(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int pos, QVector< QAbstractTextDocumentLayout::Selection >* output) {
  new(output) QVector< QAbstractTextDocumentLayout::Selection >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_mid_to_output_pos_len(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int pos, int len, QVector< QAbstractTextDocumentLayout::Selection >* output) {
  new(output) QVector< QAbstractTextDocumentLayout::Selection >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_move(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QAbstractTextDocumentLayout::Selection >* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_add_assign_l(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, const QVector< QAbstractTextDocumentLayout::Selection >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QAbstractTextDocumentLayout::Selection >* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_add_assign_t(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, const QAbstractTextDocumentLayout::Selection* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_add_to_output(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, const QVector< QAbstractTextDocumentLayout::Selection >* l, QVector< QAbstractTextDocumentLayout::Selection >* output) {
  new(output) QVector< QAbstractTextDocumentLayout::Selection >(this_ptr->operator+(*l));
}

QVector< QAbstractTextDocumentLayout::Selection >* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_assign(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, const QVector< QAbstractTextDocumentLayout::Selection >* v) {
  return &this_ptr->operator=(*v);
}

QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_index(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_index_const(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QVector< QAbstractTextDocumentLayout::Selection >* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_shl_l(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, const QVector< QAbstractTextDocumentLayout::Selection >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QAbstractTextDocumentLayout::Selection >* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_shl_t(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, const QAbstractTextDocumentLayout::Selection* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_pop_back(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_pop_front(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_prepend(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, const QAbstractTextDocumentLayout::Selection* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_push_back(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, const QAbstractTextDocumentLayout::Selection* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_push_front(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, const QAbstractTextDocumentLayout::Selection* t) {
  this_ptr->push_front(*t);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_removeAt(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_removeFirst(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_removeLast(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  this_ptr->removeLast();
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_remove_i(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_remove_i_n(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_replace(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int i, const QAbstractTextDocumentLayout::Selection* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_reserve(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_resize(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_size(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_squeeze(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  this_ptr->squeeze();
}

void qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_swap(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, QVector< QAbstractTextDocumentLayout::Selection >* other) {
  this_ptr->swap(*other);
}

QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_takeAt_as_ptr(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int i) {
  return new QAbstractTextDocumentLayout::Selection(this_ptr->takeAt(i));
}

QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_takeFirst_as_ptr(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return new QAbstractTextDocumentLayout::Selection(this_ptr->takeFirst());
}

QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_takeLast_as_ptr(QVector< QAbstractTextDocumentLayout::Selection >* this_ptr) {
  return new QAbstractTextDocumentLayout::Selection(this_ptr->takeLast());
}

QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_value_as_ptr_i(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int i) {
  return new QAbstractTextDocumentLayout::Selection(this_ptr->value(i));
}

QAbstractTextDocumentLayout::Selection* qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_value_as_ptr_i_defaultValue(const QVector< QAbstractTextDocumentLayout::Selection >* this_ptr, int i, const QAbstractTextDocumentLayout::Selection* defaultValue) {
  return new QAbstractTextDocumentLayout::Selection(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QVector_QLineF_append_l(QVector< QLineF >* this_ptr, const QVector< QLineF >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_QLineF_append_t(QVector< QLineF >* this_ptr, const QLineF* t) {
  this_ptr->append(*t);
}

const QLineF* qt_gui_c_QVector_QLineF_at(const QVector< QLineF >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QLineF* qt_gui_c_QVector_QLineF_back(QVector< QLineF >* this_ptr) {
  return &this_ptr->back();
}

const QLineF* qt_gui_c_QVector_QLineF_back_const(const QVector< QLineF >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_QLineF_capacity(const QVector< QLineF >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_QLineF_clear(QVector< QLineF >* this_ptr) {
  this_ptr->clear();
}

const QLineF* qt_gui_c_QVector_QLineF_constData(const QVector< QLineF >* this_ptr) {
  return this_ptr->constData();
}

const QLineF* qt_gui_c_QVector_QLineF_constFirst(const QVector< QLineF >* this_ptr) {
  return &this_ptr->constFirst();
}

const QLineF* qt_gui_c_QVector_QLineF_constLast(const QVector< QLineF >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_QLineF_constructor_no_args(QVector< QLineF >* output) {
  new(output) QVector< QLineF >();
}

void qt_gui_c_QVector_QLineF_constructor_size(int size, QVector< QLineF >* output) {
  new(output) QVector< QLineF >(size);
}

void qt_gui_c_QVector_QLineF_constructor_size_t(int size, const QLineF* t, QVector< QLineF >* output) {
  new(output) QVector< QLineF >(size, *t);
}

void qt_gui_c_QVector_QLineF_constructor_v(const QVector< QLineF >* v, QVector< QLineF >* output) {
  new(output) QVector< QLineF >(*v);
}

bool qt_gui_c_QVector_QLineF_contains(const QVector< QLineF >* this_ptr, const QLineF* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QVector_QLineF_count_no_args(const QVector< QLineF >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QVector_QLineF_count_t(const QVector< QLineF >* this_ptr, const QLineF* t) {
  return this_ptr->count(*t);
}

QLineF* qt_gui_c_QVector_QLineF_data(QVector< QLineF >* this_ptr) {
  return this_ptr->data();
}

const QLineF* qt_gui_c_QVector_QLineF_data_const(const QVector< QLineF >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_QLineF_destructor(QVector< QLineF >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_QLineF_empty(const QVector< QLineF >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QVector_QLineF_endsWith(const QVector< QLineF >* this_ptr, const QLineF* t) {
  return this_ptr->endsWith(*t);
}

QVector< QLineF >* qt_gui_c_QVector_QLineF_fill_t(QVector< QLineF >* this_ptr, const QLineF* t) {
  return &this_ptr->fill(*t);
}

QVector< QLineF >* qt_gui_c_QVector_QLineF_fill_t_size(QVector< QLineF >* this_ptr, const QLineF* t, int size) {
  return &this_ptr->fill(*t, size);
}

QLineF* qt_gui_c_QVector_QLineF_first(QVector< QLineF >* this_ptr) {
  return &this_ptr->first();
}

const QLineF* qt_gui_c_QVector_QLineF_first_const(const QVector< QLineF >* this_ptr) {
  return &this_ptr->first();
}

QLineF* qt_gui_c_QVector_QLineF_front(QVector< QLineF >* this_ptr) {
  return &this_ptr->front();
}

const QLineF* qt_gui_c_QVector_QLineF_front_const(const QVector< QLineF >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QVector_QLineF_indexOf_t(const QVector< QLineF >* this_ptr, const QLineF* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QVector_QLineF_indexOf_t_from(const QVector< QLineF >* this_ptr, const QLineF* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QVector_QLineF_insert_i_n_t(QVector< QLineF >* this_ptr, int i, int n, const QLineF* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_QLineF_insert_i_t(QVector< QLineF >* this_ptr, int i, const QLineF* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_QLineF_isEmpty(const QVector< QLineF >* this_ptr) {
  return this_ptr->isEmpty();
}

QLineF* qt_gui_c_QVector_QLineF_last(QVector< QLineF >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QLineF_lastIndexOf_t(const QVector< QLineF >* this_ptr, const QLineF* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QVector_QLineF_lastIndexOf_t_from(const QVector< QLineF >* this_ptr, const QLineF* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QLineF* qt_gui_c_QVector_QLineF_last_const(const QVector< QLineF >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QLineF_length(const QVector< QLineF >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_QLineF_mid_to_output_pos(const QVector< QLineF >* this_ptr, int pos, QVector< QLineF >* output) {
  new(output) QVector< QLineF >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_QLineF_mid_to_output_pos_len(const QVector< QLineF >* this_ptr, int pos, int len, QVector< QLineF >* output) {
  new(output) QVector< QLineF >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_QLineF_move(QVector< QLineF >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QLineF >* qt_gui_c_QVector_QLineF_operator_add_assign_l(QVector< QLineF >* this_ptr, const QVector< QLineF >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QLineF >* qt_gui_c_QVector_QLineF_operator_add_assign_t(QVector< QLineF >* this_ptr, const QLineF* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_QLineF_operator_add_to_output(const QVector< QLineF >* this_ptr, const QVector< QLineF >* l, QVector< QLineF >* output) {
  new(output) QVector< QLineF >(this_ptr->operator+(*l));
}

QVector< QLineF >* qt_gui_c_QVector_QLineF_operator_assign(QVector< QLineF >* this_ptr, const QVector< QLineF >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_gui_c_QVector_QLineF_operator_eq(const QVector< QLineF >* this_ptr, const QVector< QLineF >* v) {
  return this_ptr->operator==(*v);
}

QLineF* qt_gui_c_QVector_QLineF_operator_index(QVector< QLineF >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QLineF* qt_gui_c_QVector_QLineF_operator_index_const(const QVector< QLineF >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QVector_QLineF_operator_neq(const QVector< QLineF >* this_ptr, const QVector< QLineF >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QLineF >* qt_gui_c_QVector_QLineF_operator_shl_l(QVector< QLineF >* this_ptr, const QVector< QLineF >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QLineF >* qt_gui_c_QVector_QLineF_operator_shl_t(QVector< QLineF >* this_ptr, const QLineF* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_QLineF_pop_back(QVector< QLineF >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_QLineF_pop_front(QVector< QLineF >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_QLineF_prepend(QVector< QLineF >* this_ptr, const QLineF* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_QLineF_push_back(QVector< QLineF >* this_ptr, const QLineF* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_QLineF_push_front(QVector< QLineF >* this_ptr, const QLineF* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QVector_QLineF_removeAll(QVector< QLineF >* this_ptr, const QLineF* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QVector_QLineF_removeAt(QVector< QLineF >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_QLineF_removeFirst(QVector< QLineF >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_QLineF_removeLast(QVector< QLineF >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QVector_QLineF_removeOne(QVector< QLineF >* this_ptr, const QLineF* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QVector_QLineF_remove_i(QVector< QLineF >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_QLineF_remove_i_n(QVector< QLineF >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_QLineF_replace(QVector< QLineF >* this_ptr, int i, const QLineF* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_QLineF_reserve(QVector< QLineF >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_QLineF_resize(QVector< QLineF >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_QLineF_size(const QVector< QLineF >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_QLineF_squeeze(QVector< QLineF >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_gui_c_QVector_QLineF_startsWith(const QVector< QLineF >* this_ptr, const QLineF* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QVector_QLineF_swap(QVector< QLineF >* this_ptr, QVector< QLineF >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QVector_QLineF_takeAt_to_output(QVector< QLineF >* this_ptr, int i, QLineF* output) {
  new(output) QLineF(this_ptr->takeAt(i));
}

void qt_gui_c_QVector_QLineF_takeFirst_to_output(QVector< QLineF >* this_ptr, QLineF* output) {
  new(output) QLineF(this_ptr->takeFirst());
}

void qt_gui_c_QVector_QLineF_takeLast_to_output(QVector< QLineF >* this_ptr, QLineF* output) {
  new(output) QLineF(this_ptr->takeLast());
}

void qt_gui_c_QVector_QLineF_value_to_output_i(const QVector< QLineF >* this_ptr, int i, QLineF* output) {
  new(output) QLineF(this_ptr->value(i));
}

void qt_gui_c_QVector_QLineF_value_to_output_i_defaultValue(const QVector< QLineF >* this_ptr, int i, const QLineF* defaultValue, QLineF* output) {
  new(output) QLineF(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QVector_QLine_append_l(QVector< QLine >* this_ptr, const QVector< QLine >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_QLine_append_t(QVector< QLine >* this_ptr, const QLine* t) {
  this_ptr->append(*t);
}

const QLine* qt_gui_c_QVector_QLine_at(const QVector< QLine >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QLine* qt_gui_c_QVector_QLine_back(QVector< QLine >* this_ptr) {
  return &this_ptr->back();
}

const QLine* qt_gui_c_QVector_QLine_back_const(const QVector< QLine >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_QLine_capacity(const QVector< QLine >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_QLine_clear(QVector< QLine >* this_ptr) {
  this_ptr->clear();
}

const QLine* qt_gui_c_QVector_QLine_constData(const QVector< QLine >* this_ptr) {
  return this_ptr->constData();
}

const QLine* qt_gui_c_QVector_QLine_constFirst(const QVector< QLine >* this_ptr) {
  return &this_ptr->constFirst();
}

const QLine* qt_gui_c_QVector_QLine_constLast(const QVector< QLine >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_QLine_constructor_no_args(QVector< QLine >* output) {
  new(output) QVector< QLine >();
}

void qt_gui_c_QVector_QLine_constructor_size(int size, QVector< QLine >* output) {
  new(output) QVector< QLine >(size);
}

void qt_gui_c_QVector_QLine_constructor_size_t(int size, const QLine* t, QVector< QLine >* output) {
  new(output) QVector< QLine >(size, *t);
}

void qt_gui_c_QVector_QLine_constructor_v(const QVector< QLine >* v, QVector< QLine >* output) {
  new(output) QVector< QLine >(*v);
}

bool qt_gui_c_QVector_QLine_contains(const QVector< QLine >* this_ptr, const QLine* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QVector_QLine_count_no_args(const QVector< QLine >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QVector_QLine_count_t(const QVector< QLine >* this_ptr, const QLine* t) {
  return this_ptr->count(*t);
}

QLine* qt_gui_c_QVector_QLine_data(QVector< QLine >* this_ptr) {
  return this_ptr->data();
}

const QLine* qt_gui_c_QVector_QLine_data_const(const QVector< QLine >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_QLine_destructor(QVector< QLine >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_QLine_empty(const QVector< QLine >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QVector_QLine_endsWith(const QVector< QLine >* this_ptr, const QLine* t) {
  return this_ptr->endsWith(*t);
}

QVector< QLine >* qt_gui_c_QVector_QLine_fill_t(QVector< QLine >* this_ptr, const QLine* t) {
  return &this_ptr->fill(*t);
}

QVector< QLine >* qt_gui_c_QVector_QLine_fill_t_size(QVector< QLine >* this_ptr, const QLine* t, int size) {
  return &this_ptr->fill(*t, size);
}

QLine* qt_gui_c_QVector_QLine_first(QVector< QLine >* this_ptr) {
  return &this_ptr->first();
}

const QLine* qt_gui_c_QVector_QLine_first_const(const QVector< QLine >* this_ptr) {
  return &this_ptr->first();
}

QLine* qt_gui_c_QVector_QLine_front(QVector< QLine >* this_ptr) {
  return &this_ptr->front();
}

const QLine* qt_gui_c_QVector_QLine_front_const(const QVector< QLine >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QVector_QLine_indexOf_t(const QVector< QLine >* this_ptr, const QLine* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QVector_QLine_indexOf_t_from(const QVector< QLine >* this_ptr, const QLine* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QVector_QLine_insert_i_n_t(QVector< QLine >* this_ptr, int i, int n, const QLine* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_QLine_insert_i_t(QVector< QLine >* this_ptr, int i, const QLine* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_QLine_isEmpty(const QVector< QLine >* this_ptr) {
  return this_ptr->isEmpty();
}

QLine* qt_gui_c_QVector_QLine_last(QVector< QLine >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QLine_lastIndexOf_t(const QVector< QLine >* this_ptr, const QLine* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QVector_QLine_lastIndexOf_t_from(const QVector< QLine >* this_ptr, const QLine* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QLine* qt_gui_c_QVector_QLine_last_const(const QVector< QLine >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QLine_length(const QVector< QLine >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_QLine_mid_to_output_pos(const QVector< QLine >* this_ptr, int pos, QVector< QLine >* output) {
  new(output) QVector< QLine >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_QLine_mid_to_output_pos_len(const QVector< QLine >* this_ptr, int pos, int len, QVector< QLine >* output) {
  new(output) QVector< QLine >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_QLine_move(QVector< QLine >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QLine >* qt_gui_c_QVector_QLine_operator_add_assign_l(QVector< QLine >* this_ptr, const QVector< QLine >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QLine >* qt_gui_c_QVector_QLine_operator_add_assign_t(QVector< QLine >* this_ptr, const QLine* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_QLine_operator_add_to_output(const QVector< QLine >* this_ptr, const QVector< QLine >* l, QVector< QLine >* output) {
  new(output) QVector< QLine >(this_ptr->operator+(*l));
}

QVector< QLine >* qt_gui_c_QVector_QLine_operator_assign(QVector< QLine >* this_ptr, const QVector< QLine >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_gui_c_QVector_QLine_operator_eq(const QVector< QLine >* this_ptr, const QVector< QLine >* v) {
  return this_ptr->operator==(*v);
}

QLine* qt_gui_c_QVector_QLine_operator_index(QVector< QLine >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QLine* qt_gui_c_QVector_QLine_operator_index_const(const QVector< QLine >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QVector_QLine_operator_neq(const QVector< QLine >* this_ptr, const QVector< QLine >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QLine >* qt_gui_c_QVector_QLine_operator_shl_l(QVector< QLine >* this_ptr, const QVector< QLine >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QLine >* qt_gui_c_QVector_QLine_operator_shl_t(QVector< QLine >* this_ptr, const QLine* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_QLine_pop_back(QVector< QLine >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_QLine_pop_front(QVector< QLine >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_QLine_prepend(QVector< QLine >* this_ptr, const QLine* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_QLine_push_back(QVector< QLine >* this_ptr, const QLine* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_QLine_push_front(QVector< QLine >* this_ptr, const QLine* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QVector_QLine_removeAll(QVector< QLine >* this_ptr, const QLine* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QVector_QLine_removeAt(QVector< QLine >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_QLine_removeFirst(QVector< QLine >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_QLine_removeLast(QVector< QLine >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QVector_QLine_removeOne(QVector< QLine >* this_ptr, const QLine* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QVector_QLine_remove_i(QVector< QLine >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_QLine_remove_i_n(QVector< QLine >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_QLine_replace(QVector< QLine >* this_ptr, int i, const QLine* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_QLine_reserve(QVector< QLine >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_QLine_resize(QVector< QLine >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_QLine_size(const QVector< QLine >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_QLine_squeeze(QVector< QLine >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_gui_c_QVector_QLine_startsWith(const QVector< QLine >* this_ptr, const QLine* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QVector_QLine_swap(QVector< QLine >* this_ptr, QVector< QLine >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QVector_QLine_takeAt_to_output(QVector< QLine >* this_ptr, int i, QLine* output) {
  new(output) QLine(this_ptr->takeAt(i));
}

void qt_gui_c_QVector_QLine_takeFirst_to_output(QVector< QLine >* this_ptr, QLine* output) {
  new(output) QLine(this_ptr->takeFirst());
}

void qt_gui_c_QVector_QLine_takeLast_to_output(QVector< QLine >* this_ptr, QLine* output) {
  new(output) QLine(this_ptr->takeLast());
}

void qt_gui_c_QVector_QLine_value_to_output_i(const QVector< QLine >* this_ptr, int i, QLine* output) {
  new(output) QLine(this_ptr->value(i));
}

void qt_gui_c_QVector_QLine_value_to_output_i_defaultValue(const QVector< QLine >* this_ptr, int i, const QLine* defaultValue, QLine* output) {
  new(output) QLine(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QVector_QPair_double_QColor_append_l(QVector< QPair< double, QColor > >* this_ptr, const QVector< QPair< double, QColor > >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_QPair_double_QColor_append_t(QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  this_ptr->append(*t);
}

const QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_at(const QVector< QPair< double, QColor > >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_back(QVector< QPair< double, QColor > >* this_ptr) {
  return &this_ptr->back();
}

const QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_back_const(const QVector< QPair< double, QColor > >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_QPair_double_QColor_capacity(const QVector< QPair< double, QColor > >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_QPair_double_QColor_clear(QVector< QPair< double, QColor > >* this_ptr) {
  this_ptr->clear();
}

const QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_constData(const QVector< QPair< double, QColor > >* this_ptr) {
  return this_ptr->constData();
}

const QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_constFirst(const QVector< QPair< double, QColor > >* this_ptr) {
  return &this_ptr->constFirst();
}

const QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_constLast(const QVector< QPair< double, QColor > >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_QPair_double_QColor_constructor_no_args(QVector< QPair< double, QColor > >* output) {
  new(output) QVector< QPair< double, QColor > >();
}

void qt_gui_c_QVector_QPair_double_QColor_constructor_size(int size, QVector< QPair< double, QColor > >* output) {
  new(output) QVector< QPair< double, QColor > >(size);
}

void qt_gui_c_QVector_QPair_double_QColor_constructor_size_t(int size, const QPair< double, QColor >* t, QVector< QPair< double, QColor > >* output) {
  new(output) QVector< QPair< double, QColor > >(size, *t);
}

void qt_gui_c_QVector_QPair_double_QColor_constructor_v(const QVector< QPair< double, QColor > >* v, QVector< QPair< double, QColor > >* output) {
  new(output) QVector< QPair< double, QColor > >(*v);
}

bool qt_gui_c_QVector_QPair_double_QColor_contains(const QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QVector_QPair_double_QColor_count_no_args(const QVector< QPair< double, QColor > >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QVector_QPair_double_QColor_count_t(const QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  return this_ptr->count(*t);
}

QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_data(QVector< QPair< double, QColor > >* this_ptr) {
  return this_ptr->data();
}

const QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_data_const(const QVector< QPair< double, QColor > >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_QPair_double_QColor_destructor(QVector< QPair< double, QColor > >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_QPair_double_QColor_empty(const QVector< QPair< double, QColor > >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QVector_QPair_double_QColor_endsWith(const QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  return this_ptr->endsWith(*t);
}

QVector< QPair< double, QColor > >* qt_gui_c_QVector_QPair_double_QColor_fill_t(QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  return &this_ptr->fill(*t);
}

QVector< QPair< double, QColor > >* qt_gui_c_QVector_QPair_double_QColor_fill_t_size(QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t, int size) {
  return &this_ptr->fill(*t, size);
}

QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_first(QVector< QPair< double, QColor > >* this_ptr) {
  return &this_ptr->first();
}

const QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_first_const(const QVector< QPair< double, QColor > >* this_ptr) {
  return &this_ptr->first();
}

QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_front(QVector< QPair< double, QColor > >* this_ptr) {
  return &this_ptr->front();
}

const QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_front_const(const QVector< QPair< double, QColor > >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QVector_QPair_double_QColor_indexOf_t(const QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QVector_QPair_double_QColor_indexOf_t_from(const QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QVector_QPair_double_QColor_insert_i_n_t(QVector< QPair< double, QColor > >* this_ptr, int i, int n, const QPair< double, QColor >* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_QPair_double_QColor_insert_i_t(QVector< QPair< double, QColor > >* this_ptr, int i, const QPair< double, QColor >* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_QPair_double_QColor_isEmpty(const QVector< QPair< double, QColor > >* this_ptr) {
  return this_ptr->isEmpty();
}

QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_last(QVector< QPair< double, QColor > >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QPair_double_QColor_lastIndexOf_t(const QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QVector_QPair_double_QColor_lastIndexOf_t_from(const QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_last_const(const QVector< QPair< double, QColor > >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QPair_double_QColor_length(const QVector< QPair< double, QColor > >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_QPair_double_QColor_mid_to_output_pos(const QVector< QPair< double, QColor > >* this_ptr, int pos, QVector< QPair< double, QColor > >* output) {
  new(output) QVector< QPair< double, QColor > >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_QPair_double_QColor_mid_to_output_pos_len(const QVector< QPair< double, QColor > >* this_ptr, int pos, int len, QVector< QPair< double, QColor > >* output) {
  new(output) QVector< QPair< double, QColor > >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_QPair_double_QColor_move(QVector< QPair< double, QColor > >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QPair< double, QColor > >* qt_gui_c_QVector_QPair_double_QColor_operator_add_assign_l(QVector< QPair< double, QColor > >* this_ptr, const QVector< QPair< double, QColor > >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QPair< double, QColor > >* qt_gui_c_QVector_QPair_double_QColor_operator_add_assign_t(QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_QPair_double_QColor_operator_add_to_output(const QVector< QPair< double, QColor > >* this_ptr, const QVector< QPair< double, QColor > >* l, QVector< QPair< double, QColor > >* output) {
  new(output) QVector< QPair< double, QColor > >(this_ptr->operator+(*l));
}

QVector< QPair< double, QColor > >* qt_gui_c_QVector_QPair_double_QColor_operator_assign(QVector< QPair< double, QColor > >* this_ptr, const QVector< QPair< double, QColor > >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_gui_c_QVector_QPair_double_QColor_operator_eq(const QVector< QPair< double, QColor > >* this_ptr, const QVector< QPair< double, QColor > >* v) {
  return this_ptr->operator==(*v);
}

QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_operator_index(QVector< QPair< double, QColor > >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QPair< double, QColor >* qt_gui_c_QVector_QPair_double_QColor_operator_index_const(const QVector< QPair< double, QColor > >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QVector_QPair_double_QColor_operator_neq(const QVector< QPair< double, QColor > >* this_ptr, const QVector< QPair< double, QColor > >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QPair< double, QColor > >* qt_gui_c_QVector_QPair_double_QColor_operator_shl_l(QVector< QPair< double, QColor > >* this_ptr, const QVector< QPair< double, QColor > >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QPair< double, QColor > >* qt_gui_c_QVector_QPair_double_QColor_operator_shl_t(QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_QPair_double_QColor_pop_back(QVector< QPair< double, QColor > >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_QPair_double_QColor_pop_front(QVector< QPair< double, QColor > >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_QPair_double_QColor_prepend(QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_QPair_double_QColor_push_back(QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_QPair_double_QColor_push_front(QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QVector_QPair_double_QColor_removeAll(QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QVector_QPair_double_QColor_removeAt(QVector< QPair< double, QColor > >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_QPair_double_QColor_removeFirst(QVector< QPair< double, QColor > >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_QPair_double_QColor_removeLast(QVector< QPair< double, QColor > >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QVector_QPair_double_QColor_removeOne(QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QVector_QPair_double_QColor_remove_i(QVector< QPair< double, QColor > >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_QPair_double_QColor_remove_i_n(QVector< QPair< double, QColor > >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_QPair_double_QColor_replace(QVector< QPair< double, QColor > >* this_ptr, int i, const QPair< double, QColor >* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_QPair_double_QColor_reserve(QVector< QPair< double, QColor > >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_QPair_double_QColor_resize(QVector< QPair< double, QColor > >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_QPair_double_QColor_size(const QVector< QPair< double, QColor > >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_QPair_double_QColor_squeeze(QVector< QPair< double, QColor > >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_gui_c_QVector_QPair_double_QColor_startsWith(const QVector< QPair< double, QColor > >* this_ptr, const QPair< double, QColor >* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QVector_QPair_double_QColor_swap(QVector< QPair< double, QColor > >* this_ptr, QVector< QPair< double, QColor > >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QVector_QPair_double_QColor_takeAt_to_output(QVector< QPair< double, QColor > >* this_ptr, int i, QPair< double, QColor >* output) {
  new(output) QPair< double, QColor >(this_ptr->takeAt(i));
}

void qt_gui_c_QVector_QPair_double_QColor_takeFirst_to_output(QVector< QPair< double, QColor > >* this_ptr, QPair< double, QColor >* output) {
  new(output) QPair< double, QColor >(this_ptr->takeFirst());
}

void qt_gui_c_QVector_QPair_double_QColor_takeLast_to_output(QVector< QPair< double, QColor > >* this_ptr, QPair< double, QColor >* output) {
  new(output) QPair< double, QColor >(this_ptr->takeLast());
}

void qt_gui_c_QVector_QPair_double_QColor_value_to_output_i(const QVector< QPair< double, QColor > >* this_ptr, int i, QPair< double, QColor >* output) {
  new(output) QPair< double, QColor >(this_ptr->value(i));
}

void qt_gui_c_QVector_QPair_double_QColor_value_to_output_i_defaultValue(const QVector< QPair< double, QColor > >* this_ptr, int i, const QPair< double, QColor >* defaultValue, QPair< double, QColor >* output) {
  new(output) QPair< double, QColor >(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QVector_QPoint_append_l(QVector< QPoint >* this_ptr, const QVector< QPoint >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_QPoint_append_t(QVector< QPoint >* this_ptr, const QPoint* t) {
  this_ptr->append(*t);
}

const QPoint* qt_gui_c_QVector_QPoint_at(const QVector< QPoint >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QPoint* qt_gui_c_QVector_QPoint_back(QVector< QPoint >* this_ptr) {
  return &this_ptr->back();
}

const QPoint* qt_gui_c_QVector_QPoint_back_const(const QVector< QPoint >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_QPoint_capacity(const QVector< QPoint >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_QPoint_clear(QVector< QPoint >* this_ptr) {
  this_ptr->clear();
}

const QPoint* qt_gui_c_QVector_QPoint_constData(const QVector< QPoint >* this_ptr) {
  return this_ptr->constData();
}

const QPoint* qt_gui_c_QVector_QPoint_constFirst(const QVector< QPoint >* this_ptr) {
  return &this_ptr->constFirst();
}

const QPoint* qt_gui_c_QVector_QPoint_constLast(const QVector< QPoint >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_QPoint_constructor_no_args(QVector< QPoint >* output) {
  new(output) QVector< QPoint >();
}

void qt_gui_c_QVector_QPoint_constructor_size(int size, QVector< QPoint >* output) {
  new(output) QVector< QPoint >(size);
}

void qt_gui_c_QVector_QPoint_constructor_size_t(int size, const QPoint* t, QVector< QPoint >* output) {
  new(output) QVector< QPoint >(size, *t);
}

void qt_gui_c_QVector_QPoint_constructor_v(const QVector< QPoint >* v, QVector< QPoint >* output) {
  new(output) QVector< QPoint >(*v);
}

bool qt_gui_c_QVector_QPoint_contains(const QVector< QPoint >* this_ptr, const QPoint* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QVector_QPoint_count_no_args(const QVector< QPoint >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QVector_QPoint_count_t(const QVector< QPoint >* this_ptr, const QPoint* t) {
  return this_ptr->count(*t);
}

QPoint* qt_gui_c_QVector_QPoint_data(QVector< QPoint >* this_ptr) {
  return this_ptr->data();
}

const QPoint* qt_gui_c_QVector_QPoint_data_const(const QVector< QPoint >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_QPoint_destructor(QVector< QPoint >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_QPoint_empty(const QVector< QPoint >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QVector_QPoint_endsWith(const QVector< QPoint >* this_ptr, const QPoint* t) {
  return this_ptr->endsWith(*t);
}

QVector< QPoint >* qt_gui_c_QVector_QPoint_fill_t(QVector< QPoint >* this_ptr, const QPoint* t) {
  return &this_ptr->fill(*t);
}

QVector< QPoint >* qt_gui_c_QVector_QPoint_fill_t_size(QVector< QPoint >* this_ptr, const QPoint* t, int size) {
  return &this_ptr->fill(*t, size);
}

QPoint* qt_gui_c_QVector_QPoint_first(QVector< QPoint >* this_ptr) {
  return &this_ptr->first();
}

const QPoint* qt_gui_c_QVector_QPoint_first_const(const QVector< QPoint >* this_ptr) {
  return &this_ptr->first();
}

QPoint* qt_gui_c_QVector_QPoint_front(QVector< QPoint >* this_ptr) {
  return &this_ptr->front();
}

const QPoint* qt_gui_c_QVector_QPoint_front_const(const QVector< QPoint >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QVector_QPoint_indexOf_t(const QVector< QPoint >* this_ptr, const QPoint* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QVector_QPoint_indexOf_t_from(const QVector< QPoint >* this_ptr, const QPoint* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QVector_QPoint_insert_i_n_t(QVector< QPoint >* this_ptr, int i, int n, const QPoint* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_QPoint_insert_i_t(QVector< QPoint >* this_ptr, int i, const QPoint* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_QPoint_isEmpty(const QVector< QPoint >* this_ptr) {
  return this_ptr->isEmpty();
}

QPoint* qt_gui_c_QVector_QPoint_last(QVector< QPoint >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QPoint_lastIndexOf_t(const QVector< QPoint >* this_ptr, const QPoint* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QVector_QPoint_lastIndexOf_t_from(const QVector< QPoint >* this_ptr, const QPoint* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QPoint* qt_gui_c_QVector_QPoint_last_const(const QVector< QPoint >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QPoint_length(const QVector< QPoint >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_QPoint_mid_to_output_pos(const QVector< QPoint >* this_ptr, int pos, QVector< QPoint >* output) {
  new(output) QVector< QPoint >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_QPoint_mid_to_output_pos_len(const QVector< QPoint >* this_ptr, int pos, int len, QVector< QPoint >* output) {
  new(output) QVector< QPoint >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_QPoint_move(QVector< QPoint >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QPoint >* qt_gui_c_QVector_QPoint_operator_add_assign_l(QVector< QPoint >* this_ptr, const QVector< QPoint >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QPoint >* qt_gui_c_QVector_QPoint_operator_add_assign_t(QVector< QPoint >* this_ptr, const QPoint* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_QPoint_operator_add_to_output(const QVector< QPoint >* this_ptr, const QVector< QPoint >* l, QVector< QPoint >* output) {
  new(output) QVector< QPoint >(this_ptr->operator+(*l));
}

QVector< QPoint >* qt_gui_c_QVector_QPoint_operator_assign(QVector< QPoint >* this_ptr, const QVector< QPoint >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_gui_c_QVector_QPoint_operator_eq(const QVector< QPoint >* this_ptr, const QVector< QPoint >* v) {
  return this_ptr->operator==(*v);
}

QPoint* qt_gui_c_QVector_QPoint_operator_index(QVector< QPoint >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QPoint* qt_gui_c_QVector_QPoint_operator_index_const(const QVector< QPoint >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QVector_QPoint_operator_neq(const QVector< QPoint >* this_ptr, const QVector< QPoint >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QPoint >* qt_gui_c_QVector_QPoint_operator_shl_l(QVector< QPoint >* this_ptr, const QVector< QPoint >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QPoint >* qt_gui_c_QVector_QPoint_operator_shl_t(QVector< QPoint >* this_ptr, const QPoint* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_QPoint_pop_back(QVector< QPoint >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_QPoint_pop_front(QVector< QPoint >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_QPoint_prepend(QVector< QPoint >* this_ptr, const QPoint* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_QPoint_push_back(QVector< QPoint >* this_ptr, const QPoint* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_QPoint_push_front(QVector< QPoint >* this_ptr, const QPoint* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QVector_QPoint_removeAll(QVector< QPoint >* this_ptr, const QPoint* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QVector_QPoint_removeAt(QVector< QPoint >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_QPoint_removeFirst(QVector< QPoint >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_QPoint_removeLast(QVector< QPoint >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QVector_QPoint_removeOne(QVector< QPoint >* this_ptr, const QPoint* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QVector_QPoint_remove_i(QVector< QPoint >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_QPoint_remove_i_n(QVector< QPoint >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_QPoint_replace(QVector< QPoint >* this_ptr, int i, const QPoint* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_QPoint_reserve(QVector< QPoint >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_QPoint_resize(QVector< QPoint >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_QPoint_size(const QVector< QPoint >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_QPoint_squeeze(QVector< QPoint >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_gui_c_QVector_QPoint_startsWith(const QVector< QPoint >* this_ptr, const QPoint* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QVector_QPoint_swap(QVector< QPoint >* this_ptr, QVector< QPoint >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QVector_QPoint_takeAt_to_output(QVector< QPoint >* this_ptr, int i, QPoint* output) {
  new(output) QPoint(this_ptr->takeAt(i));
}

void qt_gui_c_QVector_QPoint_takeFirst_to_output(QVector< QPoint >* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->takeFirst());
}

void qt_gui_c_QVector_QPoint_takeLast_to_output(QVector< QPoint >* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->takeLast());
}

void qt_gui_c_QVector_QPoint_value_to_output_i(const QVector< QPoint >* this_ptr, int i, QPoint* output) {
  new(output) QPoint(this_ptr->value(i));
}

void qt_gui_c_QVector_QPoint_value_to_output_i_defaultValue(const QVector< QPoint >* this_ptr, int i, const QPoint* defaultValue, QPoint* output) {
  new(output) QPoint(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QVector_QRectF_append_l(QVector< QRectF >* this_ptr, const QVector< QRectF >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_QRectF_append_t(QVector< QRectF >* this_ptr, const QRectF* t) {
  this_ptr->append(*t);
}

const QRectF* qt_gui_c_QVector_QRectF_at(const QVector< QRectF >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QRectF* qt_gui_c_QVector_QRectF_back(QVector< QRectF >* this_ptr) {
  return &this_ptr->back();
}

const QRectF* qt_gui_c_QVector_QRectF_back_const(const QVector< QRectF >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_QRectF_capacity(const QVector< QRectF >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_QRectF_clear(QVector< QRectF >* this_ptr) {
  this_ptr->clear();
}

const QRectF* qt_gui_c_QVector_QRectF_constData(const QVector< QRectF >* this_ptr) {
  return this_ptr->constData();
}

const QRectF* qt_gui_c_QVector_QRectF_constFirst(const QVector< QRectF >* this_ptr) {
  return &this_ptr->constFirst();
}

const QRectF* qt_gui_c_QVector_QRectF_constLast(const QVector< QRectF >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_QRectF_constructor_no_args(QVector< QRectF >* output) {
  new(output) QVector< QRectF >();
}

void qt_gui_c_QVector_QRectF_constructor_size(int size, QVector< QRectF >* output) {
  new(output) QVector< QRectF >(size);
}

void qt_gui_c_QVector_QRectF_constructor_size_t(int size, const QRectF* t, QVector< QRectF >* output) {
  new(output) QVector< QRectF >(size, *t);
}

void qt_gui_c_QVector_QRectF_constructor_v(const QVector< QRectF >* v, QVector< QRectF >* output) {
  new(output) QVector< QRectF >(*v);
}

bool qt_gui_c_QVector_QRectF_contains(const QVector< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QVector_QRectF_count_no_args(const QVector< QRectF >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QVector_QRectF_count_t(const QVector< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->count(*t);
}

QRectF* qt_gui_c_QVector_QRectF_data(QVector< QRectF >* this_ptr) {
  return this_ptr->data();
}

const QRectF* qt_gui_c_QVector_QRectF_data_const(const QVector< QRectF >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_QRectF_destructor(QVector< QRectF >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_QRectF_empty(const QVector< QRectF >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QVector_QRectF_endsWith(const QVector< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->endsWith(*t);
}

QVector< QRectF >* qt_gui_c_QVector_QRectF_fill_t(QVector< QRectF >* this_ptr, const QRectF* t) {
  return &this_ptr->fill(*t);
}

QVector< QRectF >* qt_gui_c_QVector_QRectF_fill_t_size(QVector< QRectF >* this_ptr, const QRectF* t, int size) {
  return &this_ptr->fill(*t, size);
}

QRectF* qt_gui_c_QVector_QRectF_first(QVector< QRectF >* this_ptr) {
  return &this_ptr->first();
}

const QRectF* qt_gui_c_QVector_QRectF_first_const(const QVector< QRectF >* this_ptr) {
  return &this_ptr->first();
}

QRectF* qt_gui_c_QVector_QRectF_front(QVector< QRectF >* this_ptr) {
  return &this_ptr->front();
}

const QRectF* qt_gui_c_QVector_QRectF_front_const(const QVector< QRectF >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QVector_QRectF_indexOf_t(const QVector< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QVector_QRectF_indexOf_t_from(const QVector< QRectF >* this_ptr, const QRectF* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QVector_QRectF_insert_i_n_t(QVector< QRectF >* this_ptr, int i, int n, const QRectF* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_QRectF_insert_i_t(QVector< QRectF >* this_ptr, int i, const QRectF* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_QRectF_isEmpty(const QVector< QRectF >* this_ptr) {
  return this_ptr->isEmpty();
}

QRectF* qt_gui_c_QVector_QRectF_last(QVector< QRectF >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QRectF_lastIndexOf_t(const QVector< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QVector_QRectF_lastIndexOf_t_from(const QVector< QRectF >* this_ptr, const QRectF* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QRectF* qt_gui_c_QVector_QRectF_last_const(const QVector< QRectF >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QRectF_length(const QVector< QRectF >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_QRectF_mid_to_output_pos(const QVector< QRectF >* this_ptr, int pos, QVector< QRectF >* output) {
  new(output) QVector< QRectF >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_QRectF_mid_to_output_pos_len(const QVector< QRectF >* this_ptr, int pos, int len, QVector< QRectF >* output) {
  new(output) QVector< QRectF >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_QRectF_move(QVector< QRectF >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QRectF >* qt_gui_c_QVector_QRectF_operator_add_assign_l(QVector< QRectF >* this_ptr, const QVector< QRectF >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QRectF >* qt_gui_c_QVector_QRectF_operator_add_assign_t(QVector< QRectF >* this_ptr, const QRectF* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_QRectF_operator_add_to_output(const QVector< QRectF >* this_ptr, const QVector< QRectF >* l, QVector< QRectF >* output) {
  new(output) QVector< QRectF >(this_ptr->operator+(*l));
}

QVector< QRectF >* qt_gui_c_QVector_QRectF_operator_assign(QVector< QRectF >* this_ptr, const QVector< QRectF >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_gui_c_QVector_QRectF_operator_eq(const QVector< QRectF >* this_ptr, const QVector< QRectF >* v) {
  return this_ptr->operator==(*v);
}

QRectF* qt_gui_c_QVector_QRectF_operator_index(QVector< QRectF >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QRectF* qt_gui_c_QVector_QRectF_operator_index_const(const QVector< QRectF >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QVector_QRectF_operator_neq(const QVector< QRectF >* this_ptr, const QVector< QRectF >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QRectF >* qt_gui_c_QVector_QRectF_operator_shl_l(QVector< QRectF >* this_ptr, const QVector< QRectF >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QRectF >* qt_gui_c_QVector_QRectF_operator_shl_t(QVector< QRectF >* this_ptr, const QRectF* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_QRectF_pop_back(QVector< QRectF >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_QRectF_pop_front(QVector< QRectF >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_QRectF_prepend(QVector< QRectF >* this_ptr, const QRectF* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_QRectF_push_back(QVector< QRectF >* this_ptr, const QRectF* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_QRectF_push_front(QVector< QRectF >* this_ptr, const QRectF* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QVector_QRectF_removeAll(QVector< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QVector_QRectF_removeAt(QVector< QRectF >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_QRectF_removeFirst(QVector< QRectF >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_QRectF_removeLast(QVector< QRectF >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QVector_QRectF_removeOne(QVector< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QVector_QRectF_remove_i(QVector< QRectF >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_QRectF_remove_i_n(QVector< QRectF >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_QRectF_replace(QVector< QRectF >* this_ptr, int i, const QRectF* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_QRectF_reserve(QVector< QRectF >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_QRectF_resize(QVector< QRectF >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_QRectF_size(const QVector< QRectF >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_QRectF_squeeze(QVector< QRectF >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_gui_c_QVector_QRectF_startsWith(const QVector< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QVector_QRectF_swap(QVector< QRectF >* this_ptr, QVector< QRectF >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QVector_QRectF_takeAt_to_output(QVector< QRectF >* this_ptr, int i, QRectF* output) {
  new(output) QRectF(this_ptr->takeAt(i));
}

void qt_gui_c_QVector_QRectF_takeFirst_to_output(QVector< QRectF >* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->takeFirst());
}

void qt_gui_c_QVector_QRectF_takeLast_to_output(QVector< QRectF >* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->takeLast());
}

void qt_gui_c_QVector_QRectF_value_to_output_i(const QVector< QRectF >* this_ptr, int i, QRectF* output) {
  new(output) QRectF(this_ptr->value(i));
}

void qt_gui_c_QVector_QRectF_value_to_output_i_defaultValue(const QVector< QRectF >* this_ptr, int i, const QRectF* defaultValue, QRectF* output) {
  new(output) QRectF(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QVector_QRect_append_l(QVector< QRect >* this_ptr, const QVector< QRect >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_QRect_append_t(QVector< QRect >* this_ptr, const QRect* t) {
  this_ptr->append(*t);
}

const QRect* qt_gui_c_QVector_QRect_at(const QVector< QRect >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QRect* qt_gui_c_QVector_QRect_back(QVector< QRect >* this_ptr) {
  return &this_ptr->back();
}

const QRect* qt_gui_c_QVector_QRect_back_const(const QVector< QRect >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_QRect_capacity(const QVector< QRect >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_QRect_clear(QVector< QRect >* this_ptr) {
  this_ptr->clear();
}

const QRect* qt_gui_c_QVector_QRect_constData(const QVector< QRect >* this_ptr) {
  return this_ptr->constData();
}

const QRect* qt_gui_c_QVector_QRect_constFirst(const QVector< QRect >* this_ptr) {
  return &this_ptr->constFirst();
}

const QRect* qt_gui_c_QVector_QRect_constLast(const QVector< QRect >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_QRect_constructor_no_args(QVector< QRect >* output) {
  new(output) QVector< QRect >();
}

void qt_gui_c_QVector_QRect_constructor_size(int size, QVector< QRect >* output) {
  new(output) QVector< QRect >(size);
}

void qt_gui_c_QVector_QRect_constructor_size_t(int size, const QRect* t, QVector< QRect >* output) {
  new(output) QVector< QRect >(size, *t);
}

void qt_gui_c_QVector_QRect_constructor_v(const QVector< QRect >* v, QVector< QRect >* output) {
  new(output) QVector< QRect >(*v);
}

bool qt_gui_c_QVector_QRect_contains(const QVector< QRect >* this_ptr, const QRect* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QVector_QRect_count_no_args(const QVector< QRect >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QVector_QRect_count_t(const QVector< QRect >* this_ptr, const QRect* t) {
  return this_ptr->count(*t);
}

QRect* qt_gui_c_QVector_QRect_data(QVector< QRect >* this_ptr) {
  return this_ptr->data();
}

const QRect* qt_gui_c_QVector_QRect_data_const(const QVector< QRect >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_QRect_destructor(QVector< QRect >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_QRect_empty(const QVector< QRect >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QVector_QRect_endsWith(const QVector< QRect >* this_ptr, const QRect* t) {
  return this_ptr->endsWith(*t);
}

QVector< QRect >* qt_gui_c_QVector_QRect_fill_t(QVector< QRect >* this_ptr, const QRect* t) {
  return &this_ptr->fill(*t);
}

QVector< QRect >* qt_gui_c_QVector_QRect_fill_t_size(QVector< QRect >* this_ptr, const QRect* t, int size) {
  return &this_ptr->fill(*t, size);
}

QRect* qt_gui_c_QVector_QRect_first(QVector< QRect >* this_ptr) {
  return &this_ptr->first();
}

const QRect* qt_gui_c_QVector_QRect_first_const(const QVector< QRect >* this_ptr) {
  return &this_ptr->first();
}

QRect* qt_gui_c_QVector_QRect_front(QVector< QRect >* this_ptr) {
  return &this_ptr->front();
}

const QRect* qt_gui_c_QVector_QRect_front_const(const QVector< QRect >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QVector_QRect_indexOf_t(const QVector< QRect >* this_ptr, const QRect* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QVector_QRect_indexOf_t_from(const QVector< QRect >* this_ptr, const QRect* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QVector_QRect_insert_i_n_t(QVector< QRect >* this_ptr, int i, int n, const QRect* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_QRect_insert_i_t(QVector< QRect >* this_ptr, int i, const QRect* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_QRect_isEmpty(const QVector< QRect >* this_ptr) {
  return this_ptr->isEmpty();
}

QRect* qt_gui_c_QVector_QRect_last(QVector< QRect >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QRect_lastIndexOf_t(const QVector< QRect >* this_ptr, const QRect* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QVector_QRect_lastIndexOf_t_from(const QVector< QRect >* this_ptr, const QRect* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QRect* qt_gui_c_QVector_QRect_last_const(const QVector< QRect >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QRect_length(const QVector< QRect >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_QRect_mid_to_output_pos(const QVector< QRect >* this_ptr, int pos, QVector< QRect >* output) {
  new(output) QVector< QRect >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_QRect_mid_to_output_pos_len(const QVector< QRect >* this_ptr, int pos, int len, QVector< QRect >* output) {
  new(output) QVector< QRect >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_QRect_move(QVector< QRect >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QRect >* qt_gui_c_QVector_QRect_operator_add_assign_l(QVector< QRect >* this_ptr, const QVector< QRect >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QRect >* qt_gui_c_QVector_QRect_operator_add_assign_t(QVector< QRect >* this_ptr, const QRect* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_QRect_operator_add_to_output(const QVector< QRect >* this_ptr, const QVector< QRect >* l, QVector< QRect >* output) {
  new(output) QVector< QRect >(this_ptr->operator+(*l));
}

QVector< QRect >* qt_gui_c_QVector_QRect_operator_assign(QVector< QRect >* this_ptr, const QVector< QRect >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_gui_c_QVector_QRect_operator_eq(const QVector< QRect >* this_ptr, const QVector< QRect >* v) {
  return this_ptr->operator==(*v);
}

QRect* qt_gui_c_QVector_QRect_operator_index(QVector< QRect >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QRect* qt_gui_c_QVector_QRect_operator_index_const(const QVector< QRect >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QVector_QRect_operator_neq(const QVector< QRect >* this_ptr, const QVector< QRect >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QRect >* qt_gui_c_QVector_QRect_operator_shl_l(QVector< QRect >* this_ptr, const QVector< QRect >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QRect >* qt_gui_c_QVector_QRect_operator_shl_t(QVector< QRect >* this_ptr, const QRect* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_QRect_pop_back(QVector< QRect >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_QRect_pop_front(QVector< QRect >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_QRect_prepend(QVector< QRect >* this_ptr, const QRect* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_QRect_push_back(QVector< QRect >* this_ptr, const QRect* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_QRect_push_front(QVector< QRect >* this_ptr, const QRect* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QVector_QRect_removeAll(QVector< QRect >* this_ptr, const QRect* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QVector_QRect_removeAt(QVector< QRect >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_QRect_removeFirst(QVector< QRect >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_QRect_removeLast(QVector< QRect >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QVector_QRect_removeOne(QVector< QRect >* this_ptr, const QRect* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QVector_QRect_remove_i(QVector< QRect >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_QRect_remove_i_n(QVector< QRect >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_QRect_replace(QVector< QRect >* this_ptr, int i, const QRect* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_QRect_reserve(QVector< QRect >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_QRect_resize(QVector< QRect >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_QRect_size(const QVector< QRect >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_QRect_squeeze(QVector< QRect >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_gui_c_QVector_QRect_startsWith(const QVector< QRect >* this_ptr, const QRect* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QVector_QRect_swap(QVector< QRect >* this_ptr, QVector< QRect >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QVector_QRect_takeAt_to_output(QVector< QRect >* this_ptr, int i, QRect* output) {
  new(output) QRect(this_ptr->takeAt(i));
}

void qt_gui_c_QVector_QRect_takeFirst_to_output(QVector< QRect >* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->takeFirst());
}

void qt_gui_c_QVector_QRect_takeLast_to_output(QVector< QRect >* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->takeLast());
}

void qt_gui_c_QVector_QRect_value_to_output_i(const QVector< QRect >* this_ptr, int i, QRect* output) {
  new(output) QRect(this_ptr->value(i));
}

void qt_gui_c_QVector_QRect_value_to_output_i_defaultValue(const QVector< QRect >* this_ptr, int i, const QRect* defaultValue, QRect* output) {
  new(output) QRect(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QVector_QSize_append_l(QVector< QSize >* this_ptr, const QVector< QSize >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_QSize_append_t(QVector< QSize >* this_ptr, const QSize* t) {
  this_ptr->append(*t);
}

const QSize* qt_gui_c_QVector_QSize_at(const QVector< QSize >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QSize* qt_gui_c_QVector_QSize_back(QVector< QSize >* this_ptr) {
  return &this_ptr->back();
}

const QSize* qt_gui_c_QVector_QSize_back_const(const QVector< QSize >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_QSize_capacity(const QVector< QSize >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_QSize_clear(QVector< QSize >* this_ptr) {
  this_ptr->clear();
}

const QSize* qt_gui_c_QVector_QSize_constData(const QVector< QSize >* this_ptr) {
  return this_ptr->constData();
}

const QSize* qt_gui_c_QVector_QSize_constFirst(const QVector< QSize >* this_ptr) {
  return &this_ptr->constFirst();
}

const QSize* qt_gui_c_QVector_QSize_constLast(const QVector< QSize >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_QSize_constructor_no_args(QVector< QSize >* output) {
  new(output) QVector< QSize >();
}

void qt_gui_c_QVector_QSize_constructor_size(int size, QVector< QSize >* output) {
  new(output) QVector< QSize >(size);
}

void qt_gui_c_QVector_QSize_constructor_size_t(int size, const QSize* t, QVector< QSize >* output) {
  new(output) QVector< QSize >(size, *t);
}

void qt_gui_c_QVector_QSize_constructor_v(const QVector< QSize >* v, QVector< QSize >* output) {
  new(output) QVector< QSize >(*v);
}

bool qt_gui_c_QVector_QSize_contains(const QVector< QSize >* this_ptr, const QSize* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QVector_QSize_count_no_args(const QVector< QSize >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QVector_QSize_count_t(const QVector< QSize >* this_ptr, const QSize* t) {
  return this_ptr->count(*t);
}

QSize* qt_gui_c_QVector_QSize_data(QVector< QSize >* this_ptr) {
  return this_ptr->data();
}

const QSize* qt_gui_c_QVector_QSize_data_const(const QVector< QSize >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_QSize_destructor(QVector< QSize >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_QSize_empty(const QVector< QSize >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QVector_QSize_endsWith(const QVector< QSize >* this_ptr, const QSize* t) {
  return this_ptr->endsWith(*t);
}

QVector< QSize >* qt_gui_c_QVector_QSize_fill_t(QVector< QSize >* this_ptr, const QSize* t) {
  return &this_ptr->fill(*t);
}

QVector< QSize >* qt_gui_c_QVector_QSize_fill_t_size(QVector< QSize >* this_ptr, const QSize* t, int size) {
  return &this_ptr->fill(*t, size);
}

QSize* qt_gui_c_QVector_QSize_first(QVector< QSize >* this_ptr) {
  return &this_ptr->first();
}

const QSize* qt_gui_c_QVector_QSize_first_const(const QVector< QSize >* this_ptr) {
  return &this_ptr->first();
}

void qt_gui_c_QVector_QSize_fromList_to_output(const QList< QSize >* list, QVector< QSize >* output) {
  new(output) QVector< QSize >(QVector< QSize >::fromList(*list));
}

QSize* qt_gui_c_QVector_QSize_front(QVector< QSize >* this_ptr) {
  return &this_ptr->front();
}

const QSize* qt_gui_c_QVector_QSize_front_const(const QVector< QSize >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QVector_QSize_indexOf_t(const QVector< QSize >* this_ptr, const QSize* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QVector_QSize_indexOf_t_from(const QVector< QSize >* this_ptr, const QSize* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QVector_QSize_insert_i_n_t(QVector< QSize >* this_ptr, int i, int n, const QSize* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_QSize_insert_i_t(QVector< QSize >* this_ptr, int i, const QSize* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_QSize_isEmpty(const QVector< QSize >* this_ptr) {
  return this_ptr->isEmpty();
}

QSize* qt_gui_c_QVector_QSize_last(QVector< QSize >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QSize_lastIndexOf_t(const QVector< QSize >* this_ptr, const QSize* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QVector_QSize_lastIndexOf_t_from(const QVector< QSize >* this_ptr, const QSize* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QSize* qt_gui_c_QVector_QSize_last_const(const QVector< QSize >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QSize_length(const QVector< QSize >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_QSize_mid_to_output_pos(const QVector< QSize >* this_ptr, int pos, QVector< QSize >* output) {
  new(output) QVector< QSize >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_QSize_mid_to_output_pos_len(const QVector< QSize >* this_ptr, int pos, int len, QVector< QSize >* output) {
  new(output) QVector< QSize >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_QSize_move(QVector< QSize >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QSize >* qt_gui_c_QVector_QSize_operator_add_assign_l(QVector< QSize >* this_ptr, const QVector< QSize >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QSize >* qt_gui_c_QVector_QSize_operator_add_assign_t(QVector< QSize >* this_ptr, const QSize* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_QSize_operator_add_to_output(const QVector< QSize >* this_ptr, const QVector< QSize >* l, QVector< QSize >* output) {
  new(output) QVector< QSize >(this_ptr->operator+(*l));
}

QVector< QSize >* qt_gui_c_QVector_QSize_operator_assign(QVector< QSize >* this_ptr, const QVector< QSize >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_gui_c_QVector_QSize_operator_eq(const QVector< QSize >* this_ptr, const QVector< QSize >* v) {
  return this_ptr->operator==(*v);
}

QSize* qt_gui_c_QVector_QSize_operator_index(QVector< QSize >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QSize* qt_gui_c_QVector_QSize_operator_index_const(const QVector< QSize >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QVector_QSize_operator_neq(const QVector< QSize >* this_ptr, const QVector< QSize >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QSize >* qt_gui_c_QVector_QSize_operator_shl_l(QVector< QSize >* this_ptr, const QVector< QSize >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QSize >* qt_gui_c_QVector_QSize_operator_shl_t(QVector< QSize >* this_ptr, const QSize* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_QSize_pop_back(QVector< QSize >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_QSize_pop_front(QVector< QSize >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_QSize_prepend(QVector< QSize >* this_ptr, const QSize* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_QSize_push_back(QVector< QSize >* this_ptr, const QSize* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_QSize_push_front(QVector< QSize >* this_ptr, const QSize* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QVector_QSize_removeAll(QVector< QSize >* this_ptr, const QSize* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QVector_QSize_removeAt(QVector< QSize >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_QSize_removeFirst(QVector< QSize >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_QSize_removeLast(QVector< QSize >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QVector_QSize_removeOne(QVector< QSize >* this_ptr, const QSize* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QVector_QSize_remove_i(QVector< QSize >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_QSize_remove_i_n(QVector< QSize >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_QSize_replace(QVector< QSize >* this_ptr, int i, const QSize* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_QSize_reserve(QVector< QSize >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_QSize_resize(QVector< QSize >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_QSize_size(const QVector< QSize >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_QSize_squeeze(QVector< QSize >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_gui_c_QVector_QSize_startsWith(const QVector< QSize >* this_ptr, const QSize* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QVector_QSize_swap(QVector< QSize >* this_ptr, QVector< QSize >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QVector_QSize_takeAt_to_output(QVector< QSize >* this_ptr, int i, QSize* output) {
  new(output) QSize(this_ptr->takeAt(i));
}

void qt_gui_c_QVector_QSize_takeFirst_to_output(QVector< QSize >* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->takeFirst());
}

void qt_gui_c_QVector_QSize_takeLast_to_output(QVector< QSize >* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->takeLast());
}

void qt_gui_c_QVector_QSize_toList_to_output(const QVector< QSize >* this_ptr, QList< QSize >* output) {
  new(output) QList< QSize >(this_ptr->toList());
}

void qt_gui_c_QVector_QSize_value_to_output_i(const QVector< QSize >* this_ptr, int i, QSize* output) {
  new(output) QSize(this_ptr->value(i));
}

void qt_gui_c_QVector_QSize_value_to_output_i_defaultValue(const QVector< QSize >* this_ptr, int i, const QSize* defaultValue, QSize* output) {
  new(output) QSize(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QVector_QTextFormat_append_l(QVector< QTextFormat >* this_ptr, const QVector< QTextFormat >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_QTextFormat_append_t(QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  this_ptr->append(*t);
}

const QTextFormat* qt_gui_c_QVector_QTextFormat_at(const QVector< QTextFormat >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QTextFormat* qt_gui_c_QVector_QTextFormat_back(QVector< QTextFormat >* this_ptr) {
  return &this_ptr->back();
}

const QTextFormat* qt_gui_c_QVector_QTextFormat_back_const(const QVector< QTextFormat >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_QTextFormat_capacity(const QVector< QTextFormat >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_QTextFormat_clear(QVector< QTextFormat >* this_ptr) {
  this_ptr->clear();
}

const QTextFormat* qt_gui_c_QVector_QTextFormat_constData(const QVector< QTextFormat >* this_ptr) {
  return this_ptr->constData();
}

const QTextFormat* qt_gui_c_QVector_QTextFormat_constFirst(const QVector< QTextFormat >* this_ptr) {
  return &this_ptr->constFirst();
}

const QTextFormat* qt_gui_c_QVector_QTextFormat_constLast(const QVector< QTextFormat >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_QTextFormat_constructor_no_args(QVector< QTextFormat >* output) {
  new(output) QVector< QTextFormat >();
}

void qt_gui_c_QVector_QTextFormat_constructor_size(int size, QVector< QTextFormat >* output) {
  new(output) QVector< QTextFormat >(size);
}

void qt_gui_c_QVector_QTextFormat_constructor_size_t(int size, const QTextFormat* t, QVector< QTextFormat >* output) {
  new(output) QVector< QTextFormat >(size, *t);
}

void qt_gui_c_QVector_QTextFormat_constructor_v(const QVector< QTextFormat >* v, QVector< QTextFormat >* output) {
  new(output) QVector< QTextFormat >(*v);
}

bool qt_gui_c_QVector_QTextFormat_contains(const QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QVector_QTextFormat_count_no_args(const QVector< QTextFormat >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QVector_QTextFormat_count_t(const QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  return this_ptr->count(*t);
}

QTextFormat* qt_gui_c_QVector_QTextFormat_data(QVector< QTextFormat >* this_ptr) {
  return this_ptr->data();
}

const QTextFormat* qt_gui_c_QVector_QTextFormat_data_const(const QVector< QTextFormat >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_QTextFormat_destructor(QVector< QTextFormat >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_QTextFormat_empty(const QVector< QTextFormat >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QVector_QTextFormat_endsWith(const QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  return this_ptr->endsWith(*t);
}

QVector< QTextFormat >* qt_gui_c_QVector_QTextFormat_fill_t(QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  return &this_ptr->fill(*t);
}

QVector< QTextFormat >* qt_gui_c_QVector_QTextFormat_fill_t_size(QVector< QTextFormat >* this_ptr, const QTextFormat* t, int size) {
  return &this_ptr->fill(*t, size);
}

QTextFormat* qt_gui_c_QVector_QTextFormat_first(QVector< QTextFormat >* this_ptr) {
  return &this_ptr->first();
}

const QTextFormat* qt_gui_c_QVector_QTextFormat_first_const(const QVector< QTextFormat >* this_ptr) {
  return &this_ptr->first();
}

QTextFormat* qt_gui_c_QVector_QTextFormat_front(QVector< QTextFormat >* this_ptr) {
  return &this_ptr->front();
}

const QTextFormat* qt_gui_c_QVector_QTextFormat_front_const(const QVector< QTextFormat >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QVector_QTextFormat_indexOf_t(const QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QVector_QTextFormat_indexOf_t_from(const QVector< QTextFormat >* this_ptr, const QTextFormat* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QVector_QTextFormat_insert_i_n_t(QVector< QTextFormat >* this_ptr, int i, int n, const QTextFormat* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_QTextFormat_insert_i_t(QVector< QTextFormat >* this_ptr, int i, const QTextFormat* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_QTextFormat_isEmpty(const QVector< QTextFormat >* this_ptr) {
  return this_ptr->isEmpty();
}

QTextFormat* qt_gui_c_QVector_QTextFormat_last(QVector< QTextFormat >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QTextFormat_lastIndexOf_t(const QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QVector_QTextFormat_lastIndexOf_t_from(const QVector< QTextFormat >* this_ptr, const QTextFormat* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QTextFormat* qt_gui_c_QVector_QTextFormat_last_const(const QVector< QTextFormat >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QTextFormat_length(const QVector< QTextFormat >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_QTextFormat_mid_to_output_pos(const QVector< QTextFormat >* this_ptr, int pos, QVector< QTextFormat >* output) {
  new(output) QVector< QTextFormat >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_QTextFormat_mid_to_output_pos_len(const QVector< QTextFormat >* this_ptr, int pos, int len, QVector< QTextFormat >* output) {
  new(output) QVector< QTextFormat >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_QTextFormat_move(QVector< QTextFormat >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QTextFormat >* qt_gui_c_QVector_QTextFormat_operator_add_assign_l(QVector< QTextFormat >* this_ptr, const QVector< QTextFormat >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QTextFormat >* qt_gui_c_QVector_QTextFormat_operator_add_assign_t(QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_QTextFormat_operator_add_to_output(const QVector< QTextFormat >* this_ptr, const QVector< QTextFormat >* l, QVector< QTextFormat >* output) {
  new(output) QVector< QTextFormat >(this_ptr->operator+(*l));
}

QVector< QTextFormat >* qt_gui_c_QVector_QTextFormat_operator_assign(QVector< QTextFormat >* this_ptr, const QVector< QTextFormat >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_gui_c_QVector_QTextFormat_operator_eq(const QVector< QTextFormat >* this_ptr, const QVector< QTextFormat >* v) {
  return this_ptr->operator==(*v);
}

QTextFormat* qt_gui_c_QVector_QTextFormat_operator_index(QVector< QTextFormat >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QTextFormat* qt_gui_c_QVector_QTextFormat_operator_index_const(const QVector< QTextFormat >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QVector_QTextFormat_operator_neq(const QVector< QTextFormat >* this_ptr, const QVector< QTextFormat >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QTextFormat >* qt_gui_c_QVector_QTextFormat_operator_shl_l(QVector< QTextFormat >* this_ptr, const QVector< QTextFormat >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QTextFormat >* qt_gui_c_QVector_QTextFormat_operator_shl_t(QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_QTextFormat_pop_back(QVector< QTextFormat >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_QTextFormat_pop_front(QVector< QTextFormat >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_QTextFormat_prepend(QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_QTextFormat_push_back(QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_QTextFormat_push_front(QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QVector_QTextFormat_removeAll(QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QVector_QTextFormat_removeAt(QVector< QTextFormat >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_QTextFormat_removeFirst(QVector< QTextFormat >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_QTextFormat_removeLast(QVector< QTextFormat >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QVector_QTextFormat_removeOne(QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QVector_QTextFormat_remove_i(QVector< QTextFormat >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_QTextFormat_remove_i_n(QVector< QTextFormat >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_QTextFormat_replace(QVector< QTextFormat >* this_ptr, int i, const QTextFormat* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_QTextFormat_reserve(QVector< QTextFormat >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_QTextFormat_resize(QVector< QTextFormat >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_QTextFormat_size(const QVector< QTextFormat >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_QTextFormat_squeeze(QVector< QTextFormat >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_gui_c_QVector_QTextFormat_startsWith(const QVector< QTextFormat >* this_ptr, const QTextFormat* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QVector_QTextFormat_swap(QVector< QTextFormat >* this_ptr, QVector< QTextFormat >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QVector_QTextFormat_takeAt_to_output(QVector< QTextFormat >* this_ptr, int i, QTextFormat* output) {
  new(output) QTextFormat(this_ptr->takeAt(i));
}

void qt_gui_c_QVector_QTextFormat_takeFirst_to_output(QVector< QTextFormat >* this_ptr, QTextFormat* output) {
  new(output) QTextFormat(this_ptr->takeFirst());
}

void qt_gui_c_QVector_QTextFormat_takeLast_to_output(QVector< QTextFormat >* this_ptr, QTextFormat* output) {
  new(output) QTextFormat(this_ptr->takeLast());
}

void qt_gui_c_QVector_QTextFormat_value_to_output_i(const QVector< QTextFormat >* this_ptr, int i, QTextFormat* output) {
  new(output) QTextFormat(this_ptr->value(i));
}

void qt_gui_c_QVector_QTextFormat_value_to_output_i_defaultValue(const QVector< QTextFormat >* this_ptr, int i, const QTextFormat* defaultValue, QTextFormat* output) {
  new(output) QTextFormat(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QVector_QTextLayout_FormatRange_append_l(QVector< QTextLayout::FormatRange >* this_ptr, const QVector< QTextLayout::FormatRange >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_QTextLayout_FormatRange_append_t(QVector< QTextLayout::FormatRange >* this_ptr, const QTextLayout::FormatRange* t) {
  this_ptr->append(*t);
}

const QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_at(const QVector< QTextLayout::FormatRange >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_back(QVector< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->back();
}

const QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_back_const(const QVector< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_QTextLayout_FormatRange_capacity(const QVector< QTextLayout::FormatRange >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_QTextLayout_FormatRange_clear(QVector< QTextLayout::FormatRange >* this_ptr) {
  this_ptr->clear();
}

const QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_constData(const QVector< QTextLayout::FormatRange >* this_ptr) {
  return this_ptr->constData();
}

const QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_constFirst(const QVector< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->constFirst();
}

const QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_constLast(const QVector< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_QTextLayout_FormatRange_constructor_no_args(QVector< QTextLayout::FormatRange >* output) {
  new(output) QVector< QTextLayout::FormatRange >();
}

void qt_gui_c_QVector_QTextLayout_FormatRange_constructor_size(int size, QVector< QTextLayout::FormatRange >* output) {
  new(output) QVector< QTextLayout::FormatRange >(size);
}

void qt_gui_c_QVector_QTextLayout_FormatRange_constructor_size_t(int size, const QTextLayout::FormatRange* t, QVector< QTextLayout::FormatRange >* output) {
  new(output) QVector< QTextLayout::FormatRange >(size, *t);
}

void qt_gui_c_QVector_QTextLayout_FormatRange_constructor_v(const QVector< QTextLayout::FormatRange >* v, QVector< QTextLayout::FormatRange >* output) {
  new(output) QVector< QTextLayout::FormatRange >(*v);
}

int qt_gui_c_QVector_QTextLayout_FormatRange_count(const QVector< QTextLayout::FormatRange >* this_ptr) {
  return this_ptr->count();
}

QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_data(QVector< QTextLayout::FormatRange >* this_ptr) {
  return this_ptr->data();
}

const QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_data_const(const QVector< QTextLayout::FormatRange >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_QTextLayout_FormatRange_destructor(QVector< QTextLayout::FormatRange >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_QTextLayout_FormatRange_empty(const QVector< QTextLayout::FormatRange >* this_ptr) {
  return this_ptr->empty();
}

QVector< QTextLayout::FormatRange >* qt_gui_c_QVector_QTextLayout_FormatRange_fill_t(QVector< QTextLayout::FormatRange >* this_ptr, const QTextLayout::FormatRange* t) {
  return &this_ptr->fill(*t);
}

QVector< QTextLayout::FormatRange >* qt_gui_c_QVector_QTextLayout_FormatRange_fill_t_size(QVector< QTextLayout::FormatRange >* this_ptr, const QTextLayout::FormatRange* t, int size) {
  return &this_ptr->fill(*t, size);
}

QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_first(QVector< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->first();
}

const QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_first_const(const QVector< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->first();
}

void qt_gui_c_QVector_QTextLayout_FormatRange_fromList_to_output(const QList< QTextLayout::FormatRange >* list, QVector< QTextLayout::FormatRange >* output) {
  new(output) QVector< QTextLayout::FormatRange >(QVector< QTextLayout::FormatRange >::fromList(*list));
}

QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_front(QVector< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->front();
}

const QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_front_const(const QVector< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->front();
}

void qt_gui_c_QVector_QTextLayout_FormatRange_insert_i_n_t(QVector< QTextLayout::FormatRange >* this_ptr, int i, int n, const QTextLayout::FormatRange* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_QTextLayout_FormatRange_insert_i_t(QVector< QTextLayout::FormatRange >* this_ptr, int i, const QTextLayout::FormatRange* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_QTextLayout_FormatRange_isEmpty(const QVector< QTextLayout::FormatRange >* this_ptr) {
  return this_ptr->isEmpty();
}

QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_last(QVector< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->last();
}

const QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_last_const(const QVector< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QTextLayout_FormatRange_length(const QVector< QTextLayout::FormatRange >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_QTextLayout_FormatRange_mid_to_output_pos(const QVector< QTextLayout::FormatRange >* this_ptr, int pos, QVector< QTextLayout::FormatRange >* output) {
  new(output) QVector< QTextLayout::FormatRange >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_QTextLayout_FormatRange_mid_to_output_pos_len(const QVector< QTextLayout::FormatRange >* this_ptr, int pos, int len, QVector< QTextLayout::FormatRange >* output) {
  new(output) QVector< QTextLayout::FormatRange >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_QTextLayout_FormatRange_move(QVector< QTextLayout::FormatRange >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QTextLayout::FormatRange >* qt_gui_c_QVector_QTextLayout_FormatRange_operator_add_assign_l(QVector< QTextLayout::FormatRange >* this_ptr, const QVector< QTextLayout::FormatRange >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QTextLayout::FormatRange >* qt_gui_c_QVector_QTextLayout_FormatRange_operator_add_assign_t(QVector< QTextLayout::FormatRange >* this_ptr, const QTextLayout::FormatRange* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_QTextLayout_FormatRange_operator_add_to_output(const QVector< QTextLayout::FormatRange >* this_ptr, const QVector< QTextLayout::FormatRange >* l, QVector< QTextLayout::FormatRange >* output) {
  new(output) QVector< QTextLayout::FormatRange >(this_ptr->operator+(*l));
}

QVector< QTextLayout::FormatRange >* qt_gui_c_QVector_QTextLayout_FormatRange_operator_assign(QVector< QTextLayout::FormatRange >* this_ptr, const QVector< QTextLayout::FormatRange >* v) {
  return &this_ptr->operator=(*v);
}

QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_operator_index(QVector< QTextLayout::FormatRange >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_operator_index_const(const QVector< QTextLayout::FormatRange >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QVector< QTextLayout::FormatRange >* qt_gui_c_QVector_QTextLayout_FormatRange_operator_shl_l(QVector< QTextLayout::FormatRange >* this_ptr, const QVector< QTextLayout::FormatRange >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QTextLayout::FormatRange >* qt_gui_c_QVector_QTextLayout_FormatRange_operator_shl_t(QVector< QTextLayout::FormatRange >* this_ptr, const QTextLayout::FormatRange* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_QTextLayout_FormatRange_pop_back(QVector< QTextLayout::FormatRange >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_QTextLayout_FormatRange_pop_front(QVector< QTextLayout::FormatRange >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_QTextLayout_FormatRange_prepend(QVector< QTextLayout::FormatRange >* this_ptr, const QTextLayout::FormatRange* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_QTextLayout_FormatRange_push_back(QVector< QTextLayout::FormatRange >* this_ptr, const QTextLayout::FormatRange* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_QTextLayout_FormatRange_push_front(QVector< QTextLayout::FormatRange >* this_ptr, const QTextLayout::FormatRange* t) {
  this_ptr->push_front(*t);
}

void qt_gui_c_QVector_QTextLayout_FormatRange_removeAt(QVector< QTextLayout::FormatRange >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_QTextLayout_FormatRange_removeFirst(QVector< QTextLayout::FormatRange >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_QTextLayout_FormatRange_removeLast(QVector< QTextLayout::FormatRange >* this_ptr) {
  this_ptr->removeLast();
}

void qt_gui_c_QVector_QTextLayout_FormatRange_remove_i(QVector< QTextLayout::FormatRange >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_QTextLayout_FormatRange_remove_i_n(QVector< QTextLayout::FormatRange >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_QTextLayout_FormatRange_replace(QVector< QTextLayout::FormatRange >* this_ptr, int i, const QTextLayout::FormatRange* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_QTextLayout_FormatRange_reserve(QVector< QTextLayout::FormatRange >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_QTextLayout_FormatRange_resize(QVector< QTextLayout::FormatRange >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_QTextLayout_FormatRange_size(const QVector< QTextLayout::FormatRange >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_QTextLayout_FormatRange_squeeze(QVector< QTextLayout::FormatRange >* this_ptr) {
  this_ptr->squeeze();
}

void qt_gui_c_QVector_QTextLayout_FormatRange_swap(QVector< QTextLayout::FormatRange >* this_ptr, QVector< QTextLayout::FormatRange >* other) {
  this_ptr->swap(*other);
}

QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_takeAt_as_ptr(QVector< QTextLayout::FormatRange >* this_ptr, int i) {
  return new QTextLayout::FormatRange(this_ptr->takeAt(i));
}

QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_takeFirst_as_ptr(QVector< QTextLayout::FormatRange >* this_ptr) {
  return new QTextLayout::FormatRange(this_ptr->takeFirst());
}

QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_takeLast_as_ptr(QVector< QTextLayout::FormatRange >* this_ptr) {
  return new QTextLayout::FormatRange(this_ptr->takeLast());
}

QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_value_as_ptr_i(const QVector< QTextLayout::FormatRange >* this_ptr, int i) {
  return new QTextLayout::FormatRange(this_ptr->value(i));
}

QTextLayout::FormatRange* qt_gui_c_QVector_QTextLayout_FormatRange_value_as_ptr_i_defaultValue(const QVector< QTextLayout::FormatRange >* this_ptr, int i, const QTextLayout::FormatRange* defaultValue) {
  return new QTextLayout::FormatRange(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QVector_QTextLength_append_l(QVector< QTextLength >* this_ptr, const QVector< QTextLength >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_QTextLength_append_t(QVector< QTextLength >* this_ptr, const QTextLength* t) {
  this_ptr->append(*t);
}

const QTextLength* qt_gui_c_QVector_QTextLength_at(const QVector< QTextLength >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QTextLength* qt_gui_c_QVector_QTextLength_back(QVector< QTextLength >* this_ptr) {
  return &this_ptr->back();
}

const QTextLength* qt_gui_c_QVector_QTextLength_back_const(const QVector< QTextLength >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_QTextLength_capacity(const QVector< QTextLength >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_QTextLength_clear(QVector< QTextLength >* this_ptr) {
  this_ptr->clear();
}

const QTextLength* qt_gui_c_QVector_QTextLength_constData(const QVector< QTextLength >* this_ptr) {
  return this_ptr->constData();
}

const QTextLength* qt_gui_c_QVector_QTextLength_constFirst(const QVector< QTextLength >* this_ptr) {
  return &this_ptr->constFirst();
}

const QTextLength* qt_gui_c_QVector_QTextLength_constLast(const QVector< QTextLength >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_QTextLength_constructor_no_args(QVector< QTextLength >* output) {
  new(output) QVector< QTextLength >();
}

void qt_gui_c_QVector_QTextLength_constructor_size(int size, QVector< QTextLength >* output) {
  new(output) QVector< QTextLength >(size);
}

void qt_gui_c_QVector_QTextLength_constructor_size_t(int size, const QTextLength* t, QVector< QTextLength >* output) {
  new(output) QVector< QTextLength >(size, *t);
}

void qt_gui_c_QVector_QTextLength_constructor_v(const QVector< QTextLength >* v, QVector< QTextLength >* output) {
  new(output) QVector< QTextLength >(*v);
}

bool qt_gui_c_QVector_QTextLength_contains(const QVector< QTextLength >* this_ptr, const QTextLength* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QVector_QTextLength_count_no_args(const QVector< QTextLength >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QVector_QTextLength_count_t(const QVector< QTextLength >* this_ptr, const QTextLength* t) {
  return this_ptr->count(*t);
}

QTextLength* qt_gui_c_QVector_QTextLength_data(QVector< QTextLength >* this_ptr) {
  return this_ptr->data();
}

const QTextLength* qt_gui_c_QVector_QTextLength_data_const(const QVector< QTextLength >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_QTextLength_destructor(QVector< QTextLength >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_QTextLength_empty(const QVector< QTextLength >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QVector_QTextLength_endsWith(const QVector< QTextLength >* this_ptr, const QTextLength* t) {
  return this_ptr->endsWith(*t);
}

QVector< QTextLength >* qt_gui_c_QVector_QTextLength_fill_t(QVector< QTextLength >* this_ptr, const QTextLength* t) {
  return &this_ptr->fill(*t);
}

QVector< QTextLength >* qt_gui_c_QVector_QTextLength_fill_t_size(QVector< QTextLength >* this_ptr, const QTextLength* t, int size) {
  return &this_ptr->fill(*t, size);
}

QTextLength* qt_gui_c_QVector_QTextLength_first(QVector< QTextLength >* this_ptr) {
  return &this_ptr->first();
}

const QTextLength* qt_gui_c_QVector_QTextLength_first_const(const QVector< QTextLength >* this_ptr) {
  return &this_ptr->first();
}

QTextLength* qt_gui_c_QVector_QTextLength_front(QVector< QTextLength >* this_ptr) {
  return &this_ptr->front();
}

const QTextLength* qt_gui_c_QVector_QTextLength_front_const(const QVector< QTextLength >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QVector_QTextLength_indexOf_t(const QVector< QTextLength >* this_ptr, const QTextLength* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QVector_QTextLength_indexOf_t_from(const QVector< QTextLength >* this_ptr, const QTextLength* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QVector_QTextLength_insert_i_n_t(QVector< QTextLength >* this_ptr, int i, int n, const QTextLength* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_QTextLength_insert_i_t(QVector< QTextLength >* this_ptr, int i, const QTextLength* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_QTextLength_isEmpty(const QVector< QTextLength >* this_ptr) {
  return this_ptr->isEmpty();
}

QTextLength* qt_gui_c_QVector_QTextLength_last(QVector< QTextLength >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QTextLength_lastIndexOf_t(const QVector< QTextLength >* this_ptr, const QTextLength* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QVector_QTextLength_lastIndexOf_t_from(const QVector< QTextLength >* this_ptr, const QTextLength* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QTextLength* qt_gui_c_QVector_QTextLength_last_const(const QVector< QTextLength >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_QTextLength_length(const QVector< QTextLength >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_QTextLength_mid_to_output_pos(const QVector< QTextLength >* this_ptr, int pos, QVector< QTextLength >* output) {
  new(output) QVector< QTextLength >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_QTextLength_mid_to_output_pos_len(const QVector< QTextLength >* this_ptr, int pos, int len, QVector< QTextLength >* output) {
  new(output) QVector< QTextLength >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_QTextLength_move(QVector< QTextLength >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< QTextLength >* qt_gui_c_QVector_QTextLength_operator_add_assign_l(QVector< QTextLength >* this_ptr, const QVector< QTextLength >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< QTextLength >* qt_gui_c_QVector_QTextLength_operator_add_assign_t(QVector< QTextLength >* this_ptr, const QTextLength* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_QTextLength_operator_add_to_output(const QVector< QTextLength >* this_ptr, const QVector< QTextLength >* l, QVector< QTextLength >* output) {
  new(output) QVector< QTextLength >(this_ptr->operator+(*l));
}

QVector< QTextLength >* qt_gui_c_QVector_QTextLength_operator_assign(QVector< QTextLength >* this_ptr, const QVector< QTextLength >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_gui_c_QVector_QTextLength_operator_eq(const QVector< QTextLength >* this_ptr, const QVector< QTextLength >* v) {
  return this_ptr->operator==(*v);
}

QTextLength* qt_gui_c_QVector_QTextLength_operator_index(QVector< QTextLength >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QTextLength* qt_gui_c_QVector_QTextLength_operator_index_const(const QVector< QTextLength >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QVector_QTextLength_operator_neq(const QVector< QTextLength >* this_ptr, const QVector< QTextLength >* v) {
  return this_ptr->operator!=(*v);
}

QVector< QTextLength >* qt_gui_c_QVector_QTextLength_operator_shl_l(QVector< QTextLength >* this_ptr, const QVector< QTextLength >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< QTextLength >* qt_gui_c_QVector_QTextLength_operator_shl_t(QVector< QTextLength >* this_ptr, const QTextLength* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_QTextLength_pop_back(QVector< QTextLength >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_QTextLength_pop_front(QVector< QTextLength >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_QTextLength_prepend(QVector< QTextLength >* this_ptr, const QTextLength* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_QTextLength_push_back(QVector< QTextLength >* this_ptr, const QTextLength* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_QTextLength_push_front(QVector< QTextLength >* this_ptr, const QTextLength* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QVector_QTextLength_removeAll(QVector< QTextLength >* this_ptr, const QTextLength* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QVector_QTextLength_removeAt(QVector< QTextLength >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_QTextLength_removeFirst(QVector< QTextLength >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_QTextLength_removeLast(QVector< QTextLength >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QVector_QTextLength_removeOne(QVector< QTextLength >* this_ptr, const QTextLength* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QVector_QTextLength_remove_i(QVector< QTextLength >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_QTextLength_remove_i_n(QVector< QTextLength >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_QTextLength_replace(QVector< QTextLength >* this_ptr, int i, const QTextLength* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_QTextLength_reserve(QVector< QTextLength >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_QTextLength_resize(QVector< QTextLength >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_QTextLength_size(const QVector< QTextLength >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_QTextLength_squeeze(QVector< QTextLength >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_gui_c_QVector_QTextLength_startsWith(const QVector< QTextLength >* this_ptr, const QTextLength* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QVector_QTextLength_swap(QVector< QTextLength >* this_ptr, QVector< QTextLength >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QVector_QTextLength_takeAt_to_output(QVector< QTextLength >* this_ptr, int i, QTextLength* output) {
  new(output) QTextLength(this_ptr->takeAt(i));
}

void qt_gui_c_QVector_QTextLength_takeFirst_to_output(QVector< QTextLength >* this_ptr, QTextLength* output) {
  new(output) QTextLength(this_ptr->takeFirst());
}

void qt_gui_c_QVector_QTextLength_takeLast_to_output(QVector< QTextLength >* this_ptr, QTextLength* output) {
  new(output) QTextLength(this_ptr->takeLast());
}

void qt_gui_c_QVector_QTextLength_value_to_output_i(const QVector< QTextLength >* this_ptr, int i, QTextLength* output) {
  new(output) QTextLength(this_ptr->value(i));
}

void qt_gui_c_QVector_QTextLength_value_to_output_i_defaultValue(const QVector< QTextLength >* this_ptr, int i, const QTextLength* defaultValue, QTextLength* output) {
  new(output) QTextLength(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QVector_double_append_l(QVector< double >* this_ptr, const QVector< double >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_double_append_t(QVector< double >* this_ptr, const double* t) {
  this_ptr->append(*t);
}

const double* qt_gui_c_QVector_double_at(const QVector< double >* this_ptr, int i) {
  return &this_ptr->at(i);
}

double* qt_gui_c_QVector_double_back(QVector< double >* this_ptr) {
  return &this_ptr->back();
}

const double* qt_gui_c_QVector_double_back_const(const QVector< double >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_double_capacity(const QVector< double >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_double_clear(QVector< double >* this_ptr) {
  this_ptr->clear();
}

const double* qt_gui_c_QVector_double_constData(const QVector< double >* this_ptr) {
  return this_ptr->constData();
}

const double* qt_gui_c_QVector_double_constFirst(const QVector< double >* this_ptr) {
  return &this_ptr->constFirst();
}

const double* qt_gui_c_QVector_double_constLast(const QVector< double >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_double_constructor_no_args(QVector< double >* output) {
  new(output) QVector< double >();
}

void qt_gui_c_QVector_double_constructor_size(int size, QVector< double >* output) {
  new(output) QVector< double >(size);
}

void qt_gui_c_QVector_double_constructor_size_t(int size, const double* t, QVector< double >* output) {
  new(output) QVector< double >(size, *t);
}

void qt_gui_c_QVector_double_constructor_v(const QVector< double >* v, QVector< double >* output) {
  new(output) QVector< double >(*v);
}

bool qt_gui_c_QVector_double_contains(const QVector< double >* this_ptr, const double* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QVector_double_count_no_args(const QVector< double >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QVector_double_count_t(const QVector< double >* this_ptr, const double* t) {
  return this_ptr->count(*t);
}

double* qt_gui_c_QVector_double_data(QVector< double >* this_ptr) {
  return this_ptr->data();
}

const double* qt_gui_c_QVector_double_data_const(const QVector< double >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_double_destructor(QVector< double >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_double_empty(const QVector< double >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QVector_double_endsWith(const QVector< double >* this_ptr, const double* t) {
  return this_ptr->endsWith(*t);
}

QVector< double >* qt_gui_c_QVector_double_fill_t(QVector< double >* this_ptr, const double* t) {
  return &this_ptr->fill(*t);
}

QVector< double >* qt_gui_c_QVector_double_fill_t_size(QVector< double >* this_ptr, const double* t, int size) {
  return &this_ptr->fill(*t, size);
}

double* qt_gui_c_QVector_double_first(QVector< double >* this_ptr) {
  return &this_ptr->first();
}

const double* qt_gui_c_QVector_double_first_const(const QVector< double >* this_ptr) {
  return &this_ptr->first();
}

void qt_gui_c_QVector_double_fromList_to_output(const QList< double >* list, QVector< double >* output) {
  new(output) QVector< double >(QVector< double >::fromList(*list));
}

double* qt_gui_c_QVector_double_front(QVector< double >* this_ptr) {
  return &this_ptr->front();
}

const double* qt_gui_c_QVector_double_front_const(const QVector< double >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QVector_double_indexOf_t(const QVector< double >* this_ptr, const double* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QVector_double_indexOf_t_from(const QVector< double >* this_ptr, const double* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QVector_double_insert_i_n_t(QVector< double >* this_ptr, int i, int n, const double* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_double_insert_i_t(QVector< double >* this_ptr, int i, const double* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_double_isEmpty(const QVector< double >* this_ptr) {
  return this_ptr->isEmpty();
}

double* qt_gui_c_QVector_double_last(QVector< double >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_double_lastIndexOf_t(const QVector< double >* this_ptr, const double* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QVector_double_lastIndexOf_t_from(const QVector< double >* this_ptr, const double* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const double* qt_gui_c_QVector_double_last_const(const QVector< double >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_double_length(const QVector< double >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_double_mid_to_output_pos(const QVector< double >* this_ptr, int pos, QVector< double >* output) {
  new(output) QVector< double >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_double_mid_to_output_pos_len(const QVector< double >* this_ptr, int pos, int len, QVector< double >* output) {
  new(output) QVector< double >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_double_move(QVector< double >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< double >* qt_gui_c_QVector_double_operator_add_assign_l(QVector< double >* this_ptr, const QVector< double >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< double >* qt_gui_c_QVector_double_operator_add_assign_t(QVector< double >* this_ptr, const double* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_double_operator_add_to_output(const QVector< double >* this_ptr, const QVector< double >* l, QVector< double >* output) {
  new(output) QVector< double >(this_ptr->operator+(*l));
}

QVector< double >* qt_gui_c_QVector_double_operator_assign(QVector< double >* this_ptr, const QVector< double >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_gui_c_QVector_double_operator_eq(const QVector< double >* this_ptr, const QVector< double >* v) {
  return this_ptr->operator==(*v);
}

double* qt_gui_c_QVector_double_operator_index(QVector< double >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const double* qt_gui_c_QVector_double_operator_index_const(const QVector< double >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QVector_double_operator_neq(const QVector< double >* this_ptr, const QVector< double >* v) {
  return this_ptr->operator!=(*v);
}

QVector< double >* qt_gui_c_QVector_double_operator_shl_l(QVector< double >* this_ptr, const QVector< double >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< double >* qt_gui_c_QVector_double_operator_shl_t(QVector< double >* this_ptr, const double* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_double_pop_back(QVector< double >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_double_pop_front(QVector< double >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_double_prepend(QVector< double >* this_ptr, const double* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_double_push_back(QVector< double >* this_ptr, const double* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_double_push_front(QVector< double >* this_ptr, const double* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QVector_double_removeAll(QVector< double >* this_ptr, const double* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QVector_double_removeAt(QVector< double >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_double_removeFirst(QVector< double >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_double_removeLast(QVector< double >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QVector_double_removeOne(QVector< double >* this_ptr, const double* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QVector_double_remove_i(QVector< double >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_double_remove_i_n(QVector< double >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_double_replace(QVector< double >* this_ptr, int i, const double* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_double_reserve(QVector< double >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_double_resize(QVector< double >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_double_size(const QVector< double >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_double_squeeze(QVector< double >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_gui_c_QVector_double_startsWith(const QVector< double >* this_ptr, const double* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QVector_double_swap(QVector< double >* this_ptr, QVector< double >* other) {
  this_ptr->swap(*other);
}

double qt_gui_c_QVector_double_takeAt(QVector< double >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

double qt_gui_c_QVector_double_takeFirst(QVector< double >* this_ptr) {
  return this_ptr->takeFirst();
}

double qt_gui_c_QVector_double_takeLast(QVector< double >* this_ptr) {
  return this_ptr->takeLast();
}

void qt_gui_c_QVector_double_toList_to_output(const QVector< double >* this_ptr, QList< double >* output) {
  new(output) QList< double >(this_ptr->toList());
}

double qt_gui_c_QVector_double_value_i(const QVector< double >* this_ptr, int i) {
  return this_ptr->value(i);
}

double qt_gui_c_QVector_double_value_i_defaultValue(const QVector< double >* this_ptr, int i, const double* defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_gui_c_QVector_float_append_l(QVector< float >* this_ptr, const QVector< float >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_float_append_t(QVector< float >* this_ptr, const float* t) {
  this_ptr->append(*t);
}

const float* qt_gui_c_QVector_float_at(const QVector< float >* this_ptr, int i) {
  return &this_ptr->at(i);
}

float* qt_gui_c_QVector_float_back(QVector< float >* this_ptr) {
  return &this_ptr->back();
}

const float* qt_gui_c_QVector_float_back_const(const QVector< float >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_float_capacity(const QVector< float >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_float_clear(QVector< float >* this_ptr) {
  this_ptr->clear();
}

const float* qt_gui_c_QVector_float_constData(const QVector< float >* this_ptr) {
  return this_ptr->constData();
}

const float* qt_gui_c_QVector_float_constFirst(const QVector< float >* this_ptr) {
  return &this_ptr->constFirst();
}

const float* qt_gui_c_QVector_float_constLast(const QVector< float >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_float_constructor_no_args(QVector< float >* output) {
  new(output) QVector< float >();
}

void qt_gui_c_QVector_float_constructor_size(int size, QVector< float >* output) {
  new(output) QVector< float >(size);
}

void qt_gui_c_QVector_float_constructor_size_t(int size, const float* t, QVector< float >* output) {
  new(output) QVector< float >(size, *t);
}

void qt_gui_c_QVector_float_constructor_v(const QVector< float >* v, QVector< float >* output) {
  new(output) QVector< float >(*v);
}

bool qt_gui_c_QVector_float_contains(const QVector< float >* this_ptr, const float* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QVector_float_count_no_args(const QVector< float >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QVector_float_count_t(const QVector< float >* this_ptr, const float* t) {
  return this_ptr->count(*t);
}

float* qt_gui_c_QVector_float_data(QVector< float >* this_ptr) {
  return this_ptr->data();
}

const float* qt_gui_c_QVector_float_data_const(const QVector< float >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_float_destructor(QVector< float >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_float_empty(const QVector< float >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QVector_float_endsWith(const QVector< float >* this_ptr, const float* t) {
  return this_ptr->endsWith(*t);
}

QVector< float >* qt_gui_c_QVector_float_fill_t(QVector< float >* this_ptr, const float* t) {
  return &this_ptr->fill(*t);
}

QVector< float >* qt_gui_c_QVector_float_fill_t_size(QVector< float >* this_ptr, const float* t, int size) {
  return &this_ptr->fill(*t, size);
}

float* qt_gui_c_QVector_float_first(QVector< float >* this_ptr) {
  return &this_ptr->first();
}

const float* qt_gui_c_QVector_float_first_const(const QVector< float >* this_ptr) {
  return &this_ptr->first();
}

float* qt_gui_c_QVector_float_front(QVector< float >* this_ptr) {
  return &this_ptr->front();
}

const float* qt_gui_c_QVector_float_front_const(const QVector< float >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QVector_float_indexOf_t(const QVector< float >* this_ptr, const float* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QVector_float_indexOf_t_from(const QVector< float >* this_ptr, const float* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QVector_float_insert_i_n_t(QVector< float >* this_ptr, int i, int n, const float* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_float_insert_i_t(QVector< float >* this_ptr, int i, const float* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_float_isEmpty(const QVector< float >* this_ptr) {
  return this_ptr->isEmpty();
}

float* qt_gui_c_QVector_float_last(QVector< float >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_float_lastIndexOf_t(const QVector< float >* this_ptr, const float* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QVector_float_lastIndexOf_t_from(const QVector< float >* this_ptr, const float* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const float* qt_gui_c_QVector_float_last_const(const QVector< float >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_float_length(const QVector< float >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_float_mid_to_output_pos(const QVector< float >* this_ptr, int pos, QVector< float >* output) {
  new(output) QVector< float >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_float_mid_to_output_pos_len(const QVector< float >* this_ptr, int pos, int len, QVector< float >* output) {
  new(output) QVector< float >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_float_move(QVector< float >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< float >* qt_gui_c_QVector_float_operator_add_assign_l(QVector< float >* this_ptr, const QVector< float >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< float >* qt_gui_c_QVector_float_operator_add_assign_t(QVector< float >* this_ptr, const float* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_float_operator_add_to_output(const QVector< float >* this_ptr, const QVector< float >* l, QVector< float >* output) {
  new(output) QVector< float >(this_ptr->operator+(*l));
}

QVector< float >* qt_gui_c_QVector_float_operator_assign(QVector< float >* this_ptr, const QVector< float >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_gui_c_QVector_float_operator_eq(const QVector< float >* this_ptr, const QVector< float >* v) {
  return this_ptr->operator==(*v);
}

float* qt_gui_c_QVector_float_operator_index(QVector< float >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const float* qt_gui_c_QVector_float_operator_index_const(const QVector< float >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QVector_float_operator_neq(const QVector< float >* this_ptr, const QVector< float >* v) {
  return this_ptr->operator!=(*v);
}

QVector< float >* qt_gui_c_QVector_float_operator_shl_l(QVector< float >* this_ptr, const QVector< float >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< float >* qt_gui_c_QVector_float_operator_shl_t(QVector< float >* this_ptr, const float* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_float_pop_back(QVector< float >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_float_pop_front(QVector< float >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_float_prepend(QVector< float >* this_ptr, const float* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_float_push_back(QVector< float >* this_ptr, const float* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_float_push_front(QVector< float >* this_ptr, const float* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QVector_float_removeAll(QVector< float >* this_ptr, const float* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QVector_float_removeAt(QVector< float >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_float_removeFirst(QVector< float >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_float_removeLast(QVector< float >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QVector_float_removeOne(QVector< float >* this_ptr, const float* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QVector_float_remove_i(QVector< float >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_float_remove_i_n(QVector< float >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_float_replace(QVector< float >* this_ptr, int i, const float* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_float_reserve(QVector< float >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_float_resize(QVector< float >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_float_size(const QVector< float >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_float_squeeze(QVector< float >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_gui_c_QVector_float_startsWith(const QVector< float >* this_ptr, const float* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QVector_float_swap(QVector< float >* this_ptr, QVector< float >* other) {
  this_ptr->swap(*other);
}

float qt_gui_c_QVector_float_takeAt(QVector< float >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

float qt_gui_c_QVector_float_takeFirst(QVector< float >* this_ptr) {
  return this_ptr->takeFirst();
}

float qt_gui_c_QVector_float_takeLast(QVector< float >* this_ptr) {
  return this_ptr->takeLast();
}

float qt_gui_c_QVector_float_value_i(const QVector< float >* this_ptr, int i) {
  return this_ptr->value(i);
}

float qt_gui_c_QVector_float_value_i_defaultValue(const QVector< float >* this_ptr, int i, const float* defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_gui_c_QVector_quint32_append_l(QVector< quint32 >* this_ptr, const QVector< quint32 >* l) {
  this_ptr->append(*l);
}

void qt_gui_c_QVector_quint32_append_t(QVector< quint32 >* this_ptr, const quint32* t) {
  this_ptr->append(*t);
}

const quint32* qt_gui_c_QVector_quint32_at(const QVector< quint32 >* this_ptr, int i) {
  return &this_ptr->at(i);
}

quint32* qt_gui_c_QVector_quint32_back(QVector< quint32 >* this_ptr) {
  return &this_ptr->back();
}

const quint32* qt_gui_c_QVector_quint32_back_const(const QVector< quint32 >* this_ptr) {
  return &this_ptr->back();
}

int qt_gui_c_QVector_quint32_capacity(const QVector< quint32 >* this_ptr) {
  return this_ptr->capacity();
}

void qt_gui_c_QVector_quint32_clear(QVector< quint32 >* this_ptr) {
  this_ptr->clear();
}

const quint32* qt_gui_c_QVector_quint32_constData(const QVector< quint32 >* this_ptr) {
  return this_ptr->constData();
}

const quint32* qt_gui_c_QVector_quint32_constFirst(const QVector< quint32 >* this_ptr) {
  return &this_ptr->constFirst();
}

const quint32* qt_gui_c_QVector_quint32_constLast(const QVector< quint32 >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QVector_quint32_constructor_no_args(QVector< quint32 >* output) {
  new(output) QVector< quint32 >();
}

void qt_gui_c_QVector_quint32_constructor_size(int size, QVector< quint32 >* output) {
  new(output) QVector< quint32 >(size);
}

void qt_gui_c_QVector_quint32_constructor_size_t(int size, const quint32* t, QVector< quint32 >* output) {
  new(output) QVector< quint32 >(size, *t);
}

void qt_gui_c_QVector_quint32_constructor_v(const QVector< quint32 >* v, QVector< quint32 >* output) {
  new(output) QVector< quint32 >(*v);
}

bool qt_gui_c_QVector_quint32_contains(const QVector< quint32 >* this_ptr, const quint32* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QVector_quint32_count_no_args(const QVector< quint32 >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QVector_quint32_count_t(const QVector< quint32 >* this_ptr, const quint32* t) {
  return this_ptr->count(*t);
}

quint32* qt_gui_c_QVector_quint32_data(QVector< quint32 >* this_ptr) {
  return this_ptr->data();
}

const quint32* qt_gui_c_QVector_quint32_data_const(const QVector< quint32 >* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QVector_quint32_destructor(QVector< quint32 >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QVector_quint32_empty(const QVector< quint32 >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QVector_quint32_endsWith(const QVector< quint32 >* this_ptr, const quint32* t) {
  return this_ptr->endsWith(*t);
}

QVector< quint32 >* qt_gui_c_QVector_quint32_fill_t(QVector< quint32 >* this_ptr, const quint32* t) {
  return &this_ptr->fill(*t);
}

QVector< quint32 >* qt_gui_c_QVector_quint32_fill_t_size(QVector< quint32 >* this_ptr, const quint32* t, int size) {
  return &this_ptr->fill(*t, size);
}

quint32* qt_gui_c_QVector_quint32_first(QVector< quint32 >* this_ptr) {
  return &this_ptr->first();
}

const quint32* qt_gui_c_QVector_quint32_first_const(const QVector< quint32 >* this_ptr) {
  return &this_ptr->first();
}

quint32* qt_gui_c_QVector_quint32_front(QVector< quint32 >* this_ptr) {
  return &this_ptr->front();
}

const quint32* qt_gui_c_QVector_quint32_front_const(const QVector< quint32 >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QVector_quint32_indexOf_t(const QVector< quint32 >* this_ptr, const quint32* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QVector_quint32_indexOf_t_from(const QVector< quint32 >* this_ptr, const quint32* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QVector_quint32_insert_i_n_t(QVector< quint32 >* this_ptr, int i, int n, const quint32* t) {
  this_ptr->insert(i, n, *t);
}

void qt_gui_c_QVector_quint32_insert_i_t(QVector< quint32 >* this_ptr, int i, const quint32* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QVector_quint32_isEmpty(const QVector< quint32 >* this_ptr) {
  return this_ptr->isEmpty();
}

quint32* qt_gui_c_QVector_quint32_last(QVector< quint32 >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_quint32_lastIndexOf_t(const QVector< quint32 >* this_ptr, const quint32* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QVector_quint32_lastIndexOf_t_from(const QVector< quint32 >* this_ptr, const quint32* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const quint32* qt_gui_c_QVector_quint32_last_const(const QVector< quint32 >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QVector_quint32_length(const QVector< quint32 >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QVector_quint32_mid_to_output_pos(const QVector< quint32 >* this_ptr, int pos, QVector< quint32 >* output) {
  new(output) QVector< quint32 >(this_ptr->mid(pos));
}

void qt_gui_c_QVector_quint32_mid_to_output_pos_len(const QVector< quint32 >* this_ptr, int pos, int len, QVector< quint32 >* output) {
  new(output) QVector< quint32 >(this_ptr->mid(pos, len));
}

void qt_gui_c_QVector_quint32_move(QVector< quint32 >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QVector< quint32 >* qt_gui_c_QVector_quint32_operator_add_assign_l(QVector< quint32 >* this_ptr, const QVector< quint32 >* l) {
  return &this_ptr->operator+=(*l);
}

QVector< quint32 >* qt_gui_c_QVector_quint32_operator_add_assign_t(QVector< quint32 >* this_ptr, const quint32* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QVector_quint32_operator_add_to_output(const QVector< quint32 >* this_ptr, const QVector< quint32 >* l, QVector< quint32 >* output) {
  new(output) QVector< quint32 >(this_ptr->operator+(*l));
}

QVector< quint32 >* qt_gui_c_QVector_quint32_operator_assign(QVector< quint32 >* this_ptr, const QVector< quint32 >* v) {
  return &this_ptr->operator=(*v);
}

bool qt_gui_c_QVector_quint32_operator_eq(const QVector< quint32 >* this_ptr, const QVector< quint32 >* v) {
  return this_ptr->operator==(*v);
}

quint32* qt_gui_c_QVector_quint32_operator_index(QVector< quint32 >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const quint32* qt_gui_c_QVector_quint32_operator_index_const(const QVector< quint32 >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QVector_quint32_operator_neq(const QVector< quint32 >* this_ptr, const QVector< quint32 >* v) {
  return this_ptr->operator!=(*v);
}

QVector< quint32 >* qt_gui_c_QVector_quint32_operator_shl_l(QVector< quint32 >* this_ptr, const QVector< quint32 >* l) {
  return &this_ptr->operator<<(*l);
}

QVector< quint32 >* qt_gui_c_QVector_quint32_operator_shl_t(QVector< quint32 >* this_ptr, const quint32* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QVector_quint32_pop_back(QVector< quint32 >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QVector_quint32_pop_front(QVector< quint32 >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QVector_quint32_prepend(QVector< quint32 >* this_ptr, const quint32* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QVector_quint32_push_back(QVector< quint32 >* this_ptr, const quint32* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QVector_quint32_push_front(QVector< quint32 >* this_ptr, const quint32* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QVector_quint32_removeAll(QVector< quint32 >* this_ptr, const quint32* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QVector_quint32_removeAt(QVector< quint32 >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QVector_quint32_removeFirst(QVector< quint32 >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QVector_quint32_removeLast(QVector< quint32 >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QVector_quint32_removeOne(QVector< quint32 >* this_ptr, const quint32* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QVector_quint32_remove_i(QVector< quint32 >* this_ptr, int i) {
  this_ptr->remove(i);
}

void qt_gui_c_QVector_quint32_remove_i_n(QVector< quint32 >* this_ptr, int i, int n) {
  this_ptr->remove(i, n);
}

void qt_gui_c_QVector_quint32_replace(QVector< quint32 >* this_ptr, int i, const quint32* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QVector_quint32_reserve(QVector< quint32 >* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_gui_c_QVector_quint32_resize(QVector< quint32 >* this_ptr, int size) {
  this_ptr->resize(size);
}

int qt_gui_c_QVector_quint32_size(const QVector< quint32 >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QVector_quint32_squeeze(QVector< quint32 >* this_ptr) {
  this_ptr->squeeze();
}

bool qt_gui_c_QVector_quint32_startsWith(const QVector< quint32 >* this_ptr, const quint32* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QVector_quint32_swap(QVector< quint32 >* this_ptr, QVector< quint32 >* other) {
  this_ptr->swap(*other);
}

quint32 qt_gui_c_QVector_quint32_takeAt(QVector< quint32 >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

quint32 qt_gui_c_QVector_quint32_takeFirst(QVector< quint32 >* this_ptr) {
  return this_ptr->takeFirst();
}

quint32 qt_gui_c_QVector_quint32_takeLast(QVector< quint32 >* this_ptr) {
  return this_ptr->takeLast();
}

quint32 qt_gui_c_QVector_quint32_value_i(const QVector< quint32 >* this_ptr, int i) {
  return this_ptr->value(i);
}

quint32 qt_gui_c_QVector_quint32_value_i_defaultValue(const QVector< quint32 >* this_ptr, int i, const quint32* defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

