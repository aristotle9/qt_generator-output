#include "qt_gui_c_QShortcutEvent.h"

QEvent* qt_gui_c_QShortcutEvent_G_static_cast_QEvent_ptr(QShortcutEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QShortcutEvent* qt_gui_c_QShortcutEvent_G_static_cast_QShortcutEvent_ptr(QEvent* ptr) {
  return static_cast<QShortcutEvent*>(ptr);
}

void qt_gui_c_QShortcutEvent_delete(QShortcutEvent* this_ptr) {
  delete this_ptr;
}

bool qt_gui_c_QShortcutEvent_isAmbiguous(const QShortcutEvent* this_ptr) {
  return this_ptr->isAmbiguous();
}

const QKeySequence* qt_gui_c_QShortcutEvent_key(const QShortcutEvent* this_ptr) {
  return &this_ptr->key();
}

QShortcutEvent* qt_gui_c_QShortcutEvent_new_key_id(const QKeySequence* key, int id) {
  return new QShortcutEvent(*key, id);
}

QShortcutEvent* qt_gui_c_QShortcutEvent_new_key_id_ambiguous(const QKeySequence* key, int id, bool ambiguous) {
  return new QShortcutEvent(*key, id, ambiguous);
}

int qt_gui_c_QShortcutEvent_shortcutId(const QShortcutEvent* this_ptr) {
  return this_ptr->shortcutId();
}

