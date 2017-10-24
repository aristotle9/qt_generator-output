#ifndef QT_GUI_C_QSYNTAXHIGHLIGHTER_H
#define QT_GUI_C_QSYNTAXHIGHLIGHTER_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QObject* qt_gui_c_QSyntaxHighlighter_G_static_cast_QObject_ptr(QSyntaxHighlighter* ptr);
QT_GUI_C_EXPORT QSyntaxHighlighter* qt_gui_c_QSyntaxHighlighter_G_static_cast_QSyntaxHighlighter_ptr(QObject* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSyntaxHighlighter_delete(QSyntaxHighlighter* this_ptr);
QT_GUI_C_EXPORT QTextDocument* qt_gui_c_QSyntaxHighlighter_document(const QSyntaxHighlighter* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QSyntaxHighlighter_metaObject(const QSyntaxHighlighter* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QSyntaxHighlighter_qt_metacall(QSyntaxHighlighter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QSyntaxHighlighter_qt_metacast(QSyntaxHighlighter* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QSyntaxHighlighter_rehighlight(QSyntaxHighlighter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSyntaxHighlighter_rehighlightBlock(QSyntaxHighlighter* this_ptr, const QTextBlock* block);
QT_GUI_C_EXPORT void qt_gui_c_QSyntaxHighlighter_setDocument(QSyntaxHighlighter* this_ptr, QTextDocument* doc);
QT_GUI_C_EXPORT void qt_gui_c_QSyntaxHighlighter_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QSyntaxHighlighter_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QSYNTAXHIGHLIGHTER_H
