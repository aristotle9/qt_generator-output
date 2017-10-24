#include "qt_core_c_QXmlStreamNotationDeclaration.h"

void qt_core_c_QXmlStreamNotationDeclaration_constructor_arg1(const QXmlStreamNotationDeclaration* arg1, QXmlStreamNotationDeclaration* output) {
  new(output) QXmlStreamNotationDeclaration(*arg1);
}

void qt_core_c_QXmlStreamNotationDeclaration_constructor_no_args(QXmlStreamNotationDeclaration* output) {
  new(output) QXmlStreamNotationDeclaration();
}

void qt_core_c_QXmlStreamNotationDeclaration_destructor(QXmlStreamNotationDeclaration* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QXmlStreamNotationDeclaration_name_to_output(const QXmlStreamNotationDeclaration* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->name());
}

QXmlStreamNotationDeclaration* qt_core_c_QXmlStreamNotationDeclaration_operator_assign(QXmlStreamNotationDeclaration* this_ptr, const QXmlStreamNotationDeclaration* arg1) {
  return &this_ptr->operator=(*arg1);
}

bool qt_core_c_QXmlStreamNotationDeclaration_operator_eq(const QXmlStreamNotationDeclaration* this_ptr, const QXmlStreamNotationDeclaration* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QXmlStreamNotationDeclaration_operator_neq(const QXmlStreamNotationDeclaration* this_ptr, const QXmlStreamNotationDeclaration* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QXmlStreamNotationDeclaration_publicId_to_output(const QXmlStreamNotationDeclaration* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->publicId());
}

void qt_core_c_QXmlStreamNotationDeclaration_systemId_to_output(const QXmlStreamNotationDeclaration* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->systemId());
}

