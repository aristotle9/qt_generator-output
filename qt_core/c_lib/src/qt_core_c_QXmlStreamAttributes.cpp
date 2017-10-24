#include "qt_core_c_QXmlStreamAttributes.h"

QVector< QXmlStreamAttribute >* qt_core_c_QXmlStreamAttributes_G_static_cast_QVector_QXmlStreamAttribute_ptr(QXmlStreamAttributes* ptr) {
  return static_cast<QVector< QXmlStreamAttribute >*>(ptr);
}

QXmlStreamAttributes* qt_core_c_QXmlStreamAttributes_G_static_cast_QXmlStreamAttributes_ptr(QVector< QXmlStreamAttribute >* ptr) {
  return static_cast<QXmlStreamAttributes*>(ptr);
}

void qt_core_c_QXmlStreamAttributes_append_namespaceUri_name_value(QXmlStreamAttributes* this_ptr, const QString* namespaceUri, const QString* name, const QString* value) {
  this_ptr->append(*namespaceUri, *name, *value);
}

void qt_core_c_QXmlStreamAttributes_append_qualifiedName_value(QXmlStreamAttributes* this_ptr, const QString* qualifiedName, const QString* value) {
  this_ptr->append(*qualifiedName, *value);
}

void qt_core_c_QXmlStreamAttributes_constructor(QXmlStreamAttributes* output) {
  new(output) QXmlStreamAttributes();
}

void qt_core_c_QXmlStreamAttributes_destructor(QXmlStreamAttributes* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QXmlStreamAttributes_hasAttribute_QLatin1String(const QXmlStreamAttributes* this_ptr, const QLatin1String* qualifiedName) {
  return this_ptr->hasAttribute(*qualifiedName);
}

bool qt_core_c_QXmlStreamAttributes_hasAttribute_QString(const QXmlStreamAttributes* this_ptr, const QString* qualifiedName) {
  return this_ptr->hasAttribute(*qualifiedName);
}

bool qt_core_c_QXmlStreamAttributes_hasAttribute_QString_QString(const QXmlStreamAttributes* this_ptr, const QString* namespaceUri, const QString* name) {
  return this_ptr->hasAttribute(*namespaceUri, *name);
}

void qt_core_c_QXmlStreamAttributes_value_to_output_QLatin1String(const QXmlStreamAttributes* this_ptr, const QLatin1String* qualifiedName, QStringRef* output) {
  new(output) QStringRef(this_ptr->value(*qualifiedName));
}

void qt_core_c_QXmlStreamAttributes_value_to_output_QLatin1String_QLatin1String(const QXmlStreamAttributes* this_ptr, const QLatin1String* namespaceUri, const QLatin1String* name, QStringRef* output) {
  new(output) QStringRef(this_ptr->value(*namespaceUri, *name));
}

void qt_core_c_QXmlStreamAttributes_value_to_output_QString(const QXmlStreamAttributes* this_ptr, const QString* qualifiedName, QStringRef* output) {
  new(output) QStringRef(this_ptr->value(*qualifiedName));
}

void qt_core_c_QXmlStreamAttributes_value_to_output_QString_QLatin1String(const QXmlStreamAttributes* this_ptr, const QString* namespaceUri, const QLatin1String* name, QStringRef* output) {
  new(output) QStringRef(this_ptr->value(*namespaceUri, *name));
}

void qt_core_c_QXmlStreamAttributes_value_to_output_QString_QString(const QXmlStreamAttributes* this_ptr, const QString* namespaceUri, const QString* name, QStringRef* output) {
  new(output) QStringRef(this_ptr->value(*namespaceUri, *name));
}

