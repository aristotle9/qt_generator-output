#ifndef QT_3D_CORE_C_QASPECTENGINE_H
#define QT_3D_CORE_C_QASPECTENGINE_H

#include "qt_3d_core_c_global.h"

extern "C" {

QT_3D_CORE_C_EXPORT QObject* qt_3d_core_c_QAspectEngine_G_static_cast_QObject_ptr(Qt3DCore::QAspectEngine* ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QAspectEngine* qt_3d_core_c_QAspectEngine_G_static_cast_Qt3DCore_QAspectEngine_ptr(QObject* ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectEngine_aspects_to_output(const Qt3DCore::QAspectEngine* this_ptr, QVector< Qt3DCore::QAbstractAspect* >* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectEngine_delete(Qt3DCore::QAspectEngine* this_ptr);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectEngine_executeCommand_to_output(Qt3DCore::QAspectEngine* this_ptr, const QString* command, QVariant* output);
QT_3D_CORE_C_EXPORT const QMetaObject* qt_3d_core_c_Qt3DCore_QAspectEngine_metaObject(const Qt3DCore::QAspectEngine* this_ptr);
QT_3D_CORE_C_EXPORT Qt3DCore::QAspectEngine* qt_3d_core_c_Qt3DCore_QAspectEngine_new_no_args();
QT_3D_CORE_C_EXPORT Qt3DCore::QAspectEngine* qt_3d_core_c_Qt3DCore_QAspectEngine_new_parent(QObject* parent);
QT_3D_CORE_C_EXPORT int qt_3d_core_c_Qt3DCore_QAspectEngine_qt_metacall(Qt3DCore::QAspectEngine* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_CORE_C_EXPORT void* qt_3d_core_c_Qt3DCore_QAspectEngine_qt_metacast(Qt3DCore::QAspectEngine* this_ptr, const char* arg1);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectEngine_registerAspect_aspect(Qt3DCore::QAspectEngine* this_ptr, Qt3DCore::QAbstractAspect* aspect);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectEngine_registerAspect_name(Qt3DCore::QAspectEngine* this_ptr, const QString* name);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectEngine_rootEntity_to_output(const Qt3DCore::QAspectEngine* this_ptr, QSharedPointer< Qt3DCore::QEntity >* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectEngine_setRootEntity(Qt3DCore::QAspectEngine* this_ptr, const QSharedPointer< Qt3DCore::QEntity >* root);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectEngine_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectEngine_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectEngine_unregisterAspect_aspect(Qt3DCore::QAspectEngine* this_ptr, Qt3DCore::QAbstractAspect* aspect);
QT_3D_CORE_C_EXPORT void qt_3d_core_c_Qt3DCore_QAspectEngine_unregisterAspect_name(Qt3DCore::QAspectEngine* this_ptr, const QString* name);

} // extern "C"

#endif // QT_3D_CORE_C_QASPECTENGINE_H
