#include "qt_core_c_QMimeType.h"

void qt_core_c_QMimeType_G_operator_shl_to_output(const QDebug* debug, const QMimeType* mime, QDebug* output) {
  new(output) QDebug(operator<<(*debug, *mime));
}

unsigned int qt_core_c_QMimeType_G_qHash_key(const QMimeType* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QMimeType_G_qHash_key_seed(const QMimeType* key, unsigned int seed) {
  return qHash(*key, seed);
}

void qt_core_c_QMimeType_G_swap(QMimeType* value1, QMimeType* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QMimeType_aliases_to_output(const QMimeType* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->aliases());
}

void qt_core_c_QMimeType_allAncestors_to_output(const QMimeType* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->allAncestors());
}

void qt_core_c_QMimeType_comment_to_output(const QMimeType* this_ptr, QString* output) {
  new(output) QString(this_ptr->comment());
}

void qt_core_c_QMimeType_constructor_no_args(QMimeType* output) {
  new(output) QMimeType();
}

void qt_core_c_QMimeType_constructor_other(const QMimeType* other, QMimeType* output) {
  new(output) QMimeType(*other);
}

void qt_core_c_QMimeType_destructor(QMimeType* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QMimeType_filterString_to_output(const QMimeType* this_ptr, QString* output) {
  new(output) QString(this_ptr->filterString());
}

void qt_core_c_QMimeType_genericIconName_to_output(const QMimeType* this_ptr, QString* output) {
  new(output) QString(this_ptr->genericIconName());
}

void qt_core_c_QMimeType_globPatterns_to_output(const QMimeType* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->globPatterns());
}

void qt_core_c_QMimeType_iconName_to_output(const QMimeType* this_ptr, QString* output) {
  new(output) QString(this_ptr->iconName());
}

bool qt_core_c_QMimeType_inherits(const QMimeType* this_ptr, const QString* mimeTypeName) {
  return this_ptr->inherits(*mimeTypeName);
}

bool qt_core_c_QMimeType_isDefault(const QMimeType* this_ptr) {
  return this_ptr->isDefault();
}

bool qt_core_c_QMimeType_isValid(const QMimeType* this_ptr) {
  return this_ptr->isValid();
}

void qt_core_c_QMimeType_name_to_output(const QMimeType* this_ptr, QString* output) {
  new(output) QString(this_ptr->name());
}

QMimeType* qt_core_c_QMimeType_operator_assign(QMimeType* this_ptr, const QMimeType* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QMimeType_operator_eq(const QMimeType* this_ptr, const QMimeType* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QMimeType_operator_neq(const QMimeType* this_ptr, const QMimeType* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QMimeType_parentMimeTypes_to_output(const QMimeType* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->parentMimeTypes());
}

void qt_core_c_QMimeType_preferredSuffix_to_output(const QMimeType* this_ptr, QString* output) {
  new(output) QString(this_ptr->preferredSuffix());
}

void qt_core_c_QMimeType_suffixes_to_output(const QMimeType* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->suffixes());
}

void qt_core_c_QMimeType_swap(QMimeType* this_ptr, QMimeType* other) {
  this_ptr->swap(*other);
}

