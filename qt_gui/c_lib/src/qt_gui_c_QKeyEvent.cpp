#include "qt_gui_c_QKeyEvent.h"

QKeyEvent* qt_gui_c_QKeyEvent_G_dynamic_cast_QKeyEvent_ptr(QInputEvent* ptr) {
  return dynamic_cast<QKeyEvent*>(ptr);
}

QEvent* qt_gui_c_QKeyEvent_G_static_cast_QEvent_ptr(QKeyEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QInputEvent* qt_gui_c_QKeyEvent_G_static_cast_QInputEvent_ptr(QKeyEvent* ptr) {
  return static_cast<QInputEvent*>(ptr);
}

QKeyEvent* qt_gui_c_QKeyEvent_G_static_cast_QKeyEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QKeyEvent*>(ptr);
}

QKeyEvent* qt_gui_c_QKeyEvent_G_static_cast_QKeyEvent_ptr_QInputEvent(QInputEvent* ptr) {
  return static_cast<QKeyEvent*>(ptr);
}

int qt_gui_c_QKeyEvent_count(const QKeyEvent* this_ptr) {
  return this_ptr->count();
}

void qt_gui_c_QKeyEvent_delete(QKeyEvent* this_ptr) {
  delete this_ptr;
}

bool qt_gui_c_QKeyEvent_isAutoRepeat(const QKeyEvent* this_ptr) {
  return this_ptr->isAutoRepeat();
}

int qt_gui_c_QKeyEvent_key(const QKeyEvent* this_ptr) {
  return this_ptr->key();
}

bool qt_gui_c_QKeyEvent_matches(const QKeyEvent* this_ptr, const QKeySequence::StandardKey* key) {
  return this_ptr->matches(*key);
}

quint32 qt_gui_c_QKeyEvent_nativeModifiers(const QKeyEvent* this_ptr) {
  return this_ptr->nativeModifiers();
}

quint32 qt_gui_c_QKeyEvent_nativeScanCode(const QKeyEvent* this_ptr) {
  return this_ptr->nativeScanCode();
}

quint32 qt_gui_c_QKeyEvent_nativeVirtualKey(const QKeyEvent* this_ptr) {
  return this_ptr->nativeVirtualKey();
}

void qt_gui_c_QKeyEvent_text_to_output(const QKeyEvent* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

