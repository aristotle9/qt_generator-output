#ifndef QT_3D_INPUT_C_QAXISSETTING_H
#define QT_3D_INPUT_C_QAXISSETTING_H

#include "qt_3d_input_c_global.h"

extern "C" {

QT_3D_INPUT_C_EXPORT QObject* qt_3d_input_c_QAxisSetting_G_static_cast_QObject_ptr(Qt3DInput::QAxisSetting* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QNode* qt_3d_input_c_QAxisSetting_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QAxisSetting* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxisSetting* qt_3d_input_c_QAxisSetting_G_static_cast_Qt3DInput_QAxisSetting_ptr_QObject(QObject* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxisSetting* qt_3d_input_c_QAxisSetting_G_static_cast_Qt3DInput_QAxisSetting_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxisSetting_axes_to_output(const Qt3DInput::QAxisSetting* this_ptr, QVector< int >* output);
QT_3D_INPUT_C_EXPORT float qt_3d_input_c_Qt3DInput_QAxisSetting_deadZoneRadius(const Qt3DInput::QAxisSetting* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxisSetting_delete(Qt3DInput::QAxisSetting* this_ptr);
QT_3D_INPUT_C_EXPORT bool qt_3d_input_c_Qt3DInput_QAxisSetting_isSmoothEnabled(const Qt3DInput::QAxisSetting* this_ptr);
QT_3D_INPUT_C_EXPORT const QMetaObject* qt_3d_input_c_Qt3DInput_QAxisSetting_metaObject(const Qt3DInput::QAxisSetting* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxisSetting* qt_3d_input_c_Qt3DInput_QAxisSetting_new_no_args();
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxisSetting* qt_3d_input_c_Qt3DInput_QAxisSetting_new_parent(Qt3DCore::QNode* parent);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QAxisSetting_qt_metacall(Qt3DInput::QAxisSetting* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_INPUT_C_EXPORT void* qt_3d_input_c_Qt3DInput_QAxisSetting_qt_metacast(Qt3DInput::QAxisSetting* this_ptr, const char* arg1);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxisSetting_setAxes(Qt3DInput::QAxisSetting* this_ptr, const QVector< int >* axes);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxisSetting_setDeadZoneRadius(Qt3DInput::QAxisSetting* this_ptr, float deadZoneRadius);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxisSetting_setSmoothEnabled(Qt3DInput::QAxisSetting* this_ptr, bool enabled);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxisSetting_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxisSetting_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_INPUT_C_QAXISSETTING_H
