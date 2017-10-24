#include "qt_gui_c_QAccessibleActionInterface.h"

void qt_gui_c_QAccessibleActionInterface_actionNames_to_output(const QAccessibleActionInterface* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->actionNames());
}

const QString* qt_gui_c_QAccessibleActionInterface_decreaseAction() {
  return &QAccessibleActionInterface::decreaseAction();
}

void qt_gui_c_QAccessibleActionInterface_delete(QAccessibleActionInterface* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QAccessibleActionInterface_doAction(QAccessibleActionInterface* this_ptr, const QString* actionName) {
  this_ptr->doAction(*actionName);
}

const QString* qt_gui_c_QAccessibleActionInterface_increaseAction() {
  return &QAccessibleActionInterface::increaseAction();
}

void qt_gui_c_QAccessibleActionInterface_keyBindingsForAction_to_output(const QAccessibleActionInterface* this_ptr, const QString* actionName, QStringList* output) {
  new(output) QStringList(this_ptr->keyBindingsForAction(*actionName));
}

void qt_gui_c_QAccessibleActionInterface_localizedActionDescription_to_output(const QAccessibleActionInterface* this_ptr, const QString* name, QString* output) {
  new(output) QString(this_ptr->localizedActionDescription(*name));
}

void qt_gui_c_QAccessibleActionInterface_localizedActionName_to_output(const QAccessibleActionInterface* this_ptr, const QString* name, QString* output) {
  new(output) QString(this_ptr->localizedActionName(*name));
}

void qt_gui_c_QAccessibleActionInterface_nextPageAction_to_output(QString* output) {
  new(output) QString(QAccessibleActionInterface::nextPageAction());
}

const QString* qt_gui_c_QAccessibleActionInterface_pressAction() {
  return &QAccessibleActionInterface::pressAction();
}

void qt_gui_c_QAccessibleActionInterface_previousPageAction_to_output(QString* output) {
  new(output) QString(QAccessibleActionInterface::previousPageAction());
}

void qt_gui_c_QAccessibleActionInterface_scrollDownAction_to_output(QString* output) {
  new(output) QString(QAccessibleActionInterface::scrollDownAction());
}

void qt_gui_c_QAccessibleActionInterface_scrollLeftAction_to_output(QString* output) {
  new(output) QString(QAccessibleActionInterface::scrollLeftAction());
}

void qt_gui_c_QAccessibleActionInterface_scrollRightAction_to_output(QString* output) {
  new(output) QString(QAccessibleActionInterface::scrollRightAction());
}

void qt_gui_c_QAccessibleActionInterface_scrollUpAction_to_output(QString* output) {
  new(output) QString(QAccessibleActionInterface::scrollUpAction());
}

const QString* qt_gui_c_QAccessibleActionInterface_setFocusAction() {
  return &QAccessibleActionInterface::setFocusAction();
}

const QString* qt_gui_c_QAccessibleActionInterface_showMenuAction() {
  return &QAccessibleActionInterface::showMenuAction();
}

const QString* qt_gui_c_QAccessibleActionInterface_toggleAction() {
  return &QAccessibleActionInterface::toggleAction();
}

void qt_gui_c_QAccessibleActionInterface_trUtf8_to_output(const char* sourceText, const char* disambiguation, int n, QString* output) {
  new(output) QString(QAccessibleActionInterface::trUtf8(sourceText, disambiguation, n));
}

void qt_gui_c_QAccessibleActionInterface_tr_to_output(const char* sourceText, const char* disambiguation, int n, QString* output) {
  new(output) QString(QAccessibleActionInterface::tr(sourceText, disambiguation, n));
}

