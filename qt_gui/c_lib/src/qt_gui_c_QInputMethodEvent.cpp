#include "qt_gui_c_QInputMethodEvent.h"

void qt_gui_c_QInputMethodEvent_Attribute_constructor_typ_s_l(QInputMethodEvent::AttributeType typ, int s, int l, QInputMethodEvent::Attribute* output) {
  new(output) QInputMethodEvent::Attribute(typ, s, l);
}

void qt_gui_c_QInputMethodEvent_Attribute_constructor_typ_s_l_val(QInputMethodEvent::AttributeType typ, int s, int l, const QVariant* val, QInputMethodEvent::Attribute* output) {
  new(output) QInputMethodEvent::Attribute(typ, s, l, *val);
}

void qt_gui_c_QInputMethodEvent_Attribute_destructor(QInputMethodEvent::Attribute* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

int qt_gui_c_QInputMethodEvent_Attribute_length(const QInputMethodEvent::Attribute* this_ptr) {
  return this_ptr->length;
}

void qt_gui_c_QInputMethodEvent_Attribute_set_length(QInputMethodEvent::Attribute* this_ptr, int value) {
  this_ptr->length = value;
}

void qt_gui_c_QInputMethodEvent_Attribute_set_start(QInputMethodEvent::Attribute* this_ptr, int value) {
  this_ptr->start = value;
}

void qt_gui_c_QInputMethodEvent_Attribute_set_type(QInputMethodEvent::Attribute* this_ptr, QInputMethodEvent::AttributeType value) {
  this_ptr->type = value;
}

void qt_gui_c_QInputMethodEvent_Attribute_set_value(QInputMethodEvent::Attribute* this_ptr, const QVariant* value) {
  this_ptr->value = *value;
}

int qt_gui_c_QInputMethodEvent_Attribute_start(const QInputMethodEvent::Attribute* this_ptr) {
  return this_ptr->start;
}

QInputMethodEvent::AttributeType qt_gui_c_QInputMethodEvent_Attribute_type(const QInputMethodEvent::Attribute* this_ptr) {
  return this_ptr->type;
}

const QVariant* qt_gui_c_QInputMethodEvent_Attribute_value(const QInputMethodEvent::Attribute* this_ptr) {
  return &this_ptr->value;
}

QVariant* qt_gui_c_QInputMethodEvent_Attribute_value_mut(QInputMethodEvent::Attribute* this_ptr) {
  return &this_ptr->value;
}

QEvent* qt_gui_c_QInputMethodEvent_G_static_cast_QEvent_ptr(QInputMethodEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QInputMethodEvent* qt_gui_c_QInputMethodEvent_G_static_cast_QInputMethodEvent_ptr(QEvent* ptr) {
  return static_cast<QInputMethodEvent*>(ptr);
}

const QList< QInputMethodEvent::Attribute >* qt_gui_c_QInputMethodEvent_attributes(const QInputMethodEvent* this_ptr) {
  return &this_ptr->attributes();
}

const QString* qt_gui_c_QInputMethodEvent_commitString(const QInputMethodEvent* this_ptr) {
  return &this_ptr->commitString();
}

void qt_gui_c_QInputMethodEvent_delete(QInputMethodEvent* this_ptr) {
  delete this_ptr;
}

QInputMethodEvent* qt_gui_c_QInputMethodEvent_new_no_args() {
  return new QInputMethodEvent();
}

QInputMethodEvent* qt_gui_c_QInputMethodEvent_new_other(const QInputMethodEvent* other) {
  return new QInputMethodEvent(*other);
}

QInputMethodEvent* qt_gui_c_QInputMethodEvent_new_preeditText_attributes(const QString* preeditText, const QList< QInputMethodEvent::Attribute >* attributes) {
  return new QInputMethodEvent(*preeditText, *attributes);
}

const QString* qt_gui_c_QInputMethodEvent_preeditString(const QInputMethodEvent* this_ptr) {
  return &this_ptr->preeditString();
}

int qt_gui_c_QInputMethodEvent_replacementLength(const QInputMethodEvent* this_ptr) {
  return this_ptr->replacementLength();
}

int qt_gui_c_QInputMethodEvent_replacementStart(const QInputMethodEvent* this_ptr) {
  return this_ptr->replacementStart();
}

void qt_gui_c_QInputMethodEvent_setCommitString_commitString(QInputMethodEvent* this_ptr, const QString* commitString) {
  this_ptr->setCommitString(*commitString);
}

void qt_gui_c_QInputMethodEvent_setCommitString_commitString_replaceFrom(QInputMethodEvent* this_ptr, const QString* commitString, int replaceFrom) {
  this_ptr->setCommitString(*commitString, replaceFrom);
}

void qt_gui_c_QInputMethodEvent_setCommitString_commitString_replaceFrom_replaceLength(QInputMethodEvent* this_ptr, const QString* commitString, int replaceFrom, int replaceLength) {
  this_ptr->setCommitString(*commitString, replaceFrom, replaceLength);
}

