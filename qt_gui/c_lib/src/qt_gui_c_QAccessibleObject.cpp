#include "qt_gui_c_QAccessibleObject.h"

QAccessibleObject* qt_gui_c_QAccessibleObject_G_dynamic_cast_QAccessibleObject_ptr(QAccessibleInterface* ptr) {
  return dynamic_cast<QAccessibleObject*>(ptr);
}

QAccessibleInterface* qt_gui_c_QAccessibleObject_G_static_cast_QAccessibleInterface_ptr(QAccessibleObject* ptr) {
  return static_cast<QAccessibleInterface*>(ptr);
}

QAccessibleObject* qt_gui_c_QAccessibleObject_G_static_cast_QAccessibleObject_ptr(QAccessibleInterface* ptr) {
  return static_cast<QAccessibleObject*>(ptr);
}

QAccessibleInterface* qt_gui_c_QAccessibleObject_childAt(const QAccessibleObject* this_ptr, int x, int y) {
  return this_ptr->childAt(x, y);
}

bool qt_gui_c_QAccessibleObject_isValid(const QAccessibleObject* this_ptr) {
  return this_ptr->isValid();
}

QObject* qt_gui_c_QAccessibleObject_object(const QAccessibleObject* this_ptr) {
  return this_ptr->object();
}

void qt_gui_c_QAccessibleObject_rect_to_output(const QAccessibleObject* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->rect());
}

void qt_gui_c_QAccessibleObject_setText(QAccessibleObject* this_ptr, const QAccessible::Text* t, const QString* text) {
  this_ptr->setText(*t, *text);
}

