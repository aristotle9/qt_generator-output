#ifndef QT_3D_INPUT_C_QKEYEVENT_H
#define QT_3D_INPUT_C_QKEYEVENT_H

#include "qt_3d_input_c_global.h"

extern "C" {

QT_3D_INPUT_C_EXPORT QObject* qt_3d_input_c_QKeyEvent_G_static_cast_QObject_ptr(Qt3DInput::QKeyEvent* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QKeyEvent* qt_3d_input_c_QKeyEvent_G_static_cast_Qt3DInput_QKeyEvent_ptr(QObject* ptr);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QKeyEvent_count(const Qt3DInput::QKeyEvent* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QKeyEvent_delete(Qt3DInput::QKeyEvent* this_ptr);
QT_3D_INPUT_C_EXPORT bool qt_3d_input_c_Qt3DInput_QKeyEvent_isAccepted(const Qt3DInput::QKeyEvent* this_ptr);
QT_3D_INPUT_C_EXPORT bool qt_3d_input_c_Qt3DInput_QKeyEvent_isAutoRepeat(const Qt3DInput::QKeyEvent* this_ptr);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QKeyEvent_key(const Qt3DInput::QKeyEvent* this_ptr);
QT_3D_INPUT_C_EXPORT bool qt_3d_input_c_Qt3DInput_QKeyEvent_matches(const Qt3DInput::QKeyEvent* this_ptr, const QKeySequence::StandardKey* key_);
QT_3D_INPUT_C_EXPORT const QMetaObject* qt_3d_input_c_Qt3DInput_QKeyEvent_metaObject(const Qt3DInput::QKeyEvent* this_ptr);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QKeyEvent_modifiers(const Qt3DInput::QKeyEvent* this_ptr);
QT_3D_INPUT_C_EXPORT quint32 qt_3d_input_c_Qt3DInput_QKeyEvent_nativeScanCode(const Qt3DInput::QKeyEvent* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QKeyEvent* qt_3d_input_c_Qt3DInput_QKeyEvent_new(const QKeyEvent* ke);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QKeyEvent_qt_metacall(Qt3DInput::QKeyEvent* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_INPUT_C_EXPORT void* qt_3d_input_c_Qt3DInput_QKeyEvent_qt_metacast(Qt3DInput::QKeyEvent* this_ptr, const char* arg1);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QKeyEvent_setAccepted(Qt3DInput::QKeyEvent* this_ptr, bool accepted);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QKeyEvent_text_to_output(const Qt3DInput::QKeyEvent* this_ptr, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QKeyEvent_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QKeyEvent_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_INPUT_C_QKEYEVENT_H
