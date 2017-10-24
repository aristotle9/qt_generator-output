#include "qt_core_c_QXmlStreamStringRef.h"

void qt_core_c_QXmlStreamStringRef_clear(QXmlStreamStringRef* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QXmlStreamStringRef_constructor_QString(const QString* aString, QXmlStreamStringRef* output) {
  new(output) QXmlStreamStringRef(*aString);
}

void qt_core_c_QXmlStreamStringRef_constructor_QStringRef(const QStringRef* aString, QXmlStreamStringRef* output) {
  new(output) QXmlStreamStringRef(*aString);
}

void qt_core_c_QXmlStreamStringRef_constructor_QXmlStreamStringRef(const QXmlStreamStringRef* other, QXmlStreamStringRef* output) {
  new(output) QXmlStreamStringRef(*other);
}

void qt_core_c_QXmlStreamStringRef_constructor_no_args(QXmlStreamStringRef* output) {
  new(output) QXmlStreamStringRef();
}

void qt_core_c_QXmlStreamStringRef_convert_to_QStringRef_to_output(const QXmlStreamStringRef* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->operator QStringRef());
}

void qt_core_c_QXmlStreamStringRef_destructor(QXmlStreamStringRef* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QXmlStreamStringRef* qt_core_c_QXmlStreamStringRef_operator_assign(QXmlStreamStringRef* this_ptr, const QXmlStreamStringRef* other) {
  return &this_ptr->operator=(*other);
}

int qt_core_c_QXmlStreamStringRef_position(const QXmlStreamStringRef* this_ptr) {
  return this_ptr->position();
}

int qt_core_c_QXmlStreamStringRef_size(const QXmlStreamStringRef* this_ptr) {
  return this_ptr->size();
}

const QString* qt_core_c_QXmlStreamStringRef_string(const QXmlStreamStringRef* this_ptr) {
  return this_ptr->string();
}

void qt_core_c_QXmlStreamStringRef_swap(QXmlStreamStringRef* this_ptr, QXmlStreamStringRef* other) {
  this_ptr->swap(*other);
}

