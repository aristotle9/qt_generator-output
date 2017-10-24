#include "qt_core_c_QObjectCleanupHandler.h"

QObjectCleanupHandler* qt_core_c_QObjectCleanupHandler_G_dynamic_cast_QObjectCleanupHandler_ptr(QObject* ptr) {
  return dynamic_cast<QObjectCleanupHandler*>(ptr);
}

QObjectCleanupHandler* qt_core_c_QObjectCleanupHandler_G_static_cast_QObjectCleanupHandler_ptr(QObject* ptr) {
  return static_cast<QObjectCleanupHandler*>(ptr);
}

QObject* qt_core_c_QObjectCleanupHandler_G_static_cast_QObject_ptr(QObjectCleanupHandler* ptr) {
  return static_cast<QObject*>(ptr);
}

QObject* qt_core_c_QObjectCleanupHandler_add(QObjectCleanupHandler* this_ptr, QObject* object) {
  return this_ptr->add(object);
}

void qt_core_c_QObjectCleanupHandler_clear(QObjectCleanupHandler* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QObjectCleanupHandler_delete(QObjectCleanupHandler* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QObjectCleanupHandler_isEmpty(const QObjectCleanupHandler* this_ptr) {
  return this_ptr->isEmpty();
}

const QMetaObject* qt_core_c_QObjectCleanupHandler_metaObject(const QObjectCleanupHandler* this_ptr) {
  return this_ptr->metaObject();
}

QObjectCleanupHandler* qt_core_c_QObjectCleanupHandler_new() {
  return new QObjectCleanupHandler();
}

void qt_core_c_QObjectCleanupHandler_remove(QObjectCleanupHandler* this_ptr, QObject* object) {
  this_ptr->remove(object);
}

void qt_core_c_QObjectCleanupHandler_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QObjectCleanupHandler::trUtf8(s, c, n));
}

void qt_core_c_QObjectCleanupHandler_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QObjectCleanupHandler::tr(s, c, n));
}

