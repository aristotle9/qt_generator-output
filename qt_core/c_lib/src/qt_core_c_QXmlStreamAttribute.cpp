#include "qt_core_c_QXmlStreamAttribute.h"

void qt_core_c_QXmlStreamAttribute_G_swap(QXmlStreamStringRef* value1, QXmlStreamStringRef* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QXmlStreamAttribute_constructor_arg1(const QXmlStreamAttribute* arg1, QXmlStreamAttribute* output) {
  new(output) QXmlStreamAttribute(*arg1);
}

void qt_core_c_QXmlStreamAttribute_constructor_namespaceUri_name_value(const QString* namespaceUri, const QString* name, const QString* value, QXmlStreamAttribute* output) {
  new(output) QXmlStreamAttribute(*namespaceUri, *name, *value);
}

void qt_core_c_QXmlStreamAttribute_constructor_no_args(QXmlStreamAttribute* output) {
  new(output) QXmlStreamAttribute();
}

void qt_core_c_QXmlStreamAttribute_constructor_qualifiedName_value(const QString* qualifiedName, const QString* value, QXmlStreamAttribute* output) {
  new(output) QXmlStreamAttribute(*qualifiedName, *value);
}

void qt_core_c_QXmlStreamAttribute_destructor(QXmlStreamAttribute* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QXmlStreamAttribute_isDefault(const QXmlStreamAttribute* this_ptr) {
  return this_ptr->isDefault();
}

void qt_core_c_QXmlStreamAttribute_name_to_output(const QXmlStreamAttribute* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->name());
}

void qt_core_c_QXmlStreamAttribute_namespaceUri_to_output(const QXmlStreamAttribute* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->namespaceUri());
}

QXmlStreamAttribute* qt_core_c_QXmlStreamAttribute_operator_assign(QXmlStreamAttribute* this_ptr, const QXmlStreamAttribute* arg1) {
  return &this_ptr->operator=(*arg1);
}

bool qt_core_c_QXmlStreamAttribute_operator_eq(const QXmlStreamAttribute* this_ptr, const QXmlStreamAttribute* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QXmlStreamAttribute_operator_neq(const QXmlStreamAttribute* this_ptr, const QXmlStreamAttribute* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QXmlStreamAttribute_prefix_to_output(const QXmlStreamAttribute* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->prefix());
}

void qt_core_c_QXmlStreamAttribute_qualifiedName_to_output(const QXmlStreamAttribute* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->qualifiedName());
}

void qt_core_c_QXmlStreamAttribute_value_to_output(const QXmlStreamAttribute* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->value());
}

