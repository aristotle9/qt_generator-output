#ifndef QT_3D_EXTRAS_C_QSPHEREGEOMETRY_H
#define QT_3D_EXTRAS_C_QSPHEREGEOMETRY_H

#include "qt_3d_extras_c_global.h"

extern "C" {

QT_3D_EXTRAS_C_EXPORT QObject* qt_3d_extras_c_QSphereGeometry_G_static_cast_QObject_ptr(Qt3DExtras::QSphereGeometry* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DCore::QNode* qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QSphereGeometry* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::QSphereGeometry* qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DExtras_QSphereGeometry_ptr_QObject(QObject* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::QSphereGeometry* qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DExtras_QSphereGeometry_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::QSphereGeometry* qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DExtras_QSphereGeometry_ptr_Qt3DRender_QGeometry(Qt3DRender::QGeometry* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DRender::QGeometry* qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(Qt3DExtras::QSphereGeometry* ptr);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_delete(Qt3DExtras::QSphereGeometry* this_ptr);
QT_3D_EXTRAS_C_EXPORT bool qt_3d_extras_c_Qt3DExtras_QSphereGeometry_generateTangents(const Qt3DExtras::QSphereGeometry* this_ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_indexAttribute(const Qt3DExtras::QSphereGeometry* this_ptr);
QT_3D_EXTRAS_C_EXPORT const QMetaObject* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_metaObject(const Qt3DExtras::QSphereGeometry* this_ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::QSphereGeometry* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_new_no_args();
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::QSphereGeometry* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_new_parent(Qt3DCore::QNode* parent);
QT_3D_EXTRAS_C_EXPORT Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_normalAttribute(const Qt3DExtras::QSphereGeometry* this_ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_positionAttribute(const Qt3DExtras::QSphereGeometry* this_ptr);
QT_3D_EXTRAS_C_EXPORT int qt_3d_extras_c_Qt3DExtras_QSphereGeometry_qt_metacall(Qt3DExtras::QSphereGeometry* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_EXTRAS_C_EXPORT void* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_qt_metacast(Qt3DExtras::QSphereGeometry* this_ptr, const char* arg1);
QT_3D_EXTRAS_C_EXPORT float qt_3d_extras_c_Qt3DExtras_QSphereGeometry_radius(const Qt3DExtras::QSphereGeometry* this_ptr);
QT_3D_EXTRAS_C_EXPORT int qt_3d_extras_c_Qt3DExtras_QSphereGeometry_rings(const Qt3DExtras::QSphereGeometry* this_ptr);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_setGenerateTangents(Qt3DExtras::QSphereGeometry* this_ptr, bool gen);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_setRadius(Qt3DExtras::QSphereGeometry* this_ptr, float radius);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_setRings(Qt3DExtras::QSphereGeometry* this_ptr, int rings);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_setSlices(Qt3DExtras::QSphereGeometry* this_ptr, int slices);
QT_3D_EXTRAS_C_EXPORT int qt_3d_extras_c_Qt3DExtras_QSphereGeometry_slices(const Qt3DExtras::QSphereGeometry* this_ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_tangentAttribute(const Qt3DExtras::QSphereGeometry* this_ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_texCoordAttribute(const Qt3DExtras::QSphereGeometry* this_ptr);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_updateIndices(Qt3DExtras::QSphereGeometry* this_ptr);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_updateVertices(Qt3DExtras::QSphereGeometry* this_ptr);

} // extern "C"

#endif // QT_3D_EXTRAS_C_QSPHEREGEOMETRY_H
