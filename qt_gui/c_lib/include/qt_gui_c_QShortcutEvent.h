#ifndef QT_GUI_C_QSHORTCUTEVENT_H
#define QT_GUI_C_QSHORTCUTEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QShortcutEvent_G_static_cast_QEvent_ptr(QShortcutEvent* ptr);
QT_GUI_C_EXPORT QShortcutEvent* qt_gui_c_QShortcutEvent_G_static_cast_QShortcutEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QShortcutEvent_delete(QShortcutEvent* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QShortcutEvent_isAmbiguous(const QShortcutEvent* this_ptr);
QT_GUI_C_EXPORT const QKeySequence* qt_gui_c_QShortcutEvent_key(const QShortcutEvent* this_ptr);
QT_GUI_C_EXPORT QShortcutEvent* qt_gui_c_QShortcutEvent_new_key_id(const QKeySequence* key, int id);
QT_GUI_C_EXPORT QShortcutEvent* qt_gui_c_QShortcutEvent_new_key_id_ambiguous(const QKeySequence* key, int id, bool ambiguous);
QT_GUI_C_EXPORT int qt_gui_c_QShortcutEvent_shortcutId(const QShortcutEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QSHORTCUTEVENT_H
