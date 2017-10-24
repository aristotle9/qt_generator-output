#include "qt_core_c_QCommandLineOption.h"

void qt_core_c_QCommandLineOption_G_swap(QCommandLineOption* value1, QCommandLineOption* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QCommandLineOption_constructor_name(const QString* name, QCommandLineOption* output) {
  new(output) QCommandLineOption(*name);
}

void qt_core_c_QCommandLineOption_constructor_name_description(const QString* name, const QString* description, QCommandLineOption* output) {
  new(output) QCommandLineOption(*name, *description);
}

void qt_core_c_QCommandLineOption_constructor_name_description_valueName(const QString* name, const QString* description, const QString* valueName, QCommandLineOption* output) {
  new(output) QCommandLineOption(*name, *description, *valueName);
}

void qt_core_c_QCommandLineOption_constructor_name_description_valueName_defaultValue(const QString* name, const QString* description, const QString* valueName, const QString* defaultValue, QCommandLineOption* output) {
  new(output) QCommandLineOption(*name, *description, *valueName, *defaultValue);
}

void qt_core_c_QCommandLineOption_constructor_names(const QStringList* names, QCommandLineOption* output) {
  new(output) QCommandLineOption(*names);
}

void qt_core_c_QCommandLineOption_constructor_names_description(const QStringList* names, const QString* description, QCommandLineOption* output) {
  new(output) QCommandLineOption(*names, *description);
}

void qt_core_c_QCommandLineOption_constructor_names_description_valueName(const QStringList* names, const QString* description, const QString* valueName, QCommandLineOption* output) {
  new(output) QCommandLineOption(*names, *description, *valueName);
}

void qt_core_c_QCommandLineOption_constructor_names_description_valueName_defaultValue(const QStringList* names, const QString* description, const QString* valueName, const QString* defaultValue, QCommandLineOption* output) {
  new(output) QCommandLineOption(*names, *description, *valueName, *defaultValue);
}

void qt_core_c_QCommandLineOption_constructor_other(const QCommandLineOption* other, QCommandLineOption* output) {
  new(output) QCommandLineOption(*other);
}

void qt_core_c_QCommandLineOption_defaultValues_to_output(const QCommandLineOption* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->defaultValues());
}

void qt_core_c_QCommandLineOption_description_to_output(const QCommandLineOption* this_ptr, QString* output) {
  new(output) QString(this_ptr->description());
}

void qt_core_c_QCommandLineOption_destructor(QCommandLineOption* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

unsigned int qt_core_c_QCommandLineOption_flags(const QCommandLineOption* this_ptr) {
  return uint(this_ptr->flags());
}

bool qt_core_c_QCommandLineOption_isHidden(const QCommandLineOption* this_ptr) {
  return this_ptr->isHidden();
}

void qt_core_c_QCommandLineOption_names_to_output(const QCommandLineOption* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->names());
}

QCommandLineOption* qt_core_c_QCommandLineOption_operator_assign(QCommandLineOption* this_ptr, const QCommandLineOption* other) {
  return &this_ptr->operator=(*other);
}

void qt_core_c_QCommandLineOption_setDefaultValue(QCommandLineOption* this_ptr, const QString* defaultValue) {
  this_ptr->setDefaultValue(*defaultValue);
}

void qt_core_c_QCommandLineOption_setDefaultValues(QCommandLineOption* this_ptr, const QStringList* defaultValues) {
  this_ptr->setDefaultValues(*defaultValues);
}

void qt_core_c_QCommandLineOption_setDescription(QCommandLineOption* this_ptr, const QString* description) {
  this_ptr->setDescription(*description);
}

void qt_core_c_QCommandLineOption_setFlags(QCommandLineOption* this_ptr, unsigned int aflags) {
  this_ptr->setFlags(QFlags< QCommandLineOption::Flag >(aflags));
}

void qt_core_c_QCommandLineOption_setHidden(QCommandLineOption* this_ptr, bool hidden) {
  this_ptr->setHidden(hidden);
}

void qt_core_c_QCommandLineOption_setValueName(QCommandLineOption* this_ptr, const QString* name) {
  this_ptr->setValueName(*name);
}

void qt_core_c_QCommandLineOption_swap(QCommandLineOption* this_ptr, QCommandLineOption* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QCommandLineOption_valueName_to_output(const QCommandLineOption* this_ptr, QString* output) {
  new(output) QString(this_ptr->valueName());
}

