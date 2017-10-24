#include "qt_gui_c_QInputMethod.h"

QInputMethod* qt_gui_c_QInputMethod_G_static_cast_QInputMethod_ptr(QObject* ptr) {
  return static_cast<QInputMethod*>(ptr);
}

QObject* qt_gui_c_QInputMethod_G_static_cast_QObject_ptr(QInputMethod* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_gui_c_QInputMethod_anchorRectangle_to_output(const QInputMethod* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->anchorRectangle());
}

void qt_gui_c_QInputMethod_commit(QInputMethod* this_ptr) {
  this_ptr->commit();
}

void qt_gui_c_QInputMethod_cursorRectangle_to_output(const QInputMethod* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->cursorRectangle());
}

void qt_gui_c_QInputMethod_hide(QInputMethod* this_ptr) {
  this_ptr->hide();
}

void qt_gui_c_QInputMethod_inputItemClipRectangle_to_output(const QInputMethod* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->inputItemClipRectangle());
}

void qt_gui_c_QInputMethod_inputItemRectangle_to_output(const QInputMethod* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->inputItemRectangle());
}

void qt_gui_c_QInputMethod_inputItemTransform_to_output(const QInputMethod* this_ptr, QTransform* output) {
  new(output) QTransform(this_ptr->inputItemTransform());
}

void qt_gui_c_QInputMethod_invokeAction(QInputMethod* this_ptr, QInputMethod::Action a, int cursorPosition) {
  this_ptr->invokeAction(a, cursorPosition);
}

bool qt_gui_c_QInputMethod_isAnimating(const QInputMethod* this_ptr) {
  return this_ptr->isAnimating();
}

bool qt_gui_c_QInputMethod_isVisible(const QInputMethod* this_ptr) {
  return this_ptr->isVisible();
}

void qt_gui_c_QInputMethod_keyboardRectangle_to_output(const QInputMethod* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->keyboardRectangle());
}

void qt_gui_c_QInputMethod_locale_to_output(const QInputMethod* this_ptr, QLocale* output) {
  new(output) QLocale(this_ptr->locale());
}

const QMetaObject* qt_gui_c_QInputMethod_metaObject(const QInputMethod* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QInputMethod_qt_metacall(QInputMethod* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QInputMethod_qt_metacast(QInputMethod* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QInputMethod_queryFocusObject_to_output(const Qt::InputMethodQuery* query, const QVariant* argument, QVariant* output) {
  new(output) QVariant(QInputMethod::queryFocusObject(*query, *argument));
}

void qt_gui_c_QInputMethod_reset(QInputMethod* this_ptr) {
  this_ptr->reset();
}

void qt_gui_c_QInputMethod_setInputItemRectangle(QInputMethod* this_ptr, const QRectF* rect) {
  this_ptr->setInputItemRectangle(*rect);
}

void qt_gui_c_QInputMethod_setInputItemTransform(QInputMethod* this_ptr, const QTransform* transform) {
  this_ptr->setInputItemTransform(*transform);
}

void qt_gui_c_QInputMethod_setVisible(QInputMethod* this_ptr, bool visible) {
  this_ptr->setVisible(visible);
}

void qt_gui_c_QInputMethod_show(QInputMethod* this_ptr) {
  this_ptr->show();
}

void qt_gui_c_QInputMethod_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QInputMethod::trUtf8(s, c, n));
}

void qt_gui_c_QInputMethod_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QInputMethod::tr(s, c, n));
}

