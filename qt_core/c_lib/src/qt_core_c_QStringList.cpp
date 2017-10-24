#include "qt_core_c_QStringList.h"

void qt_core_c_QStringList_G_operator_add_to_output(const QList< QString >* one, const QStringList* other, QStringList* output) {
  new(output) QStringList(operator+(*one, *other));
}

QList< QString >* qt_core_c_QStringList_G_static_cast_QList_QString_ptr(QStringList* ptr) {
  return static_cast<QList< QString >*>(ptr);
}

QStringList* qt_core_c_QStringList_G_static_cast_QStringList_ptr(QList< QString >* ptr) {
  return static_cast<QStringList*>(ptr);
}

void qt_core_c_QStringList_constructor_i(const QString* i, QStringList* output) {
  new(output) QStringList(*i);
}

void qt_core_c_QStringList_constructor_l(const QList< QString >* l, QStringList* output) {
  new(output) QStringList(*l);
}

void qt_core_c_QStringList_constructor_no_args(QStringList* output) {
  new(output) QStringList();
}

bool qt_core_c_QStringList_contains_str(const QStringList* this_ptr, const QString* str) {
  return this_ptr->contains(*str);
}

bool qt_core_c_QStringList_contains_str_cs(const QStringList* this_ptr, const QString* str, const Qt::CaseSensitivity* cs) {
  return this_ptr->contains(*str, *cs);
}

void qt_core_c_QStringList_destructor(QStringList* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

int qt_core_c_QStringList_indexOf_QRegExp_ref(const QStringList* this_ptr, QRegExp* rx) {
  return this_ptr->indexOf(*rx);
}

int qt_core_c_QStringList_indexOf_QRegExp_ref_int(const QStringList* this_ptr, QRegExp* rx, int from) {
  return this_ptr->indexOf(*rx, from);
}

int qt_core_c_QStringList_indexOf_const_QRegExp_ref(const QStringList* this_ptr, const QRegExp* rx) {
  return this_ptr->indexOf(*rx);
}

int qt_core_c_QStringList_indexOf_const_QRegExp_ref_int(const QStringList* this_ptr, const QRegExp* rx, int from) {
  return this_ptr->indexOf(*rx, from);
}

int qt_core_c_QStringList_indexOf_const_QRegularExpression_ref(const QStringList* this_ptr, const QRegularExpression* re) {
  return this_ptr->indexOf(*re);
}

int qt_core_c_QStringList_indexOf_const_QRegularExpression_ref_int(const QStringList* this_ptr, const QRegularExpression* re, int from) {
  return this_ptr->indexOf(*re, from);
}

int qt_core_c_QStringList_lastIndexOf_QRegExp_ref(const QStringList* this_ptr, QRegExp* rx) {
  return this_ptr->lastIndexOf(*rx);
}

int qt_core_c_QStringList_lastIndexOf_QRegExp_ref_int(const QStringList* this_ptr, QRegExp* rx, int from) {
  return this_ptr->lastIndexOf(*rx, from);
}

int qt_core_c_QStringList_lastIndexOf_const_QRegExp_ref(const QStringList* this_ptr, const QRegExp* rx) {
  return this_ptr->lastIndexOf(*rx);
}

int qt_core_c_QStringList_lastIndexOf_const_QRegExp_ref_int(const QStringList* this_ptr, const QRegExp* rx, int from) {
  return this_ptr->lastIndexOf(*rx, from);
}

int qt_core_c_QStringList_lastIndexOf_const_QRegularExpression_ref(const QStringList* this_ptr, const QRegularExpression* re) {
  return this_ptr->lastIndexOf(*re);
}

int qt_core_c_QStringList_lastIndexOf_const_QRegularExpression_ref_int(const QStringList* this_ptr, const QRegularExpression* re, int from) {
  return this_ptr->lastIndexOf(*re, from);
}

void qt_core_c_QStringList_operator_add_to_output(const QStringList* this_ptr, const QStringList* other, QStringList* output) {
  new(output) QStringList(this_ptr->operator+(*other));
}

QStringList* qt_core_c_QStringList_operator_assign(QStringList* this_ptr, const QList< QString >* other) {
  return &this_ptr->operator=(*other);
}

QStringList* qt_core_c_QStringList_operator_shl_QList_QString(QStringList* this_ptr, const QList< QString >* l) {
  return &this_ptr->operator<<(*l);
}

QStringList* qt_core_c_QStringList_operator_shl_QString(QStringList* this_ptr, const QString* str) {
  return &this_ptr->operator<<(*str);
}

QStringList* qt_core_c_QStringList_operator_shl_QStringList(QStringList* this_ptr, const QStringList* l) {
  return &this_ptr->operator<<(*l);
}

