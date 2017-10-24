#ifndef QT_3D_INPUT_C_QAXIS_H
#define QT_3D_INPUT_C_QAXIS_H

#include "qt_3d_input_c_global.h"

extern "C" {

QT_3D_INPUT_C_EXPORT QObject* qt_3d_input_c_QAxis_G_static_cast_QObject_ptr(Qt3DInput::QAxis* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QNode* qt_3d_input_c_QAxis_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QAxis* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxis* qt_3d_input_c_QAxis_G_static_cast_Qt3DInput_QAxis_ptr_QObject(QObject* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxis* qt_3d_input_c_QAxis_G_static_cast_Qt3DInput_QAxis_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxis_addInput(Qt3DInput::QAxis* this_ptr, Qt3DInput::QAbstractAxisInput* input);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxis_delete(Qt3DInput::QAxis* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxis_inputs_to_output(const Qt3DInput::QAxis* this_ptr, QVector< Qt3DInput::QAbstractAxisInput* >* output);
QT_3D_INPUT_C_EXPORT const QMetaObject* qt_3d_input_c_Qt3DInput_QAxis_metaObject(const Qt3DInput::QAxis* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxis* qt_3d_input_c_Qt3DInput_QAxis_new_no_args();
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxis* qt_3d_input_c_Qt3DInput_QAxis_new_parent(Qt3DCore::QNode* parent);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QAxis_qt_metacall(Qt3DInput::QAxis* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_INPUT_C_EXPORT void* qt_3d_input_c_Qt3DInput_QAxis_qt_metacast(Qt3DInput::QAxis* this_ptr, const char* arg1);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxis_removeInput(Qt3DInput::QAxis* this_ptr, Qt3DInput::QAbstractAxisInput* input);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxis_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxis_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT float qt_3d_input_c_Qt3DInput_QAxis_value(const Qt3DInput::QAxis* this_ptr);

} // extern "C"

#endif // QT_3D_INPUT_C_QAXIS_H
