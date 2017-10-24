#include "qt_gui_c_QList.h"

void qt_gui_c_QList_QAccessibleInterface_ptr_append_QAccessibleInterface(QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QAccessibleInterface_ptr_append_QList_QAccessibleInterface_ptr(QList< QAccessibleInterface* >* this_ptr, const QList< QAccessibleInterface* >* t) {
  this_ptr->append(*t);
}

QAccessibleInterface* const * qt_gui_c_QList_QAccessibleInterface_ptr_at(const QList< QAccessibleInterface* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QAccessibleInterface** qt_gui_c_QList_QAccessibleInterface_ptr_back(QList< QAccessibleInterface* >* this_ptr) {
  return &this_ptr->back();
}

QAccessibleInterface* const * qt_gui_c_QList_QAccessibleInterface_ptr_back_const(const QList< QAccessibleInterface* >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QAccessibleInterface_ptr_clear(QList< QAccessibleInterface* >* this_ptr) {
  this_ptr->clear();
}

QAccessibleInterface* const * qt_gui_c_QList_QAccessibleInterface_ptr_constFirst(const QList< QAccessibleInterface* >* this_ptr) {
  return &this_ptr->constFirst();
}

QAccessibleInterface* const * qt_gui_c_QList_QAccessibleInterface_ptr_constLast(const QList< QAccessibleInterface* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QAccessibleInterface_ptr_constructor_l(const QList< QAccessibleInterface* >* l, QList< QAccessibleInterface* >* output) {
  new(output) QList< QAccessibleInterface* >(*l);
}

void qt_gui_c_QList_QAccessibleInterface_ptr_constructor_no_args(QList< QAccessibleInterface* >* output) {
  new(output) QList< QAccessibleInterface* >();
}

bool qt_gui_c_QList_QAccessibleInterface_ptr_contains(const QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QAccessibleInterface_ptr_count_no_args(const QList< QAccessibleInterface* >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QAccessibleInterface_ptr_count_t(const QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QAccessibleInterface_ptr_destructor(QList< QAccessibleInterface* >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QAccessibleInterface_ptr_empty(const QList< QAccessibleInterface* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QAccessibleInterface_ptr_endsWith(const QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t) {
  return this_ptr->endsWith(*t);
}

QAccessibleInterface** qt_gui_c_QList_QAccessibleInterface_ptr_first(QList< QAccessibleInterface* >* this_ptr) {
  return &this_ptr->first();
}

QAccessibleInterface* const * qt_gui_c_QList_QAccessibleInterface_ptr_first_const(const QList< QAccessibleInterface* >* this_ptr) {
  return &this_ptr->first();
}

QAccessibleInterface** qt_gui_c_QList_QAccessibleInterface_ptr_front(QList< QAccessibleInterface* >* this_ptr) {
  return &this_ptr->front();
}

QAccessibleInterface* const * qt_gui_c_QList_QAccessibleInterface_ptr_front_const(const QList< QAccessibleInterface* >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QAccessibleInterface_ptr_indexOf_t(const QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QAccessibleInterface_ptr_indexOf_t_from(const QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QAccessibleInterface_ptr_insert(QList< QAccessibleInterface* >* this_ptr, int i, QAccessibleInterface* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QAccessibleInterface_ptr_isEmpty(const QList< QAccessibleInterface* >* this_ptr) {
  return this_ptr->isEmpty();
}

QAccessibleInterface** qt_gui_c_QList_QAccessibleInterface_ptr_last(QList< QAccessibleInterface* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QAccessibleInterface_ptr_lastIndexOf_t(const QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QAccessibleInterface_ptr_lastIndexOf_t_from(const QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QAccessibleInterface* const * qt_gui_c_QList_QAccessibleInterface_ptr_last_const(const QList< QAccessibleInterface* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QAccessibleInterface_ptr_length(const QList< QAccessibleInterface* >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QAccessibleInterface_ptr_mid_to_output_pos(const QList< QAccessibleInterface* >* this_ptr, int pos, QList< QAccessibleInterface* >* output) {
  new(output) QList< QAccessibleInterface* >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QAccessibleInterface_ptr_mid_to_output_pos_length(const QList< QAccessibleInterface* >* this_ptr, int pos, int length, QList< QAccessibleInterface* >* output) {
  new(output) QList< QAccessibleInterface* >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QAccessibleInterface_ptr_move(QList< QAccessibleInterface* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QAccessibleInterface* >* qt_gui_c_QList_QAccessibleInterface_ptr_operator_add_assign_l(QList< QAccessibleInterface* >* this_ptr, const QList< QAccessibleInterface* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QAccessibleInterface* >* qt_gui_c_QList_QAccessibleInterface_ptr_operator_add_assign_t(QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QAccessibleInterface_ptr_operator_add_to_output(const QList< QAccessibleInterface* >* this_ptr, const QList< QAccessibleInterface* >* l, QList< QAccessibleInterface* >* output) {
  new(output) QList< QAccessibleInterface* >(this_ptr->operator+(*l));
}

QList< QAccessibleInterface* >* qt_gui_c_QList_QAccessibleInterface_ptr_operator_assign(QList< QAccessibleInterface* >* this_ptr, const QList< QAccessibleInterface* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QAccessibleInterface_ptr_operator_eq(const QList< QAccessibleInterface* >* this_ptr, const QList< QAccessibleInterface* >* l) {
  return this_ptr->operator==(*l);
}

QAccessibleInterface** qt_gui_c_QList_QAccessibleInterface_ptr_operator_index(QList< QAccessibleInterface* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QAccessibleInterface* const * qt_gui_c_QList_QAccessibleInterface_ptr_operator_index_const(const QList< QAccessibleInterface* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QAccessibleInterface_ptr_operator_neq(const QList< QAccessibleInterface* >* this_ptr, const QList< QAccessibleInterface* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QAccessibleInterface* >* qt_gui_c_QList_QAccessibleInterface_ptr_operator_shl_l(QList< QAccessibleInterface* >* this_ptr, const QList< QAccessibleInterface* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QAccessibleInterface* >* qt_gui_c_QList_QAccessibleInterface_ptr_operator_shl_t(QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QAccessibleInterface_ptr_pop_back(QList< QAccessibleInterface* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QAccessibleInterface_ptr_pop_front(QList< QAccessibleInterface* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QAccessibleInterface_ptr_prepend(QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QAccessibleInterface_ptr_push_back(QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QAccessibleInterface_ptr_push_front(QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QAccessibleInterface_ptr_removeAll(QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QAccessibleInterface_ptr_removeAt(QList< QAccessibleInterface* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QAccessibleInterface_ptr_removeFirst(QList< QAccessibleInterface* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QAccessibleInterface_ptr_removeLast(QList< QAccessibleInterface* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QAccessibleInterface_ptr_removeOne(QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QAccessibleInterface_ptr_replace(QList< QAccessibleInterface* >* this_ptr, int i, QAccessibleInterface* const * t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QAccessibleInterface_ptr_reserve(QList< QAccessibleInterface* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QAccessibleInterface_ptr_size(const QList< QAccessibleInterface* >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QAccessibleInterface_ptr_startsWith(const QList< QAccessibleInterface* >* this_ptr, QAccessibleInterface* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QAccessibleInterface_ptr_swap_i_j(QList< QAccessibleInterface* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QAccessibleInterface_ptr_swap_other(QList< QAccessibleInterface* >* this_ptr, QList< QAccessibleInterface* >* other) {
  this_ptr->swap(*other);
}

QAccessibleInterface* qt_gui_c_QList_QAccessibleInterface_ptr_takeAt(QList< QAccessibleInterface* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QAccessibleInterface* qt_gui_c_QList_QAccessibleInterface_ptr_takeFirst(QList< QAccessibleInterface* >* this_ptr) {
  return this_ptr->takeFirst();
}

QAccessibleInterface* qt_gui_c_QList_QAccessibleInterface_ptr_takeLast(QList< QAccessibleInterface* >* this_ptr) {
  return this_ptr->takeLast();
}

QAccessibleInterface* qt_gui_c_QList_QAccessibleInterface_ptr_value_i(const QList< QAccessibleInterface* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QAccessibleInterface* qt_gui_c_QList_QAccessibleInterface_ptr_value_i_defaultValue(const QList< QAccessibleInterface* >* this_ptr, int i, QAccessibleInterface* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_append_QFontDatabase_WritingSystem(QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_append_QList_QFontDatabase_WritingSystem(QList< QFontDatabase::WritingSystem >* this_ptr, const QList< QFontDatabase::WritingSystem >* t) {
  this_ptr->append(*t);
}

const QFontDatabase::WritingSystem* qt_gui_c_QList_QFontDatabase_WritingSystem_at(const QList< QFontDatabase::WritingSystem >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QFontDatabase::WritingSystem* qt_gui_c_QList_QFontDatabase_WritingSystem_back(QList< QFontDatabase::WritingSystem >* this_ptr) {
  return &this_ptr->back();
}

const QFontDatabase::WritingSystem* qt_gui_c_QList_QFontDatabase_WritingSystem_back_const(const QList< QFontDatabase::WritingSystem >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_clear(QList< QFontDatabase::WritingSystem >* this_ptr) {
  this_ptr->clear();
}

const QFontDatabase::WritingSystem* qt_gui_c_QList_QFontDatabase_WritingSystem_constFirst(const QList< QFontDatabase::WritingSystem >* this_ptr) {
  return &this_ptr->constFirst();
}

const QFontDatabase::WritingSystem* qt_gui_c_QList_QFontDatabase_WritingSystem_constLast(const QList< QFontDatabase::WritingSystem >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_constructor_l(const QList< QFontDatabase::WritingSystem >* l, QList< QFontDatabase::WritingSystem >* output) {
  new(output) QList< QFontDatabase::WritingSystem >(*l);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_constructor_no_args(QList< QFontDatabase::WritingSystem >* output) {
  new(output) QList< QFontDatabase::WritingSystem >();
}

bool qt_gui_c_QList_QFontDatabase_WritingSystem_contains(const QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QFontDatabase_WritingSystem_count_no_args(const QList< QFontDatabase::WritingSystem >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QFontDatabase_WritingSystem_count_t(const QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_destructor(QList< QFontDatabase::WritingSystem >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QFontDatabase_WritingSystem_empty(const QList< QFontDatabase::WritingSystem >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QFontDatabase_WritingSystem_endsWith(const QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t) {
  return this_ptr->endsWith(*t);
}

QFontDatabase::WritingSystem* qt_gui_c_QList_QFontDatabase_WritingSystem_first(QList< QFontDatabase::WritingSystem >* this_ptr) {
  return &this_ptr->first();
}

const QFontDatabase::WritingSystem* qt_gui_c_QList_QFontDatabase_WritingSystem_first_const(const QList< QFontDatabase::WritingSystem >* this_ptr) {
  return &this_ptr->first();
}

QFontDatabase::WritingSystem* qt_gui_c_QList_QFontDatabase_WritingSystem_front(QList< QFontDatabase::WritingSystem >* this_ptr) {
  return &this_ptr->front();
}

const QFontDatabase::WritingSystem* qt_gui_c_QList_QFontDatabase_WritingSystem_front_const(const QList< QFontDatabase::WritingSystem >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QFontDatabase_WritingSystem_indexOf_t(const QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QFontDatabase_WritingSystem_indexOf_t_from(const QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_insert(QList< QFontDatabase::WritingSystem >* this_ptr, int i, const QFontDatabase::WritingSystem* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QFontDatabase_WritingSystem_isEmpty(const QList< QFontDatabase::WritingSystem >* this_ptr) {
  return this_ptr->isEmpty();
}

QFontDatabase::WritingSystem* qt_gui_c_QList_QFontDatabase_WritingSystem_last(QList< QFontDatabase::WritingSystem >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QFontDatabase_WritingSystem_lastIndexOf_t(const QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QFontDatabase_WritingSystem_lastIndexOf_t_from(const QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QFontDatabase::WritingSystem* qt_gui_c_QList_QFontDatabase_WritingSystem_last_const(const QList< QFontDatabase::WritingSystem >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QFontDatabase_WritingSystem_length(const QList< QFontDatabase::WritingSystem >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_mid_to_output_pos(const QList< QFontDatabase::WritingSystem >* this_ptr, int pos, QList< QFontDatabase::WritingSystem >* output) {
  new(output) QList< QFontDatabase::WritingSystem >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_mid_to_output_pos_length(const QList< QFontDatabase::WritingSystem >* this_ptr, int pos, int length, QList< QFontDatabase::WritingSystem >* output) {
  new(output) QList< QFontDatabase::WritingSystem >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_move(QList< QFontDatabase::WritingSystem >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QFontDatabase::WritingSystem >* qt_gui_c_QList_QFontDatabase_WritingSystem_operator_add_assign_l(QList< QFontDatabase::WritingSystem >* this_ptr, const QList< QFontDatabase::WritingSystem >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QFontDatabase::WritingSystem >* qt_gui_c_QList_QFontDatabase_WritingSystem_operator_add_assign_t(QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_operator_add_to_output(const QList< QFontDatabase::WritingSystem >* this_ptr, const QList< QFontDatabase::WritingSystem >* l, QList< QFontDatabase::WritingSystem >* output) {
  new(output) QList< QFontDatabase::WritingSystem >(this_ptr->operator+(*l));
}

QList< QFontDatabase::WritingSystem >* qt_gui_c_QList_QFontDatabase_WritingSystem_operator_assign(QList< QFontDatabase::WritingSystem >* this_ptr, const QList< QFontDatabase::WritingSystem >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QFontDatabase_WritingSystem_operator_eq(const QList< QFontDatabase::WritingSystem >* this_ptr, const QList< QFontDatabase::WritingSystem >* l) {
  return this_ptr->operator==(*l);
}

QFontDatabase::WritingSystem* qt_gui_c_QList_QFontDatabase_WritingSystem_operator_index(QList< QFontDatabase::WritingSystem >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QFontDatabase::WritingSystem* qt_gui_c_QList_QFontDatabase_WritingSystem_operator_index_const(const QList< QFontDatabase::WritingSystem >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QFontDatabase_WritingSystem_operator_neq(const QList< QFontDatabase::WritingSystem >* this_ptr, const QList< QFontDatabase::WritingSystem >* l) {
  return this_ptr->operator!=(*l);
}

QList< QFontDatabase::WritingSystem >* qt_gui_c_QList_QFontDatabase_WritingSystem_operator_shl_l(QList< QFontDatabase::WritingSystem >* this_ptr, const QList< QFontDatabase::WritingSystem >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QFontDatabase::WritingSystem >* qt_gui_c_QList_QFontDatabase_WritingSystem_operator_shl_t(QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_pop_back(QList< QFontDatabase::WritingSystem >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_pop_front(QList< QFontDatabase::WritingSystem >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_prepend(QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_push_back(QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_push_front(QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QFontDatabase_WritingSystem_removeAll(QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_removeAt(QList< QFontDatabase::WritingSystem >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_removeFirst(QList< QFontDatabase::WritingSystem >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_removeLast(QList< QFontDatabase::WritingSystem >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QFontDatabase_WritingSystem_removeOne(QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_replace(QList< QFontDatabase::WritingSystem >* this_ptr, int i, const QFontDatabase::WritingSystem* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_reserve(QList< QFontDatabase::WritingSystem >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QFontDatabase_WritingSystem_size(const QList< QFontDatabase::WritingSystem >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QFontDatabase_WritingSystem_startsWith(const QList< QFontDatabase::WritingSystem >* this_ptr, const QFontDatabase::WritingSystem* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_swap_i_j(QList< QFontDatabase::WritingSystem >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QFontDatabase_WritingSystem_swap_other(QList< QFontDatabase::WritingSystem >* this_ptr, QList< QFontDatabase::WritingSystem >* other) {
  this_ptr->swap(*other);
}

QFontDatabase::WritingSystem qt_gui_c_QList_QFontDatabase_WritingSystem_takeAt(QList< QFontDatabase::WritingSystem >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QFontDatabase::WritingSystem qt_gui_c_QList_QFontDatabase_WritingSystem_takeFirst(QList< QFontDatabase::WritingSystem >* this_ptr) {
  return this_ptr->takeFirst();
}

QFontDatabase::WritingSystem qt_gui_c_QList_QFontDatabase_WritingSystem_takeLast(QList< QFontDatabase::WritingSystem >* this_ptr) {
  return this_ptr->takeLast();
}

QFontDatabase::WritingSystem qt_gui_c_QList_QFontDatabase_WritingSystem_value_i(const QList< QFontDatabase::WritingSystem >* this_ptr, int i) {
  return this_ptr->value(i);
}

QFontDatabase::WritingSystem qt_gui_c_QList_QFontDatabase_WritingSystem_value_i_defaultValue(const QList< QFontDatabase::WritingSystem >* this_ptr, int i, const QFontDatabase::WritingSystem* defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_gui_c_QList_QGlyphRun_append_QGlyphRun(QList< QGlyphRun >* this_ptr, const QGlyphRun* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QGlyphRun_append_QList_QGlyphRun(QList< QGlyphRun >* this_ptr, const QList< QGlyphRun >* t) {
  this_ptr->append(*t);
}

const QGlyphRun* qt_gui_c_QList_QGlyphRun_at(const QList< QGlyphRun >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QGlyphRun* qt_gui_c_QList_QGlyphRun_back(QList< QGlyphRun >* this_ptr) {
  return &this_ptr->back();
}

const QGlyphRun* qt_gui_c_QList_QGlyphRun_back_const(const QList< QGlyphRun >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QGlyphRun_clear(QList< QGlyphRun >* this_ptr) {
  this_ptr->clear();
}

const QGlyphRun* qt_gui_c_QList_QGlyphRun_constFirst(const QList< QGlyphRun >* this_ptr) {
  return &this_ptr->constFirst();
}

const QGlyphRun* qt_gui_c_QList_QGlyphRun_constLast(const QList< QGlyphRun >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QGlyphRun_constructor_l(const QList< QGlyphRun >* l, QList< QGlyphRun >* output) {
  new(output) QList< QGlyphRun >(*l);
}

void qt_gui_c_QList_QGlyphRun_constructor_no_args(QList< QGlyphRun >* output) {
  new(output) QList< QGlyphRun >();
}

bool qt_gui_c_QList_QGlyphRun_contains(const QList< QGlyphRun >* this_ptr, const QGlyphRun* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QGlyphRun_count_no_args(const QList< QGlyphRun >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QGlyphRun_count_t(const QList< QGlyphRun >* this_ptr, const QGlyphRun* t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QGlyphRun_destructor(QList< QGlyphRun >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QGlyphRun_empty(const QList< QGlyphRun >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QGlyphRun_endsWith(const QList< QGlyphRun >* this_ptr, const QGlyphRun* t) {
  return this_ptr->endsWith(*t);
}

QGlyphRun* qt_gui_c_QList_QGlyphRun_first(QList< QGlyphRun >* this_ptr) {
  return &this_ptr->first();
}

const QGlyphRun* qt_gui_c_QList_QGlyphRun_first_const(const QList< QGlyphRun >* this_ptr) {
  return &this_ptr->first();
}

QGlyphRun* qt_gui_c_QList_QGlyphRun_front(QList< QGlyphRun >* this_ptr) {
  return &this_ptr->front();
}

const QGlyphRun* qt_gui_c_QList_QGlyphRun_front_const(const QList< QGlyphRun >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QGlyphRun_indexOf_t(const QList< QGlyphRun >* this_ptr, const QGlyphRun* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QGlyphRun_indexOf_t_from(const QList< QGlyphRun >* this_ptr, const QGlyphRun* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QGlyphRun_insert(QList< QGlyphRun >* this_ptr, int i, const QGlyphRun* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QGlyphRun_isEmpty(const QList< QGlyphRun >* this_ptr) {
  return this_ptr->isEmpty();
}

QGlyphRun* qt_gui_c_QList_QGlyphRun_last(QList< QGlyphRun >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QGlyphRun_lastIndexOf_t(const QList< QGlyphRun >* this_ptr, const QGlyphRun* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QGlyphRun_lastIndexOf_t_from(const QList< QGlyphRun >* this_ptr, const QGlyphRun* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QGlyphRun* qt_gui_c_QList_QGlyphRun_last_const(const QList< QGlyphRun >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QGlyphRun_length(const QList< QGlyphRun >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QGlyphRun_mid_to_output_pos(const QList< QGlyphRun >* this_ptr, int pos, QList< QGlyphRun >* output) {
  new(output) QList< QGlyphRun >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QGlyphRun_mid_to_output_pos_length(const QList< QGlyphRun >* this_ptr, int pos, int length, QList< QGlyphRun >* output) {
  new(output) QList< QGlyphRun >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QGlyphRun_move(QList< QGlyphRun >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QGlyphRun >* qt_gui_c_QList_QGlyphRun_operator_add_assign_l(QList< QGlyphRun >* this_ptr, const QList< QGlyphRun >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QGlyphRun >* qt_gui_c_QList_QGlyphRun_operator_add_assign_t(QList< QGlyphRun >* this_ptr, const QGlyphRun* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QGlyphRun_operator_add_to_output(const QList< QGlyphRun >* this_ptr, const QList< QGlyphRun >* l, QList< QGlyphRun >* output) {
  new(output) QList< QGlyphRun >(this_ptr->operator+(*l));
}

QList< QGlyphRun >* qt_gui_c_QList_QGlyphRun_operator_assign(QList< QGlyphRun >* this_ptr, const QList< QGlyphRun >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QGlyphRun_operator_eq(const QList< QGlyphRun >* this_ptr, const QList< QGlyphRun >* l) {
  return this_ptr->operator==(*l);
}

QGlyphRun* qt_gui_c_QList_QGlyphRun_operator_index(QList< QGlyphRun >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QGlyphRun* qt_gui_c_QList_QGlyphRun_operator_index_const(const QList< QGlyphRun >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QGlyphRun_operator_neq(const QList< QGlyphRun >* this_ptr, const QList< QGlyphRun >* l) {
  return this_ptr->operator!=(*l);
}

QList< QGlyphRun >* qt_gui_c_QList_QGlyphRun_operator_shl_l(QList< QGlyphRun >* this_ptr, const QList< QGlyphRun >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QGlyphRun >* qt_gui_c_QList_QGlyphRun_operator_shl_t(QList< QGlyphRun >* this_ptr, const QGlyphRun* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QGlyphRun_pop_back(QList< QGlyphRun >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QGlyphRun_pop_front(QList< QGlyphRun >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QGlyphRun_prepend(QList< QGlyphRun >* this_ptr, const QGlyphRun* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QGlyphRun_push_back(QList< QGlyphRun >* this_ptr, const QGlyphRun* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QGlyphRun_push_front(QList< QGlyphRun >* this_ptr, const QGlyphRun* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QGlyphRun_removeAll(QList< QGlyphRun >* this_ptr, const QGlyphRun* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QGlyphRun_removeAt(QList< QGlyphRun >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QGlyphRun_removeFirst(QList< QGlyphRun >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QGlyphRun_removeLast(QList< QGlyphRun >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QGlyphRun_removeOne(QList< QGlyphRun >* this_ptr, const QGlyphRun* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QGlyphRun_replace(QList< QGlyphRun >* this_ptr, int i, const QGlyphRun* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QGlyphRun_reserve(QList< QGlyphRun >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QGlyphRun_size(const QList< QGlyphRun >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QGlyphRun_startsWith(const QList< QGlyphRun >* this_ptr, const QGlyphRun* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QGlyphRun_swap_i_j(QList< QGlyphRun >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QGlyphRun_swap_other(QList< QGlyphRun >* this_ptr, QList< QGlyphRun >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QList_QGlyphRun_takeAt_to_output(QList< QGlyphRun >* this_ptr, int i, QGlyphRun* output) {
  new(output) QGlyphRun(this_ptr->takeAt(i));
}

void qt_gui_c_QList_QGlyphRun_takeFirst_to_output(QList< QGlyphRun >* this_ptr, QGlyphRun* output) {
  new(output) QGlyphRun(this_ptr->takeFirst());
}

void qt_gui_c_QList_QGlyphRun_takeLast_to_output(QList< QGlyphRun >* this_ptr, QGlyphRun* output) {
  new(output) QGlyphRun(this_ptr->takeLast());
}

void qt_gui_c_QList_QGlyphRun_value_to_output_i(const QList< QGlyphRun >* this_ptr, int i, QGlyphRun* output) {
  new(output) QGlyphRun(this_ptr->value(i));
}

void qt_gui_c_QList_QGlyphRun_value_to_output_i_defaultValue(const QList< QGlyphRun >* this_ptr, int i, const QGlyphRun* defaultValue, QGlyphRun* output) {
  new(output) QGlyphRun(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_append_QInputMethodEvent_Attribute(QList< QInputMethodEvent::Attribute >* this_ptr, const QInputMethodEvent::Attribute* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_append_QList_QInputMethodEvent_Attribute(QList< QInputMethodEvent::Attribute >* this_ptr, const QList< QInputMethodEvent::Attribute >* t) {
  this_ptr->append(*t);
}

const QInputMethodEvent::Attribute* qt_gui_c_QList_QInputMethodEvent_Attribute_at(const QList< QInputMethodEvent::Attribute >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QInputMethodEvent::Attribute* qt_gui_c_QList_QInputMethodEvent_Attribute_back(QList< QInputMethodEvent::Attribute >* this_ptr) {
  return &this_ptr->back();
}

const QInputMethodEvent::Attribute* qt_gui_c_QList_QInputMethodEvent_Attribute_back_const(const QList< QInputMethodEvent::Attribute >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_clear(QList< QInputMethodEvent::Attribute >* this_ptr) {
  this_ptr->clear();
}

const QInputMethodEvent::Attribute* qt_gui_c_QList_QInputMethodEvent_Attribute_constFirst(const QList< QInputMethodEvent::Attribute >* this_ptr) {
  return &this_ptr->constFirst();
}

const QInputMethodEvent::Attribute* qt_gui_c_QList_QInputMethodEvent_Attribute_constLast(const QList< QInputMethodEvent::Attribute >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_constructor_l(const QList< QInputMethodEvent::Attribute >* l, QList< QInputMethodEvent::Attribute >* output) {
  new(output) QList< QInputMethodEvent::Attribute >(*l);
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_constructor_no_args(QList< QInputMethodEvent::Attribute >* output) {
  new(output) QList< QInputMethodEvent::Attribute >();
}

int qt_gui_c_QList_QInputMethodEvent_Attribute_count(const QList< QInputMethodEvent::Attribute >* this_ptr) {
  return this_ptr->count();
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_destructor(QList< QInputMethodEvent::Attribute >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QInputMethodEvent_Attribute_empty(const QList< QInputMethodEvent::Attribute >* this_ptr) {
  return this_ptr->empty();
}

QInputMethodEvent::Attribute* qt_gui_c_QList_QInputMethodEvent_Attribute_first(QList< QInputMethodEvent::Attribute >* this_ptr) {
  return &this_ptr->first();
}

const QInputMethodEvent::Attribute* qt_gui_c_QList_QInputMethodEvent_Attribute_first_const(const QList< QInputMethodEvent::Attribute >* this_ptr) {
  return &this_ptr->first();
}

QInputMethodEvent::Attribute* qt_gui_c_QList_QInputMethodEvent_Attribute_front(QList< QInputMethodEvent::Attribute >* this_ptr) {
  return &this_ptr->front();
}

const QInputMethodEvent::Attribute* qt_gui_c_QList_QInputMethodEvent_Attribute_front_const(const QList< QInputMethodEvent::Attribute >* this_ptr) {
  return &this_ptr->front();
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_insert(QList< QInputMethodEvent::Attribute >* this_ptr, int i, const QInputMethodEvent::Attribute* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QInputMethodEvent_Attribute_isEmpty(const QList< QInputMethodEvent::Attribute >* this_ptr) {
  return this_ptr->isEmpty();
}

QInputMethodEvent::Attribute* qt_gui_c_QList_QInputMethodEvent_Attribute_last(QList< QInputMethodEvent::Attribute >* this_ptr) {
  return &this_ptr->last();
}

const QInputMethodEvent::Attribute* qt_gui_c_QList_QInputMethodEvent_Attribute_last_const(const QList< QInputMethodEvent::Attribute >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QInputMethodEvent_Attribute_length(const QList< QInputMethodEvent::Attribute >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_mid_to_output_pos(const QList< QInputMethodEvent::Attribute >* this_ptr, int pos, QList< QInputMethodEvent::Attribute >* output) {
  new(output) QList< QInputMethodEvent::Attribute >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_mid_to_output_pos_length(const QList< QInputMethodEvent::Attribute >* this_ptr, int pos, int length, QList< QInputMethodEvent::Attribute >* output) {
  new(output) QList< QInputMethodEvent::Attribute >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_move(QList< QInputMethodEvent::Attribute >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QInputMethodEvent::Attribute >* qt_gui_c_QList_QInputMethodEvent_Attribute_operator_add_assign_l(QList< QInputMethodEvent::Attribute >* this_ptr, const QList< QInputMethodEvent::Attribute >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QInputMethodEvent::Attribute >* qt_gui_c_QList_QInputMethodEvent_Attribute_operator_add_assign_t(QList< QInputMethodEvent::Attribute >* this_ptr, const QInputMethodEvent::Attribute* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_operator_add_to_output(const QList< QInputMethodEvent::Attribute >* this_ptr, const QList< QInputMethodEvent::Attribute >* l, QList< QInputMethodEvent::Attribute >* output) {
  new(output) QList< QInputMethodEvent::Attribute >(this_ptr->operator+(*l));
}

QList< QInputMethodEvent::Attribute >* qt_gui_c_QList_QInputMethodEvent_Attribute_operator_assign(QList< QInputMethodEvent::Attribute >* this_ptr, const QList< QInputMethodEvent::Attribute >* l) {
  return &this_ptr->operator=(*l);
}

QInputMethodEvent::Attribute* qt_gui_c_QList_QInputMethodEvent_Attribute_operator_index(QList< QInputMethodEvent::Attribute >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QInputMethodEvent::Attribute* qt_gui_c_QList_QInputMethodEvent_Attribute_operator_index_const(const QList< QInputMethodEvent::Attribute >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QList< QInputMethodEvent::Attribute >* qt_gui_c_QList_QInputMethodEvent_Attribute_operator_shl_l(QList< QInputMethodEvent::Attribute >* this_ptr, const QList< QInputMethodEvent::Attribute >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QInputMethodEvent::Attribute >* qt_gui_c_QList_QInputMethodEvent_Attribute_operator_shl_t(QList< QInputMethodEvent::Attribute >* this_ptr, const QInputMethodEvent::Attribute* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_pop_back(QList< QInputMethodEvent::Attribute >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_pop_front(QList< QInputMethodEvent::Attribute >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_prepend(QList< QInputMethodEvent::Attribute >* this_ptr, const QInputMethodEvent::Attribute* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_push_back(QList< QInputMethodEvent::Attribute >* this_ptr, const QInputMethodEvent::Attribute* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_push_front(QList< QInputMethodEvent::Attribute >* this_ptr, const QInputMethodEvent::Attribute* t) {
  this_ptr->push_front(*t);
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_removeAt(QList< QInputMethodEvent::Attribute >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_removeFirst(QList< QInputMethodEvent::Attribute >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_removeLast(QList< QInputMethodEvent::Attribute >* this_ptr) {
  this_ptr->removeLast();
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_replace(QList< QInputMethodEvent::Attribute >* this_ptr, int i, const QInputMethodEvent::Attribute* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_reserve(QList< QInputMethodEvent::Attribute >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QInputMethodEvent_Attribute_size(const QList< QInputMethodEvent::Attribute >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_swap_i_j(QList< QInputMethodEvent::Attribute >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_swap_other(QList< QInputMethodEvent::Attribute >* this_ptr, QList< QInputMethodEvent::Attribute >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_takeAt_to_output(QList< QInputMethodEvent::Attribute >* this_ptr, int i, QInputMethodEvent::Attribute* output) {
  new(output) QInputMethodEvent::Attribute(this_ptr->takeAt(i));
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_takeFirst_to_output(QList< QInputMethodEvent::Attribute >* this_ptr, QInputMethodEvent::Attribute* output) {
  new(output) QInputMethodEvent::Attribute(this_ptr->takeFirst());
}

void qt_gui_c_QList_QInputMethodEvent_Attribute_takeLast_to_output(QList< QInputMethodEvent::Attribute >* this_ptr, QInputMethodEvent::Attribute* output) {
  new(output) QInputMethodEvent::Attribute(this_ptr->takeLast());
}

void qt_gui_c_QList_QKeySequence_append_QKeySequence(QList< QKeySequence >* this_ptr, const QKeySequence* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QKeySequence_append_QList_QKeySequence(QList< QKeySequence >* this_ptr, const QList< QKeySequence >* t) {
  this_ptr->append(*t);
}

const QKeySequence* qt_gui_c_QList_QKeySequence_at(const QList< QKeySequence >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QKeySequence* qt_gui_c_QList_QKeySequence_back(QList< QKeySequence >* this_ptr) {
  return &this_ptr->back();
}

const QKeySequence* qt_gui_c_QList_QKeySequence_back_const(const QList< QKeySequence >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QKeySequence_clear(QList< QKeySequence >* this_ptr) {
  this_ptr->clear();
}

const QKeySequence* qt_gui_c_QList_QKeySequence_constFirst(const QList< QKeySequence >* this_ptr) {
  return &this_ptr->constFirst();
}

const QKeySequence* qt_gui_c_QList_QKeySequence_constLast(const QList< QKeySequence >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QKeySequence_constructor_l(const QList< QKeySequence >* l, QList< QKeySequence >* output) {
  new(output) QList< QKeySequence >(*l);
}

void qt_gui_c_QList_QKeySequence_constructor_no_args(QList< QKeySequence >* output) {
  new(output) QList< QKeySequence >();
}

bool qt_gui_c_QList_QKeySequence_contains(const QList< QKeySequence >* this_ptr, const QKeySequence* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QKeySequence_count_no_args(const QList< QKeySequence >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QKeySequence_count_t(const QList< QKeySequence >* this_ptr, const QKeySequence* t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QKeySequence_destructor(QList< QKeySequence >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QKeySequence_empty(const QList< QKeySequence >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QKeySequence_endsWith(const QList< QKeySequence >* this_ptr, const QKeySequence* t) {
  return this_ptr->endsWith(*t);
}

QKeySequence* qt_gui_c_QList_QKeySequence_first(QList< QKeySequence >* this_ptr) {
  return &this_ptr->first();
}

const QKeySequence* qt_gui_c_QList_QKeySequence_first_const(const QList< QKeySequence >* this_ptr) {
  return &this_ptr->first();
}

QKeySequence* qt_gui_c_QList_QKeySequence_front(QList< QKeySequence >* this_ptr) {
  return &this_ptr->front();
}

const QKeySequence* qt_gui_c_QList_QKeySequence_front_const(const QList< QKeySequence >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QKeySequence_indexOf_t(const QList< QKeySequence >* this_ptr, const QKeySequence* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QKeySequence_indexOf_t_from(const QList< QKeySequence >* this_ptr, const QKeySequence* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QKeySequence_insert(QList< QKeySequence >* this_ptr, int i, const QKeySequence* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QKeySequence_isEmpty(const QList< QKeySequence >* this_ptr) {
  return this_ptr->isEmpty();
}

QKeySequence* qt_gui_c_QList_QKeySequence_last(QList< QKeySequence >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QKeySequence_lastIndexOf_t(const QList< QKeySequence >* this_ptr, const QKeySequence* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QKeySequence_lastIndexOf_t_from(const QList< QKeySequence >* this_ptr, const QKeySequence* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QKeySequence* qt_gui_c_QList_QKeySequence_last_const(const QList< QKeySequence >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QKeySequence_length(const QList< QKeySequence >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QKeySequence_mid_to_output_pos(const QList< QKeySequence >* this_ptr, int pos, QList< QKeySequence >* output) {
  new(output) QList< QKeySequence >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QKeySequence_mid_to_output_pos_length(const QList< QKeySequence >* this_ptr, int pos, int length, QList< QKeySequence >* output) {
  new(output) QList< QKeySequence >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QKeySequence_move(QList< QKeySequence >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QKeySequence >* qt_gui_c_QList_QKeySequence_operator_add_assign_l(QList< QKeySequence >* this_ptr, const QList< QKeySequence >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QKeySequence >* qt_gui_c_QList_QKeySequence_operator_add_assign_t(QList< QKeySequence >* this_ptr, const QKeySequence* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QKeySequence_operator_add_to_output(const QList< QKeySequence >* this_ptr, const QList< QKeySequence >* l, QList< QKeySequence >* output) {
  new(output) QList< QKeySequence >(this_ptr->operator+(*l));
}

QList< QKeySequence >* qt_gui_c_QList_QKeySequence_operator_assign(QList< QKeySequence >* this_ptr, const QList< QKeySequence >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QKeySequence_operator_eq(const QList< QKeySequence >* this_ptr, const QList< QKeySequence >* l) {
  return this_ptr->operator==(*l);
}

QKeySequence* qt_gui_c_QList_QKeySequence_operator_index(QList< QKeySequence >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QKeySequence* qt_gui_c_QList_QKeySequence_operator_index_const(const QList< QKeySequence >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QKeySequence_operator_neq(const QList< QKeySequence >* this_ptr, const QList< QKeySequence >* l) {
  return this_ptr->operator!=(*l);
}

QList< QKeySequence >* qt_gui_c_QList_QKeySequence_operator_shl_l(QList< QKeySequence >* this_ptr, const QList< QKeySequence >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QKeySequence >* qt_gui_c_QList_QKeySequence_operator_shl_t(QList< QKeySequence >* this_ptr, const QKeySequence* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QKeySequence_pop_back(QList< QKeySequence >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QKeySequence_pop_front(QList< QKeySequence >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QKeySequence_prepend(QList< QKeySequence >* this_ptr, const QKeySequence* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QKeySequence_push_back(QList< QKeySequence >* this_ptr, const QKeySequence* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QKeySequence_push_front(QList< QKeySequence >* this_ptr, const QKeySequence* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QKeySequence_removeAll(QList< QKeySequence >* this_ptr, const QKeySequence* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QKeySequence_removeAt(QList< QKeySequence >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QKeySequence_removeFirst(QList< QKeySequence >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QKeySequence_removeLast(QList< QKeySequence >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QKeySequence_removeOne(QList< QKeySequence >* this_ptr, const QKeySequence* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QKeySequence_replace(QList< QKeySequence >* this_ptr, int i, const QKeySequence* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QKeySequence_reserve(QList< QKeySequence >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QKeySequence_size(const QList< QKeySequence >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QKeySequence_startsWith(const QList< QKeySequence >* this_ptr, const QKeySequence* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QKeySequence_swap_i_j(QList< QKeySequence >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QKeySequence_swap_other(QList< QKeySequence >* this_ptr, QList< QKeySequence >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QList_QKeySequence_takeAt_to_output(QList< QKeySequence >* this_ptr, int i, QKeySequence* output) {
  new(output) QKeySequence(this_ptr->takeAt(i));
}

void qt_gui_c_QList_QKeySequence_takeFirst_to_output(QList< QKeySequence >* this_ptr, QKeySequence* output) {
  new(output) QKeySequence(this_ptr->takeFirst());
}

void qt_gui_c_QList_QKeySequence_takeLast_to_output(QList< QKeySequence >* this_ptr, QKeySequence* output) {
  new(output) QKeySequence(this_ptr->takeLast());
}

void qt_gui_c_QList_QKeySequence_value_to_output_i(const QList< QKeySequence >* this_ptr, int i, QKeySequence* output) {
  new(output) QKeySequence(this_ptr->value(i));
}

void qt_gui_c_QList_QKeySequence_value_to_output_i_defaultValue(const QList< QKeySequence >* this_ptr, int i, const QKeySequence* defaultValue, QKeySequence* output) {
  new(output) QKeySequence(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QList_QOpenGLContext_ptr_append_QList_QOpenGLContext_ptr(QList< QOpenGLContext* >* this_ptr, const QList< QOpenGLContext* >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QOpenGLContext_ptr_append_QOpenGLContext(QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t) {
  this_ptr->append(*t);
}

QOpenGLContext* const * qt_gui_c_QList_QOpenGLContext_ptr_at(const QList< QOpenGLContext* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QOpenGLContext** qt_gui_c_QList_QOpenGLContext_ptr_back(QList< QOpenGLContext* >* this_ptr) {
  return &this_ptr->back();
}

QOpenGLContext* const * qt_gui_c_QList_QOpenGLContext_ptr_back_const(const QList< QOpenGLContext* >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QOpenGLContext_ptr_clear(QList< QOpenGLContext* >* this_ptr) {
  this_ptr->clear();
}

QOpenGLContext* const * qt_gui_c_QList_QOpenGLContext_ptr_constFirst(const QList< QOpenGLContext* >* this_ptr) {
  return &this_ptr->constFirst();
}

QOpenGLContext* const * qt_gui_c_QList_QOpenGLContext_ptr_constLast(const QList< QOpenGLContext* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QOpenGLContext_ptr_constructor_l(const QList< QOpenGLContext* >* l, QList< QOpenGLContext* >* output) {
  new(output) QList< QOpenGLContext* >(*l);
}

void qt_gui_c_QList_QOpenGLContext_ptr_constructor_no_args(QList< QOpenGLContext* >* output) {
  new(output) QList< QOpenGLContext* >();
}

bool qt_gui_c_QList_QOpenGLContext_ptr_contains(const QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QOpenGLContext_ptr_count_no_args(const QList< QOpenGLContext* >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QOpenGLContext_ptr_count_t(const QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QOpenGLContext_ptr_destructor(QList< QOpenGLContext* >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QOpenGLContext_ptr_empty(const QList< QOpenGLContext* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QOpenGLContext_ptr_endsWith(const QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t) {
  return this_ptr->endsWith(*t);
}

QOpenGLContext** qt_gui_c_QList_QOpenGLContext_ptr_first(QList< QOpenGLContext* >* this_ptr) {
  return &this_ptr->first();
}

QOpenGLContext* const * qt_gui_c_QList_QOpenGLContext_ptr_first_const(const QList< QOpenGLContext* >* this_ptr) {
  return &this_ptr->first();
}

QOpenGLContext** qt_gui_c_QList_QOpenGLContext_ptr_front(QList< QOpenGLContext* >* this_ptr) {
  return &this_ptr->front();
}

QOpenGLContext* const * qt_gui_c_QList_QOpenGLContext_ptr_front_const(const QList< QOpenGLContext* >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QOpenGLContext_ptr_indexOf_t(const QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QOpenGLContext_ptr_indexOf_t_from(const QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QOpenGLContext_ptr_insert(QList< QOpenGLContext* >* this_ptr, int i, QOpenGLContext* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QOpenGLContext_ptr_isEmpty(const QList< QOpenGLContext* >* this_ptr) {
  return this_ptr->isEmpty();
}

QOpenGLContext** qt_gui_c_QList_QOpenGLContext_ptr_last(QList< QOpenGLContext* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QOpenGLContext_ptr_lastIndexOf_t(const QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QOpenGLContext_ptr_lastIndexOf_t_from(const QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QOpenGLContext* const * qt_gui_c_QList_QOpenGLContext_ptr_last_const(const QList< QOpenGLContext* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QOpenGLContext_ptr_length(const QList< QOpenGLContext* >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QOpenGLContext_ptr_mid_to_output_pos(const QList< QOpenGLContext* >* this_ptr, int pos, QList< QOpenGLContext* >* output) {
  new(output) QList< QOpenGLContext* >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QOpenGLContext_ptr_mid_to_output_pos_length(const QList< QOpenGLContext* >* this_ptr, int pos, int length, QList< QOpenGLContext* >* output) {
  new(output) QList< QOpenGLContext* >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QOpenGLContext_ptr_move(QList< QOpenGLContext* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QOpenGLContext* >* qt_gui_c_QList_QOpenGLContext_ptr_operator_add_assign_l(QList< QOpenGLContext* >* this_ptr, const QList< QOpenGLContext* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QOpenGLContext* >* qt_gui_c_QList_QOpenGLContext_ptr_operator_add_assign_t(QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QOpenGLContext_ptr_operator_add_to_output(const QList< QOpenGLContext* >* this_ptr, const QList< QOpenGLContext* >* l, QList< QOpenGLContext* >* output) {
  new(output) QList< QOpenGLContext* >(this_ptr->operator+(*l));
}

QList< QOpenGLContext* >* qt_gui_c_QList_QOpenGLContext_ptr_operator_assign(QList< QOpenGLContext* >* this_ptr, const QList< QOpenGLContext* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QOpenGLContext_ptr_operator_eq(const QList< QOpenGLContext* >* this_ptr, const QList< QOpenGLContext* >* l) {
  return this_ptr->operator==(*l);
}

QOpenGLContext** qt_gui_c_QList_QOpenGLContext_ptr_operator_index(QList< QOpenGLContext* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QOpenGLContext* const * qt_gui_c_QList_QOpenGLContext_ptr_operator_index_const(const QList< QOpenGLContext* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QOpenGLContext_ptr_operator_neq(const QList< QOpenGLContext* >* this_ptr, const QList< QOpenGLContext* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QOpenGLContext* >* qt_gui_c_QList_QOpenGLContext_ptr_operator_shl_l(QList< QOpenGLContext* >* this_ptr, const QList< QOpenGLContext* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QOpenGLContext* >* qt_gui_c_QList_QOpenGLContext_ptr_operator_shl_t(QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QOpenGLContext_ptr_pop_back(QList< QOpenGLContext* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QOpenGLContext_ptr_pop_front(QList< QOpenGLContext* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QOpenGLContext_ptr_prepend(QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QOpenGLContext_ptr_push_back(QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QOpenGLContext_ptr_push_front(QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QOpenGLContext_ptr_removeAll(QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QOpenGLContext_ptr_removeAt(QList< QOpenGLContext* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QOpenGLContext_ptr_removeFirst(QList< QOpenGLContext* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QOpenGLContext_ptr_removeLast(QList< QOpenGLContext* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QOpenGLContext_ptr_removeOne(QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QOpenGLContext_ptr_replace(QList< QOpenGLContext* >* this_ptr, int i, QOpenGLContext* const * t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QOpenGLContext_ptr_reserve(QList< QOpenGLContext* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QOpenGLContext_ptr_size(const QList< QOpenGLContext* >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QOpenGLContext_ptr_startsWith(const QList< QOpenGLContext* >* this_ptr, QOpenGLContext* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QOpenGLContext_ptr_swap_i_j(QList< QOpenGLContext* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QOpenGLContext_ptr_swap_other(QList< QOpenGLContext* >* this_ptr, QList< QOpenGLContext* >* other) {
  this_ptr->swap(*other);
}

QOpenGLContext* qt_gui_c_QList_QOpenGLContext_ptr_takeAt(QList< QOpenGLContext* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QOpenGLContext* qt_gui_c_QList_QOpenGLContext_ptr_takeFirst(QList< QOpenGLContext* >* this_ptr) {
  return this_ptr->takeFirst();
}

QOpenGLContext* qt_gui_c_QList_QOpenGLContext_ptr_takeLast(QList< QOpenGLContext* >* this_ptr) {
  return this_ptr->takeLast();
}

QOpenGLContext* qt_gui_c_QList_QOpenGLContext_ptr_value_i(const QList< QOpenGLContext* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QOpenGLContext* qt_gui_c_QList_QOpenGLContext_ptr_value_i_defaultValue(const QList< QOpenGLContext* >* this_ptr, int i, QOpenGLContext* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_gui_c_QList_QOpenGLDebugMessage_append_QList_QOpenGLDebugMessage(QList< QOpenGLDebugMessage >* this_ptr, const QList< QOpenGLDebugMessage >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QOpenGLDebugMessage_append_QOpenGLDebugMessage(QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t) {
  this_ptr->append(*t);
}

const QOpenGLDebugMessage* qt_gui_c_QList_QOpenGLDebugMessage_at(const QList< QOpenGLDebugMessage >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QOpenGLDebugMessage* qt_gui_c_QList_QOpenGLDebugMessage_back(QList< QOpenGLDebugMessage >* this_ptr) {
  return &this_ptr->back();
}

const QOpenGLDebugMessage* qt_gui_c_QList_QOpenGLDebugMessage_back_const(const QList< QOpenGLDebugMessage >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QOpenGLDebugMessage_clear(QList< QOpenGLDebugMessage >* this_ptr) {
  this_ptr->clear();
}

const QOpenGLDebugMessage* qt_gui_c_QList_QOpenGLDebugMessage_constFirst(const QList< QOpenGLDebugMessage >* this_ptr) {
  return &this_ptr->constFirst();
}

const QOpenGLDebugMessage* qt_gui_c_QList_QOpenGLDebugMessage_constLast(const QList< QOpenGLDebugMessage >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QOpenGLDebugMessage_constructor_l(const QList< QOpenGLDebugMessage >* l, QList< QOpenGLDebugMessage >* output) {
  new(output) QList< QOpenGLDebugMessage >(*l);
}

void qt_gui_c_QList_QOpenGLDebugMessage_constructor_no_args(QList< QOpenGLDebugMessage >* output) {
  new(output) QList< QOpenGLDebugMessage >();
}

bool qt_gui_c_QList_QOpenGLDebugMessage_contains(const QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QOpenGLDebugMessage_count_no_args(const QList< QOpenGLDebugMessage >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QOpenGLDebugMessage_count_t(const QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QOpenGLDebugMessage_destructor(QList< QOpenGLDebugMessage >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QOpenGLDebugMessage_empty(const QList< QOpenGLDebugMessage >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QOpenGLDebugMessage_endsWith(const QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t) {
  return this_ptr->endsWith(*t);
}

QOpenGLDebugMessage* qt_gui_c_QList_QOpenGLDebugMessage_first(QList< QOpenGLDebugMessage >* this_ptr) {
  return &this_ptr->first();
}

const QOpenGLDebugMessage* qt_gui_c_QList_QOpenGLDebugMessage_first_const(const QList< QOpenGLDebugMessage >* this_ptr) {
  return &this_ptr->first();
}

QOpenGLDebugMessage* qt_gui_c_QList_QOpenGLDebugMessage_front(QList< QOpenGLDebugMessage >* this_ptr) {
  return &this_ptr->front();
}

const QOpenGLDebugMessage* qt_gui_c_QList_QOpenGLDebugMessage_front_const(const QList< QOpenGLDebugMessage >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QOpenGLDebugMessage_indexOf_t(const QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QOpenGLDebugMessage_indexOf_t_from(const QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QOpenGLDebugMessage_insert(QList< QOpenGLDebugMessage >* this_ptr, int i, const QOpenGLDebugMessage* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QOpenGLDebugMessage_isEmpty(const QList< QOpenGLDebugMessage >* this_ptr) {
  return this_ptr->isEmpty();
}

QOpenGLDebugMessage* qt_gui_c_QList_QOpenGLDebugMessage_last(QList< QOpenGLDebugMessage >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QOpenGLDebugMessage_lastIndexOf_t(const QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QOpenGLDebugMessage_lastIndexOf_t_from(const QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QOpenGLDebugMessage* qt_gui_c_QList_QOpenGLDebugMessage_last_const(const QList< QOpenGLDebugMessage >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QOpenGLDebugMessage_length(const QList< QOpenGLDebugMessage >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QOpenGLDebugMessage_mid_to_output_pos(const QList< QOpenGLDebugMessage >* this_ptr, int pos, QList< QOpenGLDebugMessage >* output) {
  new(output) QList< QOpenGLDebugMessage >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QOpenGLDebugMessage_mid_to_output_pos_length(const QList< QOpenGLDebugMessage >* this_ptr, int pos, int length, QList< QOpenGLDebugMessage >* output) {
  new(output) QList< QOpenGLDebugMessage >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QOpenGLDebugMessage_move(QList< QOpenGLDebugMessage >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QOpenGLDebugMessage >* qt_gui_c_QList_QOpenGLDebugMessage_operator_add_assign_l(QList< QOpenGLDebugMessage >* this_ptr, const QList< QOpenGLDebugMessage >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QOpenGLDebugMessage >* qt_gui_c_QList_QOpenGLDebugMessage_operator_add_assign_t(QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QOpenGLDebugMessage_operator_add_to_output(const QList< QOpenGLDebugMessage >* this_ptr, const QList< QOpenGLDebugMessage >* l, QList< QOpenGLDebugMessage >* output) {
  new(output) QList< QOpenGLDebugMessage >(this_ptr->operator+(*l));
}

QList< QOpenGLDebugMessage >* qt_gui_c_QList_QOpenGLDebugMessage_operator_assign(QList< QOpenGLDebugMessage >* this_ptr, const QList< QOpenGLDebugMessage >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QOpenGLDebugMessage_operator_eq(const QList< QOpenGLDebugMessage >* this_ptr, const QList< QOpenGLDebugMessage >* l) {
  return this_ptr->operator==(*l);
}

QOpenGLDebugMessage* qt_gui_c_QList_QOpenGLDebugMessage_operator_index(QList< QOpenGLDebugMessage >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QOpenGLDebugMessage* qt_gui_c_QList_QOpenGLDebugMessage_operator_index_const(const QList< QOpenGLDebugMessage >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QOpenGLDebugMessage_operator_neq(const QList< QOpenGLDebugMessage >* this_ptr, const QList< QOpenGLDebugMessage >* l) {
  return this_ptr->operator!=(*l);
}

QList< QOpenGLDebugMessage >* qt_gui_c_QList_QOpenGLDebugMessage_operator_shl_l(QList< QOpenGLDebugMessage >* this_ptr, const QList< QOpenGLDebugMessage >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QOpenGLDebugMessage >* qt_gui_c_QList_QOpenGLDebugMessage_operator_shl_t(QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QOpenGLDebugMessage_pop_back(QList< QOpenGLDebugMessage >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QOpenGLDebugMessage_pop_front(QList< QOpenGLDebugMessage >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QOpenGLDebugMessage_prepend(QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QOpenGLDebugMessage_push_back(QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QOpenGLDebugMessage_push_front(QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QOpenGLDebugMessage_removeAll(QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QOpenGLDebugMessage_removeAt(QList< QOpenGLDebugMessage >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QOpenGLDebugMessage_removeFirst(QList< QOpenGLDebugMessage >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QOpenGLDebugMessage_removeLast(QList< QOpenGLDebugMessage >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QOpenGLDebugMessage_removeOne(QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QOpenGLDebugMessage_replace(QList< QOpenGLDebugMessage >* this_ptr, int i, const QOpenGLDebugMessage* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QOpenGLDebugMessage_reserve(QList< QOpenGLDebugMessage >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QOpenGLDebugMessage_size(const QList< QOpenGLDebugMessage >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QOpenGLDebugMessage_startsWith(const QList< QOpenGLDebugMessage >* this_ptr, const QOpenGLDebugMessage* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QOpenGLDebugMessage_swap_i_j(QList< QOpenGLDebugMessage >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QOpenGLDebugMessage_swap_other(QList< QOpenGLDebugMessage >* this_ptr, QList< QOpenGLDebugMessage >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QList_QOpenGLDebugMessage_takeAt_to_output(QList< QOpenGLDebugMessage >* this_ptr, int i, QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage(this_ptr->takeAt(i));
}

void qt_gui_c_QList_QOpenGLDebugMessage_takeFirst_to_output(QList< QOpenGLDebugMessage >* this_ptr, QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage(this_ptr->takeFirst());
}

void qt_gui_c_QList_QOpenGLDebugMessage_takeLast_to_output(QList< QOpenGLDebugMessage >* this_ptr, QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage(this_ptr->takeLast());
}

void qt_gui_c_QList_QOpenGLDebugMessage_value_to_output_i(const QList< QOpenGLDebugMessage >* this_ptr, int i, QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage(this_ptr->value(i));
}

void qt_gui_c_QList_QOpenGLDebugMessage_value_to_output_i_defaultValue(const QList< QOpenGLDebugMessage >* this_ptr, int i, const QOpenGLDebugMessage* defaultValue, QOpenGLDebugMessage* output) {
  new(output) QOpenGLDebugMessage(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QList_QOpenGLShader_ptr_append_QList_QOpenGLShader_ptr(QList< QOpenGLShader* >* this_ptr, const QList< QOpenGLShader* >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QOpenGLShader_ptr_append_QOpenGLShader(QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t) {
  this_ptr->append(*t);
}

QOpenGLShader* const * qt_gui_c_QList_QOpenGLShader_ptr_at(const QList< QOpenGLShader* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QOpenGLShader** qt_gui_c_QList_QOpenGLShader_ptr_back(QList< QOpenGLShader* >* this_ptr) {
  return &this_ptr->back();
}

QOpenGLShader* const * qt_gui_c_QList_QOpenGLShader_ptr_back_const(const QList< QOpenGLShader* >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QOpenGLShader_ptr_clear(QList< QOpenGLShader* >* this_ptr) {
  this_ptr->clear();
}

QOpenGLShader* const * qt_gui_c_QList_QOpenGLShader_ptr_constFirst(const QList< QOpenGLShader* >* this_ptr) {
  return &this_ptr->constFirst();
}

QOpenGLShader* const * qt_gui_c_QList_QOpenGLShader_ptr_constLast(const QList< QOpenGLShader* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QOpenGLShader_ptr_constructor_l(const QList< QOpenGLShader* >* l, QList< QOpenGLShader* >* output) {
  new(output) QList< QOpenGLShader* >(*l);
}

void qt_gui_c_QList_QOpenGLShader_ptr_constructor_no_args(QList< QOpenGLShader* >* output) {
  new(output) QList< QOpenGLShader* >();
}

bool qt_gui_c_QList_QOpenGLShader_ptr_contains(const QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QOpenGLShader_ptr_count_no_args(const QList< QOpenGLShader* >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QOpenGLShader_ptr_count_t(const QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QOpenGLShader_ptr_destructor(QList< QOpenGLShader* >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QOpenGLShader_ptr_empty(const QList< QOpenGLShader* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QOpenGLShader_ptr_endsWith(const QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t) {
  return this_ptr->endsWith(*t);
}

QOpenGLShader** qt_gui_c_QList_QOpenGLShader_ptr_first(QList< QOpenGLShader* >* this_ptr) {
  return &this_ptr->first();
}

QOpenGLShader* const * qt_gui_c_QList_QOpenGLShader_ptr_first_const(const QList< QOpenGLShader* >* this_ptr) {
  return &this_ptr->first();
}

QOpenGLShader** qt_gui_c_QList_QOpenGLShader_ptr_front(QList< QOpenGLShader* >* this_ptr) {
  return &this_ptr->front();
}

QOpenGLShader* const * qt_gui_c_QList_QOpenGLShader_ptr_front_const(const QList< QOpenGLShader* >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QOpenGLShader_ptr_indexOf_t(const QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QOpenGLShader_ptr_indexOf_t_from(const QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QOpenGLShader_ptr_insert(QList< QOpenGLShader* >* this_ptr, int i, QOpenGLShader* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QOpenGLShader_ptr_isEmpty(const QList< QOpenGLShader* >* this_ptr) {
  return this_ptr->isEmpty();
}

QOpenGLShader** qt_gui_c_QList_QOpenGLShader_ptr_last(QList< QOpenGLShader* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QOpenGLShader_ptr_lastIndexOf_t(const QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QOpenGLShader_ptr_lastIndexOf_t_from(const QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QOpenGLShader* const * qt_gui_c_QList_QOpenGLShader_ptr_last_const(const QList< QOpenGLShader* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QOpenGLShader_ptr_length(const QList< QOpenGLShader* >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QOpenGLShader_ptr_mid_to_output_pos(const QList< QOpenGLShader* >* this_ptr, int pos, QList< QOpenGLShader* >* output) {
  new(output) QList< QOpenGLShader* >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QOpenGLShader_ptr_mid_to_output_pos_length(const QList< QOpenGLShader* >* this_ptr, int pos, int length, QList< QOpenGLShader* >* output) {
  new(output) QList< QOpenGLShader* >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QOpenGLShader_ptr_move(QList< QOpenGLShader* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QOpenGLShader* >* qt_gui_c_QList_QOpenGLShader_ptr_operator_add_assign_l(QList< QOpenGLShader* >* this_ptr, const QList< QOpenGLShader* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QOpenGLShader* >* qt_gui_c_QList_QOpenGLShader_ptr_operator_add_assign_t(QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QOpenGLShader_ptr_operator_add_to_output(const QList< QOpenGLShader* >* this_ptr, const QList< QOpenGLShader* >* l, QList< QOpenGLShader* >* output) {
  new(output) QList< QOpenGLShader* >(this_ptr->operator+(*l));
}

QList< QOpenGLShader* >* qt_gui_c_QList_QOpenGLShader_ptr_operator_assign(QList< QOpenGLShader* >* this_ptr, const QList< QOpenGLShader* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QOpenGLShader_ptr_operator_eq(const QList< QOpenGLShader* >* this_ptr, const QList< QOpenGLShader* >* l) {
  return this_ptr->operator==(*l);
}

QOpenGLShader** qt_gui_c_QList_QOpenGLShader_ptr_operator_index(QList< QOpenGLShader* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QOpenGLShader* const * qt_gui_c_QList_QOpenGLShader_ptr_operator_index_const(const QList< QOpenGLShader* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QOpenGLShader_ptr_operator_neq(const QList< QOpenGLShader* >* this_ptr, const QList< QOpenGLShader* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QOpenGLShader* >* qt_gui_c_QList_QOpenGLShader_ptr_operator_shl_l(QList< QOpenGLShader* >* this_ptr, const QList< QOpenGLShader* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QOpenGLShader* >* qt_gui_c_QList_QOpenGLShader_ptr_operator_shl_t(QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QOpenGLShader_ptr_pop_back(QList< QOpenGLShader* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QOpenGLShader_ptr_pop_front(QList< QOpenGLShader* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QOpenGLShader_ptr_prepend(QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QOpenGLShader_ptr_push_back(QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QOpenGLShader_ptr_push_front(QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QOpenGLShader_ptr_removeAll(QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QOpenGLShader_ptr_removeAt(QList< QOpenGLShader* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QOpenGLShader_ptr_removeFirst(QList< QOpenGLShader* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QOpenGLShader_ptr_removeLast(QList< QOpenGLShader* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QOpenGLShader_ptr_removeOne(QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QOpenGLShader_ptr_replace(QList< QOpenGLShader* >* this_ptr, int i, QOpenGLShader* const * t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QOpenGLShader_ptr_reserve(QList< QOpenGLShader* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QOpenGLShader_ptr_size(const QList< QOpenGLShader* >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QOpenGLShader_ptr_startsWith(const QList< QOpenGLShader* >* this_ptr, QOpenGLShader* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QOpenGLShader_ptr_swap_i_j(QList< QOpenGLShader* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QOpenGLShader_ptr_swap_other(QList< QOpenGLShader* >* this_ptr, QList< QOpenGLShader* >* other) {
  this_ptr->swap(*other);
}

QOpenGLShader* qt_gui_c_QList_QOpenGLShader_ptr_takeAt(QList< QOpenGLShader* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QOpenGLShader* qt_gui_c_QList_QOpenGLShader_ptr_takeFirst(QList< QOpenGLShader* >* this_ptr) {
  return this_ptr->takeFirst();
}

QOpenGLShader* qt_gui_c_QList_QOpenGLShader_ptr_takeLast(QList< QOpenGLShader* >* this_ptr) {
  return this_ptr->takeLast();
}

QOpenGLShader* qt_gui_c_QList_QOpenGLShader_ptr_value_i(const QList< QOpenGLShader* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QOpenGLShader* qt_gui_c_QList_QOpenGLShader_ptr_value_i_defaultValue(const QList< QOpenGLShader* >* this_ptr, int i, QOpenGLShader* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_gui_c_QList_QPolygonF_append_QList_QPolygonF(QList< QPolygonF >* this_ptr, const QList< QPolygonF >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QPolygonF_append_QPolygonF(QList< QPolygonF >* this_ptr, const QPolygonF* t) {
  this_ptr->append(*t);
}

const QPolygonF* qt_gui_c_QList_QPolygonF_at(const QList< QPolygonF >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QPolygonF* qt_gui_c_QList_QPolygonF_back(QList< QPolygonF >* this_ptr) {
  return &this_ptr->back();
}

const QPolygonF* qt_gui_c_QList_QPolygonF_back_const(const QList< QPolygonF >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QPolygonF_clear(QList< QPolygonF >* this_ptr) {
  this_ptr->clear();
}

const QPolygonF* qt_gui_c_QList_QPolygonF_constFirst(const QList< QPolygonF >* this_ptr) {
  return &this_ptr->constFirst();
}

const QPolygonF* qt_gui_c_QList_QPolygonF_constLast(const QList< QPolygonF >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QPolygonF_constructor_l(const QList< QPolygonF >* l, QList< QPolygonF >* output) {
  new(output) QList< QPolygonF >(*l);
}

void qt_gui_c_QList_QPolygonF_constructor_no_args(QList< QPolygonF >* output) {
  new(output) QList< QPolygonF >();
}

bool qt_gui_c_QList_QPolygonF_contains(const QList< QPolygonF >* this_ptr, const QPolygonF* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QPolygonF_count_no_args(const QList< QPolygonF >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QPolygonF_count_t(const QList< QPolygonF >* this_ptr, const QPolygonF* t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QPolygonF_destructor(QList< QPolygonF >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QPolygonF_empty(const QList< QPolygonF >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QPolygonF_endsWith(const QList< QPolygonF >* this_ptr, const QPolygonF* t) {
  return this_ptr->endsWith(*t);
}

QPolygonF* qt_gui_c_QList_QPolygonF_first(QList< QPolygonF >* this_ptr) {
  return &this_ptr->first();
}

const QPolygonF* qt_gui_c_QList_QPolygonF_first_const(const QList< QPolygonF >* this_ptr) {
  return &this_ptr->first();
}

QPolygonF* qt_gui_c_QList_QPolygonF_front(QList< QPolygonF >* this_ptr) {
  return &this_ptr->front();
}

const QPolygonF* qt_gui_c_QList_QPolygonF_front_const(const QList< QPolygonF >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QPolygonF_indexOf_t(const QList< QPolygonF >* this_ptr, const QPolygonF* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QPolygonF_indexOf_t_from(const QList< QPolygonF >* this_ptr, const QPolygonF* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QPolygonF_insert(QList< QPolygonF >* this_ptr, int i, const QPolygonF* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QPolygonF_isEmpty(const QList< QPolygonF >* this_ptr) {
  return this_ptr->isEmpty();
}

QPolygonF* qt_gui_c_QList_QPolygonF_last(QList< QPolygonF >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QPolygonF_lastIndexOf_t(const QList< QPolygonF >* this_ptr, const QPolygonF* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QPolygonF_lastIndexOf_t_from(const QList< QPolygonF >* this_ptr, const QPolygonF* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QPolygonF* qt_gui_c_QList_QPolygonF_last_const(const QList< QPolygonF >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QPolygonF_length(const QList< QPolygonF >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QPolygonF_mid_to_output_pos(const QList< QPolygonF >* this_ptr, int pos, QList< QPolygonF >* output) {
  new(output) QList< QPolygonF >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QPolygonF_mid_to_output_pos_length(const QList< QPolygonF >* this_ptr, int pos, int length, QList< QPolygonF >* output) {
  new(output) QList< QPolygonF >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QPolygonF_move(QList< QPolygonF >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QPolygonF >* qt_gui_c_QList_QPolygonF_operator_add_assign_l(QList< QPolygonF >* this_ptr, const QList< QPolygonF >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QPolygonF >* qt_gui_c_QList_QPolygonF_operator_add_assign_t(QList< QPolygonF >* this_ptr, const QPolygonF* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QPolygonF_operator_add_to_output(const QList< QPolygonF >* this_ptr, const QList< QPolygonF >* l, QList< QPolygonF >* output) {
  new(output) QList< QPolygonF >(this_ptr->operator+(*l));
}

QList< QPolygonF >* qt_gui_c_QList_QPolygonF_operator_assign(QList< QPolygonF >* this_ptr, const QList< QPolygonF >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QPolygonF_operator_eq(const QList< QPolygonF >* this_ptr, const QList< QPolygonF >* l) {
  return this_ptr->operator==(*l);
}

QPolygonF* qt_gui_c_QList_QPolygonF_operator_index(QList< QPolygonF >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QPolygonF* qt_gui_c_QList_QPolygonF_operator_index_const(const QList< QPolygonF >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QPolygonF_operator_neq(const QList< QPolygonF >* this_ptr, const QList< QPolygonF >* l) {
  return this_ptr->operator!=(*l);
}

QList< QPolygonF >* qt_gui_c_QList_QPolygonF_operator_shl_l(QList< QPolygonF >* this_ptr, const QList< QPolygonF >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QPolygonF >* qt_gui_c_QList_QPolygonF_operator_shl_t(QList< QPolygonF >* this_ptr, const QPolygonF* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QPolygonF_pop_back(QList< QPolygonF >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QPolygonF_pop_front(QList< QPolygonF >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QPolygonF_prepend(QList< QPolygonF >* this_ptr, const QPolygonF* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QPolygonF_push_back(QList< QPolygonF >* this_ptr, const QPolygonF* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QPolygonF_push_front(QList< QPolygonF >* this_ptr, const QPolygonF* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QPolygonF_removeAll(QList< QPolygonF >* this_ptr, const QPolygonF* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QPolygonF_removeAt(QList< QPolygonF >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QPolygonF_removeFirst(QList< QPolygonF >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QPolygonF_removeLast(QList< QPolygonF >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QPolygonF_removeOne(QList< QPolygonF >* this_ptr, const QPolygonF* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QPolygonF_replace(QList< QPolygonF >* this_ptr, int i, const QPolygonF* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QPolygonF_reserve(QList< QPolygonF >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QPolygonF_size(const QList< QPolygonF >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QPolygonF_startsWith(const QList< QPolygonF >* this_ptr, const QPolygonF* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QPolygonF_swap_i_j(QList< QPolygonF >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QPolygonF_swap_other(QList< QPolygonF >* this_ptr, QList< QPolygonF >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QList_QPolygonF_takeAt_to_output(QList< QPolygonF >* this_ptr, int i, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->takeAt(i));
}

void qt_gui_c_QList_QPolygonF_takeFirst_to_output(QList< QPolygonF >* this_ptr, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->takeFirst());
}

void qt_gui_c_QList_QPolygonF_takeLast_to_output(QList< QPolygonF >* this_ptr, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->takeLast());
}

void qt_gui_c_QList_QPolygonF_value_to_output_i(const QList< QPolygonF >* this_ptr, int i, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->value(i));
}

void qt_gui_c_QList_QPolygonF_value_to_output_i_defaultValue(const QList< QPolygonF >* this_ptr, int i, const QPolygonF* defaultValue, QPolygonF* output) {
  new(output) QPolygonF(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QList_QScreen_ptr_append_QList_QScreen_ptr(QList< QScreen* >* this_ptr, const QList< QScreen* >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QScreen_ptr_append_QScreen(QList< QScreen* >* this_ptr, QScreen* const * t) {
  this_ptr->append(*t);
}

QScreen* const * qt_gui_c_QList_QScreen_ptr_at(const QList< QScreen* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QScreen** qt_gui_c_QList_QScreen_ptr_back(QList< QScreen* >* this_ptr) {
  return &this_ptr->back();
}

QScreen* const * qt_gui_c_QList_QScreen_ptr_back_const(const QList< QScreen* >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QScreen_ptr_clear(QList< QScreen* >* this_ptr) {
  this_ptr->clear();
}

QScreen* const * qt_gui_c_QList_QScreen_ptr_constFirst(const QList< QScreen* >* this_ptr) {
  return &this_ptr->constFirst();
}

QScreen* const * qt_gui_c_QList_QScreen_ptr_constLast(const QList< QScreen* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QScreen_ptr_constructor_l(const QList< QScreen* >* l, QList< QScreen* >* output) {
  new(output) QList< QScreen* >(*l);
}

void qt_gui_c_QList_QScreen_ptr_constructor_no_args(QList< QScreen* >* output) {
  new(output) QList< QScreen* >();
}

bool qt_gui_c_QList_QScreen_ptr_contains(const QList< QScreen* >* this_ptr, QScreen* const * t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QScreen_ptr_count_no_args(const QList< QScreen* >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QScreen_ptr_count_t(const QList< QScreen* >* this_ptr, QScreen* const * t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QScreen_ptr_destructor(QList< QScreen* >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QScreen_ptr_empty(const QList< QScreen* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QScreen_ptr_endsWith(const QList< QScreen* >* this_ptr, QScreen* const * t) {
  return this_ptr->endsWith(*t);
}

QScreen** qt_gui_c_QList_QScreen_ptr_first(QList< QScreen* >* this_ptr) {
  return &this_ptr->first();
}

QScreen* const * qt_gui_c_QList_QScreen_ptr_first_const(const QList< QScreen* >* this_ptr) {
  return &this_ptr->first();
}

QScreen** qt_gui_c_QList_QScreen_ptr_front(QList< QScreen* >* this_ptr) {
  return &this_ptr->front();
}

QScreen* const * qt_gui_c_QList_QScreen_ptr_front_const(const QList< QScreen* >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QScreen_ptr_indexOf_t(const QList< QScreen* >* this_ptr, QScreen* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QScreen_ptr_indexOf_t_from(const QList< QScreen* >* this_ptr, QScreen* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QScreen_ptr_insert(QList< QScreen* >* this_ptr, int i, QScreen* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QScreen_ptr_isEmpty(const QList< QScreen* >* this_ptr) {
  return this_ptr->isEmpty();
}

QScreen** qt_gui_c_QList_QScreen_ptr_last(QList< QScreen* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QScreen_ptr_lastIndexOf_t(const QList< QScreen* >* this_ptr, QScreen* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QScreen_ptr_lastIndexOf_t_from(const QList< QScreen* >* this_ptr, QScreen* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QScreen* const * qt_gui_c_QList_QScreen_ptr_last_const(const QList< QScreen* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QScreen_ptr_length(const QList< QScreen* >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QScreen_ptr_mid_to_output_pos(const QList< QScreen* >* this_ptr, int pos, QList< QScreen* >* output) {
  new(output) QList< QScreen* >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QScreen_ptr_mid_to_output_pos_length(const QList< QScreen* >* this_ptr, int pos, int length, QList< QScreen* >* output) {
  new(output) QList< QScreen* >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QScreen_ptr_move(QList< QScreen* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QScreen* >* qt_gui_c_QList_QScreen_ptr_operator_add_assign_l(QList< QScreen* >* this_ptr, const QList< QScreen* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QScreen* >* qt_gui_c_QList_QScreen_ptr_operator_add_assign_t(QList< QScreen* >* this_ptr, QScreen* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QScreen_ptr_operator_add_to_output(const QList< QScreen* >* this_ptr, const QList< QScreen* >* l, QList< QScreen* >* output) {
  new(output) QList< QScreen* >(this_ptr->operator+(*l));
}

QList< QScreen* >* qt_gui_c_QList_QScreen_ptr_operator_assign(QList< QScreen* >* this_ptr, const QList< QScreen* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QScreen_ptr_operator_eq(const QList< QScreen* >* this_ptr, const QList< QScreen* >* l) {
  return this_ptr->operator==(*l);
}

QScreen** qt_gui_c_QList_QScreen_ptr_operator_index(QList< QScreen* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QScreen* const * qt_gui_c_QList_QScreen_ptr_operator_index_const(const QList< QScreen* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QScreen_ptr_operator_neq(const QList< QScreen* >* this_ptr, const QList< QScreen* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QScreen* >* qt_gui_c_QList_QScreen_ptr_operator_shl_l(QList< QScreen* >* this_ptr, const QList< QScreen* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QScreen* >* qt_gui_c_QList_QScreen_ptr_operator_shl_t(QList< QScreen* >* this_ptr, QScreen* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QScreen_ptr_pop_back(QList< QScreen* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QScreen_ptr_pop_front(QList< QScreen* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QScreen_ptr_prepend(QList< QScreen* >* this_ptr, QScreen* const * t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QScreen_ptr_push_back(QList< QScreen* >* this_ptr, QScreen* const * t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QScreen_ptr_push_front(QList< QScreen* >* this_ptr, QScreen* const * t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QScreen_ptr_removeAll(QList< QScreen* >* this_ptr, QScreen* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QScreen_ptr_removeAt(QList< QScreen* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QScreen_ptr_removeFirst(QList< QScreen* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QScreen_ptr_removeLast(QList< QScreen* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QScreen_ptr_removeOne(QList< QScreen* >* this_ptr, QScreen* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QScreen_ptr_replace(QList< QScreen* >* this_ptr, int i, QScreen* const * t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QScreen_ptr_reserve(QList< QScreen* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QScreen_ptr_size(const QList< QScreen* >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QScreen_ptr_startsWith(const QList< QScreen* >* this_ptr, QScreen* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QScreen_ptr_swap_i_j(QList< QScreen* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QScreen_ptr_swap_other(QList< QScreen* >* this_ptr, QList< QScreen* >* other) {
  this_ptr->swap(*other);
}

QScreen* qt_gui_c_QList_QScreen_ptr_takeAt(QList< QScreen* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QScreen* qt_gui_c_QList_QScreen_ptr_takeFirst(QList< QScreen* >* this_ptr) {
  return this_ptr->takeFirst();
}

QScreen* qt_gui_c_QList_QScreen_ptr_takeLast(QList< QScreen* >* this_ptr) {
  return this_ptr->takeLast();
}

QScreen* qt_gui_c_QList_QScreen_ptr_value_i(const QList< QScreen* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QScreen* qt_gui_c_QList_QScreen_ptr_value_i_defaultValue(const QList< QScreen* >* this_ptr, int i, QScreen* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_gui_c_QList_QSize_append_QList_QSize(QList< QSize >* this_ptr, const QList< QSize >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QSize_append_QSize(QList< QSize >* this_ptr, const QSize* t) {
  this_ptr->append(*t);
}

const QSize* qt_gui_c_QList_QSize_at(const QList< QSize >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QSize* qt_gui_c_QList_QSize_back(QList< QSize >* this_ptr) {
  return &this_ptr->back();
}

const QSize* qt_gui_c_QList_QSize_back_const(const QList< QSize >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QSize_clear(QList< QSize >* this_ptr) {
  this_ptr->clear();
}

const QSize* qt_gui_c_QList_QSize_constFirst(const QList< QSize >* this_ptr) {
  return &this_ptr->constFirst();
}

const QSize* qt_gui_c_QList_QSize_constLast(const QList< QSize >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QSize_constructor_l(const QList< QSize >* l, QList< QSize >* output) {
  new(output) QList< QSize >(*l);
}

void qt_gui_c_QList_QSize_constructor_no_args(QList< QSize >* output) {
  new(output) QList< QSize >();
}

bool qt_gui_c_QList_QSize_contains(const QList< QSize >* this_ptr, const QSize* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QSize_count_no_args(const QList< QSize >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QSize_count_t(const QList< QSize >* this_ptr, const QSize* t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QSize_destructor(QList< QSize >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QSize_empty(const QList< QSize >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QSize_endsWith(const QList< QSize >* this_ptr, const QSize* t) {
  return this_ptr->endsWith(*t);
}

QSize* qt_gui_c_QList_QSize_first(QList< QSize >* this_ptr) {
  return &this_ptr->first();
}

const QSize* qt_gui_c_QList_QSize_first_const(const QList< QSize >* this_ptr) {
  return &this_ptr->first();
}

void qt_gui_c_QList_QSize_fromVector_to_output(const QVector< QSize >* vector, QList< QSize >* output) {
  new(output) QList< QSize >(QList< QSize >::fromVector(*vector));
}

QSize* qt_gui_c_QList_QSize_front(QList< QSize >* this_ptr) {
  return &this_ptr->front();
}

const QSize* qt_gui_c_QList_QSize_front_const(const QList< QSize >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QSize_indexOf_t(const QList< QSize >* this_ptr, const QSize* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QSize_indexOf_t_from(const QList< QSize >* this_ptr, const QSize* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QSize_insert(QList< QSize >* this_ptr, int i, const QSize* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QSize_isEmpty(const QList< QSize >* this_ptr) {
  return this_ptr->isEmpty();
}

QSize* qt_gui_c_QList_QSize_last(QList< QSize >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QSize_lastIndexOf_t(const QList< QSize >* this_ptr, const QSize* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QSize_lastIndexOf_t_from(const QList< QSize >* this_ptr, const QSize* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QSize* qt_gui_c_QList_QSize_last_const(const QList< QSize >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QSize_length(const QList< QSize >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QSize_mid_to_output_pos(const QList< QSize >* this_ptr, int pos, QList< QSize >* output) {
  new(output) QList< QSize >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QSize_mid_to_output_pos_length(const QList< QSize >* this_ptr, int pos, int length, QList< QSize >* output) {
  new(output) QList< QSize >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QSize_move(QList< QSize >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QSize >* qt_gui_c_QList_QSize_operator_add_assign_l(QList< QSize >* this_ptr, const QList< QSize >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QSize >* qt_gui_c_QList_QSize_operator_add_assign_t(QList< QSize >* this_ptr, const QSize* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QSize_operator_add_to_output(const QList< QSize >* this_ptr, const QList< QSize >* l, QList< QSize >* output) {
  new(output) QList< QSize >(this_ptr->operator+(*l));
}

QList< QSize >* qt_gui_c_QList_QSize_operator_assign(QList< QSize >* this_ptr, const QList< QSize >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QSize_operator_eq(const QList< QSize >* this_ptr, const QList< QSize >* l) {
  return this_ptr->operator==(*l);
}

QSize* qt_gui_c_QList_QSize_operator_index(QList< QSize >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QSize* qt_gui_c_QList_QSize_operator_index_const(const QList< QSize >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QSize_operator_neq(const QList< QSize >* this_ptr, const QList< QSize >* l) {
  return this_ptr->operator!=(*l);
}

QList< QSize >* qt_gui_c_QList_QSize_operator_shl_l(QList< QSize >* this_ptr, const QList< QSize >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QSize >* qt_gui_c_QList_QSize_operator_shl_t(QList< QSize >* this_ptr, const QSize* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QSize_pop_back(QList< QSize >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QSize_pop_front(QList< QSize >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QSize_prepend(QList< QSize >* this_ptr, const QSize* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QSize_push_back(QList< QSize >* this_ptr, const QSize* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QSize_push_front(QList< QSize >* this_ptr, const QSize* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QSize_removeAll(QList< QSize >* this_ptr, const QSize* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QSize_removeAt(QList< QSize >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QSize_removeFirst(QList< QSize >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QSize_removeLast(QList< QSize >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QSize_removeOne(QList< QSize >* this_ptr, const QSize* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QSize_replace(QList< QSize >* this_ptr, int i, const QSize* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QSize_reserve(QList< QSize >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QSize_size(const QList< QSize >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QSize_startsWith(const QList< QSize >* this_ptr, const QSize* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QSize_swap_i_j(QList< QSize >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QSize_swap_other(QList< QSize >* this_ptr, QList< QSize >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QList_QSize_takeAt_to_output(QList< QSize >* this_ptr, int i, QSize* output) {
  new(output) QSize(this_ptr->takeAt(i));
}

void qt_gui_c_QList_QSize_takeFirst_to_output(QList< QSize >* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->takeFirst());
}

void qt_gui_c_QList_QSize_takeLast_to_output(QList< QSize >* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->takeLast());
}

void qt_gui_c_QList_QSize_toVector_to_output(const QList< QSize >* this_ptr, QVector< QSize >* output) {
  new(output) QVector< QSize >(this_ptr->toVector());
}

void qt_gui_c_QList_QSize_value_to_output_i(const QList< QSize >* this_ptr, int i, QSize* output) {
  new(output) QSize(this_ptr->value(i));
}

void qt_gui_c_QList_QSize_value_to_output_i_defaultValue(const QList< QSize >* this_ptr, int i, const QSize* defaultValue, QSize* output) {
  new(output) QSize(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QList_QStandardItem_ptr_append_QList_QStandardItem_ptr(QList< QStandardItem* >* this_ptr, const QList< QStandardItem* >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QStandardItem_ptr_append_QStandardItem(QList< QStandardItem* >* this_ptr, QStandardItem* const * t) {
  this_ptr->append(*t);
}

QStandardItem* const * qt_gui_c_QList_QStandardItem_ptr_at(const QList< QStandardItem* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QStandardItem** qt_gui_c_QList_QStandardItem_ptr_back(QList< QStandardItem* >* this_ptr) {
  return &this_ptr->back();
}

QStandardItem* const * qt_gui_c_QList_QStandardItem_ptr_back_const(const QList< QStandardItem* >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QStandardItem_ptr_clear(QList< QStandardItem* >* this_ptr) {
  this_ptr->clear();
}

QStandardItem* const * qt_gui_c_QList_QStandardItem_ptr_constFirst(const QList< QStandardItem* >* this_ptr) {
  return &this_ptr->constFirst();
}

QStandardItem* const * qt_gui_c_QList_QStandardItem_ptr_constLast(const QList< QStandardItem* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QStandardItem_ptr_constructor_l(const QList< QStandardItem* >* l, QList< QStandardItem* >* output) {
  new(output) QList< QStandardItem* >(*l);
}

void qt_gui_c_QList_QStandardItem_ptr_constructor_no_args(QList< QStandardItem* >* output) {
  new(output) QList< QStandardItem* >();
}

bool qt_gui_c_QList_QStandardItem_ptr_contains(const QList< QStandardItem* >* this_ptr, QStandardItem* const * t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QStandardItem_ptr_count_no_args(const QList< QStandardItem* >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QStandardItem_ptr_count_t(const QList< QStandardItem* >* this_ptr, QStandardItem* const * t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QStandardItem_ptr_destructor(QList< QStandardItem* >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QStandardItem_ptr_empty(const QList< QStandardItem* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QStandardItem_ptr_endsWith(const QList< QStandardItem* >* this_ptr, QStandardItem* const * t) {
  return this_ptr->endsWith(*t);
}

QStandardItem** qt_gui_c_QList_QStandardItem_ptr_first(QList< QStandardItem* >* this_ptr) {
  return &this_ptr->first();
}

QStandardItem* const * qt_gui_c_QList_QStandardItem_ptr_first_const(const QList< QStandardItem* >* this_ptr) {
  return &this_ptr->first();
}

QStandardItem** qt_gui_c_QList_QStandardItem_ptr_front(QList< QStandardItem* >* this_ptr) {
  return &this_ptr->front();
}

QStandardItem* const * qt_gui_c_QList_QStandardItem_ptr_front_const(const QList< QStandardItem* >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QStandardItem_ptr_indexOf_t(const QList< QStandardItem* >* this_ptr, QStandardItem* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QStandardItem_ptr_indexOf_t_from(const QList< QStandardItem* >* this_ptr, QStandardItem* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QStandardItem_ptr_insert(QList< QStandardItem* >* this_ptr, int i, QStandardItem* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QStandardItem_ptr_isEmpty(const QList< QStandardItem* >* this_ptr) {
  return this_ptr->isEmpty();
}

QStandardItem** qt_gui_c_QList_QStandardItem_ptr_last(QList< QStandardItem* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QStandardItem_ptr_lastIndexOf_t(const QList< QStandardItem* >* this_ptr, QStandardItem* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QStandardItem_ptr_lastIndexOf_t_from(const QList< QStandardItem* >* this_ptr, QStandardItem* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QStandardItem* const * qt_gui_c_QList_QStandardItem_ptr_last_const(const QList< QStandardItem* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QStandardItem_ptr_length(const QList< QStandardItem* >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QStandardItem_ptr_mid_to_output_pos(const QList< QStandardItem* >* this_ptr, int pos, QList< QStandardItem* >* output) {
  new(output) QList< QStandardItem* >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QStandardItem_ptr_mid_to_output_pos_length(const QList< QStandardItem* >* this_ptr, int pos, int length, QList< QStandardItem* >* output) {
  new(output) QList< QStandardItem* >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QStandardItem_ptr_move(QList< QStandardItem* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QStandardItem* >* qt_gui_c_QList_QStandardItem_ptr_operator_add_assign_l(QList< QStandardItem* >* this_ptr, const QList< QStandardItem* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QStandardItem* >* qt_gui_c_QList_QStandardItem_ptr_operator_add_assign_t(QList< QStandardItem* >* this_ptr, QStandardItem* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QStandardItem_ptr_operator_add_to_output(const QList< QStandardItem* >* this_ptr, const QList< QStandardItem* >* l, QList< QStandardItem* >* output) {
  new(output) QList< QStandardItem* >(this_ptr->operator+(*l));
}

QList< QStandardItem* >* qt_gui_c_QList_QStandardItem_ptr_operator_assign(QList< QStandardItem* >* this_ptr, const QList< QStandardItem* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QStandardItem_ptr_operator_eq(const QList< QStandardItem* >* this_ptr, const QList< QStandardItem* >* l) {
  return this_ptr->operator==(*l);
}

QStandardItem** qt_gui_c_QList_QStandardItem_ptr_operator_index(QList< QStandardItem* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QStandardItem* const * qt_gui_c_QList_QStandardItem_ptr_operator_index_const(const QList< QStandardItem* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QStandardItem_ptr_operator_neq(const QList< QStandardItem* >* this_ptr, const QList< QStandardItem* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QStandardItem* >* qt_gui_c_QList_QStandardItem_ptr_operator_shl_l(QList< QStandardItem* >* this_ptr, const QList< QStandardItem* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QStandardItem* >* qt_gui_c_QList_QStandardItem_ptr_operator_shl_t(QList< QStandardItem* >* this_ptr, QStandardItem* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QStandardItem_ptr_pop_back(QList< QStandardItem* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QStandardItem_ptr_pop_front(QList< QStandardItem* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QStandardItem_ptr_prepend(QList< QStandardItem* >* this_ptr, QStandardItem* const * t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QStandardItem_ptr_push_back(QList< QStandardItem* >* this_ptr, QStandardItem* const * t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QStandardItem_ptr_push_front(QList< QStandardItem* >* this_ptr, QStandardItem* const * t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QStandardItem_ptr_removeAll(QList< QStandardItem* >* this_ptr, QStandardItem* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QStandardItem_ptr_removeAt(QList< QStandardItem* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QStandardItem_ptr_removeFirst(QList< QStandardItem* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QStandardItem_ptr_removeLast(QList< QStandardItem* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QStandardItem_ptr_removeOne(QList< QStandardItem* >* this_ptr, QStandardItem* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QStandardItem_ptr_replace(QList< QStandardItem* >* this_ptr, int i, QStandardItem* const * t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QStandardItem_ptr_reserve(QList< QStandardItem* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QStandardItem_ptr_size(const QList< QStandardItem* >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QStandardItem_ptr_startsWith(const QList< QStandardItem* >* this_ptr, QStandardItem* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QStandardItem_ptr_swap_i_j(QList< QStandardItem* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QStandardItem_ptr_swap_other(QList< QStandardItem* >* this_ptr, QList< QStandardItem* >* other) {
  this_ptr->swap(*other);
}

QStandardItem* qt_gui_c_QList_QStandardItem_ptr_takeAt(QList< QStandardItem* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QStandardItem* qt_gui_c_QList_QStandardItem_ptr_takeFirst(QList< QStandardItem* >* this_ptr) {
  return this_ptr->takeFirst();
}

QStandardItem* qt_gui_c_QList_QStandardItem_ptr_takeLast(QList< QStandardItem* >* this_ptr) {
  return this_ptr->takeLast();
}

QStandardItem* qt_gui_c_QList_QStandardItem_ptr_value_i(const QList< QStandardItem* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QStandardItem* qt_gui_c_QList_QStandardItem_ptr_value_i_defaultValue(const QList< QStandardItem* >* this_ptr, int i, QStandardItem* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_gui_c_QList_QTextBlock_append_QList_QTextBlock(QList< QTextBlock >* this_ptr, const QList< QTextBlock >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QTextBlock_append_QTextBlock(QList< QTextBlock >* this_ptr, const QTextBlock* t) {
  this_ptr->append(*t);
}

const QTextBlock* qt_gui_c_QList_QTextBlock_at(const QList< QTextBlock >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QTextBlock* qt_gui_c_QList_QTextBlock_back(QList< QTextBlock >* this_ptr) {
  return &this_ptr->back();
}

const QTextBlock* qt_gui_c_QList_QTextBlock_back_const(const QList< QTextBlock >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QTextBlock_clear(QList< QTextBlock >* this_ptr) {
  this_ptr->clear();
}

const QTextBlock* qt_gui_c_QList_QTextBlock_constFirst(const QList< QTextBlock >* this_ptr) {
  return &this_ptr->constFirst();
}

const QTextBlock* qt_gui_c_QList_QTextBlock_constLast(const QList< QTextBlock >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QTextBlock_constructor_l(const QList< QTextBlock >* l, QList< QTextBlock >* output) {
  new(output) QList< QTextBlock >(*l);
}

void qt_gui_c_QList_QTextBlock_constructor_no_args(QList< QTextBlock >* output) {
  new(output) QList< QTextBlock >();
}

bool qt_gui_c_QList_QTextBlock_contains(const QList< QTextBlock >* this_ptr, const QTextBlock* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QTextBlock_count_no_args(const QList< QTextBlock >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QTextBlock_count_t(const QList< QTextBlock >* this_ptr, const QTextBlock* t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QTextBlock_destructor(QList< QTextBlock >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QTextBlock_empty(const QList< QTextBlock >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QTextBlock_endsWith(const QList< QTextBlock >* this_ptr, const QTextBlock* t) {
  return this_ptr->endsWith(*t);
}

QTextBlock* qt_gui_c_QList_QTextBlock_first(QList< QTextBlock >* this_ptr) {
  return &this_ptr->first();
}

const QTextBlock* qt_gui_c_QList_QTextBlock_first_const(const QList< QTextBlock >* this_ptr) {
  return &this_ptr->first();
}

QTextBlock* qt_gui_c_QList_QTextBlock_front(QList< QTextBlock >* this_ptr) {
  return &this_ptr->front();
}

const QTextBlock* qt_gui_c_QList_QTextBlock_front_const(const QList< QTextBlock >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QTextBlock_indexOf_t(const QList< QTextBlock >* this_ptr, const QTextBlock* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QTextBlock_indexOf_t_from(const QList< QTextBlock >* this_ptr, const QTextBlock* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QTextBlock_insert(QList< QTextBlock >* this_ptr, int i, const QTextBlock* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QTextBlock_isEmpty(const QList< QTextBlock >* this_ptr) {
  return this_ptr->isEmpty();
}

QTextBlock* qt_gui_c_QList_QTextBlock_last(QList< QTextBlock >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QTextBlock_lastIndexOf_t(const QList< QTextBlock >* this_ptr, const QTextBlock* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QTextBlock_lastIndexOf_t_from(const QList< QTextBlock >* this_ptr, const QTextBlock* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QTextBlock* qt_gui_c_QList_QTextBlock_last_const(const QList< QTextBlock >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QTextBlock_length(const QList< QTextBlock >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QTextBlock_mid_to_output_pos(const QList< QTextBlock >* this_ptr, int pos, QList< QTextBlock >* output) {
  new(output) QList< QTextBlock >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QTextBlock_mid_to_output_pos_length(const QList< QTextBlock >* this_ptr, int pos, int length, QList< QTextBlock >* output) {
  new(output) QList< QTextBlock >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QTextBlock_move(QList< QTextBlock >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QTextBlock >* qt_gui_c_QList_QTextBlock_operator_add_assign_l(QList< QTextBlock >* this_ptr, const QList< QTextBlock >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QTextBlock >* qt_gui_c_QList_QTextBlock_operator_add_assign_t(QList< QTextBlock >* this_ptr, const QTextBlock* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QTextBlock_operator_add_to_output(const QList< QTextBlock >* this_ptr, const QList< QTextBlock >* l, QList< QTextBlock >* output) {
  new(output) QList< QTextBlock >(this_ptr->operator+(*l));
}

QList< QTextBlock >* qt_gui_c_QList_QTextBlock_operator_assign(QList< QTextBlock >* this_ptr, const QList< QTextBlock >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QTextBlock_operator_eq(const QList< QTextBlock >* this_ptr, const QList< QTextBlock >* l) {
  return this_ptr->operator==(*l);
}

QTextBlock* qt_gui_c_QList_QTextBlock_operator_index(QList< QTextBlock >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QTextBlock* qt_gui_c_QList_QTextBlock_operator_index_const(const QList< QTextBlock >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QTextBlock_operator_neq(const QList< QTextBlock >* this_ptr, const QList< QTextBlock >* l) {
  return this_ptr->operator!=(*l);
}

QList< QTextBlock >* qt_gui_c_QList_QTextBlock_operator_shl_l(QList< QTextBlock >* this_ptr, const QList< QTextBlock >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QTextBlock >* qt_gui_c_QList_QTextBlock_operator_shl_t(QList< QTextBlock >* this_ptr, const QTextBlock* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QTextBlock_pop_back(QList< QTextBlock >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QTextBlock_pop_front(QList< QTextBlock >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QTextBlock_prepend(QList< QTextBlock >* this_ptr, const QTextBlock* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QTextBlock_push_back(QList< QTextBlock >* this_ptr, const QTextBlock* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QTextBlock_push_front(QList< QTextBlock >* this_ptr, const QTextBlock* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QTextBlock_removeAll(QList< QTextBlock >* this_ptr, const QTextBlock* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QTextBlock_removeAt(QList< QTextBlock >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QTextBlock_removeFirst(QList< QTextBlock >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QTextBlock_removeLast(QList< QTextBlock >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QTextBlock_removeOne(QList< QTextBlock >* this_ptr, const QTextBlock* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QTextBlock_replace(QList< QTextBlock >* this_ptr, int i, const QTextBlock* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QTextBlock_reserve(QList< QTextBlock >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QTextBlock_size(const QList< QTextBlock >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QTextBlock_startsWith(const QList< QTextBlock >* this_ptr, const QTextBlock* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QTextBlock_swap_i_j(QList< QTextBlock >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QTextBlock_swap_other(QList< QTextBlock >* this_ptr, QList< QTextBlock >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QList_QTextBlock_takeAt_to_output(QList< QTextBlock >* this_ptr, int i, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->takeAt(i));
}

void qt_gui_c_QList_QTextBlock_takeFirst_to_output(QList< QTextBlock >* this_ptr, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->takeFirst());
}

void qt_gui_c_QList_QTextBlock_takeLast_to_output(QList< QTextBlock >* this_ptr, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->takeLast());
}

void qt_gui_c_QList_QTextBlock_value_to_output_i(const QList< QTextBlock >* this_ptr, int i, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->value(i));
}

void qt_gui_c_QList_QTextBlock_value_to_output_i_defaultValue(const QList< QTextBlock >* this_ptr, int i, const QTextBlock* defaultValue, QTextBlock* output) {
  new(output) QTextBlock(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QList_QTextFrame_ptr_append_QList_QTextFrame_ptr(QList< QTextFrame* >* this_ptr, const QList< QTextFrame* >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QTextFrame_ptr_append_QTextFrame(QList< QTextFrame* >* this_ptr, QTextFrame* const * t) {
  this_ptr->append(*t);
}

QTextFrame* const * qt_gui_c_QList_QTextFrame_ptr_at(const QList< QTextFrame* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QTextFrame** qt_gui_c_QList_QTextFrame_ptr_back(QList< QTextFrame* >* this_ptr) {
  return &this_ptr->back();
}

QTextFrame* const * qt_gui_c_QList_QTextFrame_ptr_back_const(const QList< QTextFrame* >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QTextFrame_ptr_clear(QList< QTextFrame* >* this_ptr) {
  this_ptr->clear();
}

QTextFrame* const * qt_gui_c_QList_QTextFrame_ptr_constFirst(const QList< QTextFrame* >* this_ptr) {
  return &this_ptr->constFirst();
}

QTextFrame* const * qt_gui_c_QList_QTextFrame_ptr_constLast(const QList< QTextFrame* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QTextFrame_ptr_constructor_l(const QList< QTextFrame* >* l, QList< QTextFrame* >* output) {
  new(output) QList< QTextFrame* >(*l);
}

void qt_gui_c_QList_QTextFrame_ptr_constructor_no_args(QList< QTextFrame* >* output) {
  new(output) QList< QTextFrame* >();
}

bool qt_gui_c_QList_QTextFrame_ptr_contains(const QList< QTextFrame* >* this_ptr, QTextFrame* const * t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QTextFrame_ptr_count_no_args(const QList< QTextFrame* >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QTextFrame_ptr_count_t(const QList< QTextFrame* >* this_ptr, QTextFrame* const * t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QTextFrame_ptr_destructor(QList< QTextFrame* >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QTextFrame_ptr_empty(const QList< QTextFrame* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QTextFrame_ptr_endsWith(const QList< QTextFrame* >* this_ptr, QTextFrame* const * t) {
  return this_ptr->endsWith(*t);
}

QTextFrame** qt_gui_c_QList_QTextFrame_ptr_first(QList< QTextFrame* >* this_ptr) {
  return &this_ptr->first();
}

QTextFrame* const * qt_gui_c_QList_QTextFrame_ptr_first_const(const QList< QTextFrame* >* this_ptr) {
  return &this_ptr->first();
}

QTextFrame** qt_gui_c_QList_QTextFrame_ptr_front(QList< QTextFrame* >* this_ptr) {
  return &this_ptr->front();
}

QTextFrame* const * qt_gui_c_QList_QTextFrame_ptr_front_const(const QList< QTextFrame* >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QTextFrame_ptr_indexOf_t(const QList< QTextFrame* >* this_ptr, QTextFrame* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QTextFrame_ptr_indexOf_t_from(const QList< QTextFrame* >* this_ptr, QTextFrame* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QTextFrame_ptr_insert(QList< QTextFrame* >* this_ptr, int i, QTextFrame* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QTextFrame_ptr_isEmpty(const QList< QTextFrame* >* this_ptr) {
  return this_ptr->isEmpty();
}

QTextFrame** qt_gui_c_QList_QTextFrame_ptr_last(QList< QTextFrame* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QTextFrame_ptr_lastIndexOf_t(const QList< QTextFrame* >* this_ptr, QTextFrame* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QTextFrame_ptr_lastIndexOf_t_from(const QList< QTextFrame* >* this_ptr, QTextFrame* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QTextFrame* const * qt_gui_c_QList_QTextFrame_ptr_last_const(const QList< QTextFrame* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QTextFrame_ptr_length(const QList< QTextFrame* >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QTextFrame_ptr_mid_to_output_pos(const QList< QTextFrame* >* this_ptr, int pos, QList< QTextFrame* >* output) {
  new(output) QList< QTextFrame* >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QTextFrame_ptr_mid_to_output_pos_length(const QList< QTextFrame* >* this_ptr, int pos, int length, QList< QTextFrame* >* output) {
  new(output) QList< QTextFrame* >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QTextFrame_ptr_move(QList< QTextFrame* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QTextFrame* >* qt_gui_c_QList_QTextFrame_ptr_operator_add_assign_l(QList< QTextFrame* >* this_ptr, const QList< QTextFrame* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QTextFrame* >* qt_gui_c_QList_QTextFrame_ptr_operator_add_assign_t(QList< QTextFrame* >* this_ptr, QTextFrame* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QTextFrame_ptr_operator_add_to_output(const QList< QTextFrame* >* this_ptr, const QList< QTextFrame* >* l, QList< QTextFrame* >* output) {
  new(output) QList< QTextFrame* >(this_ptr->operator+(*l));
}

QList< QTextFrame* >* qt_gui_c_QList_QTextFrame_ptr_operator_assign(QList< QTextFrame* >* this_ptr, const QList< QTextFrame* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QTextFrame_ptr_operator_eq(const QList< QTextFrame* >* this_ptr, const QList< QTextFrame* >* l) {
  return this_ptr->operator==(*l);
}

QTextFrame** qt_gui_c_QList_QTextFrame_ptr_operator_index(QList< QTextFrame* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QTextFrame* const * qt_gui_c_QList_QTextFrame_ptr_operator_index_const(const QList< QTextFrame* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QTextFrame_ptr_operator_neq(const QList< QTextFrame* >* this_ptr, const QList< QTextFrame* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QTextFrame* >* qt_gui_c_QList_QTextFrame_ptr_operator_shl_l(QList< QTextFrame* >* this_ptr, const QList< QTextFrame* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QTextFrame* >* qt_gui_c_QList_QTextFrame_ptr_operator_shl_t(QList< QTextFrame* >* this_ptr, QTextFrame* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QTextFrame_ptr_pop_back(QList< QTextFrame* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QTextFrame_ptr_pop_front(QList< QTextFrame* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QTextFrame_ptr_prepend(QList< QTextFrame* >* this_ptr, QTextFrame* const * t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QTextFrame_ptr_push_back(QList< QTextFrame* >* this_ptr, QTextFrame* const * t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QTextFrame_ptr_push_front(QList< QTextFrame* >* this_ptr, QTextFrame* const * t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QTextFrame_ptr_removeAll(QList< QTextFrame* >* this_ptr, QTextFrame* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QTextFrame_ptr_removeAt(QList< QTextFrame* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QTextFrame_ptr_removeFirst(QList< QTextFrame* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QTextFrame_ptr_removeLast(QList< QTextFrame* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QTextFrame_ptr_removeOne(QList< QTextFrame* >* this_ptr, QTextFrame* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QTextFrame_ptr_replace(QList< QTextFrame* >* this_ptr, int i, QTextFrame* const * t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QTextFrame_ptr_reserve(QList< QTextFrame* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QTextFrame_ptr_size(const QList< QTextFrame* >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QTextFrame_ptr_startsWith(const QList< QTextFrame* >* this_ptr, QTextFrame* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QTextFrame_ptr_swap_i_j(QList< QTextFrame* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QTextFrame_ptr_swap_other(QList< QTextFrame* >* this_ptr, QList< QTextFrame* >* other) {
  this_ptr->swap(*other);
}

QTextFrame* qt_gui_c_QList_QTextFrame_ptr_takeAt(QList< QTextFrame* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QTextFrame* qt_gui_c_QList_QTextFrame_ptr_takeFirst(QList< QTextFrame* >* this_ptr) {
  return this_ptr->takeFirst();
}

QTextFrame* qt_gui_c_QList_QTextFrame_ptr_takeLast(QList< QTextFrame* >* this_ptr) {
  return this_ptr->takeLast();
}

QTextFrame* qt_gui_c_QList_QTextFrame_ptr_value_i(const QList< QTextFrame* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QTextFrame* qt_gui_c_QList_QTextFrame_ptr_value_i_defaultValue(const QList< QTextFrame* >* this_ptr, int i, QTextFrame* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_gui_c_QList_QTextLayout_FormatRange_append_QList_QTextLayout_FormatRange(QList< QTextLayout::FormatRange >* this_ptr, const QList< QTextLayout::FormatRange >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QTextLayout_FormatRange_append_QTextLayout_FormatRange(QList< QTextLayout::FormatRange >* this_ptr, const QTextLayout::FormatRange* t) {
  this_ptr->append(*t);
}

const QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_at(const QList< QTextLayout::FormatRange >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_back(QList< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->back();
}

const QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_back_const(const QList< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QTextLayout_FormatRange_clear(QList< QTextLayout::FormatRange >* this_ptr) {
  this_ptr->clear();
}

const QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_constFirst(const QList< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->constFirst();
}

const QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_constLast(const QList< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QTextLayout_FormatRange_constructor_l(const QList< QTextLayout::FormatRange >* l, QList< QTextLayout::FormatRange >* output) {
  new(output) QList< QTextLayout::FormatRange >(*l);
}

void qt_gui_c_QList_QTextLayout_FormatRange_constructor_no_args(QList< QTextLayout::FormatRange >* output) {
  new(output) QList< QTextLayout::FormatRange >();
}

int qt_gui_c_QList_QTextLayout_FormatRange_count(const QList< QTextLayout::FormatRange >* this_ptr) {
  return this_ptr->count();
}

void qt_gui_c_QList_QTextLayout_FormatRange_destructor(QList< QTextLayout::FormatRange >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QTextLayout_FormatRange_empty(const QList< QTextLayout::FormatRange >* this_ptr) {
  return this_ptr->empty();
}

QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_first(QList< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->first();
}

const QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_first_const(const QList< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->first();
}

void qt_gui_c_QList_QTextLayout_FormatRange_fromVector_to_output(const QVector< QTextLayout::FormatRange >* vector, QList< QTextLayout::FormatRange >* output) {
  new(output) QList< QTextLayout::FormatRange >(QList< QTextLayout::FormatRange >::fromVector(*vector));
}

QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_front(QList< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->front();
}

const QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_front_const(const QList< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->front();
}

void qt_gui_c_QList_QTextLayout_FormatRange_insert(QList< QTextLayout::FormatRange >* this_ptr, int i, const QTextLayout::FormatRange* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QTextLayout_FormatRange_isEmpty(const QList< QTextLayout::FormatRange >* this_ptr) {
  return this_ptr->isEmpty();
}

QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_last(QList< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->last();
}

const QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_last_const(const QList< QTextLayout::FormatRange >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QTextLayout_FormatRange_length(const QList< QTextLayout::FormatRange >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QTextLayout_FormatRange_mid_to_output_pos(const QList< QTextLayout::FormatRange >* this_ptr, int pos, QList< QTextLayout::FormatRange >* output) {
  new(output) QList< QTextLayout::FormatRange >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QTextLayout_FormatRange_mid_to_output_pos_length(const QList< QTextLayout::FormatRange >* this_ptr, int pos, int length, QList< QTextLayout::FormatRange >* output) {
  new(output) QList< QTextLayout::FormatRange >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QTextLayout_FormatRange_move(QList< QTextLayout::FormatRange >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QTextLayout::FormatRange >* qt_gui_c_QList_QTextLayout_FormatRange_operator_add_assign_l(QList< QTextLayout::FormatRange >* this_ptr, const QList< QTextLayout::FormatRange >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QTextLayout::FormatRange >* qt_gui_c_QList_QTextLayout_FormatRange_operator_add_assign_t(QList< QTextLayout::FormatRange >* this_ptr, const QTextLayout::FormatRange* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QTextLayout_FormatRange_operator_add_to_output(const QList< QTextLayout::FormatRange >* this_ptr, const QList< QTextLayout::FormatRange >* l, QList< QTextLayout::FormatRange >* output) {
  new(output) QList< QTextLayout::FormatRange >(this_ptr->operator+(*l));
}

QList< QTextLayout::FormatRange >* qt_gui_c_QList_QTextLayout_FormatRange_operator_assign(QList< QTextLayout::FormatRange >* this_ptr, const QList< QTextLayout::FormatRange >* l) {
  return &this_ptr->operator=(*l);
}

QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_operator_index(QList< QTextLayout::FormatRange >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_operator_index_const(const QList< QTextLayout::FormatRange >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QList< QTextLayout::FormatRange >* qt_gui_c_QList_QTextLayout_FormatRange_operator_shl_l(QList< QTextLayout::FormatRange >* this_ptr, const QList< QTextLayout::FormatRange >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QTextLayout::FormatRange >* qt_gui_c_QList_QTextLayout_FormatRange_operator_shl_t(QList< QTextLayout::FormatRange >* this_ptr, const QTextLayout::FormatRange* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QTextLayout_FormatRange_pop_back(QList< QTextLayout::FormatRange >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QTextLayout_FormatRange_pop_front(QList< QTextLayout::FormatRange >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QTextLayout_FormatRange_prepend(QList< QTextLayout::FormatRange >* this_ptr, const QTextLayout::FormatRange* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QTextLayout_FormatRange_push_back(QList< QTextLayout::FormatRange >* this_ptr, const QTextLayout::FormatRange* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QTextLayout_FormatRange_push_front(QList< QTextLayout::FormatRange >* this_ptr, const QTextLayout::FormatRange* t) {
  this_ptr->push_front(*t);
}

void qt_gui_c_QList_QTextLayout_FormatRange_removeAt(QList< QTextLayout::FormatRange >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QTextLayout_FormatRange_removeFirst(QList< QTextLayout::FormatRange >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QTextLayout_FormatRange_removeLast(QList< QTextLayout::FormatRange >* this_ptr) {
  this_ptr->removeLast();
}

void qt_gui_c_QList_QTextLayout_FormatRange_replace(QList< QTextLayout::FormatRange >* this_ptr, int i, const QTextLayout::FormatRange* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QTextLayout_FormatRange_reserve(QList< QTextLayout::FormatRange >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QTextLayout_FormatRange_size(const QList< QTextLayout::FormatRange >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QList_QTextLayout_FormatRange_swap_i_j(QList< QTextLayout::FormatRange >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QTextLayout_FormatRange_swap_other(QList< QTextLayout::FormatRange >* this_ptr, QList< QTextLayout::FormatRange >* other) {
  this_ptr->swap(*other);
}

QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_takeAt_as_ptr(QList< QTextLayout::FormatRange >* this_ptr, int i) {
  return new QTextLayout::FormatRange(this_ptr->takeAt(i));
}

QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_takeFirst_as_ptr(QList< QTextLayout::FormatRange >* this_ptr) {
  return new QTextLayout::FormatRange(this_ptr->takeFirst());
}

QTextLayout::FormatRange* qt_gui_c_QList_QTextLayout_FormatRange_takeLast_as_ptr(QList< QTextLayout::FormatRange >* this_ptr) {
  return new QTextLayout::FormatRange(this_ptr->takeLast());
}

void qt_gui_c_QList_QTextOption_Tab_append_QList_QTextOption_Tab(QList< QTextOption::Tab >* this_ptr, const QList< QTextOption::Tab >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QTextOption_Tab_append_QTextOption_Tab(QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t) {
  this_ptr->append(*t);
}

const QTextOption::Tab* qt_gui_c_QList_QTextOption_Tab_at(const QList< QTextOption::Tab >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QTextOption::Tab* qt_gui_c_QList_QTextOption_Tab_back(QList< QTextOption::Tab >* this_ptr) {
  return &this_ptr->back();
}

const QTextOption::Tab* qt_gui_c_QList_QTextOption_Tab_back_const(const QList< QTextOption::Tab >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QTextOption_Tab_clear(QList< QTextOption::Tab >* this_ptr) {
  this_ptr->clear();
}

const QTextOption::Tab* qt_gui_c_QList_QTextOption_Tab_constFirst(const QList< QTextOption::Tab >* this_ptr) {
  return &this_ptr->constFirst();
}

const QTextOption::Tab* qt_gui_c_QList_QTextOption_Tab_constLast(const QList< QTextOption::Tab >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QTextOption_Tab_constructor_l(const QList< QTextOption::Tab >* l, QList< QTextOption::Tab >* output) {
  new(output) QList< QTextOption::Tab >(*l);
}

void qt_gui_c_QList_QTextOption_Tab_constructor_no_args(QList< QTextOption::Tab >* output) {
  new(output) QList< QTextOption::Tab >();
}

bool qt_gui_c_QList_QTextOption_Tab_contains(const QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QTextOption_Tab_count_no_args(const QList< QTextOption::Tab >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QTextOption_Tab_count_t(const QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QTextOption_Tab_destructor(QList< QTextOption::Tab >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QTextOption_Tab_empty(const QList< QTextOption::Tab >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QTextOption_Tab_endsWith(const QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t) {
  return this_ptr->endsWith(*t);
}

QTextOption::Tab* qt_gui_c_QList_QTextOption_Tab_first(QList< QTextOption::Tab >* this_ptr) {
  return &this_ptr->first();
}

const QTextOption::Tab* qt_gui_c_QList_QTextOption_Tab_first_const(const QList< QTextOption::Tab >* this_ptr) {
  return &this_ptr->first();
}

QTextOption::Tab* qt_gui_c_QList_QTextOption_Tab_front(QList< QTextOption::Tab >* this_ptr) {
  return &this_ptr->front();
}

const QTextOption::Tab* qt_gui_c_QList_QTextOption_Tab_front_const(const QList< QTextOption::Tab >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QTextOption_Tab_indexOf_t(const QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QTextOption_Tab_indexOf_t_from(const QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QTextOption_Tab_insert(QList< QTextOption::Tab >* this_ptr, int i, const QTextOption::Tab* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QTextOption_Tab_isEmpty(const QList< QTextOption::Tab >* this_ptr) {
  return this_ptr->isEmpty();
}

QTextOption::Tab* qt_gui_c_QList_QTextOption_Tab_last(QList< QTextOption::Tab >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QTextOption_Tab_lastIndexOf_t(const QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QTextOption_Tab_lastIndexOf_t_from(const QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QTextOption::Tab* qt_gui_c_QList_QTextOption_Tab_last_const(const QList< QTextOption::Tab >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QTextOption_Tab_length(const QList< QTextOption::Tab >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QTextOption_Tab_mid_to_output_pos(const QList< QTextOption::Tab >* this_ptr, int pos, QList< QTextOption::Tab >* output) {
  new(output) QList< QTextOption::Tab >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QTextOption_Tab_mid_to_output_pos_length(const QList< QTextOption::Tab >* this_ptr, int pos, int length, QList< QTextOption::Tab >* output) {
  new(output) QList< QTextOption::Tab >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QTextOption_Tab_move(QList< QTextOption::Tab >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QTextOption::Tab >* qt_gui_c_QList_QTextOption_Tab_operator_add_assign_l(QList< QTextOption::Tab >* this_ptr, const QList< QTextOption::Tab >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QTextOption::Tab >* qt_gui_c_QList_QTextOption_Tab_operator_add_assign_t(QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QTextOption_Tab_operator_add_to_output(const QList< QTextOption::Tab >* this_ptr, const QList< QTextOption::Tab >* l, QList< QTextOption::Tab >* output) {
  new(output) QList< QTextOption::Tab >(this_ptr->operator+(*l));
}

QList< QTextOption::Tab >* qt_gui_c_QList_QTextOption_Tab_operator_assign(QList< QTextOption::Tab >* this_ptr, const QList< QTextOption::Tab >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QTextOption_Tab_operator_eq(const QList< QTextOption::Tab >* this_ptr, const QList< QTextOption::Tab >* l) {
  return this_ptr->operator==(*l);
}

QTextOption::Tab* qt_gui_c_QList_QTextOption_Tab_operator_index(QList< QTextOption::Tab >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QTextOption::Tab* qt_gui_c_QList_QTextOption_Tab_operator_index_const(const QList< QTextOption::Tab >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QTextOption_Tab_operator_neq(const QList< QTextOption::Tab >* this_ptr, const QList< QTextOption::Tab >* l) {
  return this_ptr->operator!=(*l);
}

QList< QTextOption::Tab >* qt_gui_c_QList_QTextOption_Tab_operator_shl_l(QList< QTextOption::Tab >* this_ptr, const QList< QTextOption::Tab >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QTextOption::Tab >* qt_gui_c_QList_QTextOption_Tab_operator_shl_t(QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QTextOption_Tab_pop_back(QList< QTextOption::Tab >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QTextOption_Tab_pop_front(QList< QTextOption::Tab >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QTextOption_Tab_prepend(QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QTextOption_Tab_push_back(QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QTextOption_Tab_push_front(QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QTextOption_Tab_removeAll(QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QTextOption_Tab_removeAt(QList< QTextOption::Tab >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QTextOption_Tab_removeFirst(QList< QTextOption::Tab >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QTextOption_Tab_removeLast(QList< QTextOption::Tab >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QTextOption_Tab_removeOne(QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QTextOption_Tab_replace(QList< QTextOption::Tab >* this_ptr, int i, const QTextOption::Tab* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QTextOption_Tab_reserve(QList< QTextOption::Tab >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QTextOption_Tab_size(const QList< QTextOption::Tab >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QTextOption_Tab_startsWith(const QList< QTextOption::Tab >* this_ptr, const QTextOption::Tab* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QTextOption_Tab_swap_i_j(QList< QTextOption::Tab >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QTextOption_Tab_swap_other(QList< QTextOption::Tab >* this_ptr, QList< QTextOption::Tab >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QList_QTextOption_Tab_takeAt_to_output(QList< QTextOption::Tab >* this_ptr, int i, QTextOption::Tab* output) {
  new(output) QTextOption::Tab(this_ptr->takeAt(i));
}

void qt_gui_c_QList_QTextOption_Tab_takeFirst_to_output(QList< QTextOption::Tab >* this_ptr, QTextOption::Tab* output) {
  new(output) QTextOption::Tab(this_ptr->takeFirst());
}

void qt_gui_c_QList_QTextOption_Tab_takeLast_to_output(QList< QTextOption::Tab >* this_ptr, QTextOption::Tab* output) {
  new(output) QTextOption::Tab(this_ptr->takeLast());
}

void qt_gui_c_QList_QTextOption_Tab_value_to_output_i(const QList< QTextOption::Tab >* this_ptr, int i, QTextOption::Tab* output) {
  new(output) QTextOption::Tab(this_ptr->value(i));
}

void qt_gui_c_QList_QTextOption_Tab_value_to_output_i_defaultValue(const QList< QTextOption::Tab >* this_ptr, int i, const QTextOption::Tab* defaultValue, QTextOption::Tab* output) {
  new(output) QTextOption::Tab(this_ptr->value(i, *defaultValue));
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_append_QList_QTouchEvent_TouchPoint(QList< QTouchEvent::TouchPoint >* this_ptr, const QList< QTouchEvent::TouchPoint >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_append_QTouchEvent_TouchPoint(QList< QTouchEvent::TouchPoint >* this_ptr, const QTouchEvent::TouchPoint* t) {
  this_ptr->append(*t);
}

const QTouchEvent::TouchPoint* qt_gui_c_QList_QTouchEvent_TouchPoint_at(const QList< QTouchEvent::TouchPoint >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QTouchEvent::TouchPoint* qt_gui_c_QList_QTouchEvent_TouchPoint_back(QList< QTouchEvent::TouchPoint >* this_ptr) {
  return &this_ptr->back();
}

const QTouchEvent::TouchPoint* qt_gui_c_QList_QTouchEvent_TouchPoint_back_const(const QList< QTouchEvent::TouchPoint >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_clear(QList< QTouchEvent::TouchPoint >* this_ptr) {
  this_ptr->clear();
}

const QTouchEvent::TouchPoint* qt_gui_c_QList_QTouchEvent_TouchPoint_constFirst(const QList< QTouchEvent::TouchPoint >* this_ptr) {
  return &this_ptr->constFirst();
}

const QTouchEvent::TouchPoint* qt_gui_c_QList_QTouchEvent_TouchPoint_constLast(const QList< QTouchEvent::TouchPoint >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_constructor_l(const QList< QTouchEvent::TouchPoint >* l, QList< QTouchEvent::TouchPoint >* output) {
  new(output) QList< QTouchEvent::TouchPoint >(*l);
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_constructor_no_args(QList< QTouchEvent::TouchPoint >* output) {
  new(output) QList< QTouchEvent::TouchPoint >();
}

int qt_gui_c_QList_QTouchEvent_TouchPoint_count(const QList< QTouchEvent::TouchPoint >* this_ptr) {
  return this_ptr->count();
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_destructor(QList< QTouchEvent::TouchPoint >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QTouchEvent_TouchPoint_empty(const QList< QTouchEvent::TouchPoint >* this_ptr) {
  return this_ptr->empty();
}

QTouchEvent::TouchPoint* qt_gui_c_QList_QTouchEvent_TouchPoint_first(QList< QTouchEvent::TouchPoint >* this_ptr) {
  return &this_ptr->first();
}

const QTouchEvent::TouchPoint* qt_gui_c_QList_QTouchEvent_TouchPoint_first_const(const QList< QTouchEvent::TouchPoint >* this_ptr) {
  return &this_ptr->first();
}

QTouchEvent::TouchPoint* qt_gui_c_QList_QTouchEvent_TouchPoint_front(QList< QTouchEvent::TouchPoint >* this_ptr) {
  return &this_ptr->front();
}

const QTouchEvent::TouchPoint* qt_gui_c_QList_QTouchEvent_TouchPoint_front_const(const QList< QTouchEvent::TouchPoint >* this_ptr) {
  return &this_ptr->front();
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_insert(QList< QTouchEvent::TouchPoint >* this_ptr, int i, const QTouchEvent::TouchPoint* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QTouchEvent_TouchPoint_isEmpty(const QList< QTouchEvent::TouchPoint >* this_ptr) {
  return this_ptr->isEmpty();
}

QTouchEvent::TouchPoint* qt_gui_c_QList_QTouchEvent_TouchPoint_last(QList< QTouchEvent::TouchPoint >* this_ptr) {
  return &this_ptr->last();
}

const QTouchEvent::TouchPoint* qt_gui_c_QList_QTouchEvent_TouchPoint_last_const(const QList< QTouchEvent::TouchPoint >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QTouchEvent_TouchPoint_length(const QList< QTouchEvent::TouchPoint >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_mid_to_output_pos(const QList< QTouchEvent::TouchPoint >* this_ptr, int pos, QList< QTouchEvent::TouchPoint >* output) {
  new(output) QList< QTouchEvent::TouchPoint >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_mid_to_output_pos_length(const QList< QTouchEvent::TouchPoint >* this_ptr, int pos, int length, QList< QTouchEvent::TouchPoint >* output) {
  new(output) QList< QTouchEvent::TouchPoint >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_move(QList< QTouchEvent::TouchPoint >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QTouchEvent::TouchPoint >* qt_gui_c_QList_QTouchEvent_TouchPoint_operator_add_assign_l(QList< QTouchEvent::TouchPoint >* this_ptr, const QList< QTouchEvent::TouchPoint >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QTouchEvent::TouchPoint >* qt_gui_c_QList_QTouchEvent_TouchPoint_operator_add_assign_t(QList< QTouchEvent::TouchPoint >* this_ptr, const QTouchEvent::TouchPoint* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_operator_add_to_output(const QList< QTouchEvent::TouchPoint >* this_ptr, const QList< QTouchEvent::TouchPoint >* l, QList< QTouchEvent::TouchPoint >* output) {
  new(output) QList< QTouchEvent::TouchPoint >(this_ptr->operator+(*l));
}

QList< QTouchEvent::TouchPoint >* qt_gui_c_QList_QTouchEvent_TouchPoint_operator_assign(QList< QTouchEvent::TouchPoint >* this_ptr, const QList< QTouchEvent::TouchPoint >* l) {
  return &this_ptr->operator=(*l);
}

QTouchEvent::TouchPoint* qt_gui_c_QList_QTouchEvent_TouchPoint_operator_index(QList< QTouchEvent::TouchPoint >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QTouchEvent::TouchPoint* qt_gui_c_QList_QTouchEvent_TouchPoint_operator_index_const(const QList< QTouchEvent::TouchPoint >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QList< QTouchEvent::TouchPoint >* qt_gui_c_QList_QTouchEvent_TouchPoint_operator_shl_l(QList< QTouchEvent::TouchPoint >* this_ptr, const QList< QTouchEvent::TouchPoint >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QTouchEvent::TouchPoint >* qt_gui_c_QList_QTouchEvent_TouchPoint_operator_shl_t(QList< QTouchEvent::TouchPoint >* this_ptr, const QTouchEvent::TouchPoint* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_pop_back(QList< QTouchEvent::TouchPoint >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_pop_front(QList< QTouchEvent::TouchPoint >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_prepend(QList< QTouchEvent::TouchPoint >* this_ptr, const QTouchEvent::TouchPoint* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_push_back(QList< QTouchEvent::TouchPoint >* this_ptr, const QTouchEvent::TouchPoint* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_push_front(QList< QTouchEvent::TouchPoint >* this_ptr, const QTouchEvent::TouchPoint* t) {
  this_ptr->push_front(*t);
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_removeAt(QList< QTouchEvent::TouchPoint >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_removeFirst(QList< QTouchEvent::TouchPoint >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_removeLast(QList< QTouchEvent::TouchPoint >* this_ptr) {
  this_ptr->removeLast();
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_replace(QList< QTouchEvent::TouchPoint >* this_ptr, int i, const QTouchEvent::TouchPoint* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_reserve(QList< QTouchEvent::TouchPoint >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QTouchEvent_TouchPoint_size(const QList< QTouchEvent::TouchPoint >* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_swap_i_j(QList< QTouchEvent::TouchPoint >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_swap_other(QList< QTouchEvent::TouchPoint >* this_ptr, QList< QTouchEvent::TouchPoint >* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_takeAt_to_output(QList< QTouchEvent::TouchPoint >* this_ptr, int i, QTouchEvent::TouchPoint* output) {
  new(output) QTouchEvent::TouchPoint(this_ptr->takeAt(i));
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_takeFirst_to_output(QList< QTouchEvent::TouchPoint >* this_ptr, QTouchEvent::TouchPoint* output) {
  new(output) QTouchEvent::TouchPoint(this_ptr->takeFirst());
}

void qt_gui_c_QList_QTouchEvent_TouchPoint_takeLast_to_output(QList< QTouchEvent::TouchPoint >* this_ptr, QTouchEvent::TouchPoint* output) {
  new(output) QTouchEvent::TouchPoint(this_ptr->takeLast());
}

void qt_gui_c_QList_QWindow_ptr_append_QList_QWindow_ptr(QList< QWindow* >* this_ptr, const QList< QWindow* >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_QWindow_ptr_append_QWindow(QList< QWindow* >* this_ptr, QWindow* const * t) {
  this_ptr->append(*t);
}

QWindow* const * qt_gui_c_QList_QWindow_ptr_at(const QList< QWindow* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QWindow** qt_gui_c_QList_QWindow_ptr_back(QList< QWindow* >* this_ptr) {
  return &this_ptr->back();
}

QWindow* const * qt_gui_c_QList_QWindow_ptr_back_const(const QList< QWindow* >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_QWindow_ptr_clear(QList< QWindow* >* this_ptr) {
  this_ptr->clear();
}

QWindow* const * qt_gui_c_QList_QWindow_ptr_constFirst(const QList< QWindow* >* this_ptr) {
  return &this_ptr->constFirst();
}

QWindow* const * qt_gui_c_QList_QWindow_ptr_constLast(const QList< QWindow* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_QWindow_ptr_constructor_l(const QList< QWindow* >* l, QList< QWindow* >* output) {
  new(output) QList< QWindow* >(*l);
}

void qt_gui_c_QList_QWindow_ptr_constructor_no_args(QList< QWindow* >* output) {
  new(output) QList< QWindow* >();
}

bool qt_gui_c_QList_QWindow_ptr_contains(const QList< QWindow* >* this_ptr, QWindow* const * t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_QWindow_ptr_count_no_args(const QList< QWindow* >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_QWindow_ptr_count_t(const QList< QWindow* >* this_ptr, QWindow* const * t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_QWindow_ptr_destructor(QList< QWindow* >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_QWindow_ptr_empty(const QList< QWindow* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_QWindow_ptr_endsWith(const QList< QWindow* >* this_ptr, QWindow* const * t) {
  return this_ptr->endsWith(*t);
}

QWindow** qt_gui_c_QList_QWindow_ptr_first(QList< QWindow* >* this_ptr) {
  return &this_ptr->first();
}

QWindow* const * qt_gui_c_QList_QWindow_ptr_first_const(const QList< QWindow* >* this_ptr) {
  return &this_ptr->first();
}

QWindow** qt_gui_c_QList_QWindow_ptr_front(QList< QWindow* >* this_ptr) {
  return &this_ptr->front();
}

QWindow* const * qt_gui_c_QList_QWindow_ptr_front_const(const QList< QWindow* >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_QWindow_ptr_indexOf_t(const QList< QWindow* >* this_ptr, QWindow* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_QWindow_ptr_indexOf_t_from(const QList< QWindow* >* this_ptr, QWindow* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_QWindow_ptr_insert(QList< QWindow* >* this_ptr, int i, QWindow* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_QWindow_ptr_isEmpty(const QList< QWindow* >* this_ptr) {
  return this_ptr->isEmpty();
}

QWindow** qt_gui_c_QList_QWindow_ptr_last(QList< QWindow* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QWindow_ptr_lastIndexOf_t(const QList< QWindow* >* this_ptr, QWindow* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_QWindow_ptr_lastIndexOf_t_from(const QList< QWindow* >* this_ptr, QWindow* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QWindow* const * qt_gui_c_QList_QWindow_ptr_last_const(const QList< QWindow* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_QWindow_ptr_length(const QList< QWindow* >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_QWindow_ptr_mid_to_output_pos(const QList< QWindow* >* this_ptr, int pos, QList< QWindow* >* output) {
  new(output) QList< QWindow* >(this_ptr->mid(pos));
}

void qt_gui_c_QList_QWindow_ptr_mid_to_output_pos_length(const QList< QWindow* >* this_ptr, int pos, int length, QList< QWindow* >* output) {
  new(output) QList< QWindow* >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_QWindow_ptr_move(QList< QWindow* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QWindow* >* qt_gui_c_QList_QWindow_ptr_operator_add_assign_l(QList< QWindow* >* this_ptr, const QList< QWindow* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QWindow* >* qt_gui_c_QList_QWindow_ptr_operator_add_assign_t(QList< QWindow* >* this_ptr, QWindow* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_QWindow_ptr_operator_add_to_output(const QList< QWindow* >* this_ptr, const QList< QWindow* >* l, QList< QWindow* >* output) {
  new(output) QList< QWindow* >(this_ptr->operator+(*l));
}

QList< QWindow* >* qt_gui_c_QList_QWindow_ptr_operator_assign(QList< QWindow* >* this_ptr, const QList< QWindow* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_QWindow_ptr_operator_eq(const QList< QWindow* >* this_ptr, const QList< QWindow* >* l) {
  return this_ptr->operator==(*l);
}

QWindow** qt_gui_c_QList_QWindow_ptr_operator_index(QList< QWindow* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QWindow* const * qt_gui_c_QList_QWindow_ptr_operator_index_const(const QList< QWindow* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_QWindow_ptr_operator_neq(const QList< QWindow* >* this_ptr, const QList< QWindow* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QWindow* >* qt_gui_c_QList_QWindow_ptr_operator_shl_l(QList< QWindow* >* this_ptr, const QList< QWindow* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QWindow* >* qt_gui_c_QList_QWindow_ptr_operator_shl_t(QList< QWindow* >* this_ptr, QWindow* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_QWindow_ptr_pop_back(QList< QWindow* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_QWindow_ptr_pop_front(QList< QWindow* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_QWindow_ptr_prepend(QList< QWindow* >* this_ptr, QWindow* const * t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_QWindow_ptr_push_back(QList< QWindow* >* this_ptr, QWindow* const * t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_QWindow_ptr_push_front(QList< QWindow* >* this_ptr, QWindow* const * t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_QWindow_ptr_removeAll(QList< QWindow* >* this_ptr, QWindow* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_QWindow_ptr_removeAt(QList< QWindow* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_QWindow_ptr_removeFirst(QList< QWindow* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_QWindow_ptr_removeLast(QList< QWindow* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_QWindow_ptr_removeOne(QList< QWindow* >* this_ptr, QWindow* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_QWindow_ptr_replace(QList< QWindow* >* this_ptr, int i, QWindow* const * t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_QWindow_ptr_reserve(QList< QWindow* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_QWindow_ptr_size(const QList< QWindow* >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_QWindow_ptr_startsWith(const QList< QWindow* >* this_ptr, QWindow* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_QWindow_ptr_swap_i_j(QList< QWindow* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_QWindow_ptr_swap_other(QList< QWindow* >* this_ptr, QList< QWindow* >* other) {
  this_ptr->swap(*other);
}

QWindow* qt_gui_c_QList_QWindow_ptr_takeAt(QList< QWindow* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QWindow* qt_gui_c_QList_QWindow_ptr_takeFirst(QList< QWindow* >* this_ptr) {
  return this_ptr->takeFirst();
}

QWindow* qt_gui_c_QList_QWindow_ptr_takeLast(QList< QWindow* >* this_ptr) {
  return this_ptr->takeLast();
}

QWindow* qt_gui_c_QList_QWindow_ptr_value_i(const QList< QWindow* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QWindow* qt_gui_c_QList_QWindow_ptr_value_i_defaultValue(const QList< QWindow* >* this_ptr, int i, QWindow* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_append_QList_const_QTouchDevice_ptr(QList< const QTouchDevice* >* this_ptr, const QList< const QTouchDevice* >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_append_QTouchDevice(QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t) {
  this_ptr->append(*t);
}

const QTouchDevice* const * qt_gui_c_QList_const_QTouchDevice_ptr_at(const QList< const QTouchDevice* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

const QTouchDevice** qt_gui_c_QList_const_QTouchDevice_ptr_back(QList< const QTouchDevice* >* this_ptr) {
  return &this_ptr->back();
}

const QTouchDevice* const * qt_gui_c_QList_const_QTouchDevice_ptr_back_const(const QList< const QTouchDevice* >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_const_QTouchDevice_ptr_clear(QList< const QTouchDevice* >* this_ptr) {
  this_ptr->clear();
}

const QTouchDevice* const * qt_gui_c_QList_const_QTouchDevice_ptr_constFirst(const QList< const QTouchDevice* >* this_ptr) {
  return &this_ptr->constFirst();
}

const QTouchDevice* const * qt_gui_c_QList_const_QTouchDevice_ptr_constLast(const QList< const QTouchDevice* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_const_QTouchDevice_ptr_constructor_l(const QList< const QTouchDevice* >* l, QList< const QTouchDevice* >* output) {
  new(output) QList< const QTouchDevice* >(*l);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_constructor_no_args(QList< const QTouchDevice* >* output) {
  new(output) QList< const QTouchDevice* >();
}

bool qt_gui_c_QList_const_QTouchDevice_ptr_contains(const QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_const_QTouchDevice_ptr_count_no_args(const QList< const QTouchDevice* >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_const_QTouchDevice_ptr_count_t(const QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_destructor(QList< const QTouchDevice* >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_const_QTouchDevice_ptr_empty(const QList< const QTouchDevice* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_const_QTouchDevice_ptr_endsWith(const QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t) {
  return this_ptr->endsWith(*t);
}

const QTouchDevice** qt_gui_c_QList_const_QTouchDevice_ptr_first(QList< const QTouchDevice* >* this_ptr) {
  return &this_ptr->first();
}

const QTouchDevice* const * qt_gui_c_QList_const_QTouchDevice_ptr_first_const(const QList< const QTouchDevice* >* this_ptr) {
  return &this_ptr->first();
}

const QTouchDevice** qt_gui_c_QList_const_QTouchDevice_ptr_front(QList< const QTouchDevice* >* this_ptr) {
  return &this_ptr->front();
}

const QTouchDevice* const * qt_gui_c_QList_const_QTouchDevice_ptr_front_const(const QList< const QTouchDevice* >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_const_QTouchDevice_ptr_indexOf_t(const QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_const_QTouchDevice_ptr_indexOf_t_from(const QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_insert(QList< const QTouchDevice* >* this_ptr, int i, const QTouchDevice* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_const_QTouchDevice_ptr_isEmpty(const QList< const QTouchDevice* >* this_ptr) {
  return this_ptr->isEmpty();
}

const QTouchDevice** qt_gui_c_QList_const_QTouchDevice_ptr_last(QList< const QTouchDevice* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_const_QTouchDevice_ptr_lastIndexOf_t(const QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_const_QTouchDevice_ptr_lastIndexOf_t_from(const QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QTouchDevice* const * qt_gui_c_QList_const_QTouchDevice_ptr_last_const(const QList< const QTouchDevice* >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_const_QTouchDevice_ptr_length(const QList< const QTouchDevice* >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_const_QTouchDevice_ptr_mid_to_output_pos(const QList< const QTouchDevice* >* this_ptr, int pos, QList< const QTouchDevice* >* output) {
  new(output) QList< const QTouchDevice* >(this_ptr->mid(pos));
}

void qt_gui_c_QList_const_QTouchDevice_ptr_mid_to_output_pos_length(const QList< const QTouchDevice* >* this_ptr, int pos, int length, QList< const QTouchDevice* >* output) {
  new(output) QList< const QTouchDevice* >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_const_QTouchDevice_ptr_move(QList< const QTouchDevice* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< const QTouchDevice* >* qt_gui_c_QList_const_QTouchDevice_ptr_operator_add_assign_l(QList< const QTouchDevice* >* this_ptr, const QList< const QTouchDevice* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< const QTouchDevice* >* qt_gui_c_QList_const_QTouchDevice_ptr_operator_add_assign_t(QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_operator_add_to_output(const QList< const QTouchDevice* >* this_ptr, const QList< const QTouchDevice* >* l, QList< const QTouchDevice* >* output) {
  new(output) QList< const QTouchDevice* >(this_ptr->operator+(*l));
}

QList< const QTouchDevice* >* qt_gui_c_QList_const_QTouchDevice_ptr_operator_assign(QList< const QTouchDevice* >* this_ptr, const QList< const QTouchDevice* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_const_QTouchDevice_ptr_operator_eq(const QList< const QTouchDevice* >* this_ptr, const QList< const QTouchDevice* >* l) {
  return this_ptr->operator==(*l);
}

const QTouchDevice** qt_gui_c_QList_const_QTouchDevice_ptr_operator_index(QList< const QTouchDevice* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QTouchDevice* const * qt_gui_c_QList_const_QTouchDevice_ptr_operator_index_const(const QList< const QTouchDevice* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_const_QTouchDevice_ptr_operator_neq(const QList< const QTouchDevice* >* this_ptr, const QList< const QTouchDevice* >* l) {
  return this_ptr->operator!=(*l);
}

QList< const QTouchDevice* >* qt_gui_c_QList_const_QTouchDevice_ptr_operator_shl_l(QList< const QTouchDevice* >* this_ptr, const QList< const QTouchDevice* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< const QTouchDevice* >* qt_gui_c_QList_const_QTouchDevice_ptr_operator_shl_t(QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_pop_back(QList< const QTouchDevice* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_const_QTouchDevice_ptr_pop_front(QList< const QTouchDevice* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_const_QTouchDevice_ptr_prepend(QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_push_back(QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_push_front(QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_const_QTouchDevice_ptr_removeAll(QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_removeAt(QList< const QTouchDevice* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_removeFirst(QList< const QTouchDevice* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_const_QTouchDevice_ptr_removeLast(QList< const QTouchDevice* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_const_QTouchDevice_ptr_removeOne(QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_replace(QList< const QTouchDevice* >* this_ptr, int i, const QTouchDevice* const * t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_reserve(QList< const QTouchDevice* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_const_QTouchDevice_ptr_size(const QList< const QTouchDevice* >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_const_QTouchDevice_ptr_startsWith(const QList< const QTouchDevice* >* this_ptr, const QTouchDevice* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_swap_i_j(QList< const QTouchDevice* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_const_QTouchDevice_ptr_swap_other(QList< const QTouchDevice* >* this_ptr, QList< const QTouchDevice* >* other) {
  this_ptr->swap(*other);
}

const QTouchDevice* qt_gui_c_QList_const_QTouchDevice_ptr_takeAt(QList< const QTouchDevice* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

const QTouchDevice* qt_gui_c_QList_const_QTouchDevice_ptr_takeFirst(QList< const QTouchDevice* >* this_ptr) {
  return this_ptr->takeFirst();
}

const QTouchDevice* qt_gui_c_QList_const_QTouchDevice_ptr_takeLast(QList< const QTouchDevice* >* this_ptr) {
  return this_ptr->takeLast();
}

const QTouchDevice* qt_gui_c_QList_const_QTouchDevice_ptr_value_i(const QList< const QTouchDevice* >* this_ptr, int i) {
  return this_ptr->value(i);
}

const QTouchDevice* qt_gui_c_QList_const_QTouchDevice_ptr_value_i_defaultValue(const QList< const QTouchDevice* >* this_ptr, int i, const QTouchDevice* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_gui_c_QList_double_append_QList_double(QList< double >* this_ptr, const QList< double >* t) {
  this_ptr->append(*t);
}

void qt_gui_c_QList_double_append_double(QList< double >* this_ptr, const double* t) {
  this_ptr->append(*t);
}

const double* qt_gui_c_QList_double_at(const QList< double >* this_ptr, int i) {
  return &this_ptr->at(i);
}

double* qt_gui_c_QList_double_back(QList< double >* this_ptr) {
  return &this_ptr->back();
}

const double* qt_gui_c_QList_double_back_const(const QList< double >* this_ptr) {
  return &this_ptr->back();
}

void qt_gui_c_QList_double_clear(QList< double >* this_ptr) {
  this_ptr->clear();
}

const double* qt_gui_c_QList_double_constFirst(const QList< double >* this_ptr) {
  return &this_ptr->constFirst();
}

const double* qt_gui_c_QList_double_constLast(const QList< double >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_gui_c_QList_double_constructor_l(const QList< double >* l, QList< double >* output) {
  new(output) QList< double >(*l);
}

void qt_gui_c_QList_double_constructor_no_args(QList< double >* output) {
  new(output) QList< double >();
}

bool qt_gui_c_QList_double_contains(const QList< double >* this_ptr, const double* t) {
  return this_ptr->contains(*t);
}

int qt_gui_c_QList_double_count_no_args(const QList< double >* this_ptr) {
  return this_ptr->count();
}

int qt_gui_c_QList_double_count_t(const QList< double >* this_ptr, const double* t) {
  return this_ptr->count(*t);
}

void qt_gui_c_QList_double_destructor(QList< double >* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QList_double_empty(const QList< double >* this_ptr) {
  return this_ptr->empty();
}

bool qt_gui_c_QList_double_endsWith(const QList< double >* this_ptr, const double* t) {
  return this_ptr->endsWith(*t);
}

double* qt_gui_c_QList_double_first(QList< double >* this_ptr) {
  return &this_ptr->first();
}

const double* qt_gui_c_QList_double_first_const(const QList< double >* this_ptr) {
  return &this_ptr->first();
}

void qt_gui_c_QList_double_fromVector_to_output(const QVector< double >* vector, QList< double >* output) {
  new(output) QList< double >(QList< double >::fromVector(*vector));
}

double* qt_gui_c_QList_double_front(QList< double >* this_ptr) {
  return &this_ptr->front();
}

const double* qt_gui_c_QList_double_front_const(const QList< double >* this_ptr) {
  return &this_ptr->front();
}

int qt_gui_c_QList_double_indexOf_t(const QList< double >* this_ptr, const double* t) {
  return this_ptr->indexOf(*t);
}

int qt_gui_c_QList_double_indexOf_t_from(const QList< double >* this_ptr, const double* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_gui_c_QList_double_insert(QList< double >* this_ptr, int i, const double* t) {
  this_ptr->insert(i, *t);
}

bool qt_gui_c_QList_double_isEmpty(const QList< double >* this_ptr) {
  return this_ptr->isEmpty();
}

double* qt_gui_c_QList_double_last(QList< double >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_double_lastIndexOf_t(const QList< double >* this_ptr, const double* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_gui_c_QList_double_lastIndexOf_t_from(const QList< double >* this_ptr, const double* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const double* qt_gui_c_QList_double_last_const(const QList< double >* this_ptr) {
  return &this_ptr->last();
}

int qt_gui_c_QList_double_length(const QList< double >* this_ptr) {
  return this_ptr->length();
}

void qt_gui_c_QList_double_mid_to_output_pos(const QList< double >* this_ptr, int pos, QList< double >* output) {
  new(output) QList< double >(this_ptr->mid(pos));
}

void qt_gui_c_QList_double_mid_to_output_pos_length(const QList< double >* this_ptr, int pos, int length, QList< double >* output) {
  new(output) QList< double >(this_ptr->mid(pos, length));
}

void qt_gui_c_QList_double_move(QList< double >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< double >* qt_gui_c_QList_double_operator_add_assign_l(QList< double >* this_ptr, const QList< double >* l) {
  return &this_ptr->operator+=(*l);
}

QList< double >* qt_gui_c_QList_double_operator_add_assign_t(QList< double >* this_ptr, const double* t) {
  return &this_ptr->operator+=(*t);
}

void qt_gui_c_QList_double_operator_add_to_output(const QList< double >* this_ptr, const QList< double >* l, QList< double >* output) {
  new(output) QList< double >(this_ptr->operator+(*l));
}

QList< double >* qt_gui_c_QList_double_operator_assign(QList< double >* this_ptr, const QList< double >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_gui_c_QList_double_operator_eq(const QList< double >* this_ptr, const QList< double >* l) {
  return this_ptr->operator==(*l);
}

double* qt_gui_c_QList_double_operator_index(QList< double >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const double* qt_gui_c_QList_double_operator_index_const(const QList< double >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_gui_c_QList_double_operator_neq(const QList< double >* this_ptr, const QList< double >* l) {
  return this_ptr->operator!=(*l);
}

QList< double >* qt_gui_c_QList_double_operator_shl_l(QList< double >* this_ptr, const QList< double >* l) {
  return &this_ptr->operator<<(*l);
}

QList< double >* qt_gui_c_QList_double_operator_shl_t(QList< double >* this_ptr, const double* t) {
  return &this_ptr->operator<<(*t);
}

void qt_gui_c_QList_double_pop_back(QList< double >* this_ptr) {
  this_ptr->pop_back();
}

void qt_gui_c_QList_double_pop_front(QList< double >* this_ptr) {
  this_ptr->pop_front();
}

void qt_gui_c_QList_double_prepend(QList< double >* this_ptr, const double* t) {
  this_ptr->prepend(*t);
}

void qt_gui_c_QList_double_push_back(QList< double >* this_ptr, const double* t) {
  this_ptr->push_back(*t);
}

void qt_gui_c_QList_double_push_front(QList< double >* this_ptr, const double* t) {
  this_ptr->push_front(*t);
}

int qt_gui_c_QList_double_removeAll(QList< double >* this_ptr, const double* t) {
  return this_ptr->removeAll(*t);
}

void qt_gui_c_QList_double_removeAt(QList< double >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_gui_c_QList_double_removeFirst(QList< double >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_gui_c_QList_double_removeLast(QList< double >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_gui_c_QList_double_removeOne(QList< double >* this_ptr, const double* t) {
  return this_ptr->removeOne(*t);
}

void qt_gui_c_QList_double_replace(QList< double >* this_ptr, int i, const double* t) {
  this_ptr->replace(i, *t);
}

void qt_gui_c_QList_double_reserve(QList< double >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_gui_c_QList_double_size(const QList< double >* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QList_double_startsWith(const QList< double >* this_ptr, const double* t) {
  return this_ptr->startsWith(*t);
}

void qt_gui_c_QList_double_swap_i_j(QList< double >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_gui_c_QList_double_swap_other(QList< double >* this_ptr, QList< double >* other) {
  this_ptr->swap(*other);
}

double qt_gui_c_QList_double_takeAt(QList< double >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

double qt_gui_c_QList_double_takeFirst(QList< double >* this_ptr) {
  return this_ptr->takeFirst();
}

double qt_gui_c_QList_double_takeLast(QList< double >* this_ptr) {
  return this_ptr->takeLast();
}

void qt_gui_c_QList_double_toVector_to_output(const QList< double >* this_ptr, QVector< double >* output) {
  new(output) QVector< double >(this_ptr->toVector());
}

double qt_gui_c_QList_double_value_i(const QList< double >* this_ptr, int i) {
  return this_ptr->value(i);
}

double qt_gui_c_QList_double_value_i_defaultValue(const QList< double >* this_ptr, int i, const double* defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

