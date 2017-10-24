#include "qt_core_c_QMetaClassInfo.h"

void qt_core_c_QMetaClassInfo_constructor(QMetaClassInfo* output) {
  new(output) QMetaClassInfo();
}

void qt_core_c_QMetaClassInfo_destructor(QMetaClassInfo* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

const QMetaObject* qt_core_c_QMetaClassInfo_enclosingMetaObject(const QMetaClassInfo* this_ptr) {
  return this_ptr->enclosingMetaObject();
}

const char* qt_core_c_QMetaClassInfo_name(const QMetaClassInfo* this_ptr) {
  return this_ptr->name();
}

const char* qt_core_c_QMetaClassInfo_value(const QMetaClassInfo* this_ptr) {
  return this_ptr->value();
}

