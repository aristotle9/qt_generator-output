#ifndef QT_GUI_C_QTEXTLIST_H
#define QT_GUI_C_QTEXTLIST_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QTextList* qt_gui_c_QTextList_G_dynamic_cast_QTextList_ptr_QTextBlockGroup(QTextBlockGroup* ptr);
QT_GUI_C_EXPORT QTextList* qt_gui_c_QTextList_G_dynamic_cast_QTextList_ptr_QTextObject(QTextObject* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QTextList_G_static_cast_QObject_ptr(QTextList* ptr);
QT_GUI_C_EXPORT QTextBlockGroup* qt_gui_c_QTextList_G_static_cast_QTextBlockGroup_ptr(QTextList* ptr);
QT_GUI_C_EXPORT QTextList* qt_gui_c_QTextList_G_static_cast_QTextList_ptr_QObject(QObject* ptr);
QT_GUI_C_EXPORT QTextList* qt_gui_c_QTextList_G_static_cast_QTextList_ptr_QTextBlockGroup(QTextBlockGroup* ptr);
QT_GUI_C_EXPORT QTextList* qt_gui_c_QTextList_G_static_cast_QTextList_ptr_QTextObject(QTextObject* ptr);
QT_GUI_C_EXPORT QTextObject* qt_gui_c_QTextList_G_static_cast_QTextObject_ptr(QTextList* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextList_add(QTextList* this_ptr, const QTextBlock* block);
QT_GUI_C_EXPORT int qt_gui_c_QTextList_count(const QTextList* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextList_delete(QTextList* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextList_format_to_output(const QTextList* this_ptr, QTextListFormat* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextList_isEmpty(const QTextList* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextList_itemNumber(const QTextList* this_ptr, const QTextBlock* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QTextList_itemText_to_output(const QTextList* this_ptr, const QTextBlock* arg1, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextList_item_to_output(const QTextList* this_ptr, int i, QTextBlock* output);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QTextList_metaObject(const QTextList* this_ptr);
QT_GUI_C_EXPORT QTextList* qt_gui_c_QTextList_new(QTextDocument* doc);
QT_GUI_C_EXPORT int qt_gui_c_QTextList_qt_metacall(QTextList* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QTextList_qt_metacast(QTextList* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QTextList_remove(QTextList* this_ptr, const QTextBlock* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QTextList_removeItem(QTextList* this_ptr, int i);
QT_GUI_C_EXPORT void qt_gui_c_QTextList_setFormat(QTextList* this_ptr, const QTextListFormat* format);
QT_GUI_C_EXPORT void qt_gui_c_QTextList_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextList_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QTEXTLIST_H
