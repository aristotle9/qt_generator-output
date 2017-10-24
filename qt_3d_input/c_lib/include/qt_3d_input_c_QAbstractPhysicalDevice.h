#ifndef QT_3D_INPUT_C_QABSTRACTPHYSICALDEVICE_H
#define QT_3D_INPUT_C_QABSTRACTPHYSICALDEVICE_H

#include "qt_3d_input_c_global.h"

extern "C" {

QT_3D_INPUT_C_EXPORT QObject* qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_QObject_ptr(Qt3DInput::QAbstractPhysicalDevice* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QNode* qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QAbstractPhysicalDevice* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr_QObject(QObject* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_addAxisSetting(Qt3DInput::QAbstractPhysicalDevice* this_ptr, Qt3DInput::QAxisSetting* axisSetting);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_axisCount(const Qt3DInput::QAbstractPhysicalDevice* this_ptr);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_axisIdentifier(const Qt3DInput::QAbstractPhysicalDevice* this_ptr, const QString* name);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_axisNames_to_output(const Qt3DInput::QAbstractPhysicalDevice* this_ptr, QStringList* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_axisSettings_to_output(const Qt3DInput::QAbstractPhysicalDevice* this_ptr, QVector< Qt3DInput::QAxisSetting* >* output);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_buttonCount(const Qt3DInput::QAbstractPhysicalDevice* this_ptr);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_buttonIdentifier(const Qt3DInput::QAbstractPhysicalDevice* this_ptr, const QString* name);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_buttonNames_to_output(const Qt3DInput::QAbstractPhysicalDevice* this_ptr, QStringList* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_delete(Qt3DInput::QAbstractPhysicalDevice* this_ptr);
QT_3D_INPUT_C_EXPORT const QMetaObject* qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_metaObject(const Qt3DInput::QAbstractPhysicalDevice* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_new_no_args();
QT_3D_INPUT_C_EXPORT Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_new_parent(Qt3DCore::QNode* parent);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_qt_metacall(Qt3DInput::QAbstractPhysicalDevice* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_INPUT_C_EXPORT void* qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_qt_metacast(Qt3DInput::QAbstractPhysicalDevice* this_ptr, const char* arg1);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_removeAxisSetting(Qt3DInput::QAbstractPhysicalDevice* this_ptr, Qt3DInput::QAxisSetting* axisSetting);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_INPUT_C_QABSTRACTPHYSICALDEVICE_H
