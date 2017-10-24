#ifndef QT_3D_INPUT_C_QMOUSEEVENT_H
#define QT_3D_INPUT_C_QMOUSEEVENT_H

#include "qt_3d_input_c_global.h"

extern "C" {

QT_3D_INPUT_C_EXPORT QObject* qt_3d_input_c_QMouseEvent_G_static_cast_QObject_ptr(Qt3DInput::QMouseEvent* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseEvent* qt_3d_input_c_QMouseEvent_G_static_cast_Qt3DInput_QMouseEvent_ptr(QObject* ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseEvent::Buttons qt_3d_input_c_Qt3DInput_QMouseEvent_button(const Qt3DInput::QMouseEvent* this_ptr);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QMouseEvent_buttons(const Qt3DInput::QMouseEvent* this_ptr);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QMouseEvent_delete(Qt3DInput::QMouseEvent* this_ptr);
QT_3D_INPUT_C_EXPORT bool qt_3d_input_c_Qt3DInput_QMouseEvent_isAccepted(const Qt3DInput::QMouseEvent* this_ptr);
QT_3D_INPUT_C_EXPORT const QMetaObject* qt_3d_input_c_Qt3DInput_QMouseEvent_metaObject(const Qt3DInput::QMouseEvent* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseEvent::Modifiers qt_3d_input_c_Qt3DInput_QMouseEvent_modifiers(const Qt3DInput::QMouseEvent* this_ptr);
QT_3D_INPUT_C_EXPORT Qt3DInput::QMouseEvent* qt_3d_input_c_Qt3DInput_QMouseEvent_new(const QMouseEvent* e);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QMouseEvent_qt_metacall(Qt3DInput::QMouseEvent* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_INPUT_C_EXPORT void* qt_3d_input_c_Qt3DInput_QMouseEvent_qt_metacast(Qt3DInput::QMouseEvent* this_ptr, const char* arg1);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QMouseEvent_setAccepted(Qt3DInput::QMouseEvent* this_ptr, bool accepted);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QMouseEvent_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT void qt_3d_input_c_Qt3DInput_QMouseEvent_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_INPUT_C_EXPORT bool qt_3d_input_c_Qt3DInput_QMouseEvent_wasHeld(const Qt3DInput::QMouseEvent* this_ptr);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QMouseEvent_x(const Qt3DInput::QMouseEvent* this_ptr);
QT_3D_INPUT_C_EXPORT int qt_3d_input_c_Qt3DInput_QMouseEvent_y(const Qt3DInput::QMouseEvent* this_ptr);

} // extern "C"

#endif // QT_3D_INPUT_C_QMOUSEEVENT_H
