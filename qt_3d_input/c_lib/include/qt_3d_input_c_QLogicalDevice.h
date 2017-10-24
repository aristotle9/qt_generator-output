#ifndef QT_3D_INPUT_C_QLOGICALDEVICE_H
#define QT_3D_INPUT_C_QLOGICALDEVICE_H

#include "qt_3d_input_c_global.h"

extern "C" {

QT_3D_INPUT_C_EXPORT QObject* qt_3d_input_c_QLogicalDevice_G_static_cast_QObject_ptr(Qt3DInput::QLogicalDevice* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QComponent* qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DInput::QLogicalDevice* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QNode* qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QLogicalDevice* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QLogicalDevice* qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DInput_QLogicalDevice_ptr_QObject(QObject* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QLogicalDevice* qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DInput_QLogicalDevice_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QLogicalDevice* qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DInput_QLogicalDevice_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QLogicalDevice_actions_to_output(const Qt3DInput::QLogicalDevice* this_ptr, QVector< Qt3DInput::QAction* >* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QLogicalDevice_addAction(Qt3DInput::QLogicalDevice* this_ptr, Qt3DInput::QAction* action);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QLogicalDevice_addAxis(Qt3DInput::QLogicalDevice* this_ptr, Qt3DInput::QAxis* axis);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QLogicalDevice_axes_to_output(const Qt3DInput::QLogicalDevice* this_ptr, QVector< Qt3DInput::QAxis* >* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QLogicalDevice_delete(Qt3DInput::QLogicalDevice* this_ptr);
QT_3D_INPUT_C_EXPORT const QMetaObject* qt_3d_input_c_Qt3DInput_QLogicalDevice_metaObject(const Qt3DInput::QLogicalDevice* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QLogicalDevice* qt_3d_input_c_Qt3DInput_QLogicalDevice_new_no_args();
QT_3D_INPUT_C_EXPORT Qt3DInput::QLogicalDevice* qt_3d_input_c_Qt3DInput_QLogicalDevice_new_parent(Qt3DCore::QNode* parent);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QLogicalDevice_qt_metacall(Qt3DInput::QLogicalDevice* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_INPUT_C_EXPORT void* qt_3d_input_c_Qt3DInput_QLogicalDevice_qt_metacast(Qt3DInput::QLogicalDevice* this_ptr, const char* arg1);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QLogicalDevice_removeAction(Qt3DInput::QLogicalDevice* this_ptr, Qt3DInput::QAction* action);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QLogicalDevice_removeAxis(Qt3DInput::QLogicalDevice* this_ptr, Qt3DInput::QAxis* axis);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QLogicalDevice_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QLogicalDevice_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_INPUT_C_QLOGICALDEVICE_H
