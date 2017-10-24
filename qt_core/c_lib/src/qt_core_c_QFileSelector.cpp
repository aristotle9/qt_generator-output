#include "qt_core_c_QFileSelector.h"

QFileSelector* qt_core_c_QFileSelector_G_dynamic_cast_QFileSelector_ptr(QObject* ptr) {
  return dynamic_cast<QFileSelector*>(ptr);
}

QFileSelector* qt_core_c_QFileSelector_G_static_cast_QFileSelector_ptr(QObject* ptr) {
  return static_cast<QFileSelector*>(ptr);
}

QObject* qt_core_c_QFileSelector_G_static_cast_QObject_ptr(QFileSelector* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QFileSelector_allSelectors_to_output(const QFileSelector* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->allSelectors());
}

void qt_core_c_QFileSelector_delete(QFileSelector* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QFileSelector_extraSelectors_to_output(const QFileSelector* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->extraSelectors());
}

const QMetaObject* qt_core_c_QFileSelector_metaObject(const QFileSelector* this_ptr) {
  return this_ptr->metaObject();
}

QFileSelector* qt_core_c_QFileSelector_new_no_args() {
  return new QFileSelector();
}

QFileSelector* qt_core_c_QFileSelector_new_parent(QObject* parent) {
  return new QFileSelector(parent);
}

void qt_core_c_QFileSelector_select_to_output_QString(const QFileSelector* this_ptr, const QString* filePath, QString* output) {
  new(output) QString(this_ptr->select(*filePath));
}

void qt_core_c_QFileSelector_select_to_output_QUrl(const QFileSelector* this_ptr, const QUrl* filePath, QUrl* output) {
  new(output) QUrl(this_ptr->select(*filePath));
}

void qt_core_c_QFileSelector_setExtraSelectors(QFileSelector* this_ptr, const QStringList* list) {
  this_ptr->setExtraSelectors(*list);
}

void qt_core_c_QFileSelector_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFileSelector::trUtf8(s, c, n));
}

void qt_core_c_QFileSelector_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFileSelector::tr(s, c, n));
}

