#ifndef QT_3D_RENDER_C_QGEOMETRY_H
#define QT_3D_RENDER_C_QGEOMETRY_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QGeometry_G_static_cast_QObject_ptr(Qt3DRender::QGeometry* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QGeometry_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QGeometry* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGeometry* qt_3d_render_c_QGeometry_G_static_cast_Qt3DRender_QGeometry_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGeometry* qt_3d_render_c_QGeometry_G_static_cast_Qt3DRender_QGeometry_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometry_addAttribute(Qt3DRender::QGeometry* this_ptr, Qt3DRender::QAttribute* attribute);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometry_attributes_to_output(const Qt3DRender::QGeometry* this_ptr, QVector< Qt3DRender::QAttribute* >* output);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QGeometry_boundingVolumePositionAttribute(const Qt3DRender::QGeometry* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometry_delete(Qt3DRender::QGeometry* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QGeometry_metaObject(const Qt3DRender::QGeometry* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGeometry* qt_3d_render_c_Qt3DRender_QGeometry_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QGeometry* qt_3d_render_c_Qt3DRender_QGeometry_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QGeometry_qt_metacall(Qt3DRender::QGeometry* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QGeometry_qt_metacast(Qt3DRender::QGeometry* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometry_removeAttribute(Qt3DRender::QGeometry* this_ptr, Qt3DRender::QAttribute* attribute);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometry_setBoundingVolumePositionAttribute(Qt3DRender::QGeometry* this_ptr, Qt3DRender::QAttribute* boundingVolumePositionAttribute);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometry_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometry_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QGEOMETRY_H
