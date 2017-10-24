#ifndef QT_3D_INPUT_C_QAXISACCUMULATOR_H
#define QT_3D_INPUT_C_QAXISACCUMULATOR_H

#include "qt_3d_input_c_global.h"

extern "C" {

QT_3D_INPUT_C_EXPORT QObject* qt_3d_input_c_QAxisAccumulator_G_static_cast_QObject_ptr(Qt3DInput::QAxisAccumulator* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QComponent* qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DInput::QAxisAccumulator* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QNode* qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QAxisAccumulator* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxisAccumulator* qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DInput_QAxisAccumulator_ptr_QObject(QObject* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxisAccumulator* qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DInput_QAxisAccumulator_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxisAccumulator* qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DInput_QAxisAccumulator_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxisAccumulator_delete(Qt3DInput::QAxisAccumulator* this_ptr);
QT_3D_INPUT_C_EXPORT const QMetaObject* qt_3d_input_c_Qt3DInput_QAxisAccumulator_metaObject(const Qt3DInput::QAxisAccumulator* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxisAccumulator* qt_3d_input_c_Qt3DInput_QAxisAccumulator_new_no_args();
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxisAccumulator* qt_3d_input_c_Qt3DInput_QAxisAccumulator_new_parent(Qt3DCore::QNode* parent);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QAxisAccumulator_qt_metacall(Qt3DInput::QAxisAccumulator* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_INPUT_C_EXPORT void* qt_3d_input_c_Qt3DInput_QAxisAccumulator_qt_metacast(Qt3DInput::QAxisAccumulator* this_ptr, const char* arg1);
QT_3D_INPUT_C_EXPORT float qt_3d_input_c_Qt3DInput_QAxisAccumulator_scale(const Qt3DInput::QAxisAccumulator* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxisAccumulator_setScale(Qt3DInput::QAxisAccumulator* this_ptr, float scale);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxisAccumulator_setSourceAxis(Qt3DInput::QAxisAccumulator* this_ptr, Qt3DInput::QAxis* sourceAxis);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxisAccumulator_setSourceAxisType(Qt3DInput::QAxisAccumulator* this_ptr, const Qt3DInput::QAxisAccumulator::SourceAxisType* sourceAxisType);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxis* qt_3d_input_c_Qt3DInput_QAxisAccumulator_sourceAxis(const Qt3DInput::QAxisAccumulator* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAxisAccumulator::SourceAxisType qt_3d_input_c_Qt3DInput_QAxisAccumulator_sourceAxisType(const Qt3DInput::QAxisAccumulator* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxisAccumulator_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QAxisAccumulator_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT float qt_3d_input_c_Qt3DInput_QAxisAccumulator_value(const Qt3DInput::QAxisAccumulator* this_ptr);
QT_3D_INPUT_C_EXPORT float qt_3d_input_c_Qt3DInput_QAxisAccumulator_velocity(const Qt3DInput::QAxisAccumulator* this_ptr);

} // extern "C"

#endif // QT_3D_INPUT_C_QAXISACCUMULATOR_H
