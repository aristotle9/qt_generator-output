#ifndef QT_GUI_C_QTEXTOBJECT_H
#define QT_GUI_C_QTEXTOBJECT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QObject* qt_gui_c_QTextObject_G_static_cast_QObject_ptr(QTextObject* ptr);
QT_GUI_C_EXPORT QTextObject* qt_gui_c_QTextObject_G_static_cast_QTextObject_ptr(QObject* ptr);
QT_GUI_C_EXPORT QTextDocument* qt_gui_c_QTextObject_document(const QTextObject* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextObject_formatIndex(const QTextObject* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextObject_format_to_output(const QTextObject* this_ptr, QTextFormat* output);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QTextObject_metaObject(const QTextObject* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextObject_objectIndex(const QTextObject* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextObject_qt_metacall(QTextObject* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QTextObject_qt_metacast(QTextObject* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QTextObject_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextObject_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QTEXTOBJECT_H
