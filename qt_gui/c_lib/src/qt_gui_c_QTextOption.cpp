#include "qt_gui_c_QTextOption.h"

void qt_gui_c_QTextOption_Tab_constructor_no_args(QTextOption::Tab* output) {
  new(output) QTextOption::Tab();
}

void qt_gui_c_QTextOption_Tab_constructor_pos_tabType(double pos, QTextOption::TabType tabType, QTextOption::Tab* output) {
  new(output) QTextOption::Tab(pos, tabType);
}

void qt_gui_c_QTextOption_Tab_constructor_pos_tabType_delim(double pos, QTextOption::TabType tabType, const QChar* delim, QTextOption::Tab* output) {
  new(output) QTextOption::Tab(pos, tabType, *delim);
}

const QChar* qt_gui_c_QTextOption_Tab_delimiter(const QTextOption::Tab* this_ptr) {
  return &this_ptr->delimiter;
}

QChar* qt_gui_c_QTextOption_Tab_delimiter_mut(QTextOption::Tab* this_ptr) {
  return &this_ptr->delimiter;
}

void qt_gui_c_QTextOption_Tab_destructor(QTextOption::Tab* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QTextOption_Tab_operator_eq(const QTextOption::Tab* this_ptr, const QTextOption::Tab* other) {
  return this_ptr->operator==(*other);
}

bool qt_gui_c_QTextOption_Tab_operator_neq(const QTextOption::Tab* this_ptr, const QTextOption::Tab* other) {
  return this_ptr->operator!=(*other);
}

double qt_gui_c_QTextOption_Tab_position(const QTextOption::Tab* this_ptr) {
  return this_ptr->position;
}

void qt_gui_c_QTextOption_Tab_set_delimiter(QTextOption::Tab* this_ptr, const QChar* value) {
  this_ptr->delimiter = *value;
}

void qt_gui_c_QTextOption_Tab_set_position(QTextOption::Tab* this_ptr, double value) {
  this_ptr->position = value;
}

void qt_gui_c_QTextOption_Tab_set_type(QTextOption::Tab* this_ptr, QTextOption::TabType value) {
  this_ptr->type = value;
}

QTextOption::TabType qt_gui_c_QTextOption_Tab_type(const QTextOption::Tab* this_ptr) {
  return this_ptr->type;
}

void qt_gui_c_QTextOption_constructor_no_args(QTextOption* output) {
  new(output) QTextOption();
}

void qt_gui_c_QTextOption_constructor_o(const QTextOption* o, QTextOption* output) {
  new(output) QTextOption(*o);
}

void qt_gui_c_QTextOption_destructor(QTextOption* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

unsigned int qt_gui_c_QTextOption_flags(const QTextOption* this_ptr) {
  return uint(this_ptr->flags());
}

QTextOption* qt_gui_c_QTextOption_operator_assign(QTextOption* this_ptr, const QTextOption* o) {
  return &this_ptr->operator=(*o);
}

void qt_gui_c_QTextOption_setFlags(QTextOption* this_ptr, unsigned int flags) {
  this_ptr->setFlags(QFlags< QTextOption::Flag >(flags));
}

void qt_gui_c_QTextOption_setTabArray(QTextOption* this_ptr, const QList< double >* tabStops) {
  this_ptr->setTabArray(*tabStops);
}

void qt_gui_c_QTextOption_setTabStop(QTextOption* this_ptr, double tabStop) {
  this_ptr->setTabStop(tabStop);
}

void qt_gui_c_QTextOption_setTabs(QTextOption* this_ptr, const QList< QTextOption::Tab >* tabStops) {
  this_ptr->setTabs(*tabStops);
}

void qt_gui_c_QTextOption_setTextDirection(QTextOption* this_ptr, const Qt::LayoutDirection* aDirection) {
  this_ptr->setTextDirection(*aDirection);
}

void qt_gui_c_QTextOption_setUseDesignMetrics(QTextOption* this_ptr, bool b) {
  this_ptr->setUseDesignMetrics(b);
}

void qt_gui_c_QTextOption_setWrapMode(QTextOption* this_ptr, QTextOption::WrapMode wrap) {
  this_ptr->setWrapMode(wrap);
}

void qt_gui_c_QTextOption_tabArray_to_output(const QTextOption* this_ptr, QList< double >* output) {
  new(output) QList< double >(this_ptr->tabArray());
}

double qt_gui_c_QTextOption_tabStop(const QTextOption* this_ptr) {
  return this_ptr->tabStop();
}

void qt_gui_c_QTextOption_tabs_to_output(const QTextOption* this_ptr, QList< QTextOption::Tab >* output) {
  new(output) QList< QTextOption::Tab >(this_ptr->tabs());
}

bool qt_gui_c_QTextOption_useDesignMetrics(const QTextOption* this_ptr) {
  return this_ptr->useDesignMetrics();
}

QTextOption::WrapMode qt_gui_c_QTextOption_wrapMode(const QTextOption* this_ptr) {
  return this_ptr->wrapMode();
}

