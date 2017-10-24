#ifndef QT_3D_RENDER_C_QPARAMETER_H
#define QT_3D_RENDER_C_QPARAMETER_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QParameter_G_static_cast_QObject_ptr(Qt3DRender::QParameter* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QParameter_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QParameter* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QParameter* qt_3d_render_c_QParameter_G_static_cast_Qt3DRender_QParameter_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QParameter* qt_3d_render_c_QParameter_G_static_cast_Qt3DRender_QParameter_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QParameter_delete(Qt3DRender::QParameter* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QParameter_metaObject(const Qt3DRender::QParameter* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QParameter_name_to_output(const Qt3DRender::QParameter* this_ptr, QString* output);
QT_3D_RENDER_C_EXPORT Qt3DRender::QParameter* qt_3d_render_c_Qt3DRender_QParameter_new_name_texture(const QString* name, Qt3DRender::QAbstractTexture* texture);
QT_3D_RENDER_C_EXPORT Qt3DRender::QParameter* qt_3d_render_c_Qt3DRender_QParameter_new_name_texture_parent(const QString* name, Qt3DRender::QAbstractTexture* texture, Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT Qt3DRender::QParameter* qt_3d_render_c_Qt3DRender_QParameter_new_name_value(const QString* name, const QVariant* value);
QT_3D_RENDER_C_EXPORT Qt3DRender::QParameter* qt_3d_render_c_Qt3DRender_QParameter_new_name_value_parent(const QString* name, const QVariant* value, Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT Qt3DRender::QParameter* qt_3d_render_c_Qt3DRender_QParameter_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QParameter* qt_3d_render_c_Qt3DRender_QParameter_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QParameter_qt_metacall(Qt3DRender::QParameter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QParameter_qt_metacast(Qt3DRender::QParameter* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QParameter_setName(Qt3DRender::QParameter* this_ptr, const QString* name);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QParameter_setValue(Qt3DRender::QParameter* this_ptr, const QVariant* dv);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QParameter_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QParameter_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QParameter_value_to_output(const Qt3DRender::QParameter* this_ptr, QVariant* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QPARAMETER_H
