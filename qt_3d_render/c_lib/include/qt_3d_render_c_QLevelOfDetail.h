#ifndef QT_3D_RENDER_C_QLEVELOFDETAIL_H
#define QT_3D_RENDER_C_QLEVELOFDETAIL_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QLevelOfDetail_G_static_cast_QObject_ptr(Qt3DRender::QLevelOfDetail* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QComponent* qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QLevelOfDetail* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QLevelOfDetail* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QLevelOfDetail* qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DRender_QLevelOfDetail_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QLevelOfDetail* qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DRender_QLevelOfDetail_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QLevelOfDetail* qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DRender_QLevelOfDetail_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QCamera* qt_3d_render_c_Qt3DRender_QLevelOfDetail_camera(const Qt3DRender::QLevelOfDetail* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLevelOfDetail_createBoundingSphere_to_output(Qt3DRender::QLevelOfDetail* this_ptr, const QVector3D* center, float radius, Qt3DRender::QLevelOfDetailBoundingSphere* output);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QLevelOfDetail_currentIndex(const Qt3DRender::QLevelOfDetail* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLevelOfDetail_delete(Qt3DRender::QLevelOfDetail* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QLevelOfDetail_metaObject(const Qt3DRender::QLevelOfDetail* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QLevelOfDetail* qt_3d_render_c_Qt3DRender_QLevelOfDetail_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QLevelOfDetail* qt_3d_render_c_Qt3DRender_QLevelOfDetail_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QLevelOfDetail_qt_metacall(Qt3DRender::QLevelOfDetail* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QLevelOfDetail_qt_metacast(Qt3DRender::QLevelOfDetail* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLevelOfDetail_setCamera(Qt3DRender::QLevelOfDetail* this_ptr, Qt3DRender::QCamera* camera);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLevelOfDetail_setCurrentIndex(Qt3DRender::QLevelOfDetail* this_ptr, int currentIndex);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLevelOfDetail_setThresholdType(Qt3DRender::QLevelOfDetail* this_ptr, Qt3DRender::QLevelOfDetail::ThresholdType thresholdType);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLevelOfDetail_setThresholds(Qt3DRender::QLevelOfDetail* this_ptr, const QVector< double >* thresholds);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLevelOfDetail_setVolumeOverride(Qt3DRender::QLevelOfDetail* this_ptr, const Qt3DRender::QLevelOfDetailBoundingSphere* volumeOverride);
QT_3D_RENDER_C_EXPORT Qt3DRender::QLevelOfDetail::ThresholdType qt_3d_render_c_Qt3DRender_QLevelOfDetail_thresholdType(const Qt3DRender::QLevelOfDetail* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLevelOfDetail_thresholds_to_output(const Qt3DRender::QLevelOfDetail* this_ptr, QVector< double >* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLevelOfDetail_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLevelOfDetail_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QLevelOfDetail_volumeOverride_to_output(const Qt3DRender::QLevelOfDetail* this_ptr, Qt3DRender::QLevelOfDetailBoundingSphere* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QLEVELOFDETAIL_H
