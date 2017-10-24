#include "qt_3d_render_c_QShaderData.h"

QObject* qt_3d_render_c_QShaderData_G_static_cast_QObject_ptr(Qt3DRender::QShaderData* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QShaderData_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QShaderData* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QShaderData_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QShaderData* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QShaderData* qt_3d_render_c_QShaderData_G_static_cast_Qt3DRender_QShaderData_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QShaderData*>(ptr);
}

Qt3DRender::QShaderData* qt_3d_render_c_QShaderData_G_static_cast_Qt3DRender_QShaderData_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QShaderData*>(ptr);
}

Qt3DRender::QShaderData* qt_3d_render_c_QShaderData_G_static_cast_Qt3DRender_QShaderData_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QShaderData*>(ptr);
}

void qt_3d_render_c_Qt3DRender_PropertyReaderInterface_delete(Qt3DRender::PropertyReaderInterface* this_ptr) {
  delete this_ptr;
}

void qt_3d_render_c_Qt3DRender_PropertyReaderInterface_readProperty_to_output(Qt3DRender::PropertyReaderInterface* this_ptr, const QVariant* v, QVariant* output) {
  new(output) QVariant(this_ptr->readProperty(*v));
}

void qt_3d_render_c_Qt3DRender_QShaderData_delete(Qt3DRender::QShaderData* this_ptr) {
  delete this_ptr;
}

bool qt_3d_render_c_Qt3DRender_QShaderData_event(Qt3DRender::QShaderData* this_ptr, QEvent* event) {
  return this_ptr->event(event);
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QShaderData_metaObject(const Qt3DRender::QShaderData* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QShaderData* qt_3d_render_c_Qt3DRender_QShaderData_new_no_args() {
  return new Qt3DRender::QShaderData();
}

Qt3DRender::QShaderData* qt_3d_render_c_Qt3DRender_QShaderData_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QShaderData(parent);
}

void qt_3d_render_c_Qt3DRender_QShaderData_propertyReader_to_output(const Qt3DRender::QShaderData* this_ptr, QSharedPointer< Qt3DRender::PropertyReaderInterface >* output) {
  new(output) QSharedPointer< Qt3DRender::PropertyReaderInterface >(this_ptr->propertyReader());
}

int qt_3d_render_c_Qt3DRender_QShaderData_qt_metacall(Qt3DRender::QShaderData* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QShaderData_qt_metacast(Qt3DRender::QShaderData* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QShaderData_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QShaderData::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QShaderData_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QShaderData::tr(s, c, n));
}

