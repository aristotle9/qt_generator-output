#include "qt_widgets_c_QAccessibleWidget.h"

QAccessibleActionInterface* qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleActionInterface_ptr(QAccessibleWidget* ptr) {
  return static_cast<QAccessibleActionInterface*>(ptr);
}

QAccessibleInterface* qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleInterface_ptr(QAccessibleWidget* ptr) {
  return static_cast<QAccessibleInterface*>(ptr);
}

QAccessibleObject* qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleObject_ptr(QAccessibleWidget* ptr) {
  return static_cast<QAccessibleObject*>(ptr);
}

QAccessibleWidget* qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleWidget_ptr_QAccessibleActionInterface(QAccessibleActionInterface* ptr) {
  return static_cast<QAccessibleWidget*>(ptr);
}

QAccessibleWidget* qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleWidget_ptr_QAccessibleInterface(QAccessibleInterface* ptr) {
  return static_cast<QAccessibleWidget*>(ptr);
}

QAccessibleWidget* qt_widgets_c_QAccessibleWidget_G_static_cast_QAccessibleWidget_ptr_QAccessibleObject(QAccessibleObject* ptr) {
  return static_cast<QAccessibleWidget*>(ptr);
}

void qt_widgets_c_QAccessibleWidget_actionNames_to_output(const QAccessibleWidget* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->actionNames());
}

void qt_widgets_c_QAccessibleWidget_backgroundColor_to_output(const QAccessibleWidget* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->backgroundColor());
}

QAccessibleInterface* qt_widgets_c_QAccessibleWidget_child(const QAccessibleWidget* this_ptr, int index) {
  return this_ptr->child(index);
}

int qt_widgets_c_QAccessibleWidget_childCount(const QAccessibleWidget* this_ptr) {
  return this_ptr->childCount();
}

void qt_widgets_c_QAccessibleWidget_doAction(QAccessibleWidget* this_ptr, const QString* actionName) {
  this_ptr->doAction(*actionName);
}

QAccessibleInterface* qt_widgets_c_QAccessibleWidget_focusChild(const QAccessibleWidget* this_ptr) {
  return this_ptr->focusChild();
}

void qt_widgets_c_QAccessibleWidget_foregroundColor_to_output(const QAccessibleWidget* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->foregroundColor());
}

int qt_widgets_c_QAccessibleWidget_indexOfChild(const QAccessibleWidget* this_ptr, const QAccessibleInterface* child) {
  return this_ptr->indexOfChild(child);
}

void* qt_widgets_c_QAccessibleWidget_interface_cast(QAccessibleWidget* this_ptr, const QAccessible::InterfaceType* t) {
  return this_ptr->interface_cast(*t);
}

bool qt_widgets_c_QAccessibleWidget_isValid(const QAccessibleWidget* this_ptr) {
  return this_ptr->isValid();
}

void qt_widgets_c_QAccessibleWidget_keyBindingsForAction_to_output(const QAccessibleWidget* this_ptr, const QString* actionName, QStringList* output) {
  new(output) QStringList(this_ptr->keyBindingsForAction(*actionName));
}

QAccessibleWidget* qt_widgets_c_QAccessibleWidget_new_o(QWidget* o) {
  return new QAccessibleWidget(o);
}

QAccessibleWidget* qt_widgets_c_QAccessibleWidget_new_o_r(QWidget* o, const QAccessible::Role* r) {
  return new QAccessibleWidget(o, *r);
}

QAccessibleWidget* qt_widgets_c_QAccessibleWidget_new_o_r_name(QWidget* o, const QAccessible::Role* r, const QString* name) {
  return new QAccessibleWidget(o, *r, *name);
}

QAccessibleInterface* qt_widgets_c_QAccessibleWidget_parent(const QAccessibleWidget* this_ptr) {
  return this_ptr->parent();
}

void qt_widgets_c_QAccessibleWidget_rect_to_output(const QAccessibleWidget* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->rect());
}

void qt_widgets_c_QAccessibleWidget_state_to_output(const QAccessibleWidget* this_ptr, QAccessible::State* output) {
  new(output) QAccessible::State(this_ptr->state());
}

void qt_widgets_c_QAccessibleWidget_text_to_output(const QAccessibleWidget* this_ptr, const QAccessible::Text* t, QString* output) {
  new(output) QString(this_ptr->text(*t));
}

QWindow* qt_widgets_c_QAccessibleWidget_window(const QAccessibleWidget* this_ptr) {
  return this_ptr->window();
}

