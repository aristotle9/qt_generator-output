#include "qt_core_c_QSignalMapper.h"

QSignalMapper* qt_core_c_QSignalMapper_G_dynamic_cast_QSignalMapper_ptr(QObject* ptr) {
  return dynamic_cast<QSignalMapper*>(ptr);
}

QObject* qt_core_c_QSignalMapper_G_static_cast_QObject_ptr(QSignalMapper* ptr) {
  return static_cast<QObject*>(ptr);
}

QSignalMapper* qt_core_c_QSignalMapper_G_static_cast_QSignalMapper_ptr(QObject* ptr) {
  return static_cast<QSignalMapper*>(ptr);
}

void qt_core_c_QSignalMapper_delete(QSignalMapper* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QSignalMapper_map_no_args(QSignalMapper* this_ptr) {
  this_ptr->map();
}

void qt_core_c_QSignalMapper_map_sender(QSignalMapper* this_ptr, QObject* sender) {
  this_ptr->map(sender);
}

QObject* qt_core_c_QSignalMapper_mapping_id(const QSignalMapper* this_ptr, int id) {
  return this_ptr->mapping(id);
}

QObject* qt_core_c_QSignalMapper_mapping_object(const QSignalMapper* this_ptr, QObject* object) {
  return this_ptr->mapping(object);
}

QObject* qt_core_c_QSignalMapper_mapping_text(const QSignalMapper* this_ptr, const QString* text) {
  return this_ptr->mapping(*text);
}

const QMetaObject* qt_core_c_QSignalMapper_metaObject(const QSignalMapper* this_ptr) {
  return this_ptr->metaObject();
}

QSignalMapper* qt_core_c_QSignalMapper_new_no_args() {
  return new QSignalMapper();
}

QSignalMapper* qt_core_c_QSignalMapper_new_parent(QObject* parent) {
  return new QSignalMapper(parent);
}

void qt_core_c_QSignalMapper_removeMappings(QSignalMapper* this_ptr, QObject* sender) {
  this_ptr->removeMappings(sender);
}

void qt_core_c_QSignalMapper_setMapping_sender_id(QSignalMapper* this_ptr, QObject* sender, int id) {
  this_ptr->setMapping(sender, id);
}

void qt_core_c_QSignalMapper_setMapping_sender_object(QSignalMapper* this_ptr, QObject* sender, QObject* object) {
  this_ptr->setMapping(sender, object);
}

void qt_core_c_QSignalMapper_setMapping_sender_text(QSignalMapper* this_ptr, QObject* sender, const QString* text) {
  this_ptr->setMapping(sender, *text);
}

void qt_core_c_QSignalMapper_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSignalMapper::trUtf8(s, c, n));
}

void qt_core_c_QSignalMapper_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSignalMapper::tr(s, c, n));
}

