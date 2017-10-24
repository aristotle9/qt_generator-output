#include "qt_gui_c_QAccessibleTableModelChangeEvent.h"

QAccessibleTableModelChangeEvent* qt_gui_c_QAccessibleTableModelChangeEvent_G_dynamic_cast_QAccessibleTableModelChangeEvent_ptr(QAccessibleEvent* ptr) {
  return dynamic_cast<QAccessibleTableModelChangeEvent*>(ptr);
}

QAccessibleEvent* qt_gui_c_QAccessibleTableModelChangeEvent_G_static_cast_QAccessibleEvent_ptr(QAccessibleTableModelChangeEvent* ptr) {
  return static_cast<QAccessibleEvent*>(ptr);
}

QAccessibleTableModelChangeEvent* qt_gui_c_QAccessibleTableModelChangeEvent_G_static_cast_QAccessibleTableModelChangeEvent_ptr(QAccessibleEvent* ptr) {
  return static_cast<QAccessibleTableModelChangeEvent*>(ptr);
}

void qt_gui_c_QAccessibleTableModelChangeEvent_delete(QAccessibleTableModelChangeEvent* this_ptr) {
  delete this_ptr;
}

int qt_gui_c_QAccessibleTableModelChangeEvent_firstColumn(const QAccessibleTableModelChangeEvent* this_ptr) {
  return this_ptr->firstColumn();
}

int qt_gui_c_QAccessibleTableModelChangeEvent_firstRow(const QAccessibleTableModelChangeEvent* this_ptr) {
  return this_ptr->firstRow();
}

int qt_gui_c_QAccessibleTableModelChangeEvent_lastColumn(const QAccessibleTableModelChangeEvent* this_ptr) {
  return this_ptr->lastColumn();
}

int qt_gui_c_QAccessibleTableModelChangeEvent_lastRow(const QAccessibleTableModelChangeEvent* this_ptr) {
  return this_ptr->lastRow();
}

QAccessibleTableModelChangeEvent::ModelChangeType qt_gui_c_QAccessibleTableModelChangeEvent_modelChangeType(const QAccessibleTableModelChangeEvent* this_ptr) {
  return this_ptr->modelChangeType();
}

QAccessibleTableModelChangeEvent* qt_gui_c_QAccessibleTableModelChangeEvent_new_iface_changeType(QAccessibleInterface* iface, QAccessibleTableModelChangeEvent::ModelChangeType changeType) {
  return new QAccessibleTableModelChangeEvent(iface, changeType);
}

QAccessibleTableModelChangeEvent* qt_gui_c_QAccessibleTableModelChangeEvent_new_obj_changeType(QObject* obj, QAccessibleTableModelChangeEvent::ModelChangeType changeType) {
  return new QAccessibleTableModelChangeEvent(obj, changeType);
}

void qt_gui_c_QAccessibleTableModelChangeEvent_setFirstColumn(QAccessibleTableModelChangeEvent* this_ptr, int col) {
  this_ptr->setFirstColumn(col);
}

void qt_gui_c_QAccessibleTableModelChangeEvent_setFirstRow(QAccessibleTableModelChangeEvent* this_ptr, int row) {
  this_ptr->setFirstRow(row);
}

void qt_gui_c_QAccessibleTableModelChangeEvent_setLastColumn(QAccessibleTableModelChangeEvent* this_ptr, int col) {
  this_ptr->setLastColumn(col);
}

void qt_gui_c_QAccessibleTableModelChangeEvent_setLastRow(QAccessibleTableModelChangeEvent* this_ptr, int row) {
  this_ptr->setLastRow(row);
}

void qt_gui_c_QAccessibleTableModelChangeEvent_setModelChangeType(QAccessibleTableModelChangeEvent* this_ptr, QAccessibleTableModelChangeEvent::ModelChangeType changeType) {
  this_ptr->setModelChangeType(changeType);
}

