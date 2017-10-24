#include "qt_core_c_QLoggingCategory.h"

const char* qt_core_c_QLoggingCategory_categoryName(const QLoggingCategory* this_ptr) {
  return this_ptr->categoryName();
}

void qt_core_c_QLoggingCategory_constructor_category(const char* category, QLoggingCategory* output) {
  new(output) QLoggingCategory(category);
}

void qt_core_c_QLoggingCategory_constructor_category_severityLevel(const char* category, QtMsgType severityLevel, QLoggingCategory* output) {
  new(output) QLoggingCategory(category, severityLevel);
}

QLoggingCategory* qt_core_c_QLoggingCategory_defaultCategory() {
  return QLoggingCategory::defaultCategory();
}

void qt_core_c_QLoggingCategory_destructor(QLoggingCategory* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void (*qt_core_c_QLoggingCategory_installFilter(void (*arg1)(QLoggingCategory*)))(QLoggingCategory*) {
  return QLoggingCategory::installFilter(arg1);
}

bool qt_core_c_QLoggingCategory_isCriticalEnabled(const QLoggingCategory* this_ptr) {
  return this_ptr->isCriticalEnabled();
}

bool qt_core_c_QLoggingCategory_isDebugEnabled(const QLoggingCategory* this_ptr) {
  return this_ptr->isDebugEnabled();
}

bool qt_core_c_QLoggingCategory_isEnabled(const QLoggingCategory* this_ptr, QtMsgType type) {
  return this_ptr->isEnabled(type);
}

bool qt_core_c_QLoggingCategory_isInfoEnabled(const QLoggingCategory* this_ptr) {
  return this_ptr->isInfoEnabled();
}

bool qt_core_c_QLoggingCategory_isWarningEnabled(const QLoggingCategory* this_ptr) {
  return this_ptr->isWarningEnabled();
}

QLoggingCategory* qt_core_c_QLoggingCategory_operator_call(QLoggingCategory* this_ptr) {
  return &this_ptr->operator()();
}

const QLoggingCategory* qt_core_c_QLoggingCategory_operator_call_const(const QLoggingCategory* this_ptr) {
  return &this_ptr->operator()();
}

void qt_core_c_QLoggingCategory_setEnabled(QLoggingCategory* this_ptr, QtMsgType type, bool enable) {
  this_ptr->setEnabled(type, enable);
}

void qt_core_c_QLoggingCategory_setFilterRules(const QString* rules) {
  QLoggingCategory::setFilterRules(*rules);
}

