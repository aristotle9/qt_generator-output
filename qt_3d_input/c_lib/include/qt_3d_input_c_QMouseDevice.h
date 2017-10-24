#ifndef QT_3D_INPUT_C_QMOUSEDEVICE_H
#define QT_3D_INPUT_C_QMOUSEDEVICE_H

#include "qt_3d_input_c_global.h"

extern "C" {

QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseDevice* qt_3d_input_c_QMouseDevice_G_dynamic_cast_Qt3DInput_QMouseDevice_ptr(Qt3DInput::QAbstractPhysicalDevice* ptr);
QT_3D_INPUT_C_EXPORT QObject* qt_3d_input_c_QMouseDevice_G_static_cast_QObject_ptr(Qt3DInput::QMouseDevice* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QNode* qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QMouseDevice* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr(Qt3DInput::QMouseDevice* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseDevice* qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QMouseDevice_ptr_QObject(QObject* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseDevice* qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QMouseDevice_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseDevice* qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QMouseDevice_ptr_Qt3DInput_QAbstractPhysicalDevice(Qt3DInput::QAbstractPhysicalDevice* ptr);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QMouseDevice_axisCount(const Qt3DInput::QMouseDevice* this_ptr);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QMouseDevice_axisIdentifier(const Qt3DInput::QMouseDevice* this_ptr, const QString* name);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QMouseDevice_axisNames_to_output(const Qt3DInput::QMouseDevice* this_ptr, QStringList* output);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QMouseDevice_buttonCount(const Qt3DInput::QMouseDevice* this_ptr);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QMouseDevice_buttonIdentifier(const Qt3DInput::QMouseDevice* this_ptr, const QString* name);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QMouseDevice_buttonNames_to_output(const Qt3DInput::QMouseDevice* this_ptr, QStringList* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QMouseDevice_delete(Qt3DInput::QMouseDevice* this_ptr);
QT_3D_INPUT_C_EXPORT const QMetaObject* qt_3d_input_c_Qt3DInput_QMouseDevice_metaObject(const Qt3DInput::QMouseDevice* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseDevice* qt_3d_input_c_Qt3DInput_QMouseDevice_new_no_args();
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseDevice* qt_3d_input_c_Qt3DInput_QMouseDevice_new_parent(Qt3DCore::QNode* parent);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QMouseDevice_qt_metacall(Qt3DInput::QMouseDevice* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_INPUT_C_EXPORT void* qt_3d_input_c_Qt3DInput_QMouseDevice_qt_metacast(Qt3DInput::QMouseDevice* this_ptr, const char* arg1);
QT_3D_INPUT_C_EXPORT float qt_3d_input_c_Qt3DInput_QMouseDevice_sensitivity(const Qt3DInput::QMouseDevice* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QMouseDevice_setSensitivity(Qt3DInput::QMouseDevice* this_ptr, float value);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QMouseDevice_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QMouseDevice_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_INPUT_C_QMOUSEDEVICE_H
