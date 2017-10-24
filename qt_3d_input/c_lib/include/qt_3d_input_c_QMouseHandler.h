#ifndef QT_3D_INPUT_C_QMOUSEHANDLER_H
#define QT_3D_INPUT_C_QMOUSEHANDLER_H

#include "qt_3d_input_c_global.h"

extern "C" {

QT_3D_INPUT_C_EXPORT QObject* qt_3d_input_c_QMouseHandler_G_static_cast_QObject_ptr(Qt3DInput::QMouseHandler* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QComponent* qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DInput::QMouseHandler* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QNode* qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QMouseHandler* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseHandler* qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DInput_QMouseHandler_ptr_QObject(QObject* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseHandler* qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DInput_QMouseHandler_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseHandler* qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DInput_QMouseHandler_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_INPUT_C_EXPORT bool qt_3d_input_c_Qt3DInput_QMouseHandler_containsMouse(const Qt3DInput::QMouseHandler* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QMouseHandler_delete(Qt3DInput::QMouseHandler* this_ptr);
QT_3D_INPUT_C_EXPORT const QMetaObject* qt_3d_input_c_Qt3DInput_QMouseHandler_metaObject(const Qt3DInput::QMouseHandler* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseHandler* qt_3d_input_c_Qt3DInput_QMouseHandler_new_no_args();
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseHandler* qt_3d_input_c_Qt3DInput_QMouseHandler_new_parent(Qt3DCore::QNode* parent);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QMouseHandler_qt_metacall(Qt3DInput::QMouseHandler* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_INPUT_C_EXPORT void* qt_3d_input_c_Qt3DInput_QMouseHandler_qt_metacast(Qt3DInput::QMouseHandler* this_ptr, const char* arg1);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QMouseHandler_setSourceDevice(Qt3DInput::QMouseHandler* this_ptr, Qt3DInput::QMouseDevice* mouseDevice);
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseDevice* qt_3d_input_c_Qt3DInput_QMouseHandler_sourceDevice(const Qt3DInput::QMouseHandler* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QMouseHandler_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QMouseHandler_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_INPUT_C_QMOUSEHANDLER_H
