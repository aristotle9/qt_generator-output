#ifndef QT_3D_INPUT_C_QINPUTASPECT_H
#define QT_3D_INPUT_C_QINPUTASPECT_H

#include "qt_3d_input_c_global.h"

extern "C" {

QT_3D_INPUT_C_EXPORT QObject* qt_3d_input_c_QInputAspect_G_static_cast_QObject_ptr(Qt3DInput::QInputAspect* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QAbstractAspect* qt_3d_input_c_QInputAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(Qt3DInput::QInputAspect* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QInputAspect* qt_3d_input_c_QInputAspect_G_static_cast_Qt3DInput_QInputAspect_ptr_QObject(QObject* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QInputAspect* qt_3d_input_c_QInputAspect_G_static_cast_Qt3DInput_QInputAspect_ptr_Qt3DCore_QAbstractAspect(Qt3DCore::QAbstractAspect* ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QInputAspect_availablePhysicalDevices_to_output(const Qt3DInput::QInputAspect* this_ptr, QStringList* output);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_Qt3DInput_QInputAspect_createPhysicalDevice(Qt3DInput::QInputAspect* this_ptr, const QString* name);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QInputAspect_delete(Qt3DInput::QInputAspect* this_ptr);
QT_3D_INPUT_C_EXPORT const QMetaObject* qt_3d_input_c_Qt3DInput_QInputAspect_metaObject(const Qt3DInput::QInputAspect* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QInputAspect* qt_3d_input_c_Qt3DInput_QInputAspect_new_no_args();
QT_3D_INPUT_C_EXPORT Qt3DInput::QInputAspect* qt_3d_input_c_Qt3DInput_QInputAspect_new_parent(QObject* parent);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QInputAspect_qt_metacall(Qt3DInput::QInputAspect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_INPUT_C_EXPORT void* qt_3d_input_c_Qt3DInput_QInputAspect_qt_metacast(Qt3DInput::QInputAspect* this_ptr, const char* arg1);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QInputAspect_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QInputAspect_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_INPUT_C_QINPUTASPECT_H
