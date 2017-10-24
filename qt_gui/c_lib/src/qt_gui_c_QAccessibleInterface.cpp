#include "qt_gui_c_QAccessibleInterface.h"

QAccessibleActionInterface* qt_gui_c_QAccessibleInterface_actionInterface(QAccessibleInterface* this_ptr) {
  return this_ptr->actionInterface();
}

void qt_gui_c_QAccessibleInterface_backgroundColor_to_output(const QAccessibleInterface* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->backgroundColor());
}

QAccessibleInterface* qt_gui_c_QAccessibleInterface_child(const QAccessibleInterface* this_ptr, int index) {
  return this_ptr->child(index);
}

QAccessibleInterface* qt_gui_c_QAccessibleInterface_childAt(const QAccessibleInterface* this_ptr, int x, int y) {
  return this_ptr->childAt(x, y);
}

int qt_gui_c_QAccessibleInterface_childCount(const QAccessibleInterface* this_ptr) {
  return this_ptr->childCount();
}

QAccessibleEditableTextInterface* qt_gui_c_QAccessibleInterface_editableTextInterface(QAccessibleInterface* this_ptr) {
  return this_ptr->editableTextInterface();
}

QAccessibleInterface* qt_gui_c_QAccessibleInterface_focusChild(const QAccessibleInterface* this_ptr) {
  return this_ptr->focusChild();
}

void qt_gui_c_QAccessibleInterface_foregroundColor_to_output(const QAccessibleInterface* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->foregroundColor());
}

int qt_gui_c_QAccessibleInterface_indexOfChild(const QAccessibleInterface* this_ptr, const QAccessibleInterface* arg1) {
  return this_ptr->indexOfChild(arg1);
}

void* qt_gui_c_QAccessibleInterface_interface_cast(QAccessibleInterface* this_ptr, const QAccessible::InterfaceType* arg1) {
  return this_ptr->interface_cast(*arg1);
}

bool qt_gui_c_QAccessibleInterface_isValid(const QAccessibleInterface* this_ptr) {
  return this_ptr->isValid();
}

QObject* qt_gui_c_QAccessibleInterface_object(const QAccessibleInterface* this_ptr) {
  return this_ptr->object();
}

QAccessibleInterface* qt_gui_c_QAccessibleInterface_parent(const QAccessibleInterface* this_ptr) {
  return this_ptr->parent();
}

void qt_gui_c_QAccessibleInterface_rect_to_output(const QAccessibleInterface* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->rect());
}

void qt_gui_c_QAccessibleInterface_setText(QAccessibleInterface* this_ptr, const QAccessible::Text* t, const QString* text) {
  this_ptr->setText(*t, *text);
}

void qt_gui_c_QAccessibleInterface_state_to_output(const QAccessibleInterface* this_ptr, QAccessible::State* output) {
  new(output) QAccessible::State(this_ptr->state());
}

QAccessibleTableCellInterface* qt_gui_c_QAccessibleInterface_tableCellInterface(QAccessibleInterface* this_ptr) {
  return this_ptr->tableCellInterface();
}

QAccessibleTableInterface* qt_gui_c_QAccessibleInterface_tableInterface(QAccessibleInterface* this_ptr) {
  return this_ptr->tableInterface();
}

QAccessibleTextInterface* qt_gui_c_QAccessibleInterface_textInterface(QAccessibleInterface* this_ptr) {
  return this_ptr->textInterface();
}

void qt_gui_c_QAccessibleInterface_text_to_output(const QAccessibleInterface* this_ptr, const QAccessible::Text* t, QString* output) {
  new(output) QString(this_ptr->text(*t));
}

QAccessibleValueInterface* qt_gui_c_QAccessibleInterface_valueInterface(QAccessibleInterface* this_ptr) {
  return this_ptr->valueInterface();
}

void qt_gui_c_QAccessibleInterface_virtual_hook(QAccessibleInterface* this_ptr, int id, void* data) {
  this_ptr->virtual_hook(id, data);
}

QWindow* qt_gui_c_QAccessibleInterface_window(const QAccessibleInterface* this_ptr) {
  return this_ptr->window();
}

