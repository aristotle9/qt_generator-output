#include "qt_core_c_QMetaProperty.h"

void qt_core_c_QMetaProperty_constructor(QMetaProperty* output) {
  new(output) QMetaProperty();
}

void qt_core_c_QMetaProperty_destructor(QMetaProperty* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

const QMetaObject* qt_core_c_QMetaProperty_enclosingMetaObject(const QMetaProperty* this_ptr) {
  return this_ptr->enclosingMetaObject();
}

void qt_core_c_QMetaProperty_enumerator_to_output(const QMetaProperty* this_ptr, QMetaEnum* output) {
  new(output) QMetaEnum(this_ptr->enumerator());
}

bool qt_core_c_QMetaProperty_hasNotifySignal(const QMetaProperty* this_ptr) {
  return this_ptr->hasNotifySignal();
}

bool qt_core_c_QMetaProperty_hasStdCppSet(const QMetaProperty* this_ptr) {
  return this_ptr->hasStdCppSet();
}

bool qt_core_c_QMetaProperty_isConstant(const QMetaProperty* this_ptr) {
  return this_ptr->isConstant();
}

bool qt_core_c_QMetaProperty_isDesignable_no_args(const QMetaProperty* this_ptr) {
  return this_ptr->isDesignable();
}

bool qt_core_c_QMetaProperty_isDesignable_obj(const QMetaProperty* this_ptr, const QObject* obj) {
  return this_ptr->isDesignable(obj);
}

bool qt_core_c_QMetaProperty_isEditable_no_args(const QMetaProperty* this_ptr) {
  return this_ptr->isEditable();
}

bool qt_core_c_QMetaProperty_isEditable_obj(const QMetaProperty* this_ptr, const QObject* obj) {
  return this_ptr->isEditable(obj);
}

bool qt_core_c_QMetaProperty_isEnumType(const QMetaProperty* this_ptr) {
  return this_ptr->isEnumType();
}

bool qt_core_c_QMetaProperty_isFinal(const QMetaProperty* this_ptr) {
  return this_ptr->isFinal();
}

bool qt_core_c_QMetaProperty_isFlagType(const QMetaProperty* this_ptr) {
  return this_ptr->isFlagType();
}

bool qt_core_c_QMetaProperty_isReadable(const QMetaProperty* this_ptr) {
  return this_ptr->isReadable();
}

bool qt_core_c_QMetaProperty_isResettable(const QMetaProperty* this_ptr) {
  return this_ptr->isResettable();
}

bool qt_core_c_QMetaProperty_isScriptable_no_args(const QMetaProperty* this_ptr) {
  return this_ptr->isScriptable();
}

bool qt_core_c_QMetaProperty_isScriptable_obj(const QMetaProperty* this_ptr, const QObject* obj) {
  return this_ptr->isScriptable(obj);
}

bool qt_core_c_QMetaProperty_isStored_no_args(const QMetaProperty* this_ptr) {
  return this_ptr->isStored();
}

bool qt_core_c_QMetaProperty_isStored_obj(const QMetaProperty* this_ptr, const QObject* obj) {
  return this_ptr->isStored(obj);
}

bool qt_core_c_QMetaProperty_isUser_no_args(const QMetaProperty* this_ptr) {
  return this_ptr->isUser();
}

bool qt_core_c_QMetaProperty_isUser_obj(const QMetaProperty* this_ptr, const QObject* obj) {
  return this_ptr->isUser(obj);
}

bool qt_core_c_QMetaProperty_isValid(const QMetaProperty* this_ptr) {
  return this_ptr->isValid();
}

bool qt_core_c_QMetaProperty_isWritable(const QMetaProperty* this_ptr) {
  return this_ptr->isWritable();
}

const char* qt_core_c_QMetaProperty_name(const QMetaProperty* this_ptr) {
  return this_ptr->name();
}

int qt_core_c_QMetaProperty_notifySignalIndex(const QMetaProperty* this_ptr) {
  return this_ptr->notifySignalIndex();
}

void qt_core_c_QMetaProperty_notifySignal_to_output(const QMetaProperty* this_ptr, QMetaMethod* output) {
  new(output) QMetaMethod(this_ptr->notifySignal());
}

int qt_core_c_QMetaProperty_propertyIndex(const QMetaProperty* this_ptr) {
  return this_ptr->propertyIndex();
}

void qt_core_c_QMetaProperty_readOnGadget_to_output(const QMetaProperty* this_ptr, const void* gadget, QVariant* output) {
  new(output) QVariant(this_ptr->readOnGadget(gadget));
}

void qt_core_c_QMetaProperty_read_to_output(const QMetaProperty* this_ptr, const QObject* obj, QVariant* output) {
  new(output) QVariant(this_ptr->read(obj));
}

bool qt_core_c_QMetaProperty_reset(const QMetaProperty* this_ptr, QObject* obj) {
  return this_ptr->reset(obj);
}

bool qt_core_c_QMetaProperty_resetOnGadget(const QMetaProperty* this_ptr, void* gadget) {
  return this_ptr->resetOnGadget(gadget);
}

int qt_core_c_QMetaProperty_revision(const QMetaProperty* this_ptr) {
  return this_ptr->revision();
}

const char* qt_core_c_QMetaProperty_typeName(const QMetaProperty* this_ptr) {
  return this_ptr->typeName();
}

int qt_core_c_QMetaProperty_userType(const QMetaProperty* this_ptr) {
  return this_ptr->userType();
}

bool qt_core_c_QMetaProperty_write(const QMetaProperty* this_ptr, QObject* obj, const QVariant* value) {
  return this_ptr->write(obj, *value);
}

bool qt_core_c_QMetaProperty_writeOnGadget(const QMetaProperty* this_ptr, void* gadget, const QVariant* value) {
  return this_ptr->writeOnGadget(gadget, *value);
}

