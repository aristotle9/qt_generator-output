#ifndef QT_3D_INPUT_C_QINPUTCHORD_H
#define QT_3D_INPUT_C_QINPUTCHORD_H

#include "qt_3d_input_c_global.h"

extern "C" {

QT_3D_INPUT_C_EXPORT Qt3DInput::QInputChord* qt_3d_input_c_QInputChord_G_dynamic_cast_Qt3DInput_QInputChord_ptr(Qt3DInput::QAbstractActionInput* ptr);
QT_3D_INPUT_C_EXPORT QObject* qt_3d_input_c_QInputChord_G_static_cast_QObject_ptr(Qt3DInput::QInputChord* ptr);
QT_3D_INPUT_C_EXPORT Qt3DCore::QNode* qt_3d_input_c_QInputChord_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QInputChord* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QAbstractActionInput* qt_3d_input_c_QInputChord_G_static_cast_Qt3DInput_QAbstractActionInput_ptr(Qt3DInput::QInputChord* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QInputChord* qt_3d_input_c_QInputChord_G_static_cast_Qt3DInput_QInputChord_ptr_QObject(QObject* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QInputChord* qt_3d_input_c_QInputChord_G_static_cast_Qt3DInput_QInputChord_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QInputChord* qt_3d_input_c_QInputChord_G_static_cast_Qt3DInput_QInputChord_ptr_Qt3DInput_QAbstractActionInput(Qt3DInput::QAbstractActionInput* ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QInputChord_addChord(Qt3DInput::QInputChord* this_ptr, Qt3DInput::QAbstractActionInput* input);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QInputChord_chords_to_output(const Qt3DInput::QInputChord* this_ptr, QVector< Qt3DInput::QAbstractActionInput* >* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QInputChord_delete(Qt3DInput::QInputChord* this_ptr);
QT_3D_INPUT_C_EXPORT const QMetaObject* qt_3d_input_c_Qt3DInput_QInputChord_metaObject(const Qt3DInput::QInputChord* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QInputChord* qt_3d_input_c_Qt3DInput_QInputChord_new_no_args();
QT_3D_INPUT_C_EXPORT Qt3DInput::QInputChord* qt_3d_input_c_Qt3DInput_QInputChord_new_parent(Qt3DCore::QNode* parent);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QInputChord_qt_metacall(Qt3DInput::QInputChord* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_INPUT_C_EXPORT void* qt_3d_input_c_Qt3DInput_QInputChord_qt_metacast(Qt3DInput::QInputChord* this_ptr, const char* arg1);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QInputChord_removeChord(Qt3DInput::QInputChord* this_ptr, Qt3DInput::QAbstractActionInput* input);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QInputChord_setTimeout(Qt3DInput::QInputChord* this_ptr, int timeout);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QInputChord_timeout(const Qt3DInput::QInputChord* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QInputChord_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QInputChord_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_INPUT_C_QINPUTCHORD_H
