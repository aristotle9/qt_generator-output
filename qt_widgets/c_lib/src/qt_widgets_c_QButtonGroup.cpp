#include "qt_widgets_c_QButtonGroup.h"

QButtonGroup* qt_widgets_c_QButtonGroup_G_static_cast_QButtonGroup_ptr(QObject* ptr) {
  return static_cast<QButtonGroup*>(ptr);
}

QObject* qt_widgets_c_QButtonGroup_G_static_cast_QObject_ptr(QButtonGroup* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QButtonGroup_addButton_arg1(QButtonGroup* this_ptr, QAbstractButton* arg1) {
  this_ptr->addButton(arg1);
}

void qt_widgets_c_QButtonGroup_addButton_arg1_id(QButtonGroup* this_ptr, QAbstractButton* arg1, int id) {
  this_ptr->addButton(arg1, id);
}

QAbstractButton* qt_widgets_c_QButtonGroup_button(const QButtonGroup* this_ptr, int id) {
  return this_ptr->button(id);
}

void qt_widgets_c_QButtonGroup_buttons_to_output(const QButtonGroup* this_ptr, QList< QAbstractButton* >* output) {
  new(output) QList< QAbstractButton* >(this_ptr->buttons());
}

QAbstractButton* qt_widgets_c_QButtonGroup_checkedButton(const QButtonGroup* this_ptr) {
  return this_ptr->checkedButton();
}

int qt_widgets_c_QButtonGroup_checkedId(const QButtonGroup* this_ptr) {
  return this_ptr->checkedId();
}

void qt_widgets_c_QButtonGroup_delete(QButtonGroup* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QButtonGroup_exclusive(const QButtonGroup* this_ptr) {
  return this_ptr->exclusive();
}

int qt_widgets_c_QButtonGroup_id(const QButtonGroup* this_ptr, QAbstractButton* button) {
  return this_ptr->id(button);
}

const QMetaObject* qt_widgets_c_QButtonGroup_metaObject(const QButtonGroup* this_ptr) {
  return this_ptr->metaObject();
}

QButtonGroup* qt_widgets_c_QButtonGroup_new_no_args() {
  return new QButtonGroup();
}

QButtonGroup* qt_widgets_c_QButtonGroup_new_parent(QObject* parent) {
  return new QButtonGroup(parent);
}

int qt_widgets_c_QButtonGroup_qt_metacall(QButtonGroup* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QButtonGroup_qt_metacast(QButtonGroup* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QButtonGroup_removeButton(QButtonGroup* this_ptr, QAbstractButton* arg1) {
  this_ptr->removeButton(arg1);
}

void qt_widgets_c_QButtonGroup_setExclusive(QButtonGroup* this_ptr, bool arg1) {
  this_ptr->setExclusive(arg1);
}

void qt_widgets_c_QButtonGroup_setId(QButtonGroup* this_ptr, QAbstractButton* button, int id) {
  this_ptr->setId(button, id);
}

void qt_widgets_c_QButtonGroup_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QButtonGroup::trUtf8(s, c, n));
}

void qt_widgets_c_QButtonGroup_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QButtonGroup::tr(s, c, n));
}

