#include "qt_3d_render_c_QLevelOfDetail.h"

QObject* qt_3d_render_c_QLevelOfDetail_G_static_cast_QObject_ptr(Qt3DRender::QLevelOfDetail* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QLevelOfDetail* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QLevelOfDetail* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QLevelOfDetail* qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DRender_QLevelOfDetail_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QLevelOfDetail*>(ptr);
}

Qt3DRender::QLevelOfDetail* qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DRender_QLevelOfDetail_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QLevelOfDetail*>(ptr);
}

Qt3DRender::QLevelOfDetail* qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DRender_QLevelOfDetail_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QLevelOfDetail*>(ptr);
}

Qt3DRender::QCamera* qt_3d_render_c_Qt3DRender_QLevelOfDetail_camera(const Qt3DRender::QLevelOfDetail* this_ptr) {
  return this_ptr->camera();
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetail_createBoundingSphere_to_output(Qt3DRender::QLevelOfDetail* this_ptr, const QVector3D* center, float radius, Qt3DRender::QLevelOfDetailBoundingSphere* output) {
  new(output) Qt3DRender::QLevelOfDetailBoundingSphere(this_ptr->createBoundingSphere(*center, radius));
}

int qt_3d_render_c_Qt3DRender_QLevelOfDetail_currentIndex(const Qt3DRender::QLevelOfDetail* this_ptr) {
  return this_ptr->currentIndex();
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetail_delete(Qt3DRender::QLevelOfDetail* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QLevelOfDetail_metaObject(const Qt3DRender::QLevelOfDetail* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QLevelOfDetail* qt_3d_render_c_Qt3DRender_QLevelOfDetail_new_no_args() {
  return new Qt3DRender::QLevelOfDetail();
}

Qt3DRender::QLevelOfDetail* qt_3d_render_c_Qt3DRender_QLevelOfDetail_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QLevelOfDetail(parent);
}

int qt_3d_render_c_Qt3DRender_QLevelOfDetail_qt_metacall(Qt3DRender::QLevelOfDetail* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QLevelOfDetail_qt_metacast(Qt3DRender::QLevelOfDetail* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetail_setCamera(Qt3DRender::QLevelOfDetail* this_ptr, Qt3DRender::QCamera* camera) {
  this_ptr->setCamera(camera);
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetail_setCurrentIndex(Qt3DRender::QLevelOfDetail* this_ptr, int currentIndex) {
  this_ptr->setCurrentIndex(currentIndex);
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetail_setThresholdType(Qt3DRender::QLevelOfDetail* this_ptr, Qt3DRender::QLevelOfDetail::ThresholdType thresholdType) {
  this_ptr->setThresholdType(thresholdType);
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetail_setThresholds(Qt3DRender::QLevelOfDetail* this_ptr, const QVector< double >* thresholds) {
  this_ptr->setThresholds(*thresholds);
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetail_setVolumeOverride(Qt3DRender::QLevelOfDetail* this_ptr, const Qt3DRender::QLevelOfDetailBoundingSphere* volumeOverride) {
  this_ptr->setVolumeOverride(*volumeOverride);
}

Qt3DRender::QLevelOfDetail::ThresholdType qt_3d_render_c_Qt3DRender_QLevelOfDetail_thresholdType(const Qt3DRender::QLevelOfDetail* this_ptr) {
  return this_ptr->thresholdType();
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetail_thresholds_to_output(const Qt3DRender::QLevelOfDetail* this_ptr, QVector< double >* output) {
  new(output) QVector< double >(this_ptr->thresholds());
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetail_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QLevelOfDetail::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetail_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QLevelOfDetail::tr(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetail_volumeOverride_to_output(const Qt3DRender::QLevelOfDetail* this_ptr, Qt3DRender::QLevelOfDetailBoundingSphere* output) {
  new(output) Qt3DRender::QLevelOfDetailBoundingSphere(this_ptr->volumeOverride());
}

