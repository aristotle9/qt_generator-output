#ifndef QT_3D_RENDER_C_QSCENELOADER_H
#define QT_3D_RENDER_C_QSCENELOADER_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QSceneLoader_G_static_cast_QObject_ptr(Qt3DRender::QSceneLoader* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QComponent* qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QSceneLoader* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QSceneLoader* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QSceneLoader* qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DRender_QSceneLoader_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QSceneLoader* qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DRender_QSceneLoader_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QSceneLoader* qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DRender_QSceneLoader_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QComponent* qt_3d_render_c_Qt3DRender_QSceneLoader_component(const Qt3DRender::QSceneLoader* this_ptr, const QString* entityName, Qt3DRender::QSceneLoader::ComponentType componentType);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSceneLoader_delete(Qt3DRender::QSceneLoader* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QEntity* qt_3d_render_c_Qt3DRender_QSceneLoader_entity(const Qt3DRender::QSceneLoader* this_ptr, const QString* entityName);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSceneLoader_entityNames_to_output(const Qt3DRender::QSceneLoader* this_ptr, QStringList* output);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QSceneLoader_metaObject(const Qt3DRender::QSceneLoader* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QSceneLoader* qt_3d_render_c_Qt3DRender_QSceneLoader_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QSceneLoader* qt_3d_render_c_Qt3DRender_QSceneLoader_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QSceneLoader_qt_metacall(Qt3DRender::QSceneLoader* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QSceneLoader_qt_metacast(Qt3DRender::QSceneLoader* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSceneLoader_setSource(Qt3DRender::QSceneLoader* this_ptr, const QUrl* arg);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSceneLoader_setStatus(Qt3DRender::QSceneLoader* this_ptr, Qt3DRender::QSceneLoader::Status status);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSceneLoader_source_to_output(const Qt3DRender::QSceneLoader* this_ptr, QUrl* output);
QT_3D_RENDER_C_EXPORT Qt3DRender::QSceneLoader::Status qt_3d_render_c_Qt3DRender_QSceneLoader_status(const Qt3DRender::QSceneLoader* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSceneLoader_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSceneLoader_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QSCENELOADER_H
