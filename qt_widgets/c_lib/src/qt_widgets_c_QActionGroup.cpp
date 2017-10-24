#include "qt_widgets_c_QActionGroup.h"

QActionGroup* qt_widgets_c_QActionGroup_G_static_cast_QActionGroup_ptr(QObject* ptr) {
  return static_cast<QActionGroup*>(ptr);
}

QObject* qt_widgets_c_QActionGroup_G_static_cast_QObject_ptr(QActionGroup* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QActionGroup_actions_to_output(const QActionGroup* this_ptr, QList< QAction* >* output) {
  new(output) QList< QAction* >(this_ptr->actions());
}

QAction* qt_widgets_c_QActionGroup_addAction_a(QActionGroup* this_ptr, QAction* a) {
  return this_ptr->addAction(a);
}

QAction* qt_widgets_c_QActionGroup_addAction_icon_text(QActionGroup* this_ptr, const QIcon* icon, const QString* text) {
  return this_ptr->addAction(*icon, *text);
}

QAction* qt_widgets_c_QActionGroup_addAction_text(QActionGroup* this_ptr, const QString* text) {
  return this_ptr->addAction(*text);
}

QAction* qt_widgets_c_QActionGroup_checkedAction(const QActionGroup* this_ptr) {
  return this_ptr->checkedAction();
}

void qt_widgets_c_QActionGroup_delete(QActionGroup* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QActionGroup_isEnabled(const QActionGroup* this_ptr) {
  return this_ptr->isEnabled();
}

bool qt_widgets_c_QActionGroup_isExclusive(const QActionGroup* this_ptr) {
  return this_ptr->isExclusive();
}

bool qt_widgets_c_QActionGroup_isVisible(const QActionGroup* this_ptr) {
  return this_ptr->isVisible();
}

const QMetaObject* qt_widgets_c_QActionGroup_metaObject(const QActionGroup* this_ptr) {
  return this_ptr->metaObject();
}

QActionGroup* qt_widgets_c_QActionGroup_new(QObject* parent) {
  return new QActionGroup(parent);
}

int qt_widgets_c_QActionGroup_qt_metacall(QActionGroup* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QActionGroup_qt_metacast(QActionGroup* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QActionGroup_removeAction(QActionGroup* this_ptr, QAction* a) {
  this_ptr->removeAction(a);
}

void qt_widgets_c_QActionGroup_setDisabled(QActionGroup* this_ptr, bool b) {
  this_ptr->setDisabled(b);
}

void qt_widgets_c_QActionGroup_setEnabled(QActionGroup* this_ptr, bool arg1) {
  this_ptr->setEnabled(arg1);
}

void qt_widgets_c_QActionGroup_setExclusive(QActionGroup* this_ptr, bool arg1) {
  this_ptr->setExclusive(arg1);
}

void qt_widgets_c_QActionGroup_setVisible(QActionGroup* this_ptr, bool arg1) {
  this_ptr->setVisible(arg1);
}

void qt_widgets_c_QActionGroup_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QActionGroup::trUtf8(s, c, n));
}

void qt_widgets_c_QActionGroup_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QActionGroup::tr(s, c, n));
}

