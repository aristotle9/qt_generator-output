#ifndef QT_3D_RENDER_C_QPICKINGSETTINGS_H
#define QT_3D_RENDER_C_QPICKINGSETTINGS_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QPickingSettings_G_static_cast_QObject_ptr(Qt3DRender::QPickingSettings* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QPickingSettings_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QPickingSettings* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickingSettings* qt_3d_render_c_QPickingSettings_G_static_cast_Qt3DRender_QPickingSettings_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickingSettings* qt_3d_render_c_QPickingSettings_G_static_cast_Qt3DRender_QPickingSettings_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPickingSettings_delete(Qt3DRender::QPickingSettings* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickingSettings::FaceOrientationPickingMode qt_3d_render_c_Qt3DRender_QPickingSettings_faceOrientationPickingMode(const Qt3DRender::QPickingSettings* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QPickingSettings_metaObject(const Qt3DRender::QPickingSettings* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickingSettings* qt_3d_render_c_Qt3DRender_QPickingSettings_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickingSettings* qt_3d_render_c_Qt3DRender_QPickingSettings_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickingSettings::PickMethod qt_3d_render_c_Qt3DRender_QPickingSettings_pickMethod(const Qt3DRender::QPickingSettings* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickingSettings::PickResultMode qt_3d_render_c_Qt3DRender_QPickingSettings_pickResultMode(const Qt3DRender::QPickingSettings* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QPickingSettings_qt_metacall(Qt3DRender::QPickingSettings* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QPickingSettings_qt_metacast(Qt3DRender::QPickingSettings* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPickingSettings_setFaceOrientationPickingMode(Qt3DRender::QPickingSettings* this_ptr, Qt3DRender::QPickingSettings::FaceOrientationPickingMode faceOrientationPickingMode);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPickingSettings_setPickMethod(Qt3DRender::QPickingSettings* this_ptr, Qt3DRender::QPickingSettings::PickMethod pickMethod);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPickingSettings_setPickResultMode(Qt3DRender::QPickingSettings* this_ptr, Qt3DRender::QPickingSettings::PickResultMode pickResultMode);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPickingSettings_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPickingSettings_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QPICKINGSETTINGS_H
