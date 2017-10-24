#include "qt_core_c_QSettings.h"

QSettings* qt_core_c_QSettings_G_dynamic_cast_QSettings_ptr(QObject* ptr) {
  return dynamic_cast<QSettings*>(ptr);
}

QObject* qt_core_c_QSettings_G_static_cast_QObject_ptr(QSettings* ptr) {
  return static_cast<QObject*>(ptr);
}

QSettings* qt_core_c_QSettings_G_static_cast_QSettings_ptr(QObject* ptr) {
  return static_cast<QSettings*>(ptr);
}

void qt_core_c_QSettings_allKeys_to_output(const QSettings* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->allKeys());
}

void qt_core_c_QSettings_applicationName_to_output(const QSettings* this_ptr, QString* output) {
  new(output) QString(this_ptr->applicationName());
}

void qt_core_c_QSettings_beginGroup(QSettings* this_ptr, const QString* prefix) {
  this_ptr->beginGroup(*prefix);
}

int qt_core_c_QSettings_beginReadArray(QSettings* this_ptr, const QString* prefix) {
  return this_ptr->beginReadArray(*prefix);
}

void qt_core_c_QSettings_beginWriteArray_prefix(QSettings* this_ptr, const QString* prefix) {
  this_ptr->beginWriteArray(*prefix);
}

void qt_core_c_QSettings_beginWriteArray_prefix_size(QSettings* this_ptr, const QString* prefix, int size) {
  this_ptr->beginWriteArray(*prefix, size);
}

void qt_core_c_QSettings_childGroups_to_output(const QSettings* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->childGroups());
}

void qt_core_c_QSettings_childKeys_to_output(const QSettings* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->childKeys());
}

void qt_core_c_QSettings_clear(QSettings* this_ptr) {
  this_ptr->clear();
}

bool qt_core_c_QSettings_contains(const QSettings* this_ptr, const QString* key) {
  return this_ptr->contains(*key);
}

QSettings::Format qt_core_c_QSettings_defaultFormat() {
  return QSettings::defaultFormat();
}

void qt_core_c_QSettings_delete(QSettings* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QSettings_endArray(QSettings* this_ptr) {
  this_ptr->endArray();
}

void qt_core_c_QSettings_endGroup(QSettings* this_ptr) {
  this_ptr->endGroup();
}

bool qt_core_c_QSettings_fallbacksEnabled(const QSettings* this_ptr) {
  return this_ptr->fallbacksEnabled();
}

void qt_core_c_QSettings_fileName_to_output(const QSettings* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

QSettings::Format qt_core_c_QSettings_format(const QSettings* this_ptr) {
  return this_ptr->format();
}

void qt_core_c_QSettings_group_to_output(const QSettings* this_ptr, QString* output) {
  new(output) QString(this_ptr->group());
}

QTextCodec* qt_core_c_QSettings_iniCodec(const QSettings* this_ptr) {
  return this_ptr->iniCodec();
}

bool qt_core_c_QSettings_isWritable(const QSettings* this_ptr) {
  return this_ptr->isWritable();
}

const QMetaObject* qt_core_c_QSettings_metaObject(const QSettings* this_ptr) {
  return this_ptr->metaObject();
}

QSettings* qt_core_c_QSettings_new_fileName_format(const QString* fileName, QSettings::Format format) {
  return new QSettings(*fileName, format);
}

QSettings* qt_core_c_QSettings_new_fileName_format_parent(const QString* fileName, QSettings::Format format, QObject* parent) {
  return new QSettings(*fileName, format, parent);
}

QSettings* qt_core_c_QSettings_new_format_scope_organization(QSettings::Format format, QSettings::Scope scope, const QString* organization) {
  return new QSettings(format, scope, *organization);
}

QSettings* qt_core_c_QSettings_new_format_scope_organization_application(QSettings::Format format, QSettings::Scope scope, const QString* organization, const QString* application) {
  return new QSettings(format, scope, *organization, *application);
}

QSettings* qt_core_c_QSettings_new_format_scope_organization_application_parent(QSettings::Format format, QSettings::Scope scope, const QString* organization, const QString* application, QObject* parent) {
  return new QSettings(format, scope, *organization, *application, parent);
}

QSettings* qt_core_c_QSettings_new_no_args() {
  return new QSettings();
}

QSettings* qt_core_c_QSettings_new_organization(const QString* organization) {
  return new QSettings(*organization);
}

QSettings* qt_core_c_QSettings_new_organization_application(const QString* organization, const QString* application) {
  return new QSettings(*organization, *application);
}

QSettings* qt_core_c_QSettings_new_organization_application_parent(const QString* organization, const QString* application, QObject* parent) {
  return new QSettings(*organization, *application, parent);
}

QSettings* qt_core_c_QSettings_new_parent(QObject* parent) {
  return new QSettings(parent);
}

QSettings* qt_core_c_QSettings_new_scope_organization(QSettings::Scope scope, const QString* organization) {
  return new QSettings(scope, *organization);
}

QSettings* qt_core_c_QSettings_new_scope_organization_application(QSettings::Scope scope, const QString* organization, const QString* application) {
  return new QSettings(scope, *organization, *application);
}

QSettings* qt_core_c_QSettings_new_scope_organization_application_parent(QSettings::Scope scope, const QString* organization, const QString* application, QObject* parent) {
  return new QSettings(scope, *organization, *application, parent);
}

void qt_core_c_QSettings_organizationName_to_output(const QSettings* this_ptr, QString* output) {
  new(output) QString(this_ptr->organizationName());
}

void qt_core_c_QSettings_remove(QSettings* this_ptr, const QString* key) {
  this_ptr->remove(*key);
}

QSettings::Scope qt_core_c_QSettings_scope(const QSettings* this_ptr) {
  return this_ptr->scope();
}

void qt_core_c_QSettings_setArrayIndex(QSettings* this_ptr, int i) {
  this_ptr->setArrayIndex(i);
}

void qt_core_c_QSettings_setDefaultFormat(QSettings::Format format) {
  QSettings::setDefaultFormat(format);
}

void qt_core_c_QSettings_setFallbacksEnabled(QSettings* this_ptr, bool b) {
  this_ptr->setFallbacksEnabled(b);
}

void qt_core_c_QSettings_setIniCodec_codec(QSettings* this_ptr, QTextCodec* codec) {
  this_ptr->setIniCodec(codec);
}

void qt_core_c_QSettings_setIniCodec_codecName(QSettings* this_ptr, const char* codecName) {
  this_ptr->setIniCodec(codecName);
}

void qt_core_c_QSettings_setPath(QSettings::Format format, QSettings::Scope scope, const QString* path) {
  QSettings::setPath(format, scope, *path);
}

void qt_core_c_QSettings_setSystemIniPath(const QString* dir) {
  QSettings::setSystemIniPath(*dir);
}

void qt_core_c_QSettings_setUserIniPath(const QString* dir) {
  QSettings::setUserIniPath(*dir);
}

void qt_core_c_QSettings_setValue(QSettings* this_ptr, const QString* key, const QVariant* value) {
  this_ptr->setValue(*key, *value);
}

QSettings::Status qt_core_c_QSettings_status(const QSettings* this_ptr) {
  return this_ptr->status();
}

void qt_core_c_QSettings_sync(QSettings* this_ptr) {
  this_ptr->sync();
}

void qt_core_c_QSettings_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSettings::trUtf8(s, c, n));
}

void qt_core_c_QSettings_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSettings::tr(s, c, n));
}

void qt_core_c_QSettings_value_to_output_key(const QSettings* this_ptr, const QString* key, QVariant* output) {
  new(output) QVariant(this_ptr->value(*key));
}

void qt_core_c_QSettings_value_to_output_key_defaultValue(const QSettings* this_ptr, const QString* key, const QVariant* defaultValue, QVariant* output) {
  new(output) QVariant(this_ptr->value(*key, *defaultValue));
}

