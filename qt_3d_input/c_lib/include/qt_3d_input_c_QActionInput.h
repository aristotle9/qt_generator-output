#ifndef QT_3D_INPUT_C_QACTIONINPUT_H
#define QT_3D_INPUT_C_QACTIONINPUT_H

#include "qt_3d_input_c_global.h"

extern "C" {

QT_3D_INPUT_C_EXPORT Qt3DInput::QActionInput* qt_3d_input_c_QActionInput_G_dynamic_cast_Qt3DInput_QActionInput_ptr(Qt3DInput::QAbstractActionInput* ptr);
QT_3D_INPUT_C_EXPORT QObject* qt_3d_input_c_QActionInput_G_static_cast_QObject_ptr(Qt3DInput::QActionInput* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QNode* qt_3d_input_c_QActionInput_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QActionInput* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAbstractActionInput* qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QAbstractActionInput_ptr(Qt3DInput::QActionInput* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QActionInput* qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QActionInput_ptr_QObject(QObject* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QActionInput* qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QActionInput_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QActionInput* qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QActionInput_ptr_Qt3DInput_QAbstractActionInput(Qt3DInput::QAbstractActionInput* ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QActionInput_buttons_to_output(const Qt3DInput::QActionInput* this_ptr, QVector< int >* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QActionInput_delete(Qt3DInput::QActionInput* this_ptr);
QT_3D_INPUT_C_EXPORT const QMetaObject* qt_3d_input_c_Qt3DInput_QActionInput_metaObject(const Qt3DInput::QActionInput* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QActionInput* qt_3d_input_c_Qt3DInput_QActionInput_new_no_args();
QT_3D_INPUT_C_EXPORT Qt3DInput::QActionInput* qt_3d_input_c_Qt3DInput_QActionInput_new_parent(Qt3DCore::QNode* parent);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QActionInput_qt_metacall(Qt3DInput::QActionInput* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_INPUT_C_EXPORT void* qt_3d_input_c_Qt3DInput_QActionInput_qt_metacast(Qt3DInput::QActionInput* this_ptr, const char* arg1);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QActionInput_setButtons(Qt3DInput::QActionInput* this_ptr, const QVector< int >* buttons);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QActionInput_setSourceDevice(Qt3DInput::QActionInput* this_ptr, Qt3DInput::QAbstractPhysicalDevice* sourceDevice);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_Qt3DInput_QActionInput_sourceDevice(const Qt3DInput::QActionInput* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QActionInput_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QActionInput_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_INPUT_C_QACTIONINPUT_H
