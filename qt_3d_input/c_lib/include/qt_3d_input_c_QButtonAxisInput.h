#ifndef QT_3D_INPUT_C_QBUTTONAXISINPUT_H
#define QT_3D_INPUT_C_QBUTTONAXISINPUT_H

#include "qt_3d_input_c_global.h"

extern "C" {

QT_3D_INPUT_C_EXPORT Qt3DInput::QButtonAxisInput* qt_3d_input_c_QButtonAxisInput_G_dynamic_cast_Qt3DInput_QButtonAxisInput_ptr(Qt3DInput::QAbstractAxisInput* ptr);
QT_3D_INPUT_C_EXPORT QObject* qt_3d_input_c_QButtonAxisInput_G_static_cast_QObject_ptr(Qt3DInput::QButtonAxisInput* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QNode* qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QButtonAxisInput* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAbstractAxisInput* qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr(Qt3DInput::QButtonAxisInput* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QButtonAxisInput* qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QButtonAxisInput_ptr_QObject(QObject* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QButtonAxisInput* qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QButtonAxisInput_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QButtonAxisInput* qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QButtonAxisInput_ptr_Qt3DInput_QAbstractAxisInput(Qt3DInput::QAbstractAxisInput* ptr);
QT_3D_INPUT_C_EXPORT float qt_3d_input_c_Qt3DInput_QButtonAxisInput_acceleration(const Qt3DInput::QButtonAxisInput* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QButtonAxisInput_buttons_to_output(const Qt3DInput::QButtonAxisInput* this_ptr, QVector< int >* output);
QT_3D_INPUT_C_EXPORT float qt_3d_input_c_Qt3DInput_QButtonAxisInput_deceleration(const Qt3DInput::QButtonAxisInput* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QButtonAxisInput_delete(Qt3DInput::QButtonAxisInput* this_ptr);
QT_3D_INPUT_C_EXPORT const QMetaObject* qt_3d_input_c_Qt3DInput_QButtonAxisInput_metaObject(const Qt3DInput::QButtonAxisInput* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QButtonAxisInput* qt_3d_input_c_Qt3DInput_QButtonAxisInput_new_no_args();
QT_3D_INPUT_C_EXPORT Qt3DInput::QButtonAxisInput* qt_3d_input_c_Qt3DInput_QButtonAxisInput_new_parent(Qt3DCore::QNode* parent);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QButtonAxisInput_qt_metacall(Qt3DInput::QButtonAxisInput* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_INPUT_C_EXPORT void* qt_3d_input_c_Qt3DInput_QButtonAxisInput_qt_metacast(Qt3DInput::QButtonAxisInput* this_ptr, const char* arg1);
QT_3D_INPUT_C_EXPORT float qt_3d_input_c_Qt3DInput_QButtonAxisInput_scale(const Qt3DInput::QButtonAxisInput* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QButtonAxisInput_setAcceleration(Qt3DInput::QButtonAxisInput* this_ptr, float acceleration);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QButtonAxisInput_setButtons(Qt3DInput::QButtonAxisInput* this_ptr, const QVector< int >* buttons);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QButtonAxisInput_setDeceleration(Qt3DInput::QButtonAxisInput* this_ptr, float deceleration);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QButtonAxisInput_setScale(Qt3DInput::QButtonAxisInput* this_ptr, float scale);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QButtonAxisInput_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QButtonAxisInput_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_INPUT_C_QBUTTONAXISINPUT_H
