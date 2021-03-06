#ifndef QT_3D_EXTRAS_C_QPHONGMATERIAL_H
#define QT_3D_EXTRAS_C_QPHONGMATERIAL_H

#include "qt_3d_extras_c_global.h"

extern "C" {

QT_3D_EXTRAS_C_EXPORT QObject* qt_3d_extras_c_QPhongMaterial_G_static_cast_QObject_ptr(Qt3DExtras::QPhongMaterial* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DCore::QComponent* qt_3d_extras_c_QPhongMaterial_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QPhongMaterial* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DCore::QNode* qt_3d_extras_c_QPhongMaterial_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QPhongMaterial* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::QPhongMaterial* qt_3d_extras_c_QPhongMaterial_G_static_cast_Qt3DExtras_QPhongMaterial_ptr_QObject(QObject* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::QPhongMaterial* qt_3d_extras_c_QPhongMaterial_G_static_cast_Qt3DExtras_QPhongMaterial_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::QPhongMaterial* qt_3d_extras_c_QPhongMaterial_G_static_cast_Qt3DExtras_QPhongMaterial_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::QPhongMaterial* qt_3d_extras_c_QPhongMaterial_G_static_cast_Qt3DExtras_QPhongMaterial_ptr_Qt3DRender_QMaterial(Qt3DRender::QMaterial* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DRender::QMaterial* qt_3d_extras_c_QPhongMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(Qt3DExtras::QPhongMaterial* ptr);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QPhongMaterial_ambient_to_output(const Qt3DExtras::QPhongMaterial* this_ptr, QColor* output);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QPhongMaterial_delete(Qt3DExtras::QPhongMaterial* this_ptr);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QPhongMaterial_diffuse_to_output(const Qt3DExtras::QPhongMaterial* this_ptr, QColor* output);
QT_3D_EXTRAS_C_EXPORT const QMetaObject* qt_3d_extras_c_Qt3DExtras_QPhongMaterial_metaObject(const Qt3DExtras::QPhongMaterial* this_ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::QPhongMaterial* qt_3d_extras_c_Qt3DExtras_QPhongMaterial_new_no_args();
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::QPhongMaterial* qt_3d_extras_c_Qt3DExtras_QPhongMaterial_new_parent(Qt3DCore::QNode* parent);
QT_3D_EXTRAS_C_EXPORT int qt_3d_extras_c_Qt3DExtras_QPhongMaterial_qt_metacall(Qt3DExtras::QPhongMaterial* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_EXTRAS_C_EXPORT void* qt_3d_extras_c_Qt3DExtras_QPhongMaterial_qt_metacast(Qt3DExtras::QPhongMaterial* this_ptr, const char* arg1);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QPhongMaterial_setAmbient(Qt3DExtras::QPhongMaterial* this_ptr, const QColor* ambient);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QPhongMaterial_setDiffuse(Qt3DExtras::QPhongMaterial* this_ptr, const QColor* diffuse);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QPhongMaterial_setShininess(Qt3DExtras::QPhongMaterial* this_ptr, float shininess);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QPhongMaterial_setSpecular(Qt3DExtras::QPhongMaterial* this_ptr, const QColor* specular);
QT_3D_EXTRAS_C_EXPORT float qt_3d_extras_c_Qt3DExtras_QPhongMaterial_shininess(const Qt3DExtras::QPhongMaterial* this_ptr);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QPhongMaterial_specular_to_output(const Qt3DExtras::QPhongMaterial* this_ptr, QColor* output);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QPhongMaterial_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_QPhongMaterial_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_EXTRAS_C_QPHONGMATERIAL_H
