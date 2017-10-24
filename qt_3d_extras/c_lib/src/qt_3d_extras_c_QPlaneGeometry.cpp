#include "qt_3d_extras_c_QPlaneGeometry.h"

QObject* qt_3d_extras_c_QPlaneGeometry_G_static_cast_QObject_ptr(Qt3DExtras::QPlaneGeometry* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QPlaneGeometry* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QPlaneGeometry* qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DExtras_QPlaneGeometry_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QPlaneGeometry*>(ptr);
}

Qt3DExtras::QPlaneGeometry* qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DExtras_QPlaneGeometry_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QPlaneGeometry*>(ptr);
}

Qt3DExtras::QPlaneGeometry* qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DExtras_QPlaneGeometry_ptr_Qt3DRender_QGeometry(Qt3DRender::QGeometry* ptr) {
  return static_cast<Qt3DExtras::QPlaneGeometry*>(ptr);
}

Qt3DRender::QGeometry* qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(Qt3DExtras::QPlaneGeometry* ptr) {
  return static_cast<Qt3DRender::QGeometry*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_delete(Qt3DExtras::QPlaneGeometry* this_ptr) {
  delete this_ptr;
}

float qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_height(const Qt3DExtras::QPlaneGeometry* this_ptr) {
  return this_ptr->height();
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_indexAttribute(const Qt3DExtras::QPlaneGeometry* this_ptr) {
  return this_ptr->indexAttribute();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_metaObject(const Qt3DExtras::QPlaneGeometry* this_ptr) {
  return this_ptr->metaObject();
}

bool qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_mirrored(const Qt3DExtras::QPlaneGeometry* this_ptr) {
  return this_ptr->mirrored();
}

Qt3DExtras::QPlaneGeometry* qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_new_no_args() {
  return new Qt3DExtras::QPlaneGeometry();
}

Qt3DExtras::QPlaneGeometry* qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QPlaneGeometry(parent);
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_normalAttribute(const Qt3DExtras::QPlaneGeometry* this_ptr) {
  return this_ptr->normalAttribute();
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_positionAttribute(const Qt3DExtras::QPlaneGeometry* this_ptr) {
  return this_ptr->positionAttribute();
}

int qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_qt_metacall(Qt3DExtras::QPlaneGeometry* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_qt_metacast(Qt3DExtras::QPlaneGeometry* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_resolution_to_output(const Qt3DExtras::QPlaneGeometry* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->resolution());
}

void qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_setHeight(Qt3DExtras::QPlaneGeometry* this_ptr, float height) {
  this_ptr->setHeight(height);
}

void qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_setMirrored(Qt3DExtras::QPlaneGeometry* this_ptr, bool mirrored) {
  this_ptr->setMirrored(mirrored);
}

void qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_setResolution(Qt3DExtras::QPlaneGeometry* this_ptr, const QSize* resolution) {
  this_ptr->setResolution(*resolution);
}

void qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_setWidth(Qt3DExtras::QPlaneGeometry* this_ptr, float width) {
  this_ptr->setWidth(width);
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_tangentAttribute(const Qt3DExtras::QPlaneGeometry* this_ptr) {
  return this_ptr->tangentAttribute();
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_texCoordAttribute(const Qt3DExtras::QPlaneGeometry* this_ptr) {
  return this_ptr->texCoordAttribute();
}

void qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QPlaneGeometry::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QPlaneGeometry::tr(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_updateIndices(Qt3DExtras::QPlaneGeometry* this_ptr) {
  this_ptr->updateIndices();
}

void qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_updateVertices(Qt3DExtras::QPlaneGeometry* this_ptr) {
  this_ptr->updateVertices();
}

float qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_width(const Qt3DExtras::QPlaneGeometry* this_ptr) {
  return this_ptr->width();
}

