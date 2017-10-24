#include "qt_widgets_c_QShortcut.h"

QObject* qt_widgets_c_QShortcut_G_static_cast_QObject_ptr(QShortcut* ptr) {
  return static_cast<QObject*>(ptr);
}

QShortcut* qt_widgets_c_QShortcut_G_static_cast_QShortcut_ptr(QObject* ptr) {
  return static_cast<QShortcut*>(ptr);
}

bool qt_widgets_c_QShortcut_autoRepeat(const QShortcut* this_ptr) {
  return this_ptr->autoRepeat();
}

void qt_widgets_c_QShortcut_delete(QShortcut* this_ptr) {
  delete this_ptr;
}

int qt_widgets_c_QShortcut_id(const QShortcut* this_ptr) {
  return this_ptr->id();
}

bool qt_widgets_c_QShortcut_isEnabled(const QShortcut* this_ptr) {
  return this_ptr->isEnabled();
}

void qt_widgets_c_QShortcut_key_to_output(const QShortcut* this_ptr, QKeySequence* output) {
  new(output) QKeySequence(this_ptr->key());
}

const QMetaObject* qt_widgets_c_QShortcut_metaObject(const QShortcut* this_ptr) {
  return this_ptr->metaObject();
}

QShortcut* qt_widgets_c_QShortcut_new_key_parent(const QKeySequence* key, QWidget* parent) {
  return new QShortcut(*key, parent);
}

QShortcut* qt_widgets_c_QShortcut_new_key_parent_member(const QKeySequence* key, QWidget* parent, const char* member) {
  return new QShortcut(*key, parent, member);
}

QShortcut* qt_widgets_c_QShortcut_new_key_parent_member_ambiguousMember(const QKeySequence* key, QWidget* parent, const char* member, const char* ambiguousMember) {
  return new QShortcut(*key, parent, member, ambiguousMember);
}

QShortcut* qt_widgets_c_QShortcut_new_key_parent_member_ambiguousMember_context(const QKeySequence* key, QWidget* parent, const char* member, const char* ambiguousMember, const Qt::ShortcutContext* context) {
  return new QShortcut(*key, parent, member, ambiguousMember, *context);
}

QShortcut* qt_widgets_c_QShortcut_new_parent(QWidget* parent) {
  return new QShortcut(parent);
}

QWidget* qt_widgets_c_QShortcut_parentWidget(const QShortcut* this_ptr) {
  return this_ptr->parentWidget();
}

int qt_widgets_c_QShortcut_qt_metacall(QShortcut* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QShortcut_qt_metacast(QShortcut* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QShortcut_setAutoRepeat(QShortcut* this_ptr, bool on) {
  this_ptr->setAutoRepeat(on);
}

void qt_widgets_c_QShortcut_setContext(QShortcut* this_ptr, const Qt::ShortcutContext* context) {
  this_ptr->setContext(*context);
}

void qt_widgets_c_QShortcut_setEnabled(QShortcut* this_ptr, bool enable) {
  this_ptr->setEnabled(enable);
}

void qt_widgets_c_QShortcut_setKey(QShortcut* this_ptr, const QKeySequence* key) {
  this_ptr->setKey(*key);
}

void qt_widgets_c_QShortcut_setWhatsThis(QShortcut* this_ptr, const QString* text) {
  this_ptr->setWhatsThis(*text);
}

void qt_widgets_c_QShortcut_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QShortcut::trUtf8(s, c, n));
}

void qt_widgets_c_QShortcut_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QShortcut::tr(s, c, n));
}

void qt_widgets_c_QShortcut_whatsThis_to_output(const QShortcut* this_ptr, QString* output) {
  new(output) QString(this_ptr->whatsThis());
}

