#ifndef QT_WIDGETS_C_QSHORTCUT_H
#define QT_WIDGETS_C_QSHORTCUT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QShortcut_G_static_cast_QObject_ptr(QShortcut* ptr);
QT_WIDGETS_C_EXPORT QShortcut* qt_widgets_c_QShortcut_G_static_cast_QShortcut_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QShortcut_autoRepeat(const QShortcut* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QShortcut_delete(QShortcut* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QShortcut_id(const QShortcut* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QShortcut_isEnabled(const QShortcut* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QShortcut_key_to_output(const QShortcut* this_ptr, QKeySequence* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QShortcut_metaObject(const QShortcut* this_ptr);
QT_WIDGETS_C_EXPORT QShortcut* qt_widgets_c_QShortcut_new_key_parent(const QKeySequence* key, QWidget* parent);
QT_WIDGETS_C_EXPORT QShortcut* qt_widgets_c_QShortcut_new_key_parent_member(const QKeySequence* key, QWidget* parent, const char* member);
QT_WIDGETS_C_EXPORT QShortcut* qt_widgets_c_QShortcut_new_key_parent_member_ambiguousMember(const QKeySequence* key, QWidget* parent, const char* member, const char* ambiguousMember);
QT_WIDGETS_C_EXPORT QShortcut* qt_widgets_c_QShortcut_new_key_parent_member_ambiguousMember_context(const QKeySequence* key, QWidget* parent, const char* member, const char* ambiguousMember, const Qt::ShortcutContext* context);
QT_WIDGETS_C_EXPORT QShortcut* qt_widgets_c_QShortcut_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QShortcut_parentWidget(const QShortcut* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QShortcut_qt_metacall(QShortcut* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QShortcut_qt_metacast(QShortcut* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QShortcut_setAutoRepeat(QShortcut* this_ptr, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QShortcut_setContext(QShortcut* this_ptr, const Qt::ShortcutContext* context);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QShortcut_setEnabled(QShortcut* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QShortcut_setKey(QShortcut* this_ptr, const QKeySequence* key);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QShortcut_setWhatsThis(QShortcut* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QShortcut_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QShortcut_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QShortcut_whatsThis_to_output(const QShortcut* this_ptr, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QSHORTCUT_H
