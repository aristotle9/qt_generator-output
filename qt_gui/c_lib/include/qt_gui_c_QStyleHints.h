#ifndef QT_GUI_C_QSTYLEHINTS_H
#define QT_GUI_C_QSTYLEHINTS_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QObject* qt_gui_c_QStyleHints_G_static_cast_QObject_ptr(QStyleHints* ptr);
QT_GUI_C_EXPORT QStyleHints* qt_gui_c_QStyleHints_G_static_cast_QStyleHints_ptr(QObject* ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStyleHints_cursorFlashTime(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStyleHints_delete(QStyleHints* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QStyleHints_fontSmoothingGamma(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStyleHints_keyboardAutoRepeatRate(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStyleHints_keyboardInputInterval(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QStyleHints_metaObject(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStyleHints_mouseDoubleClickInterval(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStyleHints_mousePressAndHoldInterval(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStyleHints_passwordMaskCharacter_to_output(const QStyleHints* this_ptr, QChar* output);
QT_GUI_C_EXPORT int qt_gui_c_QStyleHints_passwordMaskDelay(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStyleHints_qt_metacall(QStyleHints* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QStyleHints_qt_metacast(QStyleHints* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QStyleHints_setCursorFlashTime(QStyleHints* this_ptr, int cursorFlashTime);
QT_GUI_C_EXPORT bool qt_gui_c_QStyleHints_setFocusOnTouchRelease(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStyleHints_setKeyboardInputInterval(QStyleHints* this_ptr, int keyboardInputInterval);
QT_GUI_C_EXPORT void qt_gui_c_QStyleHints_setMouseDoubleClickInterval(QStyleHints* this_ptr, int mouseDoubleClickInterval);
QT_GUI_C_EXPORT void qt_gui_c_QStyleHints_setMousePressAndHoldInterval(QStyleHints* this_ptr, int mousePressAndHoldInterval);
QT_GUI_C_EXPORT void qt_gui_c_QStyleHints_setStartDragDistance(QStyleHints* this_ptr, int startDragDistance);
QT_GUI_C_EXPORT void qt_gui_c_QStyleHints_setStartDragTime(QStyleHints* this_ptr, int startDragTime);
QT_GUI_C_EXPORT void qt_gui_c_QStyleHints_setTabFocusBehavior(QStyleHints* this_ptr, const Qt::TabFocusBehavior* tabFocusBehavior);
QT_GUI_C_EXPORT void qt_gui_c_QStyleHints_setUseHoverEffects(QStyleHints* this_ptr, bool useHoverEffects);
QT_GUI_C_EXPORT void qt_gui_c_QStyleHints_setWheelScrollLines(QStyleHints* this_ptr, int scrollLines);
QT_GUI_C_EXPORT bool qt_gui_c_QStyleHints_showIsFullScreen(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QStyleHints_showIsMaximized(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QStyleHints_singleClickActivation(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStyleHints_startDragDistance(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStyleHints_startDragTime(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStyleHints_startDragVelocity(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStyleHints_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QStyleHints_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT bool qt_gui_c_QStyleHints_useHoverEffects(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QStyleHints_useRtlExtensions(const QStyleHints* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QStyleHints_wheelScrollLines(const QStyleHints* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QSTYLEHINTS_H
