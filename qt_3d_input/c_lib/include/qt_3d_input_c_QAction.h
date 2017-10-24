#ifndef QT_3D_INPUT_C_QACTION_H
#define QT_3D_INPUT_C_QACTION_H

#include "qt_3d_input_c_global.h"

extern "C" {

QT_3D_INPUT_C_EXPORT QObject* qt_3d_input_c_QAction_G_static_cast_QObject_ptr(Qt3DInput::QAction* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QNode* qt_3d_input_c_QAction_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QAction* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAction* qt_3d_input_c_QAction_G_static_cast_Qt3DInput_QAction_ptr_QObject(QObject* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAction* qt_3d_input_c_QAction_G_static_cast_Qt3DInput_QAction_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAction_addInput(Qt3DInput::QAction* this_ptr, Qt3DInput::QAbstractActionInput* input);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAction_delete(Qt3DInput::QAction* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAction_inputs_to_output(const Qt3DInput::QAction* this_ptr, QVector< Qt3DInput::QAbstractActionInput* >* output);
QT_3D_INPUT_C_EXPORT bool qt_3d_input_c_Qt3DInput_QAction_isActive(const Qt3DInput::QAction* this_ptr);
QT_3D_INPUT_C_EXPORT const QMetaObject* qt_3d_input_c_Qt3DInput_QAction_metaObject(const Qt3DInput::QAction* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAction* qt_3d_input_c_Qt3DInput_QAction_new_no_args();
QT_3D_INPUT_C_EXPORT Qt3DInput::QAction* qt_3d_input_c_Qt3DInput_QAction_new_parent(Qt3DCore::QNode* parent);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QAction_qt_metacall(Qt3DInput::QAction* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_INPUT_C_EXPORT void* qt_3d_input_c_Qt3DInput_QAction_qt_metacast(Qt3DInput::QAction* this_ptr, const char* arg1);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAction_removeInput(Qt3DInput::QAction* this_ptr, Qt3DInput::QAbstractActionInput* input);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAction_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAction_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_INPUT_C_QACTION_H
