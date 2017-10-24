#ifndef QT_GUI_C_QKEYEVENT_H
#define QT_GUI_C_QKEYEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QKeyEvent* qt_gui_c_QKeyEvent_G_dynamic_cast_QKeyEvent_ptr(QInputEvent* ptr);
QT_GUI_C_EXPORT QEvent* qt_gui_c_QKeyEvent_G_static_cast_QEvent_ptr(QKeyEvent* ptr);
QT_GUI_C_EXPORT QInputEvent* qt_gui_c_QKeyEvent_G_static_cast_QInputEvent_ptr(QKeyEvent* ptr);
QT_GUI_C_EXPORT QKeyEvent* qt_gui_c_QKeyEvent_G_static_cast_QKeyEvent_ptr_QEvent(QEvent* ptr);
QT_GUI_C_EXPORT QKeyEvent* qt_gui_c_QKeyEvent_G_static_cast_QKeyEvent_ptr_QInputEvent(QInputEvent* ptr);
QT_GUI_C_EXPORT int qt_gui_c_QKeyEvent_count(const QKeyEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QKeyEvent_delete(QKeyEvent* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QKeyEvent_isAutoRepeat(const QKeyEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QKeyEvent_key(const QKeyEvent* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QKeyEvent_matches(const QKeyEvent* this_ptr, const QKeySequence::StandardKey* key);
QT_GUI_C_EXPORT quint32 qt_gui_c_QKeyEvent_nativeModifiers(const QKeyEvent* this_ptr);
QT_GUI_C_EXPORT quint32 qt_gui_c_QKeyEvent_nativeScanCode(const QKeyEvent* this_ptr);
QT_GUI_C_EXPORT quint32 qt_gui_c_QKeyEvent_nativeVirtualKey(const QKeyEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QKeyEvent_text_to_output(const QKeyEvent* this_ptr, QString* output);

} // extern "C"

#endif // QT_GUI_C_QKEYEVENT_H
