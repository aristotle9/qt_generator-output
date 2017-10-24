#include "qt_3d_input_c_QInputAspect.h"

QObject* qt_3d_input_c_QInputAspect_G_static_cast_QObject_ptr(Qt3DInput::QInputAspect* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QAbstractAspect* qt_3d_input_c_QInputAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(Qt3DInput::QInputAspect* ptr) {
  return static_cast<Qt3DCore::QAbstractAspect*>(ptr);
}

Qt3DInput::QInputAspect* qt_3d_input_c_QInputAspect_G_static_cast_Qt3DInput_QInputAspect_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QInputAspect*>(ptr);
}

Qt3DInput::QInputAspect* qt_3d_input_c_QInputAspect_G_static_cast_Qt3DInput_QInputAspect_ptr_Qt3DCore_QAbstractAspect(Qt3DCore::QAbstractAspect* ptr) {
  return static_cast<Qt3DInput::QInputAspect*>(ptr);
}

void qt_3d_input_c_Qt3DInput_QInputAspect_availablePhysicalDevices_to_output(const Qt3DInput::QInputAspect* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->availablePhysicalDevices());
}

Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_Qt3DInput_QInputAspect_createPhysicalDevice(Qt3DInput::QInputAspect* this_ptr, const QString* name) {
  return this_ptr->createPhysicalDevice(*name);
}

void qt_3d_input_c_Qt3DInput_QInputAspect_delete(Qt3DInput::QInputAspect* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QInputAspect_metaObject(const Qt3DInput::QInputAspect* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QInputAspect* qt_3d_input_c_Qt3DInput_QInputAspect_new_no_args() {
  return new Qt3DInput::QInputAspect();
}

Qt3DInput::QInputAspect* qt_3d_input_c_Qt3DInput_QInputAspect_new_parent(QObject* parent) {
  return new Qt3DInput::QInputAspect(parent);
}

int qt_3d_input_c_Qt3DInput_QInputAspect_qt_metacall(Qt3DInput::QInputAspect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QInputAspect_qt_metacast(Qt3DInput::QInputAspect* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QInputAspect_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QInputAspect::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QInputAspect_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QInputAspect::tr(s, c, n));
}

