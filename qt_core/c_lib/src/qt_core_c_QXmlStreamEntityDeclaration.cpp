#include "qt_core_c_QXmlStreamEntityDeclaration.h"

void qt_core_c_QXmlStreamEntityDeclaration_constructor_arg1(const QXmlStreamEntityDeclaration* arg1, QXmlStreamEntityDeclaration* output) {
  new(output) QXmlStreamEntityDeclaration(*arg1);
}

void qt_core_c_QXmlStreamEntityDeclaration_constructor_no_args(QXmlStreamEntityDeclaration* output) {
  new(output) QXmlStreamEntityDeclaration();
}

void qt_core_c_QXmlStreamEntityDeclaration_destructor(QXmlStreamEntityDeclaration* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QXmlStreamEntityDeclaration_name_to_output(const QXmlStreamEntityDeclaration* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->name());
}

void qt_core_c_QXmlStreamEntityDeclaration_notationName_to_output(const QXmlStreamEntityDeclaration* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->notationName());
}

QXmlStreamEntityDeclaration* qt_core_c_QXmlStreamEntityDeclaration_operator_assign(QXmlStreamEntityDeclaration* this_ptr, const QXmlStreamEntityDeclaration* arg1) {
  return &this_ptr->operator=(*arg1);
}

bool qt_core_c_QXmlStreamEntityDeclaration_operator_eq(const QXmlStreamEntityDeclaration* this_ptr, const QXmlStreamEntityDeclaration* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QXmlStreamEntityDeclaration_operator_neq(const QXmlStreamEntityDeclaration* this_ptr, const QXmlStreamEntityDeclaration* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QXmlStreamEntityDeclaration_publicId_to_output(const QXmlStreamEntityDeclaration* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->publicId());
}

void qt_core_c_QXmlStreamEntityDeclaration_systemId_to_output(const QXmlStreamEntityDeclaration* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->systemId());
}

void qt_core_c_QXmlStreamEntityDeclaration_value_to_output(const QXmlStreamEntityDeclaration* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->value());
}

