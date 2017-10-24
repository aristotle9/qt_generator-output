#include "qt_core_c_QXmlStreamNamespaceDeclaration.h"

void qt_core_c_QXmlStreamNamespaceDeclaration_constructor_arg1(const QXmlStreamNamespaceDeclaration* arg1, QXmlStreamNamespaceDeclaration* output) {
  new(output) QXmlStreamNamespaceDeclaration(*arg1);
}

void qt_core_c_QXmlStreamNamespaceDeclaration_constructor_no_args(QXmlStreamNamespaceDeclaration* output) {
  new(output) QXmlStreamNamespaceDeclaration();
}

void qt_core_c_QXmlStreamNamespaceDeclaration_constructor_prefix_namespaceUri(const QString* prefix, const QString* namespaceUri, QXmlStreamNamespaceDeclaration* output) {
  new(output) QXmlStreamNamespaceDeclaration(*prefix, *namespaceUri);
}

void qt_core_c_QXmlStreamNamespaceDeclaration_destructor(QXmlStreamNamespaceDeclaration* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QXmlStreamNamespaceDeclaration_namespaceUri_to_output(const QXmlStreamNamespaceDeclaration* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->namespaceUri());
}

QXmlStreamNamespaceDeclaration* qt_core_c_QXmlStreamNamespaceDeclaration_operator_assign(QXmlStreamNamespaceDeclaration* this_ptr, const QXmlStreamNamespaceDeclaration* arg1) {
  return &this_ptr->operator=(*arg1);
}

bool qt_core_c_QXmlStreamNamespaceDeclaration_operator_eq(const QXmlStreamNamespaceDeclaration* this_ptr, const QXmlStreamNamespaceDeclaration* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QXmlStreamNamespaceDeclaration_operator_neq(const QXmlStreamNamespaceDeclaration* this_ptr, const QXmlStreamNamespaceDeclaration* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QXmlStreamNamespaceDeclaration_prefix_to_output(const QXmlStreamNamespaceDeclaration* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->prefix());
}

