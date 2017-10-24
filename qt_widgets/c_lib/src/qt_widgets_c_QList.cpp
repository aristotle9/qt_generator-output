#include "qt_widgets_c_QList.h"

void qt_widgets_c_QList_QAbstractButton_ptr_append_QAbstractButton(QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QAbstractButton_ptr_append_QList_QAbstractButton_ptr(QList< QAbstractButton* >* this_ptr, const QList< QAbstractButton* >* t) {
  this_ptr->append(*t);
}

QAbstractButton* const * qt_widgets_c_QList_QAbstractButton_ptr_at(const QList< QAbstractButton* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QAbstractButton** qt_widgets_c_QList_QAbstractButton_ptr_back(QList< QAbstractButton* >* this_ptr) {
  return &this_ptr->back();
}

QAbstractButton* const * qt_widgets_c_QList_QAbstractButton_ptr_back_const(const QList< QAbstractButton* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QAbstractButton_ptr_clear(QList< QAbstractButton* >* this_ptr) {
  this_ptr->clear();
}

QAbstractButton* const * qt_widgets_c_QList_QAbstractButton_ptr_constFirst(const QList< QAbstractButton* >* this_ptr) {
  return &this_ptr->constFirst();
}

QAbstractButton* const * qt_widgets_c_QList_QAbstractButton_ptr_constLast(const QList< QAbstractButton* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QAbstractButton_ptr_constructor_l(const QList< QAbstractButton* >* l, QList< QAbstractButton* >* output) {
  new(output) QList< QAbstractButton* >(*l);
}

void qt_widgets_c_QList_QAbstractButton_ptr_constructor_no_args(QList< QAbstractButton* >* output) {
  new(output) QList< QAbstractButton* >();
}

bool qt_widgets_c_QList_QAbstractButton_ptr_contains(const QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QAbstractButton_ptr_count_no_args(const QList< QAbstractButton* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QAbstractButton_ptr_count_t(const QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QAbstractButton_ptr_destructor(QList< QAbstractButton* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QAbstractButton_ptr_empty(const QList< QAbstractButton* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QAbstractButton_ptr_endsWith(const QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t) {
  return this_ptr->endsWith(*t);
}

QAbstractButton** qt_widgets_c_QList_QAbstractButton_ptr_first(QList< QAbstractButton* >* this_ptr) {
  return &this_ptr->first();
}

QAbstractButton* const * qt_widgets_c_QList_QAbstractButton_ptr_first_const(const QList< QAbstractButton* >* this_ptr) {
  return &this_ptr->first();
}

QAbstractButton** qt_widgets_c_QList_QAbstractButton_ptr_front(QList< QAbstractButton* >* this_ptr) {
  return &this_ptr->front();
}

QAbstractButton* const * qt_widgets_c_QList_QAbstractButton_ptr_front_const(const QList< QAbstractButton* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QAbstractButton_ptr_indexOf_t(const QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QAbstractButton_ptr_indexOf_t_from(const QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QAbstractButton_ptr_insert(QList< QAbstractButton* >* this_ptr, int i, QAbstractButton* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QAbstractButton_ptr_isEmpty(const QList< QAbstractButton* >* this_ptr) {
  return this_ptr->isEmpty();
}

QAbstractButton** qt_widgets_c_QList_QAbstractButton_ptr_last(QList< QAbstractButton* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QAbstractButton_ptr_lastIndexOf_t(const QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QAbstractButton_ptr_lastIndexOf_t_from(const QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QAbstractButton* const * qt_widgets_c_QList_QAbstractButton_ptr_last_const(const QList< QAbstractButton* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QAbstractButton_ptr_length(const QList< QAbstractButton* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QAbstractButton_ptr_mid_to_output_pos(const QList< QAbstractButton* >* this_ptr, int pos, QList< QAbstractButton* >* output) {
  new(output) QList< QAbstractButton* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QAbstractButton_ptr_mid_to_output_pos_length(const QList< QAbstractButton* >* this_ptr, int pos, int length, QList< QAbstractButton* >* output) {
  new(output) QList< QAbstractButton* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QAbstractButton_ptr_move(QList< QAbstractButton* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QAbstractButton* >* qt_widgets_c_QList_QAbstractButton_ptr_operator_add_assign_l(QList< QAbstractButton* >* this_ptr, const QList< QAbstractButton* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QAbstractButton* >* qt_widgets_c_QList_QAbstractButton_ptr_operator_add_assign_t(QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QAbstractButton_ptr_operator_add_to_output(const QList< QAbstractButton* >* this_ptr, const QList< QAbstractButton* >* l, QList< QAbstractButton* >* output) {
  new(output) QList< QAbstractButton* >(this_ptr->operator+(*l));
}

QList< QAbstractButton* >* qt_widgets_c_QList_QAbstractButton_ptr_operator_assign(QList< QAbstractButton* >* this_ptr, const QList< QAbstractButton* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QAbstractButton_ptr_operator_eq(const QList< QAbstractButton* >* this_ptr, const QList< QAbstractButton* >* l) {
  return this_ptr->operator==(*l);
}

QAbstractButton** qt_widgets_c_QList_QAbstractButton_ptr_operator_index(QList< QAbstractButton* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QAbstractButton* const * qt_widgets_c_QList_QAbstractButton_ptr_operator_index_const(const QList< QAbstractButton* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QAbstractButton_ptr_operator_neq(const QList< QAbstractButton* >* this_ptr, const QList< QAbstractButton* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QAbstractButton* >* qt_widgets_c_QList_QAbstractButton_ptr_operator_shl_l(QList< QAbstractButton* >* this_ptr, const QList< QAbstractButton* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QAbstractButton* >* qt_widgets_c_QList_QAbstractButton_ptr_operator_shl_t(QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QAbstractButton_ptr_pop_back(QList< QAbstractButton* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QAbstractButton_ptr_pop_front(QList< QAbstractButton* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QAbstractButton_ptr_prepend(QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QAbstractButton_ptr_push_back(QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QAbstractButton_ptr_push_front(QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QAbstractButton_ptr_removeAll(QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QAbstractButton_ptr_removeAt(QList< QAbstractButton* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QAbstractButton_ptr_removeFirst(QList< QAbstractButton* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QAbstractButton_ptr_removeLast(QList< QAbstractButton* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QAbstractButton_ptr_removeOne(QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QAbstractButton_ptr_replace(QList< QAbstractButton* >* this_ptr, int i, QAbstractButton* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QAbstractButton_ptr_reserve(QList< QAbstractButton* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QAbstractButton_ptr_size(const QList< QAbstractButton* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QAbstractButton_ptr_startsWith(const QList< QAbstractButton* >* this_ptr, QAbstractButton* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QAbstractButton_ptr_swap_i_j(QList< QAbstractButton* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QAbstractButton_ptr_swap_other(QList< QAbstractButton* >* this_ptr, QList< QAbstractButton* >* other) {
  this_ptr->swap(*other);
}

QAbstractButton* qt_widgets_c_QList_QAbstractButton_ptr_takeAt(QList< QAbstractButton* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QAbstractButton* qt_widgets_c_QList_QAbstractButton_ptr_takeFirst(QList< QAbstractButton* >* this_ptr) {
  return this_ptr->takeFirst();
}

QAbstractButton* qt_widgets_c_QList_QAbstractButton_ptr_takeLast(QList< QAbstractButton* >* this_ptr) {
  return this_ptr->takeLast();
}

QAbstractButton* qt_widgets_c_QList_QAbstractButton_ptr_value_i(const QList< QAbstractButton* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QAbstractButton* qt_widgets_c_QList_QAbstractButton_ptr_value_i_defaultValue(const QList< QAbstractButton* >* this_ptr, int i, QAbstractButton* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QAction_ptr_append_QAction(QList< QAction* >* this_ptr, QAction* const * t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QAction_ptr_append_QList_QAction_ptr(QList< QAction* >* this_ptr, const QList< QAction* >* t) {
  this_ptr->append(*t);
}

QAction* const * qt_widgets_c_QList_QAction_ptr_at(const QList< QAction* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QAction** qt_widgets_c_QList_QAction_ptr_back(QList< QAction* >* this_ptr) {
  return &this_ptr->back();
}

QAction* const * qt_widgets_c_QList_QAction_ptr_back_const(const QList< QAction* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QAction_ptr_clear(QList< QAction* >* this_ptr) {
  this_ptr->clear();
}

QAction* const * qt_widgets_c_QList_QAction_ptr_constFirst(const QList< QAction* >* this_ptr) {
  return &this_ptr->constFirst();
}

QAction* const * qt_widgets_c_QList_QAction_ptr_constLast(const QList< QAction* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QAction_ptr_constructor_l(const QList< QAction* >* l, QList< QAction* >* output) {
  new(output) QList< QAction* >(*l);
}

void qt_widgets_c_QList_QAction_ptr_constructor_no_args(QList< QAction* >* output) {
  new(output) QList< QAction* >();
}

bool qt_widgets_c_QList_QAction_ptr_contains(const QList< QAction* >* this_ptr, QAction* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QAction_ptr_count_no_args(const QList< QAction* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QAction_ptr_count_t(const QList< QAction* >* this_ptr, QAction* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QAction_ptr_destructor(QList< QAction* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QAction_ptr_empty(const QList< QAction* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QAction_ptr_endsWith(const QList< QAction* >* this_ptr, QAction* const * t) {
  return this_ptr->endsWith(*t);
}

QAction** qt_widgets_c_QList_QAction_ptr_first(QList< QAction* >* this_ptr) {
  return &this_ptr->first();
}

QAction* const * qt_widgets_c_QList_QAction_ptr_first_const(const QList< QAction* >* this_ptr) {
  return &this_ptr->first();
}

QAction** qt_widgets_c_QList_QAction_ptr_front(QList< QAction* >* this_ptr) {
  return &this_ptr->front();
}

QAction* const * qt_widgets_c_QList_QAction_ptr_front_const(const QList< QAction* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QAction_ptr_indexOf_t(const QList< QAction* >* this_ptr, QAction* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QAction_ptr_indexOf_t_from(const QList< QAction* >* this_ptr, QAction* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QAction_ptr_insert(QList< QAction* >* this_ptr, int i, QAction* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QAction_ptr_isEmpty(const QList< QAction* >* this_ptr) {
  return this_ptr->isEmpty();
}

QAction** qt_widgets_c_QList_QAction_ptr_last(QList< QAction* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QAction_ptr_lastIndexOf_t(const QList< QAction* >* this_ptr, QAction* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QAction_ptr_lastIndexOf_t_from(const QList< QAction* >* this_ptr, QAction* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QAction* const * qt_widgets_c_QList_QAction_ptr_last_const(const QList< QAction* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QAction_ptr_length(const QList< QAction* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QAction_ptr_mid_to_output_pos(const QList< QAction* >* this_ptr, int pos, QList< QAction* >* output) {
  new(output) QList< QAction* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QAction_ptr_mid_to_output_pos_length(const QList< QAction* >* this_ptr, int pos, int length, QList< QAction* >* output) {
  new(output) QList< QAction* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QAction_ptr_move(QList< QAction* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QAction* >* qt_widgets_c_QList_QAction_ptr_operator_add_assign_l(QList< QAction* >* this_ptr, const QList< QAction* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QAction* >* qt_widgets_c_QList_QAction_ptr_operator_add_assign_t(QList< QAction* >* this_ptr, QAction* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QAction_ptr_operator_add_to_output(const QList< QAction* >* this_ptr, const QList< QAction* >* l, QList< QAction* >* output) {
  new(output) QList< QAction* >(this_ptr->operator+(*l));
}

QList< QAction* >* qt_widgets_c_QList_QAction_ptr_operator_assign(QList< QAction* >* this_ptr, const QList< QAction* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QAction_ptr_operator_eq(const QList< QAction* >* this_ptr, const QList< QAction* >* l) {
  return this_ptr->operator==(*l);
}

QAction** qt_widgets_c_QList_QAction_ptr_operator_index(QList< QAction* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QAction* const * qt_widgets_c_QList_QAction_ptr_operator_index_const(const QList< QAction* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QAction_ptr_operator_neq(const QList< QAction* >* this_ptr, const QList< QAction* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QAction* >* qt_widgets_c_QList_QAction_ptr_operator_shl_l(QList< QAction* >* this_ptr, const QList< QAction* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QAction* >* qt_widgets_c_QList_QAction_ptr_operator_shl_t(QList< QAction* >* this_ptr, QAction* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QAction_ptr_pop_back(QList< QAction* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QAction_ptr_pop_front(QList< QAction* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QAction_ptr_prepend(QList< QAction* >* this_ptr, QAction* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QAction_ptr_push_back(QList< QAction* >* this_ptr, QAction* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QAction_ptr_push_front(QList< QAction* >* this_ptr, QAction* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QAction_ptr_removeAll(QList< QAction* >* this_ptr, QAction* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QAction_ptr_removeAt(QList< QAction* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QAction_ptr_removeFirst(QList< QAction* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QAction_ptr_removeLast(QList< QAction* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QAction_ptr_removeOne(QList< QAction* >* this_ptr, QAction* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QAction_ptr_replace(QList< QAction* >* this_ptr, int i, QAction* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QAction_ptr_reserve(QList< QAction* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QAction_ptr_size(const QList< QAction* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QAction_ptr_startsWith(const QList< QAction* >* this_ptr, QAction* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QAction_ptr_swap_i_j(QList< QAction* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QAction_ptr_swap_other(QList< QAction* >* this_ptr, QList< QAction* >* other) {
  this_ptr->swap(*other);
}

QAction* qt_widgets_c_QList_QAction_ptr_takeAt(QList< QAction* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QAction* qt_widgets_c_QList_QAction_ptr_takeFirst(QList< QAction* >* this_ptr) {
  return this_ptr->takeFirst();
}

QAction* qt_widgets_c_QList_QAction_ptr_takeLast(QList< QAction* >* this_ptr) {
  return this_ptr->takeLast();
}

QAction* qt_widgets_c_QList_QAction_ptr_value_i(const QList< QAction* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QAction* qt_widgets_c_QList_QAction_ptr_value_i_defaultValue(const QList< QAction* >* this_ptr, int i, QAction* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QDockWidget_ptr_append_QDockWidget(QList< QDockWidget* >* this_ptr, QDockWidget* const * t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QDockWidget_ptr_append_QList_QDockWidget_ptr(QList< QDockWidget* >* this_ptr, const QList< QDockWidget* >* t) {
  this_ptr->append(*t);
}

QDockWidget* const * qt_widgets_c_QList_QDockWidget_ptr_at(const QList< QDockWidget* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QDockWidget** qt_widgets_c_QList_QDockWidget_ptr_back(QList< QDockWidget* >* this_ptr) {
  return &this_ptr->back();
}

QDockWidget* const * qt_widgets_c_QList_QDockWidget_ptr_back_const(const QList< QDockWidget* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QDockWidget_ptr_clear(QList< QDockWidget* >* this_ptr) {
  this_ptr->clear();
}

QDockWidget* const * qt_widgets_c_QList_QDockWidget_ptr_constFirst(const QList< QDockWidget* >* this_ptr) {
  return &this_ptr->constFirst();
}

QDockWidget* const * qt_widgets_c_QList_QDockWidget_ptr_constLast(const QList< QDockWidget* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QDockWidget_ptr_constructor_l(const QList< QDockWidget* >* l, QList< QDockWidget* >* output) {
  new(output) QList< QDockWidget* >(*l);
}

void qt_widgets_c_QList_QDockWidget_ptr_constructor_no_args(QList< QDockWidget* >* output) {
  new(output) QList< QDockWidget* >();
}

bool qt_widgets_c_QList_QDockWidget_ptr_contains(const QList< QDockWidget* >* this_ptr, QDockWidget* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QDockWidget_ptr_count_no_args(const QList< QDockWidget* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QDockWidget_ptr_count_t(const QList< QDockWidget* >* this_ptr, QDockWidget* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QDockWidget_ptr_destructor(QList< QDockWidget* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QDockWidget_ptr_empty(const QList< QDockWidget* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QDockWidget_ptr_endsWith(const QList< QDockWidget* >* this_ptr, QDockWidget* const * t) {
  return this_ptr->endsWith(*t);
}

QDockWidget** qt_widgets_c_QList_QDockWidget_ptr_first(QList< QDockWidget* >* this_ptr) {
  return &this_ptr->first();
}

QDockWidget* const * qt_widgets_c_QList_QDockWidget_ptr_first_const(const QList< QDockWidget* >* this_ptr) {
  return &this_ptr->first();
}

QDockWidget** qt_widgets_c_QList_QDockWidget_ptr_front(QList< QDockWidget* >* this_ptr) {
  return &this_ptr->front();
}

QDockWidget* const * qt_widgets_c_QList_QDockWidget_ptr_front_const(const QList< QDockWidget* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QDockWidget_ptr_indexOf_t(const QList< QDockWidget* >* this_ptr, QDockWidget* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QDockWidget_ptr_indexOf_t_from(const QList< QDockWidget* >* this_ptr, QDockWidget* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QDockWidget_ptr_insert(QList< QDockWidget* >* this_ptr, int i, QDockWidget* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QDockWidget_ptr_isEmpty(const QList< QDockWidget* >* this_ptr) {
  return this_ptr->isEmpty();
}

QDockWidget** qt_widgets_c_QList_QDockWidget_ptr_last(QList< QDockWidget* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QDockWidget_ptr_lastIndexOf_t(const QList< QDockWidget* >* this_ptr, QDockWidget* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QDockWidget_ptr_lastIndexOf_t_from(const QList< QDockWidget* >* this_ptr, QDockWidget* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QDockWidget* const * qt_widgets_c_QList_QDockWidget_ptr_last_const(const QList< QDockWidget* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QDockWidget_ptr_length(const QList< QDockWidget* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QDockWidget_ptr_mid_to_output_pos(const QList< QDockWidget* >* this_ptr, int pos, QList< QDockWidget* >* output) {
  new(output) QList< QDockWidget* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QDockWidget_ptr_mid_to_output_pos_length(const QList< QDockWidget* >* this_ptr, int pos, int length, QList< QDockWidget* >* output) {
  new(output) QList< QDockWidget* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QDockWidget_ptr_move(QList< QDockWidget* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QDockWidget* >* qt_widgets_c_QList_QDockWidget_ptr_operator_add_assign_l(QList< QDockWidget* >* this_ptr, const QList< QDockWidget* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QDockWidget* >* qt_widgets_c_QList_QDockWidget_ptr_operator_add_assign_t(QList< QDockWidget* >* this_ptr, QDockWidget* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QDockWidget_ptr_operator_add_to_output(const QList< QDockWidget* >* this_ptr, const QList< QDockWidget* >* l, QList< QDockWidget* >* output) {
  new(output) QList< QDockWidget* >(this_ptr->operator+(*l));
}

QList< QDockWidget* >* qt_widgets_c_QList_QDockWidget_ptr_operator_assign(QList< QDockWidget* >* this_ptr, const QList< QDockWidget* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QDockWidget_ptr_operator_eq(const QList< QDockWidget* >* this_ptr, const QList< QDockWidget* >* l) {
  return this_ptr->operator==(*l);
}

QDockWidget** qt_widgets_c_QList_QDockWidget_ptr_operator_index(QList< QDockWidget* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QDockWidget* const * qt_widgets_c_QList_QDockWidget_ptr_operator_index_const(const QList< QDockWidget* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QDockWidget_ptr_operator_neq(const QList< QDockWidget* >* this_ptr, const QList< QDockWidget* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QDockWidget* >* qt_widgets_c_QList_QDockWidget_ptr_operator_shl_l(QList< QDockWidget* >* this_ptr, const QList< QDockWidget* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QDockWidget* >* qt_widgets_c_QList_QDockWidget_ptr_operator_shl_t(QList< QDockWidget* >* this_ptr, QDockWidget* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QDockWidget_ptr_pop_back(QList< QDockWidget* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QDockWidget_ptr_pop_front(QList< QDockWidget* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QDockWidget_ptr_prepend(QList< QDockWidget* >* this_ptr, QDockWidget* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QDockWidget_ptr_push_back(QList< QDockWidget* >* this_ptr, QDockWidget* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QDockWidget_ptr_push_front(QList< QDockWidget* >* this_ptr, QDockWidget* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QDockWidget_ptr_removeAll(QList< QDockWidget* >* this_ptr, QDockWidget* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QDockWidget_ptr_removeAt(QList< QDockWidget* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QDockWidget_ptr_removeFirst(QList< QDockWidget* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QDockWidget_ptr_removeLast(QList< QDockWidget* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QDockWidget_ptr_removeOne(QList< QDockWidget* >* this_ptr, QDockWidget* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QDockWidget_ptr_replace(QList< QDockWidget* >* this_ptr, int i, QDockWidget* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QDockWidget_ptr_reserve(QList< QDockWidget* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QDockWidget_ptr_size(const QList< QDockWidget* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QDockWidget_ptr_startsWith(const QList< QDockWidget* >* this_ptr, QDockWidget* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QDockWidget_ptr_swap_i_j(QList< QDockWidget* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QDockWidget_ptr_swap_other(QList< QDockWidget* >* this_ptr, QList< QDockWidget* >* other) {
  this_ptr->swap(*other);
}

QDockWidget* qt_widgets_c_QList_QDockWidget_ptr_takeAt(QList< QDockWidget* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QDockWidget* qt_widgets_c_QList_QDockWidget_ptr_takeFirst(QList< QDockWidget* >* this_ptr) {
  return this_ptr->takeFirst();
}

QDockWidget* qt_widgets_c_QList_QDockWidget_ptr_takeLast(QList< QDockWidget* >* this_ptr) {
  return this_ptr->takeLast();
}

QDockWidget* qt_widgets_c_QList_QDockWidget_ptr_value_i(const QList< QDockWidget* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QDockWidget* qt_widgets_c_QList_QDockWidget_ptr_value_i_defaultValue(const QList< QDockWidget* >* this_ptr, int i, QDockWidget* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QGesture_ptr_append_QGesture(QList< QGesture* >* this_ptr, QGesture* const * t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QGesture_ptr_append_QList_QGesture_ptr(QList< QGesture* >* this_ptr, const QList< QGesture* >* t) {
  this_ptr->append(*t);
}

QGesture* const * qt_widgets_c_QList_QGesture_ptr_at(const QList< QGesture* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QGesture** qt_widgets_c_QList_QGesture_ptr_back(QList< QGesture* >* this_ptr) {
  return &this_ptr->back();
}

QGesture* const * qt_widgets_c_QList_QGesture_ptr_back_const(const QList< QGesture* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QGesture_ptr_clear(QList< QGesture* >* this_ptr) {
  this_ptr->clear();
}

QGesture* const * qt_widgets_c_QList_QGesture_ptr_constFirst(const QList< QGesture* >* this_ptr) {
  return &this_ptr->constFirst();
}

QGesture* const * qt_widgets_c_QList_QGesture_ptr_constLast(const QList< QGesture* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QGesture_ptr_constructor_l(const QList< QGesture* >* l, QList< QGesture* >* output) {
  new(output) QList< QGesture* >(*l);
}

void qt_widgets_c_QList_QGesture_ptr_constructor_no_args(QList< QGesture* >* output) {
  new(output) QList< QGesture* >();
}

bool qt_widgets_c_QList_QGesture_ptr_contains(const QList< QGesture* >* this_ptr, QGesture* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QGesture_ptr_count_no_args(const QList< QGesture* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QGesture_ptr_count_t(const QList< QGesture* >* this_ptr, QGesture* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QGesture_ptr_destructor(QList< QGesture* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QGesture_ptr_empty(const QList< QGesture* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QGesture_ptr_endsWith(const QList< QGesture* >* this_ptr, QGesture* const * t) {
  return this_ptr->endsWith(*t);
}

QGesture** qt_widgets_c_QList_QGesture_ptr_first(QList< QGesture* >* this_ptr) {
  return &this_ptr->first();
}

QGesture* const * qt_widgets_c_QList_QGesture_ptr_first_const(const QList< QGesture* >* this_ptr) {
  return &this_ptr->first();
}

QGesture** qt_widgets_c_QList_QGesture_ptr_front(QList< QGesture* >* this_ptr) {
  return &this_ptr->front();
}

QGesture* const * qt_widgets_c_QList_QGesture_ptr_front_const(const QList< QGesture* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QGesture_ptr_indexOf_t(const QList< QGesture* >* this_ptr, QGesture* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QGesture_ptr_indexOf_t_from(const QList< QGesture* >* this_ptr, QGesture* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QGesture_ptr_insert(QList< QGesture* >* this_ptr, int i, QGesture* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QGesture_ptr_isEmpty(const QList< QGesture* >* this_ptr) {
  return this_ptr->isEmpty();
}

QGesture** qt_widgets_c_QList_QGesture_ptr_last(QList< QGesture* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QGesture_ptr_lastIndexOf_t(const QList< QGesture* >* this_ptr, QGesture* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QGesture_ptr_lastIndexOf_t_from(const QList< QGesture* >* this_ptr, QGesture* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QGesture* const * qt_widgets_c_QList_QGesture_ptr_last_const(const QList< QGesture* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QGesture_ptr_length(const QList< QGesture* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QGesture_ptr_mid_to_output_pos(const QList< QGesture* >* this_ptr, int pos, QList< QGesture* >* output) {
  new(output) QList< QGesture* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QGesture_ptr_mid_to_output_pos_length(const QList< QGesture* >* this_ptr, int pos, int length, QList< QGesture* >* output) {
  new(output) QList< QGesture* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QGesture_ptr_move(QList< QGesture* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QGesture* >* qt_widgets_c_QList_QGesture_ptr_operator_add_assign_l(QList< QGesture* >* this_ptr, const QList< QGesture* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QGesture* >* qt_widgets_c_QList_QGesture_ptr_operator_add_assign_t(QList< QGesture* >* this_ptr, QGesture* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QGesture_ptr_operator_add_to_output(const QList< QGesture* >* this_ptr, const QList< QGesture* >* l, QList< QGesture* >* output) {
  new(output) QList< QGesture* >(this_ptr->operator+(*l));
}

QList< QGesture* >* qt_widgets_c_QList_QGesture_ptr_operator_assign(QList< QGesture* >* this_ptr, const QList< QGesture* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QGesture_ptr_operator_eq(const QList< QGesture* >* this_ptr, const QList< QGesture* >* l) {
  return this_ptr->operator==(*l);
}

QGesture** qt_widgets_c_QList_QGesture_ptr_operator_index(QList< QGesture* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QGesture* const * qt_widgets_c_QList_QGesture_ptr_operator_index_const(const QList< QGesture* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QGesture_ptr_operator_neq(const QList< QGesture* >* this_ptr, const QList< QGesture* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QGesture* >* qt_widgets_c_QList_QGesture_ptr_operator_shl_l(QList< QGesture* >* this_ptr, const QList< QGesture* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QGesture* >* qt_widgets_c_QList_QGesture_ptr_operator_shl_t(QList< QGesture* >* this_ptr, QGesture* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QGesture_ptr_pop_back(QList< QGesture* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QGesture_ptr_pop_front(QList< QGesture* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QGesture_ptr_prepend(QList< QGesture* >* this_ptr, QGesture* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QGesture_ptr_push_back(QList< QGesture* >* this_ptr, QGesture* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QGesture_ptr_push_front(QList< QGesture* >* this_ptr, QGesture* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QGesture_ptr_removeAll(QList< QGesture* >* this_ptr, QGesture* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QGesture_ptr_removeAt(QList< QGesture* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QGesture_ptr_removeFirst(QList< QGesture* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QGesture_ptr_removeLast(QList< QGesture* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QGesture_ptr_removeOne(QList< QGesture* >* this_ptr, QGesture* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QGesture_ptr_replace(QList< QGesture* >* this_ptr, int i, QGesture* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QGesture_ptr_reserve(QList< QGesture* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QGesture_ptr_size(const QList< QGesture* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QGesture_ptr_startsWith(const QList< QGesture* >* this_ptr, QGesture* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QGesture_ptr_swap_i_j(QList< QGesture* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QGesture_ptr_swap_other(QList< QGesture* >* this_ptr, QList< QGesture* >* other) {
  this_ptr->swap(*other);
}

QGesture* qt_widgets_c_QList_QGesture_ptr_takeAt(QList< QGesture* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QGesture* qt_widgets_c_QList_QGesture_ptr_takeFirst(QList< QGesture* >* this_ptr) {
  return this_ptr->takeFirst();
}

QGesture* qt_widgets_c_QList_QGesture_ptr_takeLast(QList< QGesture* >* this_ptr) {
  return this_ptr->takeLast();
}

QGesture* qt_widgets_c_QList_QGesture_ptr_value_i(const QList< QGesture* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QGesture* qt_widgets_c_QList_QGesture_ptr_value_i_defaultValue(const QList< QGesture* >* this_ptr, int i, QGesture* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_append_QGraphicsItem(QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_append_QList_QGraphicsItem_ptr(QList< QGraphicsItem* >* this_ptr, const QList< QGraphicsItem* >* t) {
  this_ptr->append(*t);
}

QGraphicsItem* const * qt_widgets_c_QList_QGraphicsItem_ptr_at(const QList< QGraphicsItem* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QGraphicsItem** qt_widgets_c_QList_QGraphicsItem_ptr_back(QList< QGraphicsItem* >* this_ptr) {
  return &this_ptr->back();
}

QGraphicsItem* const * qt_widgets_c_QList_QGraphicsItem_ptr_back_const(const QList< QGraphicsItem* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QGraphicsItem_ptr_clear(QList< QGraphicsItem* >* this_ptr) {
  this_ptr->clear();
}

QGraphicsItem* const * qt_widgets_c_QList_QGraphicsItem_ptr_constFirst(const QList< QGraphicsItem* >* this_ptr) {
  return &this_ptr->constFirst();
}

QGraphicsItem* const * qt_widgets_c_QList_QGraphicsItem_ptr_constLast(const QList< QGraphicsItem* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QGraphicsItem_ptr_constructor_l(const QList< QGraphicsItem* >* l, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(*l);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_constructor_no_args(QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >();
}

bool qt_widgets_c_QList_QGraphicsItem_ptr_contains(const QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QGraphicsItem_ptr_count_no_args(const QList< QGraphicsItem* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QGraphicsItem_ptr_count_t(const QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_destructor(QList< QGraphicsItem* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QGraphicsItem_ptr_empty(const QList< QGraphicsItem* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QGraphicsItem_ptr_endsWith(const QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t) {
  return this_ptr->endsWith(*t);
}

QGraphicsItem** qt_widgets_c_QList_QGraphicsItem_ptr_first(QList< QGraphicsItem* >* this_ptr) {
  return &this_ptr->first();
}

QGraphicsItem* const * qt_widgets_c_QList_QGraphicsItem_ptr_first_const(const QList< QGraphicsItem* >* this_ptr) {
  return &this_ptr->first();
}

QGraphicsItem** qt_widgets_c_QList_QGraphicsItem_ptr_front(QList< QGraphicsItem* >* this_ptr) {
  return &this_ptr->front();
}

QGraphicsItem* const * qt_widgets_c_QList_QGraphicsItem_ptr_front_const(const QList< QGraphicsItem* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QGraphicsItem_ptr_indexOf_t(const QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QGraphicsItem_ptr_indexOf_t_from(const QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_insert(QList< QGraphicsItem* >* this_ptr, int i, QGraphicsItem* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QGraphicsItem_ptr_isEmpty(const QList< QGraphicsItem* >* this_ptr) {
  return this_ptr->isEmpty();
}

QGraphicsItem** qt_widgets_c_QList_QGraphicsItem_ptr_last(QList< QGraphicsItem* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QGraphicsItem_ptr_lastIndexOf_t(const QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QGraphicsItem_ptr_lastIndexOf_t_from(const QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QGraphicsItem* const * qt_widgets_c_QList_QGraphicsItem_ptr_last_const(const QList< QGraphicsItem* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QGraphicsItem_ptr_length(const QList< QGraphicsItem* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QGraphicsItem_ptr_mid_to_output_pos(const QList< QGraphicsItem* >* this_ptr, int pos, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QGraphicsItem_ptr_mid_to_output_pos_length(const QList< QGraphicsItem* >* this_ptr, int pos, int length, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QGraphicsItem_ptr_move(QList< QGraphicsItem* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QGraphicsItem* >* qt_widgets_c_QList_QGraphicsItem_ptr_operator_add_assign_l(QList< QGraphicsItem* >* this_ptr, const QList< QGraphicsItem* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QGraphicsItem* >* qt_widgets_c_QList_QGraphicsItem_ptr_operator_add_assign_t(QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_operator_add_to_output(const QList< QGraphicsItem* >* this_ptr, const QList< QGraphicsItem* >* l, QList< QGraphicsItem* >* output) {
  new(output) QList< QGraphicsItem* >(this_ptr->operator+(*l));
}

QList< QGraphicsItem* >* qt_widgets_c_QList_QGraphicsItem_ptr_operator_assign(QList< QGraphicsItem* >* this_ptr, const QList< QGraphicsItem* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QGraphicsItem_ptr_operator_eq(const QList< QGraphicsItem* >* this_ptr, const QList< QGraphicsItem* >* l) {
  return this_ptr->operator==(*l);
}

QGraphicsItem** qt_widgets_c_QList_QGraphicsItem_ptr_operator_index(QList< QGraphicsItem* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QGraphicsItem* const * qt_widgets_c_QList_QGraphicsItem_ptr_operator_index_const(const QList< QGraphicsItem* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QGraphicsItem_ptr_operator_neq(const QList< QGraphicsItem* >* this_ptr, const QList< QGraphicsItem* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QGraphicsItem* >* qt_widgets_c_QList_QGraphicsItem_ptr_operator_shl_l(QList< QGraphicsItem* >* this_ptr, const QList< QGraphicsItem* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QGraphicsItem* >* qt_widgets_c_QList_QGraphicsItem_ptr_operator_shl_t(QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_pop_back(QList< QGraphicsItem* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QGraphicsItem_ptr_pop_front(QList< QGraphicsItem* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QGraphicsItem_ptr_prepend(QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_push_back(QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_push_front(QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QGraphicsItem_ptr_removeAll(QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_removeAt(QList< QGraphicsItem* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_removeFirst(QList< QGraphicsItem* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QGraphicsItem_ptr_removeLast(QList< QGraphicsItem* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QGraphicsItem_ptr_removeOne(QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_replace(QList< QGraphicsItem* >* this_ptr, int i, QGraphicsItem* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_reserve(QList< QGraphicsItem* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QGraphicsItem_ptr_size(const QList< QGraphicsItem* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QGraphicsItem_ptr_startsWith(const QList< QGraphicsItem* >* this_ptr, QGraphicsItem* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_swap_i_j(QList< QGraphicsItem* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QGraphicsItem_ptr_swap_other(QList< QGraphicsItem* >* this_ptr, QList< QGraphicsItem* >* other) {
  this_ptr->swap(*other);
}

QGraphicsItem* qt_widgets_c_QList_QGraphicsItem_ptr_takeAt(QList< QGraphicsItem* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QGraphicsItem* qt_widgets_c_QList_QGraphicsItem_ptr_takeFirst(QList< QGraphicsItem* >* this_ptr) {
  return this_ptr->takeFirst();
}

QGraphicsItem* qt_widgets_c_QList_QGraphicsItem_ptr_takeLast(QList< QGraphicsItem* >* this_ptr) {
  return this_ptr->takeLast();
}

QGraphicsItem* qt_widgets_c_QList_QGraphicsItem_ptr_value_i(const QList< QGraphicsItem* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QGraphicsItem* qt_widgets_c_QList_QGraphicsItem_ptr_value_i_defaultValue(const QList< QGraphicsItem* >* this_ptr, int i, QGraphicsItem* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_append_QGraphicsTransform(QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_append_QList_QGraphicsTransform_ptr(QList< QGraphicsTransform* >* this_ptr, const QList< QGraphicsTransform* >* t) {
  this_ptr->append(*t);
}

QGraphicsTransform* const * qt_widgets_c_QList_QGraphicsTransform_ptr_at(const QList< QGraphicsTransform* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QGraphicsTransform** qt_widgets_c_QList_QGraphicsTransform_ptr_back(QList< QGraphicsTransform* >* this_ptr) {
  return &this_ptr->back();
}

QGraphicsTransform* const * qt_widgets_c_QList_QGraphicsTransform_ptr_back_const(const QList< QGraphicsTransform* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_clear(QList< QGraphicsTransform* >* this_ptr) {
  this_ptr->clear();
}

QGraphicsTransform* const * qt_widgets_c_QList_QGraphicsTransform_ptr_constFirst(const QList< QGraphicsTransform* >* this_ptr) {
  return &this_ptr->constFirst();
}

QGraphicsTransform* const * qt_widgets_c_QList_QGraphicsTransform_ptr_constLast(const QList< QGraphicsTransform* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_constructor_l(const QList< QGraphicsTransform* >* l, QList< QGraphicsTransform* >* output) {
  new(output) QList< QGraphicsTransform* >(*l);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_constructor_no_args(QList< QGraphicsTransform* >* output) {
  new(output) QList< QGraphicsTransform* >();
}

bool qt_widgets_c_QList_QGraphicsTransform_ptr_contains(const QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QGraphicsTransform_ptr_count_no_args(const QList< QGraphicsTransform* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QGraphicsTransform_ptr_count_t(const QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_destructor(QList< QGraphicsTransform* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QGraphicsTransform_ptr_empty(const QList< QGraphicsTransform* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QGraphicsTransform_ptr_endsWith(const QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t) {
  return this_ptr->endsWith(*t);
}

QGraphicsTransform** qt_widgets_c_QList_QGraphicsTransform_ptr_first(QList< QGraphicsTransform* >* this_ptr) {
  return &this_ptr->first();
}

QGraphicsTransform* const * qt_widgets_c_QList_QGraphicsTransform_ptr_first_const(const QList< QGraphicsTransform* >* this_ptr) {
  return &this_ptr->first();
}

QGraphicsTransform** qt_widgets_c_QList_QGraphicsTransform_ptr_front(QList< QGraphicsTransform* >* this_ptr) {
  return &this_ptr->front();
}

QGraphicsTransform* const * qt_widgets_c_QList_QGraphicsTransform_ptr_front_const(const QList< QGraphicsTransform* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QGraphicsTransform_ptr_indexOf_t(const QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QGraphicsTransform_ptr_indexOf_t_from(const QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_insert(QList< QGraphicsTransform* >* this_ptr, int i, QGraphicsTransform* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QGraphicsTransform_ptr_isEmpty(const QList< QGraphicsTransform* >* this_ptr) {
  return this_ptr->isEmpty();
}

QGraphicsTransform** qt_widgets_c_QList_QGraphicsTransform_ptr_last(QList< QGraphicsTransform* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QGraphicsTransform_ptr_lastIndexOf_t(const QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QGraphicsTransform_ptr_lastIndexOf_t_from(const QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QGraphicsTransform* const * qt_widgets_c_QList_QGraphicsTransform_ptr_last_const(const QList< QGraphicsTransform* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QGraphicsTransform_ptr_length(const QList< QGraphicsTransform* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_mid_to_output_pos(const QList< QGraphicsTransform* >* this_ptr, int pos, QList< QGraphicsTransform* >* output) {
  new(output) QList< QGraphicsTransform* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_mid_to_output_pos_length(const QList< QGraphicsTransform* >* this_ptr, int pos, int length, QList< QGraphicsTransform* >* output) {
  new(output) QList< QGraphicsTransform* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_move(QList< QGraphicsTransform* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QGraphicsTransform* >* qt_widgets_c_QList_QGraphicsTransform_ptr_operator_add_assign_l(QList< QGraphicsTransform* >* this_ptr, const QList< QGraphicsTransform* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QGraphicsTransform* >* qt_widgets_c_QList_QGraphicsTransform_ptr_operator_add_assign_t(QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_operator_add_to_output(const QList< QGraphicsTransform* >* this_ptr, const QList< QGraphicsTransform* >* l, QList< QGraphicsTransform* >* output) {
  new(output) QList< QGraphicsTransform* >(this_ptr->operator+(*l));
}

QList< QGraphicsTransform* >* qt_widgets_c_QList_QGraphicsTransform_ptr_operator_assign(QList< QGraphicsTransform* >* this_ptr, const QList< QGraphicsTransform* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QGraphicsTransform_ptr_operator_eq(const QList< QGraphicsTransform* >* this_ptr, const QList< QGraphicsTransform* >* l) {
  return this_ptr->operator==(*l);
}

QGraphicsTransform** qt_widgets_c_QList_QGraphicsTransform_ptr_operator_index(QList< QGraphicsTransform* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QGraphicsTransform* const * qt_widgets_c_QList_QGraphicsTransform_ptr_operator_index_const(const QList< QGraphicsTransform* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QGraphicsTransform_ptr_operator_neq(const QList< QGraphicsTransform* >* this_ptr, const QList< QGraphicsTransform* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QGraphicsTransform* >* qt_widgets_c_QList_QGraphicsTransform_ptr_operator_shl_l(QList< QGraphicsTransform* >* this_ptr, const QList< QGraphicsTransform* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QGraphicsTransform* >* qt_widgets_c_QList_QGraphicsTransform_ptr_operator_shl_t(QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_pop_back(QList< QGraphicsTransform* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_pop_front(QList< QGraphicsTransform* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_prepend(QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_push_back(QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_push_front(QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QGraphicsTransform_ptr_removeAll(QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_removeAt(QList< QGraphicsTransform* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_removeFirst(QList< QGraphicsTransform* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_removeLast(QList< QGraphicsTransform* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QGraphicsTransform_ptr_removeOne(QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_replace(QList< QGraphicsTransform* >* this_ptr, int i, QGraphicsTransform* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_reserve(QList< QGraphicsTransform* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QGraphicsTransform_ptr_size(const QList< QGraphicsTransform* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QGraphicsTransform_ptr_startsWith(const QList< QGraphicsTransform* >* this_ptr, QGraphicsTransform* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_swap_i_j(QList< QGraphicsTransform* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QGraphicsTransform_ptr_swap_other(QList< QGraphicsTransform* >* this_ptr, QList< QGraphicsTransform* >* other) {
  this_ptr->swap(*other);
}

QGraphicsTransform* qt_widgets_c_QList_QGraphicsTransform_ptr_takeAt(QList< QGraphicsTransform* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QGraphicsTransform* qt_widgets_c_QList_QGraphicsTransform_ptr_takeFirst(QList< QGraphicsTransform* >* this_ptr) {
  return this_ptr->takeFirst();
}

QGraphicsTransform* qt_widgets_c_QList_QGraphicsTransform_ptr_takeLast(QList< QGraphicsTransform* >* this_ptr) {
  return this_ptr->takeLast();
}

QGraphicsTransform* qt_widgets_c_QList_QGraphicsTransform_ptr_value_i(const QList< QGraphicsTransform* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QGraphicsTransform* qt_widgets_c_QList_QGraphicsTransform_ptr_value_i_defaultValue(const QList< QGraphicsTransform* >* this_ptr, int i, QGraphicsTransform* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QGraphicsView_ptr_append_QGraphicsView(QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QGraphicsView_ptr_append_QList_QGraphicsView_ptr(QList< QGraphicsView* >* this_ptr, const QList< QGraphicsView* >* t) {
  this_ptr->append(*t);
}

QGraphicsView* const * qt_widgets_c_QList_QGraphicsView_ptr_at(const QList< QGraphicsView* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QGraphicsView** qt_widgets_c_QList_QGraphicsView_ptr_back(QList< QGraphicsView* >* this_ptr) {
  return &this_ptr->back();
}

QGraphicsView* const * qt_widgets_c_QList_QGraphicsView_ptr_back_const(const QList< QGraphicsView* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QGraphicsView_ptr_clear(QList< QGraphicsView* >* this_ptr) {
  this_ptr->clear();
}

QGraphicsView* const * qt_widgets_c_QList_QGraphicsView_ptr_constFirst(const QList< QGraphicsView* >* this_ptr) {
  return &this_ptr->constFirst();
}

QGraphicsView* const * qt_widgets_c_QList_QGraphicsView_ptr_constLast(const QList< QGraphicsView* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QGraphicsView_ptr_constructor_l(const QList< QGraphicsView* >* l, QList< QGraphicsView* >* output) {
  new(output) QList< QGraphicsView* >(*l);
}

void qt_widgets_c_QList_QGraphicsView_ptr_constructor_no_args(QList< QGraphicsView* >* output) {
  new(output) QList< QGraphicsView* >();
}

bool qt_widgets_c_QList_QGraphicsView_ptr_contains(const QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QGraphicsView_ptr_count_no_args(const QList< QGraphicsView* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QGraphicsView_ptr_count_t(const QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QGraphicsView_ptr_destructor(QList< QGraphicsView* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QGraphicsView_ptr_empty(const QList< QGraphicsView* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QGraphicsView_ptr_endsWith(const QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t) {
  return this_ptr->endsWith(*t);
}

QGraphicsView** qt_widgets_c_QList_QGraphicsView_ptr_first(QList< QGraphicsView* >* this_ptr) {
  return &this_ptr->first();
}

QGraphicsView* const * qt_widgets_c_QList_QGraphicsView_ptr_first_const(const QList< QGraphicsView* >* this_ptr) {
  return &this_ptr->first();
}

QGraphicsView** qt_widgets_c_QList_QGraphicsView_ptr_front(QList< QGraphicsView* >* this_ptr) {
  return &this_ptr->front();
}

QGraphicsView* const * qt_widgets_c_QList_QGraphicsView_ptr_front_const(const QList< QGraphicsView* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QGraphicsView_ptr_indexOf_t(const QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QGraphicsView_ptr_indexOf_t_from(const QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QGraphicsView_ptr_insert(QList< QGraphicsView* >* this_ptr, int i, QGraphicsView* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QGraphicsView_ptr_isEmpty(const QList< QGraphicsView* >* this_ptr) {
  return this_ptr->isEmpty();
}

QGraphicsView** qt_widgets_c_QList_QGraphicsView_ptr_last(QList< QGraphicsView* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QGraphicsView_ptr_lastIndexOf_t(const QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QGraphicsView_ptr_lastIndexOf_t_from(const QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QGraphicsView* const * qt_widgets_c_QList_QGraphicsView_ptr_last_const(const QList< QGraphicsView* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QGraphicsView_ptr_length(const QList< QGraphicsView* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QGraphicsView_ptr_mid_to_output_pos(const QList< QGraphicsView* >* this_ptr, int pos, QList< QGraphicsView* >* output) {
  new(output) QList< QGraphicsView* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QGraphicsView_ptr_mid_to_output_pos_length(const QList< QGraphicsView* >* this_ptr, int pos, int length, QList< QGraphicsView* >* output) {
  new(output) QList< QGraphicsView* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QGraphicsView_ptr_move(QList< QGraphicsView* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QGraphicsView* >* qt_widgets_c_QList_QGraphicsView_ptr_operator_add_assign_l(QList< QGraphicsView* >* this_ptr, const QList< QGraphicsView* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QGraphicsView* >* qt_widgets_c_QList_QGraphicsView_ptr_operator_add_assign_t(QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QGraphicsView_ptr_operator_add_to_output(const QList< QGraphicsView* >* this_ptr, const QList< QGraphicsView* >* l, QList< QGraphicsView* >* output) {
  new(output) QList< QGraphicsView* >(this_ptr->operator+(*l));
}

QList< QGraphicsView* >* qt_widgets_c_QList_QGraphicsView_ptr_operator_assign(QList< QGraphicsView* >* this_ptr, const QList< QGraphicsView* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QGraphicsView_ptr_operator_eq(const QList< QGraphicsView* >* this_ptr, const QList< QGraphicsView* >* l) {
  return this_ptr->operator==(*l);
}

QGraphicsView** qt_widgets_c_QList_QGraphicsView_ptr_operator_index(QList< QGraphicsView* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QGraphicsView* const * qt_widgets_c_QList_QGraphicsView_ptr_operator_index_const(const QList< QGraphicsView* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QGraphicsView_ptr_operator_neq(const QList< QGraphicsView* >* this_ptr, const QList< QGraphicsView* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QGraphicsView* >* qt_widgets_c_QList_QGraphicsView_ptr_operator_shl_l(QList< QGraphicsView* >* this_ptr, const QList< QGraphicsView* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QGraphicsView* >* qt_widgets_c_QList_QGraphicsView_ptr_operator_shl_t(QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QGraphicsView_ptr_pop_back(QList< QGraphicsView* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QGraphicsView_ptr_pop_front(QList< QGraphicsView* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QGraphicsView_ptr_prepend(QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QGraphicsView_ptr_push_back(QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QGraphicsView_ptr_push_front(QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QGraphicsView_ptr_removeAll(QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QGraphicsView_ptr_removeAt(QList< QGraphicsView* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QGraphicsView_ptr_removeFirst(QList< QGraphicsView* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QGraphicsView_ptr_removeLast(QList< QGraphicsView* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QGraphicsView_ptr_removeOne(QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QGraphicsView_ptr_replace(QList< QGraphicsView* >* this_ptr, int i, QGraphicsView* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QGraphicsView_ptr_reserve(QList< QGraphicsView* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QGraphicsView_ptr_size(const QList< QGraphicsView* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QGraphicsView_ptr_startsWith(const QList< QGraphicsView* >* this_ptr, QGraphicsView* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QGraphicsView_ptr_swap_i_j(QList< QGraphicsView* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QGraphicsView_ptr_swap_other(QList< QGraphicsView* >* this_ptr, QList< QGraphicsView* >* other) {
  this_ptr->swap(*other);
}

QGraphicsView* qt_widgets_c_QList_QGraphicsView_ptr_takeAt(QList< QGraphicsView* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QGraphicsView* qt_widgets_c_QList_QGraphicsView_ptr_takeFirst(QList< QGraphicsView* >* this_ptr) {
  return this_ptr->takeFirst();
}

QGraphicsView* qt_widgets_c_QList_QGraphicsView_ptr_takeLast(QList< QGraphicsView* >* this_ptr) {
  return this_ptr->takeLast();
}

QGraphicsView* qt_widgets_c_QList_QGraphicsView_ptr_value_i(const QList< QGraphicsView* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QGraphicsView* qt_widgets_c_QList_QGraphicsView_ptr_value_i_defaultValue(const QList< QGraphicsView* >* this_ptr, int i, QGraphicsView* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_append_QGraphicsWidget(QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_append_QList_QGraphicsWidget_ptr(QList< QGraphicsWidget* >* this_ptr, const QList< QGraphicsWidget* >* t) {
  this_ptr->append(*t);
}

QGraphicsWidget* const * qt_widgets_c_QList_QGraphicsWidget_ptr_at(const QList< QGraphicsWidget* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QGraphicsWidget** qt_widgets_c_QList_QGraphicsWidget_ptr_back(QList< QGraphicsWidget* >* this_ptr) {
  return &this_ptr->back();
}

QGraphicsWidget* const * qt_widgets_c_QList_QGraphicsWidget_ptr_back_const(const QList< QGraphicsWidget* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_clear(QList< QGraphicsWidget* >* this_ptr) {
  this_ptr->clear();
}

QGraphicsWidget* const * qt_widgets_c_QList_QGraphicsWidget_ptr_constFirst(const QList< QGraphicsWidget* >* this_ptr) {
  return &this_ptr->constFirst();
}

QGraphicsWidget* const * qt_widgets_c_QList_QGraphicsWidget_ptr_constLast(const QList< QGraphicsWidget* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_constructor_l(const QList< QGraphicsWidget* >* l, QList< QGraphicsWidget* >* output) {
  new(output) QList< QGraphicsWidget* >(*l);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_constructor_no_args(QList< QGraphicsWidget* >* output) {
  new(output) QList< QGraphicsWidget* >();
}

bool qt_widgets_c_QList_QGraphicsWidget_ptr_contains(const QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QGraphicsWidget_ptr_count_no_args(const QList< QGraphicsWidget* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QGraphicsWidget_ptr_count_t(const QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_destructor(QList< QGraphicsWidget* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QGraphicsWidget_ptr_empty(const QList< QGraphicsWidget* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QGraphicsWidget_ptr_endsWith(const QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t) {
  return this_ptr->endsWith(*t);
}

QGraphicsWidget** qt_widgets_c_QList_QGraphicsWidget_ptr_first(QList< QGraphicsWidget* >* this_ptr) {
  return &this_ptr->first();
}

QGraphicsWidget* const * qt_widgets_c_QList_QGraphicsWidget_ptr_first_const(const QList< QGraphicsWidget* >* this_ptr) {
  return &this_ptr->first();
}

QGraphicsWidget** qt_widgets_c_QList_QGraphicsWidget_ptr_front(QList< QGraphicsWidget* >* this_ptr) {
  return &this_ptr->front();
}

QGraphicsWidget* const * qt_widgets_c_QList_QGraphicsWidget_ptr_front_const(const QList< QGraphicsWidget* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QGraphicsWidget_ptr_indexOf_t(const QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QGraphicsWidget_ptr_indexOf_t_from(const QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_insert(QList< QGraphicsWidget* >* this_ptr, int i, QGraphicsWidget* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QGraphicsWidget_ptr_isEmpty(const QList< QGraphicsWidget* >* this_ptr) {
  return this_ptr->isEmpty();
}

QGraphicsWidget** qt_widgets_c_QList_QGraphicsWidget_ptr_last(QList< QGraphicsWidget* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QGraphicsWidget_ptr_lastIndexOf_t(const QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QGraphicsWidget_ptr_lastIndexOf_t_from(const QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QGraphicsWidget* const * qt_widgets_c_QList_QGraphicsWidget_ptr_last_const(const QList< QGraphicsWidget* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QGraphicsWidget_ptr_length(const QList< QGraphicsWidget* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_mid_to_output_pos(const QList< QGraphicsWidget* >* this_ptr, int pos, QList< QGraphicsWidget* >* output) {
  new(output) QList< QGraphicsWidget* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_mid_to_output_pos_length(const QList< QGraphicsWidget* >* this_ptr, int pos, int length, QList< QGraphicsWidget* >* output) {
  new(output) QList< QGraphicsWidget* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_move(QList< QGraphicsWidget* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QGraphicsWidget* >* qt_widgets_c_QList_QGraphicsWidget_ptr_operator_add_assign_l(QList< QGraphicsWidget* >* this_ptr, const QList< QGraphicsWidget* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QGraphicsWidget* >* qt_widgets_c_QList_QGraphicsWidget_ptr_operator_add_assign_t(QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_operator_add_to_output(const QList< QGraphicsWidget* >* this_ptr, const QList< QGraphicsWidget* >* l, QList< QGraphicsWidget* >* output) {
  new(output) QList< QGraphicsWidget* >(this_ptr->operator+(*l));
}

QList< QGraphicsWidget* >* qt_widgets_c_QList_QGraphicsWidget_ptr_operator_assign(QList< QGraphicsWidget* >* this_ptr, const QList< QGraphicsWidget* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QGraphicsWidget_ptr_operator_eq(const QList< QGraphicsWidget* >* this_ptr, const QList< QGraphicsWidget* >* l) {
  return this_ptr->operator==(*l);
}

QGraphicsWidget** qt_widgets_c_QList_QGraphicsWidget_ptr_operator_index(QList< QGraphicsWidget* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QGraphicsWidget* const * qt_widgets_c_QList_QGraphicsWidget_ptr_operator_index_const(const QList< QGraphicsWidget* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QGraphicsWidget_ptr_operator_neq(const QList< QGraphicsWidget* >* this_ptr, const QList< QGraphicsWidget* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QGraphicsWidget* >* qt_widgets_c_QList_QGraphicsWidget_ptr_operator_shl_l(QList< QGraphicsWidget* >* this_ptr, const QList< QGraphicsWidget* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QGraphicsWidget* >* qt_widgets_c_QList_QGraphicsWidget_ptr_operator_shl_t(QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_pop_back(QList< QGraphicsWidget* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_pop_front(QList< QGraphicsWidget* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_prepend(QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_push_back(QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_push_front(QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QGraphicsWidget_ptr_removeAll(QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_removeAt(QList< QGraphicsWidget* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_removeFirst(QList< QGraphicsWidget* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_removeLast(QList< QGraphicsWidget* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QGraphicsWidget_ptr_removeOne(QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_replace(QList< QGraphicsWidget* >* this_ptr, int i, QGraphicsWidget* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_reserve(QList< QGraphicsWidget* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QGraphicsWidget_ptr_size(const QList< QGraphicsWidget* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QGraphicsWidget_ptr_startsWith(const QList< QGraphicsWidget* >* this_ptr, QGraphicsWidget* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_swap_i_j(QList< QGraphicsWidget* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QGraphicsWidget_ptr_swap_other(QList< QGraphicsWidget* >* this_ptr, QList< QGraphicsWidget* >* other) {
  this_ptr->swap(*other);
}

QGraphicsWidget* qt_widgets_c_QList_QGraphicsWidget_ptr_takeAt(QList< QGraphicsWidget* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QGraphicsWidget* qt_widgets_c_QList_QGraphicsWidget_ptr_takeFirst(QList< QGraphicsWidget* >* this_ptr) {
  return this_ptr->takeFirst();
}

QGraphicsWidget* qt_widgets_c_QList_QGraphicsWidget_ptr_takeLast(QList< QGraphicsWidget* >* this_ptr) {
  return this_ptr->takeLast();
}

QGraphicsWidget* qt_widgets_c_QList_QGraphicsWidget_ptr_value_i(const QList< QGraphicsWidget* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QGraphicsWidget* qt_widgets_c_QList_QGraphicsWidget_ptr_value_i_defaultValue(const QList< QGraphicsWidget* >* this_ptr, int i, QGraphicsWidget* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_append_QListWidgetItem(QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_append_QList_QListWidgetItem_ptr(QList< QListWidgetItem* >* this_ptr, const QList< QListWidgetItem* >* t) {
  this_ptr->append(*t);
}

QListWidgetItem* const * qt_widgets_c_QList_QListWidgetItem_ptr_at(const QList< QListWidgetItem* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QListWidgetItem** qt_widgets_c_QList_QListWidgetItem_ptr_back(QList< QListWidgetItem* >* this_ptr) {
  return &this_ptr->back();
}

QListWidgetItem* const * qt_widgets_c_QList_QListWidgetItem_ptr_back_const(const QList< QListWidgetItem* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QListWidgetItem_ptr_clear(QList< QListWidgetItem* >* this_ptr) {
  this_ptr->clear();
}

QListWidgetItem* const * qt_widgets_c_QList_QListWidgetItem_ptr_constFirst(const QList< QListWidgetItem* >* this_ptr) {
  return &this_ptr->constFirst();
}

QListWidgetItem* const * qt_widgets_c_QList_QListWidgetItem_ptr_constLast(const QList< QListWidgetItem* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QListWidgetItem_ptr_constructor_l(const QList< QListWidgetItem* >* l, QList< QListWidgetItem* >* output) {
  new(output) QList< QListWidgetItem* >(*l);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_constructor_no_args(QList< QListWidgetItem* >* output) {
  new(output) QList< QListWidgetItem* >();
}

bool qt_widgets_c_QList_QListWidgetItem_ptr_contains(const QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QListWidgetItem_ptr_count_no_args(const QList< QListWidgetItem* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QListWidgetItem_ptr_count_t(const QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_destructor(QList< QListWidgetItem* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QListWidgetItem_ptr_empty(const QList< QListWidgetItem* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QListWidgetItem_ptr_endsWith(const QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t) {
  return this_ptr->endsWith(*t);
}

QListWidgetItem** qt_widgets_c_QList_QListWidgetItem_ptr_first(QList< QListWidgetItem* >* this_ptr) {
  return &this_ptr->first();
}

QListWidgetItem* const * qt_widgets_c_QList_QListWidgetItem_ptr_first_const(const QList< QListWidgetItem* >* this_ptr) {
  return &this_ptr->first();
}

QListWidgetItem** qt_widgets_c_QList_QListWidgetItem_ptr_front(QList< QListWidgetItem* >* this_ptr) {
  return &this_ptr->front();
}

QListWidgetItem* const * qt_widgets_c_QList_QListWidgetItem_ptr_front_const(const QList< QListWidgetItem* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QListWidgetItem_ptr_indexOf_t(const QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QListWidgetItem_ptr_indexOf_t_from(const QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_insert(QList< QListWidgetItem* >* this_ptr, int i, QListWidgetItem* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QListWidgetItem_ptr_isEmpty(const QList< QListWidgetItem* >* this_ptr) {
  return this_ptr->isEmpty();
}

QListWidgetItem** qt_widgets_c_QList_QListWidgetItem_ptr_last(QList< QListWidgetItem* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QListWidgetItem_ptr_lastIndexOf_t(const QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QListWidgetItem_ptr_lastIndexOf_t_from(const QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QListWidgetItem* const * qt_widgets_c_QList_QListWidgetItem_ptr_last_const(const QList< QListWidgetItem* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QListWidgetItem_ptr_length(const QList< QListWidgetItem* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QListWidgetItem_ptr_mid_to_output_pos(const QList< QListWidgetItem* >* this_ptr, int pos, QList< QListWidgetItem* >* output) {
  new(output) QList< QListWidgetItem* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QListWidgetItem_ptr_mid_to_output_pos_length(const QList< QListWidgetItem* >* this_ptr, int pos, int length, QList< QListWidgetItem* >* output) {
  new(output) QList< QListWidgetItem* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QListWidgetItem_ptr_move(QList< QListWidgetItem* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QListWidgetItem* >* qt_widgets_c_QList_QListWidgetItem_ptr_operator_add_assign_l(QList< QListWidgetItem* >* this_ptr, const QList< QListWidgetItem* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QListWidgetItem* >* qt_widgets_c_QList_QListWidgetItem_ptr_operator_add_assign_t(QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_operator_add_to_output(const QList< QListWidgetItem* >* this_ptr, const QList< QListWidgetItem* >* l, QList< QListWidgetItem* >* output) {
  new(output) QList< QListWidgetItem* >(this_ptr->operator+(*l));
}

QList< QListWidgetItem* >* qt_widgets_c_QList_QListWidgetItem_ptr_operator_assign(QList< QListWidgetItem* >* this_ptr, const QList< QListWidgetItem* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QListWidgetItem_ptr_operator_eq(const QList< QListWidgetItem* >* this_ptr, const QList< QListWidgetItem* >* l) {
  return this_ptr->operator==(*l);
}

QListWidgetItem** qt_widgets_c_QList_QListWidgetItem_ptr_operator_index(QList< QListWidgetItem* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QListWidgetItem* const * qt_widgets_c_QList_QListWidgetItem_ptr_operator_index_const(const QList< QListWidgetItem* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QListWidgetItem_ptr_operator_neq(const QList< QListWidgetItem* >* this_ptr, const QList< QListWidgetItem* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QListWidgetItem* >* qt_widgets_c_QList_QListWidgetItem_ptr_operator_shl_l(QList< QListWidgetItem* >* this_ptr, const QList< QListWidgetItem* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QListWidgetItem* >* qt_widgets_c_QList_QListWidgetItem_ptr_operator_shl_t(QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_pop_back(QList< QListWidgetItem* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QListWidgetItem_ptr_pop_front(QList< QListWidgetItem* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QListWidgetItem_ptr_prepend(QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_push_back(QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_push_front(QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QListWidgetItem_ptr_removeAll(QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_removeAt(QList< QListWidgetItem* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_removeFirst(QList< QListWidgetItem* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QListWidgetItem_ptr_removeLast(QList< QListWidgetItem* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QListWidgetItem_ptr_removeOne(QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_replace(QList< QListWidgetItem* >* this_ptr, int i, QListWidgetItem* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_reserve(QList< QListWidgetItem* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QListWidgetItem_ptr_size(const QList< QListWidgetItem* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QListWidgetItem_ptr_startsWith(const QList< QListWidgetItem* >* this_ptr, QListWidgetItem* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_swap_i_j(QList< QListWidgetItem* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QListWidgetItem_ptr_swap_other(QList< QListWidgetItem* >* this_ptr, QList< QListWidgetItem* >* other) {
  this_ptr->swap(*other);
}

QListWidgetItem* qt_widgets_c_QList_QListWidgetItem_ptr_takeAt(QList< QListWidgetItem* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QListWidgetItem* qt_widgets_c_QList_QListWidgetItem_ptr_takeFirst(QList< QListWidgetItem* >* this_ptr) {
  return this_ptr->takeFirst();
}

QListWidgetItem* qt_widgets_c_QList_QListWidgetItem_ptr_takeLast(QList< QListWidgetItem* >* this_ptr) {
  return this_ptr->takeLast();
}

QListWidgetItem* qt_widgets_c_QList_QListWidgetItem_ptr_value_i(const QList< QListWidgetItem* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QListWidgetItem* qt_widgets_c_QList_QListWidgetItem_ptr_value_i_defaultValue(const QList< QListWidgetItem* >* this_ptr, int i, QListWidgetItem* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_append_QList_QMdiSubWindow_ptr(QList< QMdiSubWindow* >* this_ptr, const QList< QMdiSubWindow* >* t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_append_QMdiSubWindow(QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t) {
  this_ptr->append(*t);
}

QMdiSubWindow* const * qt_widgets_c_QList_QMdiSubWindow_ptr_at(const QList< QMdiSubWindow* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QMdiSubWindow** qt_widgets_c_QList_QMdiSubWindow_ptr_back(QList< QMdiSubWindow* >* this_ptr) {
  return &this_ptr->back();
}

QMdiSubWindow* const * qt_widgets_c_QList_QMdiSubWindow_ptr_back_const(const QList< QMdiSubWindow* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_clear(QList< QMdiSubWindow* >* this_ptr) {
  this_ptr->clear();
}

QMdiSubWindow* const * qt_widgets_c_QList_QMdiSubWindow_ptr_constFirst(const QList< QMdiSubWindow* >* this_ptr) {
  return &this_ptr->constFirst();
}

QMdiSubWindow* const * qt_widgets_c_QList_QMdiSubWindow_ptr_constLast(const QList< QMdiSubWindow* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_constructor_l(const QList< QMdiSubWindow* >* l, QList< QMdiSubWindow* >* output) {
  new(output) QList< QMdiSubWindow* >(*l);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_constructor_no_args(QList< QMdiSubWindow* >* output) {
  new(output) QList< QMdiSubWindow* >();
}

bool qt_widgets_c_QList_QMdiSubWindow_ptr_contains(const QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QMdiSubWindow_ptr_count_no_args(const QList< QMdiSubWindow* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QMdiSubWindow_ptr_count_t(const QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_destructor(QList< QMdiSubWindow* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QMdiSubWindow_ptr_empty(const QList< QMdiSubWindow* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QMdiSubWindow_ptr_endsWith(const QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t) {
  return this_ptr->endsWith(*t);
}

QMdiSubWindow** qt_widgets_c_QList_QMdiSubWindow_ptr_first(QList< QMdiSubWindow* >* this_ptr) {
  return &this_ptr->first();
}

QMdiSubWindow* const * qt_widgets_c_QList_QMdiSubWindow_ptr_first_const(const QList< QMdiSubWindow* >* this_ptr) {
  return &this_ptr->first();
}

QMdiSubWindow** qt_widgets_c_QList_QMdiSubWindow_ptr_front(QList< QMdiSubWindow* >* this_ptr) {
  return &this_ptr->front();
}

QMdiSubWindow* const * qt_widgets_c_QList_QMdiSubWindow_ptr_front_const(const QList< QMdiSubWindow* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QMdiSubWindow_ptr_indexOf_t(const QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QMdiSubWindow_ptr_indexOf_t_from(const QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_insert(QList< QMdiSubWindow* >* this_ptr, int i, QMdiSubWindow* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QMdiSubWindow_ptr_isEmpty(const QList< QMdiSubWindow* >* this_ptr) {
  return this_ptr->isEmpty();
}

QMdiSubWindow** qt_widgets_c_QList_QMdiSubWindow_ptr_last(QList< QMdiSubWindow* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QMdiSubWindow_ptr_lastIndexOf_t(const QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QMdiSubWindow_ptr_lastIndexOf_t_from(const QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QMdiSubWindow* const * qt_widgets_c_QList_QMdiSubWindow_ptr_last_const(const QList< QMdiSubWindow* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QMdiSubWindow_ptr_length(const QList< QMdiSubWindow* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_mid_to_output_pos(const QList< QMdiSubWindow* >* this_ptr, int pos, QList< QMdiSubWindow* >* output) {
  new(output) QList< QMdiSubWindow* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_mid_to_output_pos_length(const QList< QMdiSubWindow* >* this_ptr, int pos, int length, QList< QMdiSubWindow* >* output) {
  new(output) QList< QMdiSubWindow* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_move(QList< QMdiSubWindow* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QMdiSubWindow* >* qt_widgets_c_QList_QMdiSubWindow_ptr_operator_add_assign_l(QList< QMdiSubWindow* >* this_ptr, const QList< QMdiSubWindow* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QMdiSubWindow* >* qt_widgets_c_QList_QMdiSubWindow_ptr_operator_add_assign_t(QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_operator_add_to_output(const QList< QMdiSubWindow* >* this_ptr, const QList< QMdiSubWindow* >* l, QList< QMdiSubWindow* >* output) {
  new(output) QList< QMdiSubWindow* >(this_ptr->operator+(*l));
}

QList< QMdiSubWindow* >* qt_widgets_c_QList_QMdiSubWindow_ptr_operator_assign(QList< QMdiSubWindow* >* this_ptr, const QList< QMdiSubWindow* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QMdiSubWindow_ptr_operator_eq(const QList< QMdiSubWindow* >* this_ptr, const QList< QMdiSubWindow* >* l) {
  return this_ptr->operator==(*l);
}

QMdiSubWindow** qt_widgets_c_QList_QMdiSubWindow_ptr_operator_index(QList< QMdiSubWindow* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QMdiSubWindow* const * qt_widgets_c_QList_QMdiSubWindow_ptr_operator_index_const(const QList< QMdiSubWindow* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QMdiSubWindow_ptr_operator_neq(const QList< QMdiSubWindow* >* this_ptr, const QList< QMdiSubWindow* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QMdiSubWindow* >* qt_widgets_c_QList_QMdiSubWindow_ptr_operator_shl_l(QList< QMdiSubWindow* >* this_ptr, const QList< QMdiSubWindow* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QMdiSubWindow* >* qt_widgets_c_QList_QMdiSubWindow_ptr_operator_shl_t(QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_pop_back(QList< QMdiSubWindow* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_pop_front(QList< QMdiSubWindow* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_prepend(QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_push_back(QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_push_front(QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QMdiSubWindow_ptr_removeAll(QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_removeAt(QList< QMdiSubWindow* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_removeFirst(QList< QMdiSubWindow* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_removeLast(QList< QMdiSubWindow* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QMdiSubWindow_ptr_removeOne(QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_replace(QList< QMdiSubWindow* >* this_ptr, int i, QMdiSubWindow* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_reserve(QList< QMdiSubWindow* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QMdiSubWindow_ptr_size(const QList< QMdiSubWindow* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QMdiSubWindow_ptr_startsWith(const QList< QMdiSubWindow* >* this_ptr, QMdiSubWindow* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_swap_i_j(QList< QMdiSubWindow* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QMdiSubWindow_ptr_swap_other(QList< QMdiSubWindow* >* this_ptr, QList< QMdiSubWindow* >* other) {
  this_ptr->swap(*other);
}

QMdiSubWindow* qt_widgets_c_QList_QMdiSubWindow_ptr_takeAt(QList< QMdiSubWindow* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QMdiSubWindow* qt_widgets_c_QList_QMdiSubWindow_ptr_takeFirst(QList< QMdiSubWindow* >* this_ptr) {
  return this_ptr->takeFirst();
}

QMdiSubWindow* qt_widgets_c_QList_QMdiSubWindow_ptr_takeLast(QList< QMdiSubWindow* >* this_ptr) {
  return this_ptr->takeLast();
}

QMdiSubWindow* qt_widgets_c_QList_QMdiSubWindow_ptr_value_i(const QList< QMdiSubWindow* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QMdiSubWindow* qt_widgets_c_QList_QMdiSubWindow_ptr_value_i_defaultValue(const QList< QMdiSubWindow* >* this_ptr, int i, QMdiSubWindow* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QPair_double_QPointF_append_QList_QPair_double_QPointF(QList< QPair< double, QPointF > >* this_ptr, const QList< QPair< double, QPointF > >* t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QPair_double_QPointF_append_QPair_double_QPointF(QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t) {
  this_ptr->append(*t);
}

const QPair< double, QPointF >* qt_widgets_c_QList_QPair_double_QPointF_at(const QList< QPair< double, QPointF > >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QPair< double, QPointF >* qt_widgets_c_QList_QPair_double_QPointF_back(QList< QPair< double, QPointF > >* this_ptr) {
  return &this_ptr->back();
}

const QPair< double, QPointF >* qt_widgets_c_QList_QPair_double_QPointF_back_const(const QList< QPair< double, QPointF > >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QPair_double_QPointF_clear(QList< QPair< double, QPointF > >* this_ptr) {
  this_ptr->clear();
}

const QPair< double, QPointF >* qt_widgets_c_QList_QPair_double_QPointF_constFirst(const QList< QPair< double, QPointF > >* this_ptr) {
  return &this_ptr->constFirst();
}

const QPair< double, QPointF >* qt_widgets_c_QList_QPair_double_QPointF_constLast(const QList< QPair< double, QPointF > >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QPair_double_QPointF_constructor_l(const QList< QPair< double, QPointF > >* l, QList< QPair< double, QPointF > >* output) {
  new(output) QList< QPair< double, QPointF > >(*l);
}

void qt_widgets_c_QList_QPair_double_QPointF_constructor_no_args(QList< QPair< double, QPointF > >* output) {
  new(output) QList< QPair< double, QPointF > >();
}

bool qt_widgets_c_QList_QPair_double_QPointF_contains(const QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QPair_double_QPointF_count_no_args(const QList< QPair< double, QPointF > >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QPair_double_QPointF_count_t(const QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QPair_double_QPointF_destructor(QList< QPair< double, QPointF > >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QPair_double_QPointF_empty(const QList< QPair< double, QPointF > >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QPair_double_QPointF_endsWith(const QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t) {
  return this_ptr->endsWith(*t);
}

QPair< double, QPointF >* qt_widgets_c_QList_QPair_double_QPointF_first(QList< QPair< double, QPointF > >* this_ptr) {
  return &this_ptr->first();
}

const QPair< double, QPointF >* qt_widgets_c_QList_QPair_double_QPointF_first_const(const QList< QPair< double, QPointF > >* this_ptr) {
  return &this_ptr->first();
}

QPair< double, QPointF >* qt_widgets_c_QList_QPair_double_QPointF_front(QList< QPair< double, QPointF > >* this_ptr) {
  return &this_ptr->front();
}

const QPair< double, QPointF >* qt_widgets_c_QList_QPair_double_QPointF_front_const(const QList< QPair< double, QPointF > >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QPair_double_QPointF_indexOf_t(const QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QPair_double_QPointF_indexOf_t_from(const QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QPair_double_QPointF_insert(QList< QPair< double, QPointF > >* this_ptr, int i, const QPair< double, QPointF >* t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QPair_double_QPointF_isEmpty(const QList< QPair< double, QPointF > >* this_ptr) {
  return this_ptr->isEmpty();
}

QPair< double, QPointF >* qt_widgets_c_QList_QPair_double_QPointF_last(QList< QPair< double, QPointF > >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QPair_double_QPointF_lastIndexOf_t(const QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QPair_double_QPointF_lastIndexOf_t_from(const QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QPair< double, QPointF >* qt_widgets_c_QList_QPair_double_QPointF_last_const(const QList< QPair< double, QPointF > >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QPair_double_QPointF_length(const QList< QPair< double, QPointF > >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QPair_double_QPointF_mid_to_output_pos(const QList< QPair< double, QPointF > >* this_ptr, int pos, QList< QPair< double, QPointF > >* output) {
  new(output) QList< QPair< double, QPointF > >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QPair_double_QPointF_mid_to_output_pos_length(const QList< QPair< double, QPointF > >* this_ptr, int pos, int length, QList< QPair< double, QPointF > >* output) {
  new(output) QList< QPair< double, QPointF > >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QPair_double_QPointF_move(QList< QPair< double, QPointF > >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QPair< double, QPointF > >* qt_widgets_c_QList_QPair_double_QPointF_operator_add_assign_l(QList< QPair< double, QPointF > >* this_ptr, const QList< QPair< double, QPointF > >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QPair< double, QPointF > >* qt_widgets_c_QList_QPair_double_QPointF_operator_add_assign_t(QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QPair_double_QPointF_operator_add_to_output(const QList< QPair< double, QPointF > >* this_ptr, const QList< QPair< double, QPointF > >* l, QList< QPair< double, QPointF > >* output) {
  new(output) QList< QPair< double, QPointF > >(this_ptr->operator+(*l));
}

QList< QPair< double, QPointF > >* qt_widgets_c_QList_QPair_double_QPointF_operator_assign(QList< QPair< double, QPointF > >* this_ptr, const QList< QPair< double, QPointF > >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QPair_double_QPointF_operator_eq(const QList< QPair< double, QPointF > >* this_ptr, const QList< QPair< double, QPointF > >* l) {
  return this_ptr->operator==(*l);
}

QPair< double, QPointF >* qt_widgets_c_QList_QPair_double_QPointF_operator_index(QList< QPair< double, QPointF > >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QPair< double, QPointF >* qt_widgets_c_QList_QPair_double_QPointF_operator_index_const(const QList< QPair< double, QPointF > >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QPair_double_QPointF_operator_neq(const QList< QPair< double, QPointF > >* this_ptr, const QList< QPair< double, QPointF > >* l) {
  return this_ptr->operator!=(*l);
}

QList< QPair< double, QPointF > >* qt_widgets_c_QList_QPair_double_QPointF_operator_shl_l(QList< QPair< double, QPointF > >* this_ptr, const QList< QPair< double, QPointF > >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QPair< double, QPointF > >* qt_widgets_c_QList_QPair_double_QPointF_operator_shl_t(QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QPair_double_QPointF_pop_back(QList< QPair< double, QPointF > >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QPair_double_QPointF_pop_front(QList< QPair< double, QPointF > >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QPair_double_QPointF_prepend(QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QPair_double_QPointF_push_back(QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QPair_double_QPointF_push_front(QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QPair_double_QPointF_removeAll(QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QPair_double_QPointF_removeAt(QList< QPair< double, QPointF > >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QPair_double_QPointF_removeFirst(QList< QPair< double, QPointF > >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QPair_double_QPointF_removeLast(QList< QPair< double, QPointF > >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QPair_double_QPointF_removeOne(QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QPair_double_QPointF_replace(QList< QPair< double, QPointF > >* this_ptr, int i, const QPair< double, QPointF >* t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QPair_double_QPointF_reserve(QList< QPair< double, QPointF > >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QPair_double_QPointF_size(const QList< QPair< double, QPointF > >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QPair_double_QPointF_startsWith(const QList< QPair< double, QPointF > >* this_ptr, const QPair< double, QPointF >* t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QPair_double_QPointF_swap_i_j(QList< QPair< double, QPointF > >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QPair_double_QPointF_swap_other(QList< QPair< double, QPointF > >* this_ptr, QList< QPair< double, QPointF > >* other) {
  this_ptr->swap(*other);
}

void qt_widgets_c_QList_QPair_double_QPointF_takeAt_to_output(QList< QPair< double, QPointF > >* this_ptr, int i, QPair< double, QPointF >* output) {
  new(output) QPair< double, QPointF >(this_ptr->takeAt(i));
}

void qt_widgets_c_QList_QPair_double_QPointF_takeFirst_to_output(QList< QPair< double, QPointF > >* this_ptr, QPair< double, QPointF >* output) {
  new(output) QPair< double, QPointF >(this_ptr->takeFirst());
}

void qt_widgets_c_QList_QPair_double_QPointF_takeLast_to_output(QList< QPair< double, QPointF > >* this_ptr, QPair< double, QPointF >* output) {
  new(output) QPair< double, QPointF >(this_ptr->takeLast());
}

void qt_widgets_c_QList_QPair_double_QPointF_value_to_output_i(const QList< QPair< double, QPointF > >* this_ptr, int i, QPair< double, QPointF >* output) {
  new(output) QPair< double, QPointF >(this_ptr->value(i));
}

void qt_widgets_c_QList_QPair_double_QPointF_value_to_output_i_defaultValue(const QList< QPair< double, QPointF > >* this_ptr, int i, const QPair< double, QPointF >* defaultValue, QPair< double, QPointF >* output) {
  new(output) QPair< double, QPointF >(this_ptr->value(i, *defaultValue));
}

void qt_widgets_c_QList_QPair_double_double_append_QList_QPair_double_double(QList< QPair< double, double > >* this_ptr, const QList< QPair< double, double > >* t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QPair_double_double_append_QPair_double_double(QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t) {
  this_ptr->append(*t);
}

const QPair< double, double >* qt_widgets_c_QList_QPair_double_double_at(const QList< QPair< double, double > >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QPair< double, double >* qt_widgets_c_QList_QPair_double_double_back(QList< QPair< double, double > >* this_ptr) {
  return &this_ptr->back();
}

const QPair< double, double >* qt_widgets_c_QList_QPair_double_double_back_const(const QList< QPair< double, double > >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QPair_double_double_clear(QList< QPair< double, double > >* this_ptr) {
  this_ptr->clear();
}

const QPair< double, double >* qt_widgets_c_QList_QPair_double_double_constFirst(const QList< QPair< double, double > >* this_ptr) {
  return &this_ptr->constFirst();
}

const QPair< double, double >* qt_widgets_c_QList_QPair_double_double_constLast(const QList< QPair< double, double > >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QPair_double_double_constructor_l(const QList< QPair< double, double > >* l, QList< QPair< double, double > >* output) {
  new(output) QList< QPair< double, double > >(*l);
}

void qt_widgets_c_QList_QPair_double_double_constructor_no_args(QList< QPair< double, double > >* output) {
  new(output) QList< QPair< double, double > >();
}

bool qt_widgets_c_QList_QPair_double_double_contains(const QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QPair_double_double_count_no_args(const QList< QPair< double, double > >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QPair_double_double_count_t(const QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QPair_double_double_destructor(QList< QPair< double, double > >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QPair_double_double_empty(const QList< QPair< double, double > >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QPair_double_double_endsWith(const QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t) {
  return this_ptr->endsWith(*t);
}

QPair< double, double >* qt_widgets_c_QList_QPair_double_double_first(QList< QPair< double, double > >* this_ptr) {
  return &this_ptr->first();
}

const QPair< double, double >* qt_widgets_c_QList_QPair_double_double_first_const(const QList< QPair< double, double > >* this_ptr) {
  return &this_ptr->first();
}

QPair< double, double >* qt_widgets_c_QList_QPair_double_double_front(QList< QPair< double, double > >* this_ptr) {
  return &this_ptr->front();
}

const QPair< double, double >* qt_widgets_c_QList_QPair_double_double_front_const(const QList< QPair< double, double > >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QPair_double_double_indexOf_t(const QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QPair_double_double_indexOf_t_from(const QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QPair_double_double_insert(QList< QPair< double, double > >* this_ptr, int i, const QPair< double, double >* t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QPair_double_double_isEmpty(const QList< QPair< double, double > >* this_ptr) {
  return this_ptr->isEmpty();
}

QPair< double, double >* qt_widgets_c_QList_QPair_double_double_last(QList< QPair< double, double > >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QPair_double_double_lastIndexOf_t(const QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QPair_double_double_lastIndexOf_t_from(const QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QPair< double, double >* qt_widgets_c_QList_QPair_double_double_last_const(const QList< QPair< double, double > >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QPair_double_double_length(const QList< QPair< double, double > >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QPair_double_double_mid_to_output_pos(const QList< QPair< double, double > >* this_ptr, int pos, QList< QPair< double, double > >* output) {
  new(output) QList< QPair< double, double > >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QPair_double_double_mid_to_output_pos_length(const QList< QPair< double, double > >* this_ptr, int pos, int length, QList< QPair< double, double > >* output) {
  new(output) QList< QPair< double, double > >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QPair_double_double_move(QList< QPair< double, double > >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QPair< double, double > >* qt_widgets_c_QList_QPair_double_double_operator_add_assign_l(QList< QPair< double, double > >* this_ptr, const QList< QPair< double, double > >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QPair< double, double > >* qt_widgets_c_QList_QPair_double_double_operator_add_assign_t(QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QPair_double_double_operator_add_to_output(const QList< QPair< double, double > >* this_ptr, const QList< QPair< double, double > >* l, QList< QPair< double, double > >* output) {
  new(output) QList< QPair< double, double > >(this_ptr->operator+(*l));
}

QList< QPair< double, double > >* qt_widgets_c_QList_QPair_double_double_operator_assign(QList< QPair< double, double > >* this_ptr, const QList< QPair< double, double > >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QPair_double_double_operator_eq(const QList< QPair< double, double > >* this_ptr, const QList< QPair< double, double > >* l) {
  return this_ptr->operator==(*l);
}

QPair< double, double >* qt_widgets_c_QList_QPair_double_double_operator_index(QList< QPair< double, double > >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QPair< double, double >* qt_widgets_c_QList_QPair_double_double_operator_index_const(const QList< QPair< double, double > >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QPair_double_double_operator_neq(const QList< QPair< double, double > >* this_ptr, const QList< QPair< double, double > >* l) {
  return this_ptr->operator!=(*l);
}

QList< QPair< double, double > >* qt_widgets_c_QList_QPair_double_double_operator_shl_l(QList< QPair< double, double > >* this_ptr, const QList< QPair< double, double > >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QPair< double, double > >* qt_widgets_c_QList_QPair_double_double_operator_shl_t(QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QPair_double_double_pop_back(QList< QPair< double, double > >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QPair_double_double_pop_front(QList< QPair< double, double > >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QPair_double_double_prepend(QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QPair_double_double_push_back(QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QPair_double_double_push_front(QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QPair_double_double_removeAll(QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QPair_double_double_removeAt(QList< QPair< double, double > >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QPair_double_double_removeFirst(QList< QPair< double, double > >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QPair_double_double_removeLast(QList< QPair< double, double > >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QPair_double_double_removeOne(QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QPair_double_double_replace(QList< QPair< double, double > >* this_ptr, int i, const QPair< double, double >* t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QPair_double_double_reserve(QList< QPair< double, double > >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QPair_double_double_size(const QList< QPair< double, double > >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QPair_double_double_startsWith(const QList< QPair< double, double > >* this_ptr, const QPair< double, double >* t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QPair_double_double_swap_i_j(QList< QPair< double, double > >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QPair_double_double_swap_other(QList< QPair< double, double > >* this_ptr, QList< QPair< double, double > >* other) {
  this_ptr->swap(*other);
}

void qt_widgets_c_QList_QPair_double_double_takeAt_to_output(QList< QPair< double, double > >* this_ptr, int i, QPair< double, double >* output) {
  new(output) QPair< double, double >(this_ptr->takeAt(i));
}

void qt_widgets_c_QList_QPair_double_double_takeFirst_to_output(QList< QPair< double, double > >* this_ptr, QPair< double, double >* output) {
  new(output) QPair< double, double >(this_ptr->takeFirst());
}

void qt_widgets_c_QList_QPair_double_double_takeLast_to_output(QList< QPair< double, double > >* this_ptr, QPair< double, double >* output) {
  new(output) QPair< double, double >(this_ptr->takeLast());
}

void qt_widgets_c_QList_QPair_double_double_value_to_output_i(const QList< QPair< double, double > >* this_ptr, int i, QPair< double, double >* output) {
  new(output) QPair< double, double >(this_ptr->value(i));
}

void qt_widgets_c_QList_QPair_double_double_value_to_output_i_defaultValue(const QList< QPair< double, double > >* this_ptr, int i, const QPair< double, double >* defaultValue, QPair< double, double >* output) {
  new(output) QPair< double, double >(this_ptr->value(i, *defaultValue));
}

void qt_widgets_c_QList_QRectF_append_QList_QRectF(QList< QRectF >* this_ptr, const QList< QRectF >* t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QRectF_append_QRectF(QList< QRectF >* this_ptr, const QRectF* t) {
  this_ptr->append(*t);
}

const QRectF* qt_widgets_c_QList_QRectF_at(const QList< QRectF >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QRectF* qt_widgets_c_QList_QRectF_back(QList< QRectF >* this_ptr) {
  return &this_ptr->back();
}

const QRectF* qt_widgets_c_QList_QRectF_back_const(const QList< QRectF >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QRectF_clear(QList< QRectF >* this_ptr) {
  this_ptr->clear();
}

const QRectF* qt_widgets_c_QList_QRectF_constFirst(const QList< QRectF >* this_ptr) {
  return &this_ptr->constFirst();
}

const QRectF* qt_widgets_c_QList_QRectF_constLast(const QList< QRectF >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QRectF_constructor_l(const QList< QRectF >* l, QList< QRectF >* output) {
  new(output) QList< QRectF >(*l);
}

void qt_widgets_c_QList_QRectF_constructor_no_args(QList< QRectF >* output) {
  new(output) QList< QRectF >();
}

bool qt_widgets_c_QList_QRectF_contains(const QList< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QRectF_count_no_args(const QList< QRectF >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QRectF_count_t(const QList< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QRectF_destructor(QList< QRectF >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QRectF_empty(const QList< QRectF >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QRectF_endsWith(const QList< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->endsWith(*t);
}

QRectF* qt_widgets_c_QList_QRectF_first(QList< QRectF >* this_ptr) {
  return &this_ptr->first();
}

const QRectF* qt_widgets_c_QList_QRectF_first_const(const QList< QRectF >* this_ptr) {
  return &this_ptr->first();
}

void qt_widgets_c_QList_QRectF_fromVector_to_output(const QVector< QRectF >* vector, QList< QRectF >* output) {
  new(output) QList< QRectF >(QList< QRectF >::fromVector(*vector));
}

QRectF* qt_widgets_c_QList_QRectF_front(QList< QRectF >* this_ptr) {
  return &this_ptr->front();
}

const QRectF* qt_widgets_c_QList_QRectF_front_const(const QList< QRectF >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QRectF_indexOf_t(const QList< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QRectF_indexOf_t_from(const QList< QRectF >* this_ptr, const QRectF* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QRectF_insert(QList< QRectF >* this_ptr, int i, const QRectF* t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QRectF_isEmpty(const QList< QRectF >* this_ptr) {
  return this_ptr->isEmpty();
}

QRectF* qt_widgets_c_QList_QRectF_last(QList< QRectF >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QRectF_lastIndexOf_t(const QList< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QRectF_lastIndexOf_t_from(const QList< QRectF >* this_ptr, const QRectF* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QRectF* qt_widgets_c_QList_QRectF_last_const(const QList< QRectF >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QRectF_length(const QList< QRectF >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QRectF_mid_to_output_pos(const QList< QRectF >* this_ptr, int pos, QList< QRectF >* output) {
  new(output) QList< QRectF >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QRectF_mid_to_output_pos_length(const QList< QRectF >* this_ptr, int pos, int length, QList< QRectF >* output) {
  new(output) QList< QRectF >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QRectF_move(QList< QRectF >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QRectF >* qt_widgets_c_QList_QRectF_operator_add_assign_l(QList< QRectF >* this_ptr, const QList< QRectF >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QRectF >* qt_widgets_c_QList_QRectF_operator_add_assign_t(QList< QRectF >* this_ptr, const QRectF* t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QRectF_operator_add_to_output(const QList< QRectF >* this_ptr, const QList< QRectF >* l, QList< QRectF >* output) {
  new(output) QList< QRectF >(this_ptr->operator+(*l));
}

QList< QRectF >* qt_widgets_c_QList_QRectF_operator_assign(QList< QRectF >* this_ptr, const QList< QRectF >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QRectF_operator_eq(const QList< QRectF >* this_ptr, const QList< QRectF >* l) {
  return this_ptr->operator==(*l);
}

QRectF* qt_widgets_c_QList_QRectF_operator_index(QList< QRectF >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QRectF* qt_widgets_c_QList_QRectF_operator_index_const(const QList< QRectF >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QRectF_operator_neq(const QList< QRectF >* this_ptr, const QList< QRectF >* l) {
  return this_ptr->operator!=(*l);
}

QList< QRectF >* qt_widgets_c_QList_QRectF_operator_shl_l(QList< QRectF >* this_ptr, const QList< QRectF >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QRectF >* qt_widgets_c_QList_QRectF_operator_shl_t(QList< QRectF >* this_ptr, const QRectF* t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QRectF_pop_back(QList< QRectF >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QRectF_pop_front(QList< QRectF >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QRectF_prepend(QList< QRectF >* this_ptr, const QRectF* t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QRectF_push_back(QList< QRectF >* this_ptr, const QRectF* t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QRectF_push_front(QList< QRectF >* this_ptr, const QRectF* t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QRectF_removeAll(QList< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QRectF_removeAt(QList< QRectF >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QRectF_removeFirst(QList< QRectF >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QRectF_removeLast(QList< QRectF >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QRectF_removeOne(QList< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QRectF_replace(QList< QRectF >* this_ptr, int i, const QRectF* t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QRectF_reserve(QList< QRectF >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QRectF_size(const QList< QRectF >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QRectF_startsWith(const QList< QRectF >* this_ptr, const QRectF* t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QRectF_swap_i_j(QList< QRectF >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QRectF_swap_other(QList< QRectF >* this_ptr, QList< QRectF >* other) {
  this_ptr->swap(*other);
}

void qt_widgets_c_QList_QRectF_takeAt_to_output(QList< QRectF >* this_ptr, int i, QRectF* output) {
  new(output) QRectF(this_ptr->takeAt(i));
}

void qt_widgets_c_QList_QRectF_takeFirst_to_output(QList< QRectF >* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->takeFirst());
}

void qt_widgets_c_QList_QRectF_takeLast_to_output(QList< QRectF >* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->takeLast());
}

void qt_widgets_c_QList_QRectF_toVector_to_output(const QList< QRectF >* this_ptr, QVector< QRectF >* output) {
  new(output) QVector< QRectF >(this_ptr->toVector());
}

void qt_widgets_c_QList_QRectF_value_to_output_i(const QList< QRectF >* this_ptr, int i, QRectF* output) {
  new(output) QRectF(this_ptr->value(i));
}

void qt_widgets_c_QList_QRectF_value_to_output_i_defaultValue(const QList< QRectF >* this_ptr, int i, const QRectF* defaultValue, QRectF* output) {
  new(output) QRectF(this_ptr->value(i, *defaultValue));
}

void qt_widgets_c_QList_QScroller_ptr_append_QList_QScroller_ptr(QList< QScroller* >* this_ptr, const QList< QScroller* >* t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QScroller_ptr_append_QScroller(QList< QScroller* >* this_ptr, QScroller* const * t) {
  this_ptr->append(*t);
}

QScroller* const * qt_widgets_c_QList_QScroller_ptr_at(const QList< QScroller* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QScroller** qt_widgets_c_QList_QScroller_ptr_back(QList< QScroller* >* this_ptr) {
  return &this_ptr->back();
}

QScroller* const * qt_widgets_c_QList_QScroller_ptr_back_const(const QList< QScroller* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QScroller_ptr_clear(QList< QScroller* >* this_ptr) {
  this_ptr->clear();
}

QScroller* const * qt_widgets_c_QList_QScroller_ptr_constFirst(const QList< QScroller* >* this_ptr) {
  return &this_ptr->constFirst();
}

QScroller* const * qt_widgets_c_QList_QScroller_ptr_constLast(const QList< QScroller* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QScroller_ptr_constructor_l(const QList< QScroller* >* l, QList< QScroller* >* output) {
  new(output) QList< QScroller* >(*l);
}

void qt_widgets_c_QList_QScroller_ptr_constructor_no_args(QList< QScroller* >* output) {
  new(output) QList< QScroller* >();
}

bool qt_widgets_c_QList_QScroller_ptr_contains(const QList< QScroller* >* this_ptr, QScroller* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QScroller_ptr_count_no_args(const QList< QScroller* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QScroller_ptr_count_t(const QList< QScroller* >* this_ptr, QScroller* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QScroller_ptr_destructor(QList< QScroller* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QScroller_ptr_empty(const QList< QScroller* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QScroller_ptr_endsWith(const QList< QScroller* >* this_ptr, QScroller* const * t) {
  return this_ptr->endsWith(*t);
}

QScroller** qt_widgets_c_QList_QScroller_ptr_first(QList< QScroller* >* this_ptr) {
  return &this_ptr->first();
}

QScroller* const * qt_widgets_c_QList_QScroller_ptr_first_const(const QList< QScroller* >* this_ptr) {
  return &this_ptr->first();
}

QScroller** qt_widgets_c_QList_QScroller_ptr_front(QList< QScroller* >* this_ptr) {
  return &this_ptr->front();
}

QScroller* const * qt_widgets_c_QList_QScroller_ptr_front_const(const QList< QScroller* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QScroller_ptr_indexOf_t(const QList< QScroller* >* this_ptr, QScroller* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QScroller_ptr_indexOf_t_from(const QList< QScroller* >* this_ptr, QScroller* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QScroller_ptr_insert(QList< QScroller* >* this_ptr, int i, QScroller* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QScroller_ptr_isEmpty(const QList< QScroller* >* this_ptr) {
  return this_ptr->isEmpty();
}

QScroller** qt_widgets_c_QList_QScroller_ptr_last(QList< QScroller* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QScroller_ptr_lastIndexOf_t(const QList< QScroller* >* this_ptr, QScroller* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QScroller_ptr_lastIndexOf_t_from(const QList< QScroller* >* this_ptr, QScroller* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QScroller* const * qt_widgets_c_QList_QScroller_ptr_last_const(const QList< QScroller* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QScroller_ptr_length(const QList< QScroller* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QScroller_ptr_mid_to_output_pos(const QList< QScroller* >* this_ptr, int pos, QList< QScroller* >* output) {
  new(output) QList< QScroller* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QScroller_ptr_mid_to_output_pos_length(const QList< QScroller* >* this_ptr, int pos, int length, QList< QScroller* >* output) {
  new(output) QList< QScroller* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QScroller_ptr_move(QList< QScroller* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QScroller* >* qt_widgets_c_QList_QScroller_ptr_operator_add_assign_l(QList< QScroller* >* this_ptr, const QList< QScroller* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QScroller* >* qt_widgets_c_QList_QScroller_ptr_operator_add_assign_t(QList< QScroller* >* this_ptr, QScroller* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QScroller_ptr_operator_add_to_output(const QList< QScroller* >* this_ptr, const QList< QScroller* >* l, QList< QScroller* >* output) {
  new(output) QList< QScroller* >(this_ptr->operator+(*l));
}

QList< QScroller* >* qt_widgets_c_QList_QScroller_ptr_operator_assign(QList< QScroller* >* this_ptr, const QList< QScroller* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QScroller_ptr_operator_eq(const QList< QScroller* >* this_ptr, const QList< QScroller* >* l) {
  return this_ptr->operator==(*l);
}

QScroller** qt_widgets_c_QList_QScroller_ptr_operator_index(QList< QScroller* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QScroller* const * qt_widgets_c_QList_QScroller_ptr_operator_index_const(const QList< QScroller* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QScroller_ptr_operator_neq(const QList< QScroller* >* this_ptr, const QList< QScroller* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QScroller* >* qt_widgets_c_QList_QScroller_ptr_operator_shl_l(QList< QScroller* >* this_ptr, const QList< QScroller* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QScroller* >* qt_widgets_c_QList_QScroller_ptr_operator_shl_t(QList< QScroller* >* this_ptr, QScroller* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QScroller_ptr_pop_back(QList< QScroller* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QScroller_ptr_pop_front(QList< QScroller* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QScroller_ptr_prepend(QList< QScroller* >* this_ptr, QScroller* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QScroller_ptr_push_back(QList< QScroller* >* this_ptr, QScroller* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QScroller_ptr_push_front(QList< QScroller* >* this_ptr, QScroller* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QScroller_ptr_removeAll(QList< QScroller* >* this_ptr, QScroller* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QScroller_ptr_removeAt(QList< QScroller* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QScroller_ptr_removeFirst(QList< QScroller* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QScroller_ptr_removeLast(QList< QScroller* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QScroller_ptr_removeOne(QList< QScroller* >* this_ptr, QScroller* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QScroller_ptr_replace(QList< QScroller* >* this_ptr, int i, QScroller* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QScroller_ptr_reserve(QList< QScroller* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QScroller_ptr_size(const QList< QScroller* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QScroller_ptr_startsWith(const QList< QScroller* >* this_ptr, QScroller* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QScroller_ptr_swap_i_j(QList< QScroller* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QScroller_ptr_swap_other(QList< QScroller* >* this_ptr, QList< QScroller* >* other) {
  this_ptr->swap(*other);
}

QScroller* qt_widgets_c_QList_QScroller_ptr_takeAt(QList< QScroller* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QScroller* qt_widgets_c_QList_QScroller_ptr_takeFirst(QList< QScroller* >* this_ptr) {
  return this_ptr->takeFirst();
}

QScroller* qt_widgets_c_QList_QScroller_ptr_takeLast(QList< QScroller* >* this_ptr) {
  return this_ptr->takeLast();
}

QScroller* qt_widgets_c_QList_QScroller_ptr_value_i(const QList< QScroller* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QScroller* qt_widgets_c_QList_QScroller_ptr_value_i_defaultValue(const QList< QScroller* >* this_ptr, int i, QScroller* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_append_QList_QTableWidgetItem_ptr(QList< QTableWidgetItem* >* this_ptr, const QList< QTableWidgetItem* >* t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_append_QTableWidgetItem(QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t) {
  this_ptr->append(*t);
}

QTableWidgetItem* const * qt_widgets_c_QList_QTableWidgetItem_ptr_at(const QList< QTableWidgetItem* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QTableWidgetItem** qt_widgets_c_QList_QTableWidgetItem_ptr_back(QList< QTableWidgetItem* >* this_ptr) {
  return &this_ptr->back();
}

QTableWidgetItem* const * qt_widgets_c_QList_QTableWidgetItem_ptr_back_const(const QList< QTableWidgetItem* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_clear(QList< QTableWidgetItem* >* this_ptr) {
  this_ptr->clear();
}

QTableWidgetItem* const * qt_widgets_c_QList_QTableWidgetItem_ptr_constFirst(const QList< QTableWidgetItem* >* this_ptr) {
  return &this_ptr->constFirst();
}

QTableWidgetItem* const * qt_widgets_c_QList_QTableWidgetItem_ptr_constLast(const QList< QTableWidgetItem* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_constructor_l(const QList< QTableWidgetItem* >* l, QList< QTableWidgetItem* >* output) {
  new(output) QList< QTableWidgetItem* >(*l);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_constructor_no_args(QList< QTableWidgetItem* >* output) {
  new(output) QList< QTableWidgetItem* >();
}

bool qt_widgets_c_QList_QTableWidgetItem_ptr_contains(const QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QTableWidgetItem_ptr_count_no_args(const QList< QTableWidgetItem* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QTableWidgetItem_ptr_count_t(const QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_destructor(QList< QTableWidgetItem* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QTableWidgetItem_ptr_empty(const QList< QTableWidgetItem* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QTableWidgetItem_ptr_endsWith(const QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t) {
  return this_ptr->endsWith(*t);
}

QTableWidgetItem** qt_widgets_c_QList_QTableWidgetItem_ptr_first(QList< QTableWidgetItem* >* this_ptr) {
  return &this_ptr->first();
}

QTableWidgetItem* const * qt_widgets_c_QList_QTableWidgetItem_ptr_first_const(const QList< QTableWidgetItem* >* this_ptr) {
  return &this_ptr->first();
}

QTableWidgetItem** qt_widgets_c_QList_QTableWidgetItem_ptr_front(QList< QTableWidgetItem* >* this_ptr) {
  return &this_ptr->front();
}

QTableWidgetItem* const * qt_widgets_c_QList_QTableWidgetItem_ptr_front_const(const QList< QTableWidgetItem* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QTableWidgetItem_ptr_indexOf_t(const QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QTableWidgetItem_ptr_indexOf_t_from(const QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_insert(QList< QTableWidgetItem* >* this_ptr, int i, QTableWidgetItem* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QTableWidgetItem_ptr_isEmpty(const QList< QTableWidgetItem* >* this_ptr) {
  return this_ptr->isEmpty();
}

QTableWidgetItem** qt_widgets_c_QList_QTableWidgetItem_ptr_last(QList< QTableWidgetItem* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QTableWidgetItem_ptr_lastIndexOf_t(const QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QTableWidgetItem_ptr_lastIndexOf_t_from(const QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QTableWidgetItem* const * qt_widgets_c_QList_QTableWidgetItem_ptr_last_const(const QList< QTableWidgetItem* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QTableWidgetItem_ptr_length(const QList< QTableWidgetItem* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_mid_to_output_pos(const QList< QTableWidgetItem* >* this_ptr, int pos, QList< QTableWidgetItem* >* output) {
  new(output) QList< QTableWidgetItem* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_mid_to_output_pos_length(const QList< QTableWidgetItem* >* this_ptr, int pos, int length, QList< QTableWidgetItem* >* output) {
  new(output) QList< QTableWidgetItem* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_move(QList< QTableWidgetItem* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QTableWidgetItem* >* qt_widgets_c_QList_QTableWidgetItem_ptr_operator_add_assign_l(QList< QTableWidgetItem* >* this_ptr, const QList< QTableWidgetItem* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QTableWidgetItem* >* qt_widgets_c_QList_QTableWidgetItem_ptr_operator_add_assign_t(QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_operator_add_to_output(const QList< QTableWidgetItem* >* this_ptr, const QList< QTableWidgetItem* >* l, QList< QTableWidgetItem* >* output) {
  new(output) QList< QTableWidgetItem* >(this_ptr->operator+(*l));
}

QList< QTableWidgetItem* >* qt_widgets_c_QList_QTableWidgetItem_ptr_operator_assign(QList< QTableWidgetItem* >* this_ptr, const QList< QTableWidgetItem* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QTableWidgetItem_ptr_operator_eq(const QList< QTableWidgetItem* >* this_ptr, const QList< QTableWidgetItem* >* l) {
  return this_ptr->operator==(*l);
}

QTableWidgetItem** qt_widgets_c_QList_QTableWidgetItem_ptr_operator_index(QList< QTableWidgetItem* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QTableWidgetItem* const * qt_widgets_c_QList_QTableWidgetItem_ptr_operator_index_const(const QList< QTableWidgetItem* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QTableWidgetItem_ptr_operator_neq(const QList< QTableWidgetItem* >* this_ptr, const QList< QTableWidgetItem* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QTableWidgetItem* >* qt_widgets_c_QList_QTableWidgetItem_ptr_operator_shl_l(QList< QTableWidgetItem* >* this_ptr, const QList< QTableWidgetItem* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QTableWidgetItem* >* qt_widgets_c_QList_QTableWidgetItem_ptr_operator_shl_t(QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_pop_back(QList< QTableWidgetItem* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_pop_front(QList< QTableWidgetItem* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_prepend(QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_push_back(QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_push_front(QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QTableWidgetItem_ptr_removeAll(QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_removeAt(QList< QTableWidgetItem* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_removeFirst(QList< QTableWidgetItem* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_removeLast(QList< QTableWidgetItem* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QTableWidgetItem_ptr_removeOne(QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_replace(QList< QTableWidgetItem* >* this_ptr, int i, QTableWidgetItem* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_reserve(QList< QTableWidgetItem* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QTableWidgetItem_ptr_size(const QList< QTableWidgetItem* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QTableWidgetItem_ptr_startsWith(const QList< QTableWidgetItem* >* this_ptr, QTableWidgetItem* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_swap_i_j(QList< QTableWidgetItem* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QTableWidgetItem_ptr_swap_other(QList< QTableWidgetItem* >* this_ptr, QList< QTableWidgetItem* >* other) {
  this_ptr->swap(*other);
}

QTableWidgetItem* qt_widgets_c_QList_QTableWidgetItem_ptr_takeAt(QList< QTableWidgetItem* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QTableWidgetItem* qt_widgets_c_QList_QTableWidgetItem_ptr_takeFirst(QList< QTableWidgetItem* >* this_ptr) {
  return this_ptr->takeFirst();
}

QTableWidgetItem* qt_widgets_c_QList_QTableWidgetItem_ptr_takeLast(QList< QTableWidgetItem* >* this_ptr) {
  return this_ptr->takeLast();
}

QTableWidgetItem* qt_widgets_c_QList_QTableWidgetItem_ptr_value_i(const QList< QTableWidgetItem* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QTableWidgetItem* qt_widgets_c_QList_QTableWidgetItem_ptr_value_i_defaultValue(const QList< QTableWidgetItem* >* this_ptr, int i, QTableWidgetItem* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_append_QList_QTableWidgetSelectionRange(QList< QTableWidgetSelectionRange >* this_ptr, const QList< QTableWidgetSelectionRange >* t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_append_QTableWidgetSelectionRange(QList< QTableWidgetSelectionRange >* this_ptr, const QTableWidgetSelectionRange* t) {
  this_ptr->append(*t);
}

const QTableWidgetSelectionRange* qt_widgets_c_QList_QTableWidgetSelectionRange_at(const QList< QTableWidgetSelectionRange >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QTableWidgetSelectionRange* qt_widgets_c_QList_QTableWidgetSelectionRange_back(QList< QTableWidgetSelectionRange >* this_ptr) {
  return &this_ptr->back();
}

const QTableWidgetSelectionRange* qt_widgets_c_QList_QTableWidgetSelectionRange_back_const(const QList< QTableWidgetSelectionRange >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_clear(QList< QTableWidgetSelectionRange >* this_ptr) {
  this_ptr->clear();
}

const QTableWidgetSelectionRange* qt_widgets_c_QList_QTableWidgetSelectionRange_constFirst(const QList< QTableWidgetSelectionRange >* this_ptr) {
  return &this_ptr->constFirst();
}

const QTableWidgetSelectionRange* qt_widgets_c_QList_QTableWidgetSelectionRange_constLast(const QList< QTableWidgetSelectionRange >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_constructor_l(const QList< QTableWidgetSelectionRange >* l, QList< QTableWidgetSelectionRange >* output) {
  new(output) QList< QTableWidgetSelectionRange >(*l);
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_constructor_no_args(QList< QTableWidgetSelectionRange >* output) {
  new(output) QList< QTableWidgetSelectionRange >();
}

int qt_widgets_c_QList_QTableWidgetSelectionRange_count(const QList< QTableWidgetSelectionRange >* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_destructor(QList< QTableWidgetSelectionRange >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QTableWidgetSelectionRange_empty(const QList< QTableWidgetSelectionRange >* this_ptr) {
  return this_ptr->empty();
}

QTableWidgetSelectionRange* qt_widgets_c_QList_QTableWidgetSelectionRange_first(QList< QTableWidgetSelectionRange >* this_ptr) {
  return &this_ptr->first();
}

const QTableWidgetSelectionRange* qt_widgets_c_QList_QTableWidgetSelectionRange_first_const(const QList< QTableWidgetSelectionRange >* this_ptr) {
  return &this_ptr->first();
}

QTableWidgetSelectionRange* qt_widgets_c_QList_QTableWidgetSelectionRange_front(QList< QTableWidgetSelectionRange >* this_ptr) {
  return &this_ptr->front();
}

const QTableWidgetSelectionRange* qt_widgets_c_QList_QTableWidgetSelectionRange_front_const(const QList< QTableWidgetSelectionRange >* this_ptr) {
  return &this_ptr->front();
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_insert(QList< QTableWidgetSelectionRange >* this_ptr, int i, const QTableWidgetSelectionRange* t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QTableWidgetSelectionRange_isEmpty(const QList< QTableWidgetSelectionRange >* this_ptr) {
  return this_ptr->isEmpty();
}

QTableWidgetSelectionRange* qt_widgets_c_QList_QTableWidgetSelectionRange_last(QList< QTableWidgetSelectionRange >* this_ptr) {
  return &this_ptr->last();
}

const QTableWidgetSelectionRange* qt_widgets_c_QList_QTableWidgetSelectionRange_last_const(const QList< QTableWidgetSelectionRange >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QTableWidgetSelectionRange_length(const QList< QTableWidgetSelectionRange >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_mid_to_output_pos(const QList< QTableWidgetSelectionRange >* this_ptr, int pos, QList< QTableWidgetSelectionRange >* output) {
  new(output) QList< QTableWidgetSelectionRange >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_mid_to_output_pos_length(const QList< QTableWidgetSelectionRange >* this_ptr, int pos, int length, QList< QTableWidgetSelectionRange >* output) {
  new(output) QList< QTableWidgetSelectionRange >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_move(QList< QTableWidgetSelectionRange >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QTableWidgetSelectionRange >* qt_widgets_c_QList_QTableWidgetSelectionRange_operator_add_assign_l(QList< QTableWidgetSelectionRange >* this_ptr, const QList< QTableWidgetSelectionRange >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QTableWidgetSelectionRange >* qt_widgets_c_QList_QTableWidgetSelectionRange_operator_add_assign_t(QList< QTableWidgetSelectionRange >* this_ptr, const QTableWidgetSelectionRange* t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_operator_add_to_output(const QList< QTableWidgetSelectionRange >* this_ptr, const QList< QTableWidgetSelectionRange >* l, QList< QTableWidgetSelectionRange >* output) {
  new(output) QList< QTableWidgetSelectionRange >(this_ptr->operator+(*l));
}

QList< QTableWidgetSelectionRange >* qt_widgets_c_QList_QTableWidgetSelectionRange_operator_assign(QList< QTableWidgetSelectionRange >* this_ptr, const QList< QTableWidgetSelectionRange >* l) {
  return &this_ptr->operator=(*l);
}

QTableWidgetSelectionRange* qt_widgets_c_QList_QTableWidgetSelectionRange_operator_index(QList< QTableWidgetSelectionRange >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QTableWidgetSelectionRange* qt_widgets_c_QList_QTableWidgetSelectionRange_operator_index_const(const QList< QTableWidgetSelectionRange >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QList< QTableWidgetSelectionRange >* qt_widgets_c_QList_QTableWidgetSelectionRange_operator_shl_l(QList< QTableWidgetSelectionRange >* this_ptr, const QList< QTableWidgetSelectionRange >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QTableWidgetSelectionRange >* qt_widgets_c_QList_QTableWidgetSelectionRange_operator_shl_t(QList< QTableWidgetSelectionRange >* this_ptr, const QTableWidgetSelectionRange* t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_pop_back(QList< QTableWidgetSelectionRange >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_pop_front(QList< QTableWidgetSelectionRange >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_prepend(QList< QTableWidgetSelectionRange >* this_ptr, const QTableWidgetSelectionRange* t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_push_back(QList< QTableWidgetSelectionRange >* this_ptr, const QTableWidgetSelectionRange* t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_push_front(QList< QTableWidgetSelectionRange >* this_ptr, const QTableWidgetSelectionRange* t) {
  this_ptr->push_front(*t);
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_removeAt(QList< QTableWidgetSelectionRange >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_removeFirst(QList< QTableWidgetSelectionRange >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_removeLast(QList< QTableWidgetSelectionRange >* this_ptr) {
  this_ptr->removeLast();
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_replace(QList< QTableWidgetSelectionRange >* this_ptr, int i, const QTableWidgetSelectionRange* t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_reserve(QList< QTableWidgetSelectionRange >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QTableWidgetSelectionRange_size(const QList< QTableWidgetSelectionRange >* this_ptr) {
  return this_ptr->size();
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_swap_i_j(QList< QTableWidgetSelectionRange >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_swap_other(QList< QTableWidgetSelectionRange >* this_ptr, QList< QTableWidgetSelectionRange >* other) {
  this_ptr->swap(*other);
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_takeAt_to_output(QList< QTableWidgetSelectionRange >* this_ptr, int i, QTableWidgetSelectionRange* output) {
  new(output) QTableWidgetSelectionRange(this_ptr->takeAt(i));
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_takeFirst_to_output(QList< QTableWidgetSelectionRange >* this_ptr, QTableWidgetSelectionRange* output) {
  new(output) QTableWidgetSelectionRange(this_ptr->takeFirst());
}

void qt_widgets_c_QList_QTableWidgetSelectionRange_takeLast_to_output(QList< QTableWidgetSelectionRange >* this_ptr, QTableWidgetSelectionRange* output) {
  new(output) QTableWidgetSelectionRange(this_ptr->takeLast());
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_append_QList_QTextEdit_ExtraSelection(QList< QTextEdit::ExtraSelection >* this_ptr, const QList< QTextEdit::ExtraSelection >* t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_append_QTextEdit_ExtraSelection(QList< QTextEdit::ExtraSelection >* this_ptr, const QTextEdit::ExtraSelection* t) {
  this_ptr->append(*t);
}

const QTextEdit::ExtraSelection* qt_widgets_c_QList_QTextEdit_ExtraSelection_at(const QList< QTextEdit::ExtraSelection >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QTextEdit::ExtraSelection* qt_widgets_c_QList_QTextEdit_ExtraSelection_back(QList< QTextEdit::ExtraSelection >* this_ptr) {
  return &this_ptr->back();
}

const QTextEdit::ExtraSelection* qt_widgets_c_QList_QTextEdit_ExtraSelection_back_const(const QList< QTextEdit::ExtraSelection >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_clear(QList< QTextEdit::ExtraSelection >* this_ptr) {
  this_ptr->clear();
}

const QTextEdit::ExtraSelection* qt_widgets_c_QList_QTextEdit_ExtraSelection_constFirst(const QList< QTextEdit::ExtraSelection >* this_ptr) {
  return &this_ptr->constFirst();
}

const QTextEdit::ExtraSelection* qt_widgets_c_QList_QTextEdit_ExtraSelection_constLast(const QList< QTextEdit::ExtraSelection >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_constructor_l(const QList< QTextEdit::ExtraSelection >* l, QList< QTextEdit::ExtraSelection >* output) {
  new(output) QList< QTextEdit::ExtraSelection >(*l);
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_constructor_no_args(QList< QTextEdit::ExtraSelection >* output) {
  new(output) QList< QTextEdit::ExtraSelection >();
}

int qt_widgets_c_QList_QTextEdit_ExtraSelection_count(const QList< QTextEdit::ExtraSelection >* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_destructor(QList< QTextEdit::ExtraSelection >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QTextEdit_ExtraSelection_empty(const QList< QTextEdit::ExtraSelection >* this_ptr) {
  return this_ptr->empty();
}

QTextEdit::ExtraSelection* qt_widgets_c_QList_QTextEdit_ExtraSelection_first(QList< QTextEdit::ExtraSelection >* this_ptr) {
  return &this_ptr->first();
}

const QTextEdit::ExtraSelection* qt_widgets_c_QList_QTextEdit_ExtraSelection_first_const(const QList< QTextEdit::ExtraSelection >* this_ptr) {
  return &this_ptr->first();
}

QTextEdit::ExtraSelection* qt_widgets_c_QList_QTextEdit_ExtraSelection_front(QList< QTextEdit::ExtraSelection >* this_ptr) {
  return &this_ptr->front();
}

const QTextEdit::ExtraSelection* qt_widgets_c_QList_QTextEdit_ExtraSelection_front_const(const QList< QTextEdit::ExtraSelection >* this_ptr) {
  return &this_ptr->front();
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_insert(QList< QTextEdit::ExtraSelection >* this_ptr, int i, const QTextEdit::ExtraSelection* t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QTextEdit_ExtraSelection_isEmpty(const QList< QTextEdit::ExtraSelection >* this_ptr) {
  return this_ptr->isEmpty();
}

QTextEdit::ExtraSelection* qt_widgets_c_QList_QTextEdit_ExtraSelection_last(QList< QTextEdit::ExtraSelection >* this_ptr) {
  return &this_ptr->last();
}

const QTextEdit::ExtraSelection* qt_widgets_c_QList_QTextEdit_ExtraSelection_last_const(const QList< QTextEdit::ExtraSelection >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QTextEdit_ExtraSelection_length(const QList< QTextEdit::ExtraSelection >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_mid_to_output_pos(const QList< QTextEdit::ExtraSelection >* this_ptr, int pos, QList< QTextEdit::ExtraSelection >* output) {
  new(output) QList< QTextEdit::ExtraSelection >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_mid_to_output_pos_length(const QList< QTextEdit::ExtraSelection >* this_ptr, int pos, int length, QList< QTextEdit::ExtraSelection >* output) {
  new(output) QList< QTextEdit::ExtraSelection >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_move(QList< QTextEdit::ExtraSelection >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QTextEdit::ExtraSelection >* qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_add_assign_l(QList< QTextEdit::ExtraSelection >* this_ptr, const QList< QTextEdit::ExtraSelection >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QTextEdit::ExtraSelection >* qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_add_assign_t(QList< QTextEdit::ExtraSelection >* this_ptr, const QTextEdit::ExtraSelection* t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_add_to_output(const QList< QTextEdit::ExtraSelection >* this_ptr, const QList< QTextEdit::ExtraSelection >* l, QList< QTextEdit::ExtraSelection >* output) {
  new(output) QList< QTextEdit::ExtraSelection >(this_ptr->operator+(*l));
}

QList< QTextEdit::ExtraSelection >* qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_assign(QList< QTextEdit::ExtraSelection >* this_ptr, const QList< QTextEdit::ExtraSelection >* l) {
  return &this_ptr->operator=(*l);
}

QTextEdit::ExtraSelection* qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_index(QList< QTextEdit::ExtraSelection >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QTextEdit::ExtraSelection* qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_index_const(const QList< QTextEdit::ExtraSelection >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QList< QTextEdit::ExtraSelection >* qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_shl_l(QList< QTextEdit::ExtraSelection >* this_ptr, const QList< QTextEdit::ExtraSelection >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QTextEdit::ExtraSelection >* qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_shl_t(QList< QTextEdit::ExtraSelection >* this_ptr, const QTextEdit::ExtraSelection* t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_pop_back(QList< QTextEdit::ExtraSelection >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_pop_front(QList< QTextEdit::ExtraSelection >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_prepend(QList< QTextEdit::ExtraSelection >* this_ptr, const QTextEdit::ExtraSelection* t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_push_back(QList< QTextEdit::ExtraSelection >* this_ptr, const QTextEdit::ExtraSelection* t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_push_front(QList< QTextEdit::ExtraSelection >* this_ptr, const QTextEdit::ExtraSelection* t) {
  this_ptr->push_front(*t);
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_removeAt(QList< QTextEdit::ExtraSelection >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_removeFirst(QList< QTextEdit::ExtraSelection >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_removeLast(QList< QTextEdit::ExtraSelection >* this_ptr) {
  this_ptr->removeLast();
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_replace(QList< QTextEdit::ExtraSelection >* this_ptr, int i, const QTextEdit::ExtraSelection* t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_reserve(QList< QTextEdit::ExtraSelection >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QTextEdit_ExtraSelection_size(const QList< QTextEdit::ExtraSelection >* this_ptr) {
  return this_ptr->size();
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_swap_i_j(QList< QTextEdit::ExtraSelection >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_swap_other(QList< QTextEdit::ExtraSelection >* this_ptr, QList< QTextEdit::ExtraSelection >* other) {
  this_ptr->swap(*other);
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_takeAt_to_output(QList< QTextEdit::ExtraSelection >* this_ptr, int i, QTextEdit::ExtraSelection* output) {
  new(output) QTextEdit::ExtraSelection(this_ptr->takeAt(i));
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_takeFirst_to_output(QList< QTextEdit::ExtraSelection >* this_ptr, QTextEdit::ExtraSelection* output) {
  new(output) QTextEdit::ExtraSelection(this_ptr->takeFirst());
}

void qt_widgets_c_QList_QTextEdit_ExtraSelection_takeLast_to_output(QList< QTextEdit::ExtraSelection >* this_ptr, QTextEdit::ExtraSelection* output) {
  new(output) QTextEdit::ExtraSelection(this_ptr->takeLast());
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_append_QList_QTreeWidgetItem_ptr(QList< QTreeWidgetItem* >* this_ptr, const QList< QTreeWidgetItem* >* t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_append_QTreeWidgetItem(QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t) {
  this_ptr->append(*t);
}

QTreeWidgetItem* const * qt_widgets_c_QList_QTreeWidgetItem_ptr_at(const QList< QTreeWidgetItem* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QTreeWidgetItem** qt_widgets_c_QList_QTreeWidgetItem_ptr_back(QList< QTreeWidgetItem* >* this_ptr) {
  return &this_ptr->back();
}

QTreeWidgetItem* const * qt_widgets_c_QList_QTreeWidgetItem_ptr_back_const(const QList< QTreeWidgetItem* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_clear(QList< QTreeWidgetItem* >* this_ptr) {
  this_ptr->clear();
}

QTreeWidgetItem* const * qt_widgets_c_QList_QTreeWidgetItem_ptr_constFirst(const QList< QTreeWidgetItem* >* this_ptr) {
  return &this_ptr->constFirst();
}

QTreeWidgetItem* const * qt_widgets_c_QList_QTreeWidgetItem_ptr_constLast(const QList< QTreeWidgetItem* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_constructor_l(const QList< QTreeWidgetItem* >* l, QList< QTreeWidgetItem* >* output) {
  new(output) QList< QTreeWidgetItem* >(*l);
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_constructor_no_args(QList< QTreeWidgetItem* >* output) {
  new(output) QList< QTreeWidgetItem* >();
}

bool qt_widgets_c_QList_QTreeWidgetItem_ptr_contains(const QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QTreeWidgetItem_ptr_count_no_args(const QList< QTreeWidgetItem* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QTreeWidgetItem_ptr_count_t(const QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_destructor(QList< QTreeWidgetItem* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QTreeWidgetItem_ptr_empty(const QList< QTreeWidgetItem* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QTreeWidgetItem_ptr_endsWith(const QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t) {
  return this_ptr->endsWith(*t);
}

QTreeWidgetItem** qt_widgets_c_QList_QTreeWidgetItem_ptr_first(QList< QTreeWidgetItem* >* this_ptr) {
  return &this_ptr->first();
}

QTreeWidgetItem* const * qt_widgets_c_QList_QTreeWidgetItem_ptr_first_const(const QList< QTreeWidgetItem* >* this_ptr) {
  return &this_ptr->first();
}

QTreeWidgetItem** qt_widgets_c_QList_QTreeWidgetItem_ptr_front(QList< QTreeWidgetItem* >* this_ptr) {
  return &this_ptr->front();
}

QTreeWidgetItem* const * qt_widgets_c_QList_QTreeWidgetItem_ptr_front_const(const QList< QTreeWidgetItem* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QTreeWidgetItem_ptr_indexOf_t(const QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QTreeWidgetItem_ptr_indexOf_t_from(const QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_insert(QList< QTreeWidgetItem* >* this_ptr, int i, QTreeWidgetItem* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QTreeWidgetItem_ptr_isEmpty(const QList< QTreeWidgetItem* >* this_ptr) {
  return this_ptr->isEmpty();
}

QTreeWidgetItem** qt_widgets_c_QList_QTreeWidgetItem_ptr_last(QList< QTreeWidgetItem* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QTreeWidgetItem_ptr_lastIndexOf_t(const QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QTreeWidgetItem_ptr_lastIndexOf_t_from(const QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QTreeWidgetItem* const * qt_widgets_c_QList_QTreeWidgetItem_ptr_last_const(const QList< QTreeWidgetItem* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QTreeWidgetItem_ptr_length(const QList< QTreeWidgetItem* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_mid_to_output_pos(const QList< QTreeWidgetItem* >* this_ptr, int pos, QList< QTreeWidgetItem* >* output) {
  new(output) QList< QTreeWidgetItem* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_mid_to_output_pos_length(const QList< QTreeWidgetItem* >* this_ptr, int pos, int length, QList< QTreeWidgetItem* >* output) {
  new(output) QList< QTreeWidgetItem* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_move(QList< QTreeWidgetItem* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QTreeWidgetItem* >* qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_add_assign_l(QList< QTreeWidgetItem* >* this_ptr, const QList< QTreeWidgetItem* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QTreeWidgetItem* >* qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_add_assign_t(QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_add_to_output(const QList< QTreeWidgetItem* >* this_ptr, const QList< QTreeWidgetItem* >* l, QList< QTreeWidgetItem* >* output) {
  new(output) QList< QTreeWidgetItem* >(this_ptr->operator+(*l));
}

QList< QTreeWidgetItem* >* qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_assign(QList< QTreeWidgetItem* >* this_ptr, const QList< QTreeWidgetItem* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_eq(const QList< QTreeWidgetItem* >* this_ptr, const QList< QTreeWidgetItem* >* l) {
  return this_ptr->operator==(*l);
}

QTreeWidgetItem** qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_index(QList< QTreeWidgetItem* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QTreeWidgetItem* const * qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_index_const(const QList< QTreeWidgetItem* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_neq(const QList< QTreeWidgetItem* >* this_ptr, const QList< QTreeWidgetItem* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QTreeWidgetItem* >* qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_shl_l(QList< QTreeWidgetItem* >* this_ptr, const QList< QTreeWidgetItem* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QTreeWidgetItem* >* qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_shl_t(QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_pop_back(QList< QTreeWidgetItem* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_pop_front(QList< QTreeWidgetItem* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_prepend(QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_push_back(QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_push_front(QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QTreeWidgetItem_ptr_removeAll(QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_removeAt(QList< QTreeWidgetItem* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_removeFirst(QList< QTreeWidgetItem* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_removeLast(QList< QTreeWidgetItem* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QTreeWidgetItem_ptr_removeOne(QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_replace(QList< QTreeWidgetItem* >* this_ptr, int i, QTreeWidgetItem* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_reserve(QList< QTreeWidgetItem* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QTreeWidgetItem_ptr_size(const QList< QTreeWidgetItem* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QTreeWidgetItem_ptr_startsWith(const QList< QTreeWidgetItem* >* this_ptr, QTreeWidgetItem* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_swap_i_j(QList< QTreeWidgetItem* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QTreeWidgetItem_ptr_swap_other(QList< QTreeWidgetItem* >* this_ptr, QList< QTreeWidgetItem* >* other) {
  this_ptr->swap(*other);
}

QTreeWidgetItem* qt_widgets_c_QList_QTreeWidgetItem_ptr_takeAt(QList< QTreeWidgetItem* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QTreeWidgetItem* qt_widgets_c_QList_QTreeWidgetItem_ptr_takeFirst(QList< QTreeWidgetItem* >* this_ptr) {
  return this_ptr->takeFirst();
}

QTreeWidgetItem* qt_widgets_c_QList_QTreeWidgetItem_ptr_takeLast(QList< QTreeWidgetItem* >* this_ptr) {
  return this_ptr->takeLast();
}

QTreeWidgetItem* qt_widgets_c_QList_QTreeWidgetItem_ptr_value_i(const QList< QTreeWidgetItem* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QTreeWidgetItem* qt_widgets_c_QList_QTreeWidgetItem_ptr_value_i_defaultValue(const QList< QTreeWidgetItem* >* this_ptr, int i, QTreeWidgetItem* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QUndoStack_ptr_append_QList_QUndoStack_ptr(QList< QUndoStack* >* this_ptr, const QList< QUndoStack* >* t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QUndoStack_ptr_append_QUndoStack(QList< QUndoStack* >* this_ptr, QUndoStack* const * t) {
  this_ptr->append(*t);
}

QUndoStack* const * qt_widgets_c_QList_QUndoStack_ptr_at(const QList< QUndoStack* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QUndoStack** qt_widgets_c_QList_QUndoStack_ptr_back(QList< QUndoStack* >* this_ptr) {
  return &this_ptr->back();
}

QUndoStack* const * qt_widgets_c_QList_QUndoStack_ptr_back_const(const QList< QUndoStack* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QUndoStack_ptr_clear(QList< QUndoStack* >* this_ptr) {
  this_ptr->clear();
}

QUndoStack* const * qt_widgets_c_QList_QUndoStack_ptr_constFirst(const QList< QUndoStack* >* this_ptr) {
  return &this_ptr->constFirst();
}

QUndoStack* const * qt_widgets_c_QList_QUndoStack_ptr_constLast(const QList< QUndoStack* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QUndoStack_ptr_constructor_l(const QList< QUndoStack* >* l, QList< QUndoStack* >* output) {
  new(output) QList< QUndoStack* >(*l);
}

void qt_widgets_c_QList_QUndoStack_ptr_constructor_no_args(QList< QUndoStack* >* output) {
  new(output) QList< QUndoStack* >();
}

bool qt_widgets_c_QList_QUndoStack_ptr_contains(const QList< QUndoStack* >* this_ptr, QUndoStack* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QUndoStack_ptr_count_no_args(const QList< QUndoStack* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QUndoStack_ptr_count_t(const QList< QUndoStack* >* this_ptr, QUndoStack* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QUndoStack_ptr_destructor(QList< QUndoStack* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QUndoStack_ptr_empty(const QList< QUndoStack* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QUndoStack_ptr_endsWith(const QList< QUndoStack* >* this_ptr, QUndoStack* const * t) {
  return this_ptr->endsWith(*t);
}

QUndoStack** qt_widgets_c_QList_QUndoStack_ptr_first(QList< QUndoStack* >* this_ptr) {
  return &this_ptr->first();
}

QUndoStack* const * qt_widgets_c_QList_QUndoStack_ptr_first_const(const QList< QUndoStack* >* this_ptr) {
  return &this_ptr->first();
}

QUndoStack** qt_widgets_c_QList_QUndoStack_ptr_front(QList< QUndoStack* >* this_ptr) {
  return &this_ptr->front();
}

QUndoStack* const * qt_widgets_c_QList_QUndoStack_ptr_front_const(const QList< QUndoStack* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QUndoStack_ptr_indexOf_t(const QList< QUndoStack* >* this_ptr, QUndoStack* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QUndoStack_ptr_indexOf_t_from(const QList< QUndoStack* >* this_ptr, QUndoStack* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QUndoStack_ptr_insert(QList< QUndoStack* >* this_ptr, int i, QUndoStack* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QUndoStack_ptr_isEmpty(const QList< QUndoStack* >* this_ptr) {
  return this_ptr->isEmpty();
}

QUndoStack** qt_widgets_c_QList_QUndoStack_ptr_last(QList< QUndoStack* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QUndoStack_ptr_lastIndexOf_t(const QList< QUndoStack* >* this_ptr, QUndoStack* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QUndoStack_ptr_lastIndexOf_t_from(const QList< QUndoStack* >* this_ptr, QUndoStack* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QUndoStack* const * qt_widgets_c_QList_QUndoStack_ptr_last_const(const QList< QUndoStack* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QUndoStack_ptr_length(const QList< QUndoStack* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QUndoStack_ptr_mid_to_output_pos(const QList< QUndoStack* >* this_ptr, int pos, QList< QUndoStack* >* output) {
  new(output) QList< QUndoStack* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QUndoStack_ptr_mid_to_output_pos_length(const QList< QUndoStack* >* this_ptr, int pos, int length, QList< QUndoStack* >* output) {
  new(output) QList< QUndoStack* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QUndoStack_ptr_move(QList< QUndoStack* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QUndoStack* >* qt_widgets_c_QList_QUndoStack_ptr_operator_add_assign_l(QList< QUndoStack* >* this_ptr, const QList< QUndoStack* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QUndoStack* >* qt_widgets_c_QList_QUndoStack_ptr_operator_add_assign_t(QList< QUndoStack* >* this_ptr, QUndoStack* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QUndoStack_ptr_operator_add_to_output(const QList< QUndoStack* >* this_ptr, const QList< QUndoStack* >* l, QList< QUndoStack* >* output) {
  new(output) QList< QUndoStack* >(this_ptr->operator+(*l));
}

QList< QUndoStack* >* qt_widgets_c_QList_QUndoStack_ptr_operator_assign(QList< QUndoStack* >* this_ptr, const QList< QUndoStack* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QUndoStack_ptr_operator_eq(const QList< QUndoStack* >* this_ptr, const QList< QUndoStack* >* l) {
  return this_ptr->operator==(*l);
}

QUndoStack** qt_widgets_c_QList_QUndoStack_ptr_operator_index(QList< QUndoStack* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QUndoStack* const * qt_widgets_c_QList_QUndoStack_ptr_operator_index_const(const QList< QUndoStack* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QUndoStack_ptr_operator_neq(const QList< QUndoStack* >* this_ptr, const QList< QUndoStack* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QUndoStack* >* qt_widgets_c_QList_QUndoStack_ptr_operator_shl_l(QList< QUndoStack* >* this_ptr, const QList< QUndoStack* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QUndoStack* >* qt_widgets_c_QList_QUndoStack_ptr_operator_shl_t(QList< QUndoStack* >* this_ptr, QUndoStack* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QUndoStack_ptr_pop_back(QList< QUndoStack* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QUndoStack_ptr_pop_front(QList< QUndoStack* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QUndoStack_ptr_prepend(QList< QUndoStack* >* this_ptr, QUndoStack* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QUndoStack_ptr_push_back(QList< QUndoStack* >* this_ptr, QUndoStack* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QUndoStack_ptr_push_front(QList< QUndoStack* >* this_ptr, QUndoStack* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QUndoStack_ptr_removeAll(QList< QUndoStack* >* this_ptr, QUndoStack* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QUndoStack_ptr_removeAt(QList< QUndoStack* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QUndoStack_ptr_removeFirst(QList< QUndoStack* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QUndoStack_ptr_removeLast(QList< QUndoStack* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QUndoStack_ptr_removeOne(QList< QUndoStack* >* this_ptr, QUndoStack* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QUndoStack_ptr_replace(QList< QUndoStack* >* this_ptr, int i, QUndoStack* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QUndoStack_ptr_reserve(QList< QUndoStack* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QUndoStack_ptr_size(const QList< QUndoStack* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QUndoStack_ptr_startsWith(const QList< QUndoStack* >* this_ptr, QUndoStack* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QUndoStack_ptr_swap_i_j(QList< QUndoStack* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QUndoStack_ptr_swap_other(QList< QUndoStack* >* this_ptr, QList< QUndoStack* >* other) {
  this_ptr->swap(*other);
}

QUndoStack* qt_widgets_c_QList_QUndoStack_ptr_takeAt(QList< QUndoStack* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QUndoStack* qt_widgets_c_QList_QUndoStack_ptr_takeFirst(QList< QUndoStack* >* this_ptr) {
  return this_ptr->takeFirst();
}

QUndoStack* qt_widgets_c_QList_QUndoStack_ptr_takeLast(QList< QUndoStack* >* this_ptr) {
  return this_ptr->takeLast();
}

QUndoStack* qt_widgets_c_QList_QUndoStack_ptr_value_i(const QList< QUndoStack* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QUndoStack* qt_widgets_c_QList_QUndoStack_ptr_value_i_defaultValue(const QList< QUndoStack* >* this_ptr, int i, QUndoStack* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QWidget_ptr_append_QList_QWidget_ptr(QList< QWidget* >* this_ptr, const QList< QWidget* >* t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QWidget_ptr_append_QWidget(QList< QWidget* >* this_ptr, QWidget* const * t) {
  this_ptr->append(*t);
}

QWidget* const * qt_widgets_c_QList_QWidget_ptr_at(const QList< QWidget* >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QWidget** qt_widgets_c_QList_QWidget_ptr_back(QList< QWidget* >* this_ptr) {
  return &this_ptr->back();
}

QWidget* const * qt_widgets_c_QList_QWidget_ptr_back_const(const QList< QWidget* >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QWidget_ptr_clear(QList< QWidget* >* this_ptr) {
  this_ptr->clear();
}

QWidget* const * qt_widgets_c_QList_QWidget_ptr_constFirst(const QList< QWidget* >* this_ptr) {
  return &this_ptr->constFirst();
}

QWidget* const * qt_widgets_c_QList_QWidget_ptr_constLast(const QList< QWidget* >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QWidget_ptr_constructor_l(const QList< QWidget* >* l, QList< QWidget* >* output) {
  new(output) QList< QWidget* >(*l);
}

void qt_widgets_c_QList_QWidget_ptr_constructor_no_args(QList< QWidget* >* output) {
  new(output) QList< QWidget* >();
}

bool qt_widgets_c_QList_QWidget_ptr_contains(const QList< QWidget* >* this_ptr, QWidget* const * t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QWidget_ptr_count_no_args(const QList< QWidget* >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QWidget_ptr_count_t(const QList< QWidget* >* this_ptr, QWidget* const * t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QWidget_ptr_destructor(QList< QWidget* >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QWidget_ptr_empty(const QList< QWidget* >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QWidget_ptr_endsWith(const QList< QWidget* >* this_ptr, QWidget* const * t) {
  return this_ptr->endsWith(*t);
}

QWidget** qt_widgets_c_QList_QWidget_ptr_first(QList< QWidget* >* this_ptr) {
  return &this_ptr->first();
}

QWidget* const * qt_widgets_c_QList_QWidget_ptr_first_const(const QList< QWidget* >* this_ptr) {
  return &this_ptr->first();
}

QWidget** qt_widgets_c_QList_QWidget_ptr_front(QList< QWidget* >* this_ptr) {
  return &this_ptr->front();
}

QWidget* const * qt_widgets_c_QList_QWidget_ptr_front_const(const QList< QWidget* >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QWidget_ptr_indexOf_t(const QList< QWidget* >* this_ptr, QWidget* const * t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QWidget_ptr_indexOf_t_from(const QList< QWidget* >* this_ptr, QWidget* const * t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QWidget_ptr_insert(QList< QWidget* >* this_ptr, int i, QWidget* const * t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QWidget_ptr_isEmpty(const QList< QWidget* >* this_ptr) {
  return this_ptr->isEmpty();
}

QWidget** qt_widgets_c_QList_QWidget_ptr_last(QList< QWidget* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QWidget_ptr_lastIndexOf_t(const QList< QWidget* >* this_ptr, QWidget* const * t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QWidget_ptr_lastIndexOf_t_from(const QList< QWidget* >* this_ptr, QWidget* const * t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

QWidget* const * qt_widgets_c_QList_QWidget_ptr_last_const(const QList< QWidget* >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QWidget_ptr_length(const QList< QWidget* >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QWidget_ptr_mid_to_output_pos(const QList< QWidget* >* this_ptr, int pos, QList< QWidget* >* output) {
  new(output) QList< QWidget* >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QWidget_ptr_mid_to_output_pos_length(const QList< QWidget* >* this_ptr, int pos, int length, QList< QWidget* >* output) {
  new(output) QList< QWidget* >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QWidget_ptr_move(QList< QWidget* >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QWidget* >* qt_widgets_c_QList_QWidget_ptr_operator_add_assign_l(QList< QWidget* >* this_ptr, const QList< QWidget* >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QWidget* >* qt_widgets_c_QList_QWidget_ptr_operator_add_assign_t(QList< QWidget* >* this_ptr, QWidget* const * t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QWidget_ptr_operator_add_to_output(const QList< QWidget* >* this_ptr, const QList< QWidget* >* l, QList< QWidget* >* output) {
  new(output) QList< QWidget* >(this_ptr->operator+(*l));
}

QList< QWidget* >* qt_widgets_c_QList_QWidget_ptr_operator_assign(QList< QWidget* >* this_ptr, const QList< QWidget* >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QWidget_ptr_operator_eq(const QList< QWidget* >* this_ptr, const QList< QWidget* >* l) {
  return this_ptr->operator==(*l);
}

QWidget** qt_widgets_c_QList_QWidget_ptr_operator_index(QList< QWidget* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

QWidget* const * qt_widgets_c_QList_QWidget_ptr_operator_index_const(const QList< QWidget* >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QWidget_ptr_operator_neq(const QList< QWidget* >* this_ptr, const QList< QWidget* >* l) {
  return this_ptr->operator!=(*l);
}

QList< QWidget* >* qt_widgets_c_QList_QWidget_ptr_operator_shl_l(QList< QWidget* >* this_ptr, const QList< QWidget* >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QWidget* >* qt_widgets_c_QList_QWidget_ptr_operator_shl_t(QList< QWidget* >* this_ptr, QWidget* const * t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QWidget_ptr_pop_back(QList< QWidget* >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QWidget_ptr_pop_front(QList< QWidget* >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QWidget_ptr_prepend(QList< QWidget* >* this_ptr, QWidget* const * t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QWidget_ptr_push_back(QList< QWidget* >* this_ptr, QWidget* const * t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QWidget_ptr_push_front(QList< QWidget* >* this_ptr, QWidget* const * t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QWidget_ptr_removeAll(QList< QWidget* >* this_ptr, QWidget* const * t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QWidget_ptr_removeAt(QList< QWidget* >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QWidget_ptr_removeFirst(QList< QWidget* >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QWidget_ptr_removeLast(QList< QWidget* >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QWidget_ptr_removeOne(QList< QWidget* >* this_ptr, QWidget* const * t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QWidget_ptr_replace(QList< QWidget* >* this_ptr, int i, QWidget* const * t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QWidget_ptr_reserve(QList< QWidget* >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QWidget_ptr_size(const QList< QWidget* >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QWidget_ptr_startsWith(const QList< QWidget* >* this_ptr, QWidget* const * t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QWidget_ptr_swap_i_j(QList< QWidget* >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QWidget_ptr_swap_other(QList< QWidget* >* this_ptr, QList< QWidget* >* other) {
  this_ptr->swap(*other);
}

QWidget* qt_widgets_c_QList_QWidget_ptr_takeAt(QList< QWidget* >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QWidget* qt_widgets_c_QList_QWidget_ptr_takeFirst(QList< QWidget* >* this_ptr) {
  return this_ptr->takeFirst();
}

QWidget* qt_widgets_c_QList_QWidget_ptr_takeLast(QList< QWidget* >* this_ptr) {
  return this_ptr->takeLast();
}

QWidget* qt_widgets_c_QList_QWidget_ptr_value_i(const QList< QWidget* >* this_ptr, int i) {
  return this_ptr->value(i);
}

QWidget* qt_widgets_c_QList_QWidget_ptr_value_i_defaultValue(const QList< QWidget* >* this_ptr, int i, QWidget* const * defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

void qt_widgets_c_QList_QWizard_WizardButton_append_QList_QWizard_WizardButton(QList< QWizard::WizardButton >* this_ptr, const QList< QWizard::WizardButton >* t) {
  this_ptr->append(*t);
}

void qt_widgets_c_QList_QWizard_WizardButton_append_QWizard_WizardButton(QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t) {
  this_ptr->append(*t);
}

const QWizard::WizardButton* qt_widgets_c_QList_QWizard_WizardButton_at(const QList< QWizard::WizardButton >* this_ptr, int i) {
  return &this_ptr->at(i);
}

QWizard::WizardButton* qt_widgets_c_QList_QWizard_WizardButton_back(QList< QWizard::WizardButton >* this_ptr) {
  return &this_ptr->back();
}

const QWizard::WizardButton* qt_widgets_c_QList_QWizard_WizardButton_back_const(const QList< QWizard::WizardButton >* this_ptr) {
  return &this_ptr->back();
}

void qt_widgets_c_QList_QWizard_WizardButton_clear(QList< QWizard::WizardButton >* this_ptr) {
  this_ptr->clear();
}

const QWizard::WizardButton* qt_widgets_c_QList_QWizard_WizardButton_constFirst(const QList< QWizard::WizardButton >* this_ptr) {
  return &this_ptr->constFirst();
}

const QWizard::WizardButton* qt_widgets_c_QList_QWizard_WizardButton_constLast(const QList< QWizard::WizardButton >* this_ptr) {
  return &this_ptr->constLast();
}

void qt_widgets_c_QList_QWizard_WizardButton_constructor_l(const QList< QWizard::WizardButton >* l, QList< QWizard::WizardButton >* output) {
  new(output) QList< QWizard::WizardButton >(*l);
}

void qt_widgets_c_QList_QWizard_WizardButton_constructor_no_args(QList< QWizard::WizardButton >* output) {
  new(output) QList< QWizard::WizardButton >();
}

bool qt_widgets_c_QList_QWizard_WizardButton_contains(const QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t) {
  return this_ptr->contains(*t);
}

int qt_widgets_c_QList_QWizard_WizardButton_count_no_args(const QList< QWizard::WizardButton >* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QList_QWizard_WizardButton_count_t(const QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t) {
  return this_ptr->count(*t);
}

void qt_widgets_c_QList_QWizard_WizardButton_destructor(QList< QWizard::WizardButton >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

bool qt_widgets_c_QList_QWizard_WizardButton_empty(const QList< QWizard::WizardButton >* this_ptr) {
  return this_ptr->empty();
}

bool qt_widgets_c_QList_QWizard_WizardButton_endsWith(const QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t) {
  return this_ptr->endsWith(*t);
}

QWizard::WizardButton* qt_widgets_c_QList_QWizard_WizardButton_first(QList< QWizard::WizardButton >* this_ptr) {
  return &this_ptr->first();
}

const QWizard::WizardButton* qt_widgets_c_QList_QWizard_WizardButton_first_const(const QList< QWizard::WizardButton >* this_ptr) {
  return &this_ptr->first();
}

QWizard::WizardButton* qt_widgets_c_QList_QWizard_WizardButton_front(QList< QWizard::WizardButton >* this_ptr) {
  return &this_ptr->front();
}

const QWizard::WizardButton* qt_widgets_c_QList_QWizard_WizardButton_front_const(const QList< QWizard::WizardButton >* this_ptr) {
  return &this_ptr->front();
}

int qt_widgets_c_QList_QWizard_WizardButton_indexOf_t(const QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t) {
  return this_ptr->indexOf(*t);
}

int qt_widgets_c_QList_QWizard_WizardButton_indexOf_t_from(const QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t, int from) {
  return this_ptr->indexOf(*t, from);
}

void qt_widgets_c_QList_QWizard_WizardButton_insert(QList< QWizard::WizardButton >* this_ptr, int i, const QWizard::WizardButton* t) {
  this_ptr->insert(i, *t);
}

bool qt_widgets_c_QList_QWizard_WizardButton_isEmpty(const QList< QWizard::WizardButton >* this_ptr) {
  return this_ptr->isEmpty();
}

QWizard::WizardButton* qt_widgets_c_QList_QWizard_WizardButton_last(QList< QWizard::WizardButton >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QWizard_WizardButton_lastIndexOf_t(const QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t) {
  return this_ptr->lastIndexOf(*t);
}

int qt_widgets_c_QList_QWizard_WizardButton_lastIndexOf_t_from(const QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t, int from) {
  return this_ptr->lastIndexOf(*t, from);
}

const QWizard::WizardButton* qt_widgets_c_QList_QWizard_WizardButton_last_const(const QList< QWizard::WizardButton >* this_ptr) {
  return &this_ptr->last();
}

int qt_widgets_c_QList_QWizard_WizardButton_length(const QList< QWizard::WizardButton >* this_ptr) {
  return this_ptr->length();
}

void qt_widgets_c_QList_QWizard_WizardButton_mid_to_output_pos(const QList< QWizard::WizardButton >* this_ptr, int pos, QList< QWizard::WizardButton >* output) {
  new(output) QList< QWizard::WizardButton >(this_ptr->mid(pos));
}

void qt_widgets_c_QList_QWizard_WizardButton_mid_to_output_pos_length(const QList< QWizard::WizardButton >* this_ptr, int pos, int length, QList< QWizard::WizardButton >* output) {
  new(output) QList< QWizard::WizardButton >(this_ptr->mid(pos, length));
}

void qt_widgets_c_QList_QWizard_WizardButton_move(QList< QWizard::WizardButton >* this_ptr, int from, int to) {
  this_ptr->move(from, to);
}

QList< QWizard::WizardButton >* qt_widgets_c_QList_QWizard_WizardButton_operator_add_assign_l(QList< QWizard::WizardButton >* this_ptr, const QList< QWizard::WizardButton >* l) {
  return &this_ptr->operator+=(*l);
}

QList< QWizard::WizardButton >* qt_widgets_c_QList_QWizard_WizardButton_operator_add_assign_t(QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t) {
  return &this_ptr->operator+=(*t);
}

void qt_widgets_c_QList_QWizard_WizardButton_operator_add_to_output(const QList< QWizard::WizardButton >* this_ptr, const QList< QWizard::WizardButton >* l, QList< QWizard::WizardButton >* output) {
  new(output) QList< QWizard::WizardButton >(this_ptr->operator+(*l));
}

QList< QWizard::WizardButton >* qt_widgets_c_QList_QWizard_WizardButton_operator_assign(QList< QWizard::WizardButton >* this_ptr, const QList< QWizard::WizardButton >* l) {
  return &this_ptr->operator=(*l);
}

bool qt_widgets_c_QList_QWizard_WizardButton_operator_eq(const QList< QWizard::WizardButton >* this_ptr, const QList< QWizard::WizardButton >* l) {
  return this_ptr->operator==(*l);
}

QWizard::WizardButton* qt_widgets_c_QList_QWizard_WizardButton_operator_index(QList< QWizard::WizardButton >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

const QWizard::WizardButton* qt_widgets_c_QList_QWizard_WizardButton_operator_index_const(const QList< QWizard::WizardButton >* this_ptr, int i) {
  return &this_ptr->operator[](i);
}

bool qt_widgets_c_QList_QWizard_WizardButton_operator_neq(const QList< QWizard::WizardButton >* this_ptr, const QList< QWizard::WizardButton >* l) {
  return this_ptr->operator!=(*l);
}

QList< QWizard::WizardButton >* qt_widgets_c_QList_QWizard_WizardButton_operator_shl_l(QList< QWizard::WizardButton >* this_ptr, const QList< QWizard::WizardButton >* l) {
  return &this_ptr->operator<<(*l);
}

QList< QWizard::WizardButton >* qt_widgets_c_QList_QWizard_WizardButton_operator_shl_t(QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t) {
  return &this_ptr->operator<<(*t);
}

void qt_widgets_c_QList_QWizard_WizardButton_pop_back(QList< QWizard::WizardButton >* this_ptr) {
  this_ptr->pop_back();
}

void qt_widgets_c_QList_QWizard_WizardButton_pop_front(QList< QWizard::WizardButton >* this_ptr) {
  this_ptr->pop_front();
}

void qt_widgets_c_QList_QWizard_WizardButton_prepend(QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t) {
  this_ptr->prepend(*t);
}

void qt_widgets_c_QList_QWizard_WizardButton_push_back(QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t) {
  this_ptr->push_back(*t);
}

void qt_widgets_c_QList_QWizard_WizardButton_push_front(QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t) {
  this_ptr->push_front(*t);
}

int qt_widgets_c_QList_QWizard_WizardButton_removeAll(QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t) {
  return this_ptr->removeAll(*t);
}

void qt_widgets_c_QList_QWizard_WizardButton_removeAt(QList< QWizard::WizardButton >* this_ptr, int i) {
  this_ptr->removeAt(i);
}

void qt_widgets_c_QList_QWizard_WizardButton_removeFirst(QList< QWizard::WizardButton >* this_ptr) {
  this_ptr->removeFirst();
}

void qt_widgets_c_QList_QWizard_WizardButton_removeLast(QList< QWizard::WizardButton >* this_ptr) {
  this_ptr->removeLast();
}

bool qt_widgets_c_QList_QWizard_WizardButton_removeOne(QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t) {
  return this_ptr->removeOne(*t);
}

void qt_widgets_c_QList_QWizard_WizardButton_replace(QList< QWizard::WizardButton >* this_ptr, int i, const QWizard::WizardButton* t) {
  this_ptr->replace(i, *t);
}

void qt_widgets_c_QList_QWizard_WizardButton_reserve(QList< QWizard::WizardButton >* this_ptr, int size) {
  this_ptr->reserve(size);
}

int qt_widgets_c_QList_QWizard_WizardButton_size(const QList< QWizard::WizardButton >* this_ptr) {
  return this_ptr->size();
}

bool qt_widgets_c_QList_QWizard_WizardButton_startsWith(const QList< QWizard::WizardButton >* this_ptr, const QWizard::WizardButton* t) {
  return this_ptr->startsWith(*t);
}

void qt_widgets_c_QList_QWizard_WizardButton_swap_i_j(QList< QWizard::WizardButton >* this_ptr, int i, int j) {
  this_ptr->swap(i, j);
}

void qt_widgets_c_QList_QWizard_WizardButton_swap_other(QList< QWizard::WizardButton >* this_ptr, QList< QWizard::WizardButton >* other) {
  this_ptr->swap(*other);
}

QWizard::WizardButton qt_widgets_c_QList_QWizard_WizardButton_takeAt(QList< QWizard::WizardButton >* this_ptr, int i) {
  return this_ptr->takeAt(i);
}

QWizard::WizardButton qt_widgets_c_QList_QWizard_WizardButton_takeFirst(QList< QWizard::WizardButton >* this_ptr) {
  return this_ptr->takeFirst();
}

QWizard::WizardButton qt_widgets_c_QList_QWizard_WizardButton_takeLast(QList< QWizard::WizardButton >* this_ptr) {
  return this_ptr->takeLast();
}

QWizard::WizardButton qt_widgets_c_QList_QWizard_WizardButton_value_i(const QList< QWizard::WizardButton >* this_ptr, int i) {
  return this_ptr->value(i);
}

QWizard::WizardButton qt_widgets_c_QList_QWizard_WizardButton_value_i_defaultValue(const QList< QWizard::WizardButton >* this_ptr, int i, const QWizard::WizardButton* defaultValue) {
  return this_ptr->value(i, *defaultValue);
}

