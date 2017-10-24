#include "qt_3d_input_c_QInputSettings.h"

QObject* qt_3d_input_c_QInputSettings_G_static_cast_QObject_ptr(Qt3DInput::QInputSettings* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_input_c_QInputSettings_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DInput::QInputSettings* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QInputSettings_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QInputSettings* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QInputSettings* qt_3d_input_c_QInputSettings_G_static_cast_Qt3DInput_QInputSettings_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QInputSettings*>(ptr);
}

Qt3DInput::QInputSettings* qt_3d_input_c_QInputSettings_G_static_cast_Qt3DInput_QInputSettings_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DInput::QInputSettings*>(ptr);
}

Qt3DInput::QInputSettings* qt_3d_input_c_QInputSettings_G_static_cast_Qt3DInput_QInputSettings_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QInputSettings*>(ptr);
}

void qt_3d_input_c_Qt3DInput_QInputSettings_delete(Qt3DInput::QInputSettings* this_ptr) {
  delete this_ptr;
}

QObject* qt_3d_input_c_Qt3DInput_QInputSettings_eventSource(const Qt3DInput::QInputSettings* this_ptr) {
  return this_ptr->eventSource();
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QInputSettings_metaObject(const Qt3DInput::QInputSettings* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QInputSettings* qt_3d_input_c_Qt3DInput_QInputSettings_new_no_args() {
  return new Qt3DInput::QInputSettings();
}

Qt3DInput::QInputSettings* qt_3d_input_c_Qt3DInput_QInputSettings_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QInputSettings(parent);
}

int qt_3d_input_c_Qt3DInput_QInputSettings_qt_metacall(Qt3DInput::QInputSettings* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QInputSettings_qt_metacast(Qt3DInput::QInputSettings* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QInputSettings_setEventSource(Qt3DInput::QInputSettings* this_ptr, QObject* eventSource) {
  this_ptr->setEventSource(eventSource);
}

void qt_3d_input_c_Qt3DInput_QInputSettings_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QInputSettings::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QInputSettings_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QInputSettings::tr(s, c, n));
}

