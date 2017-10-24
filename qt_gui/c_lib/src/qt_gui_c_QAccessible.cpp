#include "qt_gui_c_QAccessible.h"

bool qt_gui_c_QAccessible_G_operator_eq(const QAccessible::State* first, const QAccessible::State* second) {
  return operator==(*first, *second);
}

void qt_gui_c_QAccessible_G_operator_shl_to_output_d_ev(const QDebug* d, const QAccessibleEvent* ev, QDebug* output) {
  new(output) QDebug(operator<<(*d, *ev));
}

void qt_gui_c_QAccessible_G_operator_shl_to_output_d_iface(const QDebug* d, const QAccessibleInterface* iface, QDebug* output) {
  new(output) QDebug(operator<<(*d, iface));
}

const char* qt_gui_c_QAccessible_G_qAccessibleEventString(const QAccessible::Event* event) {
  return qAccessibleEventString(*event);
}

void qt_gui_c_QAccessible_G_qAccessibleLocalizedActionDescription_to_output(const QString* actionName, QString* output) {
  new(output) QString(qAccessibleLocalizedActionDescription(*actionName));
}

const char* qt_gui_c_QAccessible_G_qAccessibleRoleString(const QAccessible::Role* role) {
  return qAccessibleRoleString(*role);
}

quint64 qt_gui_c_QAccessible_State_active(const QAccessible::State* this_ptr) {
  return this_ptr->active;
}

quint64 qt_gui_c_QAccessible_State_animated(const QAccessible::State* this_ptr) {
  return this_ptr->animated;
}

quint64 qt_gui_c_QAccessible_State_busy(const QAccessible::State* this_ptr) {
  return this_ptr->busy;
}

quint64 qt_gui_c_QAccessible_State_checkStateMixed(const QAccessible::State* this_ptr) {
  return this_ptr->checkStateMixed;
}

quint64 qt_gui_c_QAccessible_State_checkable(const QAccessible::State* this_ptr) {
  return this_ptr->checkable;
}

quint64 qt_gui_c_QAccessible_State_checked(const QAccessible::State* this_ptr) {
  return this_ptr->checked;
}

quint64 qt_gui_c_QAccessible_State_collapsed(const QAccessible::State* this_ptr) {
  return this_ptr->collapsed;
}

void qt_gui_c_QAccessible_State_constructor(QAccessible::State* output) {
  new(output) QAccessible::State();
}

quint64 qt_gui_c_QAccessible_State_defaultButton(const QAccessible::State* this_ptr) {
  return this_ptr->defaultButton;
}

void qt_gui_c_QAccessible_State_destructor(QAccessible::State* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

quint64 qt_gui_c_QAccessible_State_disabled(const QAccessible::State* this_ptr) {
  return this_ptr->disabled;
}

quint64 qt_gui_c_QAccessible_State_editable(const QAccessible::State* this_ptr) {
  return this_ptr->editable;
}

quint64 qt_gui_c_QAccessible_State_expandable(const QAccessible::State* this_ptr) {
  return this_ptr->expandable;
}

quint64 qt_gui_c_QAccessible_State_expanded(const QAccessible::State* this_ptr) {
  return this_ptr->expanded;
}

quint64 qt_gui_c_QAccessible_State_extSelectable(const QAccessible::State* this_ptr) {
  return this_ptr->extSelectable;
}

quint64 qt_gui_c_QAccessible_State_focusable(const QAccessible::State* this_ptr) {
  return this_ptr->focusable;
}

quint64 qt_gui_c_QAccessible_State_focused(const QAccessible::State* this_ptr) {
  return this_ptr->focused;
}

quint64 qt_gui_c_QAccessible_State_hasPopup(const QAccessible::State* this_ptr) {
  return this_ptr->hasPopup;
}

quint64 qt_gui_c_QAccessible_State_hotTracked(const QAccessible::State* this_ptr) {
  return this_ptr->hotTracked;
}

quint64 qt_gui_c_QAccessible_State_invalid(const QAccessible::State* this_ptr) {
  return this_ptr->invalid;
}

quint64 qt_gui_c_QAccessible_State_invisible(const QAccessible::State* this_ptr) {
  return this_ptr->invisible;
}

quint64 qt_gui_c_QAccessible_State_linked(const QAccessible::State* this_ptr) {
  return this_ptr->linked;
}

quint64 qt_gui_c_QAccessible_State_marqueed(const QAccessible::State* this_ptr) {
  return this_ptr->marqueed;
}

quint64 qt_gui_c_QAccessible_State_modal(const QAccessible::State* this_ptr) {
  return this_ptr->modal;
}

quint64 qt_gui_c_QAccessible_State_movable(const QAccessible::State* this_ptr) {
  return this_ptr->movable;
}

quint64 qt_gui_c_QAccessible_State_multiLine(const QAccessible::State* this_ptr) {
  return this_ptr->multiLine;
}

quint64 qt_gui_c_QAccessible_State_multiSelectable(const QAccessible::State* this_ptr) {
  return this_ptr->multiSelectable;
}

quint64 qt_gui_c_QAccessible_State_offscreen(const QAccessible::State* this_ptr) {
  return this_ptr->offscreen;
}

quint64 qt_gui_c_QAccessible_State_passwordEdit(const QAccessible::State* this_ptr) {
  return this_ptr->passwordEdit;
}

quint64 qt_gui_c_QAccessible_State_pressed(const QAccessible::State* this_ptr) {
  return this_ptr->pressed;
}

quint64 qt_gui_c_QAccessible_State_readOnly(const QAccessible::State* this_ptr) {
  return this_ptr->readOnly;
}

quint64 qt_gui_c_QAccessible_State_searchEdit(const QAccessible::State* this_ptr) {
  return this_ptr->searchEdit;
}

quint64 qt_gui_c_QAccessible_State_selectable(const QAccessible::State* this_ptr) {
  return this_ptr->selectable;
}

quint64 qt_gui_c_QAccessible_State_selectableText(const QAccessible::State* this_ptr) {
  return this_ptr->selectableText;
}

quint64 qt_gui_c_QAccessible_State_selected(const QAccessible::State* this_ptr) {
  return this_ptr->selected;
}

quint64 qt_gui_c_QAccessible_State_selfVoicing(const QAccessible::State* this_ptr) {
  return this_ptr->selfVoicing;
}

void qt_gui_c_QAccessible_State_set_active(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->active = value;
}

void qt_gui_c_QAccessible_State_set_animated(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->animated = value;
}

void qt_gui_c_QAccessible_State_set_busy(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->busy = value;
}

void qt_gui_c_QAccessible_State_set_checkStateMixed(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->checkStateMixed = value;
}

void qt_gui_c_QAccessible_State_set_checkable(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->checkable = value;
}

void qt_gui_c_QAccessible_State_set_checked(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->checked = value;
}

void qt_gui_c_QAccessible_State_set_collapsed(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->collapsed = value;
}

void qt_gui_c_QAccessible_State_set_defaultButton(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->defaultButton = value;
}

void qt_gui_c_QAccessible_State_set_disabled(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->disabled = value;
}

void qt_gui_c_QAccessible_State_set_editable(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->editable = value;
}

void qt_gui_c_QAccessible_State_set_expandable(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->expandable = value;
}

void qt_gui_c_QAccessible_State_set_expanded(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->expanded = value;
}

void qt_gui_c_QAccessible_State_set_extSelectable(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->extSelectable = value;
}

void qt_gui_c_QAccessible_State_set_focusable(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->focusable = value;
}

void qt_gui_c_QAccessible_State_set_focused(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->focused = value;
}

void qt_gui_c_QAccessible_State_set_hasPopup(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->hasPopup = value;
}

void qt_gui_c_QAccessible_State_set_hotTracked(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->hotTracked = value;
}

void qt_gui_c_QAccessible_State_set_invalid(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->invalid = value;
}

void qt_gui_c_QAccessible_State_set_invisible(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->invisible = value;
}

void qt_gui_c_QAccessible_State_set_linked(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->linked = value;
}

void qt_gui_c_QAccessible_State_set_marqueed(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->marqueed = value;
}

void qt_gui_c_QAccessible_State_set_modal(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->modal = value;
}

void qt_gui_c_QAccessible_State_set_movable(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->movable = value;
}

void qt_gui_c_QAccessible_State_set_multiLine(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->multiLine = value;
}

void qt_gui_c_QAccessible_State_set_multiSelectable(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->multiSelectable = value;
}

void qt_gui_c_QAccessible_State_set_offscreen(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->offscreen = value;
}

void qt_gui_c_QAccessible_State_set_passwordEdit(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->passwordEdit = value;
}

void qt_gui_c_QAccessible_State_set_pressed(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->pressed = value;
}

void qt_gui_c_QAccessible_State_set_readOnly(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->readOnly = value;
}

void qt_gui_c_QAccessible_State_set_searchEdit(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->searchEdit = value;
}

void qt_gui_c_QAccessible_State_set_selectable(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->selectable = value;
}

void qt_gui_c_QAccessible_State_set_selectableText(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->selectableText = value;
}

void qt_gui_c_QAccessible_State_set_selected(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->selected = value;
}

void qt_gui_c_QAccessible_State_set_selfVoicing(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->selfVoicing = value;
}

void qt_gui_c_QAccessible_State_set_sizeable(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->sizeable = value;
}

void qt_gui_c_QAccessible_State_set_supportsAutoCompletion(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->supportsAutoCompletion = value;
}

void qt_gui_c_QAccessible_State_set_traversed(QAccessible::State* this_ptr, quint64 value) {
  this_ptr->traversed = value;
}

quint64 qt_gui_c_QAccessible_State_sizeable(const QAccessible::State* this_ptr) {
  return this_ptr->sizeable;
}

quint64 qt_gui_c_QAccessible_State_supportsAutoCompletion(const QAccessible::State* this_ptr) {
  return this_ptr->supportsAutoCompletion;
}

quint64 qt_gui_c_QAccessible_State_traversed(const QAccessible::State* this_ptr) {
  return this_ptr->traversed;
}

QAccessibleInterface* qt_gui_c_QAccessible_accessibleInterface(unsigned int uniqueId) {
  return QAccessible::accessibleInterface(uniqueId);
}

void qt_gui_c_QAccessible_cleanup() {
  QAccessible::cleanup();
}

void qt_gui_c_QAccessible_delete(QAccessible* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QAccessible_deleteAccessibleInterface(unsigned int uniqueId) {
  QAccessible::deleteAccessibleInterface(uniqueId);
}

void (*qt_gui_c_QAccessible_installRootObjectHandler(void (*arg1)(QObject*)))(QObject*) {
  return QAccessible::installRootObjectHandler(arg1);
}

void (*qt_gui_c_QAccessible_installUpdateHandler(void (*arg1)(QAccessibleEvent*)))(QAccessibleEvent*) {
  return QAccessible::installUpdateHandler(arg1);
}

bool qt_gui_c_QAccessible_isActive() {
  return QAccessible::isActive();
}

void qt_gui_c_QAccessible_qAccessibleTextBoundaryHelper_to_output(const QTextCursor* cursor, QAccessible::TextBoundaryType boundaryType, QPair< int, int >* output) {
  new(output) QPair< int, int >(QAccessible::qAccessibleTextBoundaryHelper(*cursor, boundaryType));
}

QAccessibleInterface* qt_gui_c_QAccessible_queryAccessibleInterface(QObject* arg1) {
  return QAccessible::queryAccessibleInterface(arg1);
}

unsigned int qt_gui_c_QAccessible_registerAccessibleInterface(QAccessibleInterface* iface) {
  return QAccessible::registerAccessibleInterface(iface);
}

void qt_gui_c_QAccessible_setActive(bool active) {
  QAccessible::setActive(active);
}

void qt_gui_c_QAccessible_setRootObject(QObject* object) {
  QAccessible::setRootObject(object);
}

unsigned int qt_gui_c_QAccessible_uniqueId(QAccessibleInterface* iface) {
  return QAccessible::uniqueId(iface);
}

void qt_gui_c_QAccessible_updateAccessibility(QAccessibleEvent* event) {
  QAccessible::updateAccessibility(event);
}

