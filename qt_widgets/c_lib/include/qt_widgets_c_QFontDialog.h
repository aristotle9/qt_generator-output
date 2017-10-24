#ifndef QT_WIDGETS_C_QFONTDIALOG_H
#define QT_WIDGETS_C_QFONTDIALOG_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QFontDialog* qt_widgets_c_QFontDialog_G_dynamic_cast_QFontDialog_ptr_QDialog(QDialog* ptr);
QT_WIDGETS_C_EXPORT QFontDialog* qt_widgets_c_QFontDialog_G_dynamic_cast_QFontDialog_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QDialog* qt_widgets_c_QFontDialog_G_static_cast_QDialog_ptr(QFontDialog* ptr);
QT_WIDGETS_C_EXPORT QFontDialog* qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QDialog(QDialog* ptr);
QT_WIDGETS_C_EXPORT QFontDialog* qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QFontDialog* qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QFontDialog* qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QFontDialog_G_static_cast_QObject_ptr(QFontDialog* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QFontDialog_G_static_cast_QPaintDevice_ptr(QFontDialog* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QFontDialog_G_static_cast_QWidget_ptr(QFontDialog* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_currentFont_to_output(const QFontDialog* this_ptr, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_delete(QFontDialog* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_getFont_to_output_ok(bool* ok, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_getFont_to_output_ok_initial(bool* ok, const QFont* initial, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_getFont_to_output_ok_initial_parent(bool* ok, const QFont* initial, QWidget* parent, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_getFont_to_output_ok_initial_parent_title(bool* ok, const QFont* initial, QWidget* parent, const QString* title, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_getFont_to_output_ok_initial_parent_title_options(bool* ok, const QFont* initial, QWidget* parent, const QString* title, unsigned int options, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_getFont_to_output_ok_parent(bool* ok, QWidget* parent, QFont* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QFontDialog_metaObject(const QFontDialog* this_ptr);
QT_WIDGETS_C_EXPORT QFontDialog* qt_widgets_c_QFontDialog_new_initial(const QFont* initial);
QT_WIDGETS_C_EXPORT QFontDialog* qt_widgets_c_QFontDialog_new_initial_parent(const QFont* initial, QWidget* parent);
QT_WIDGETS_C_EXPORT QFontDialog* qt_widgets_c_QFontDialog_new_no_args();
QT_WIDGETS_C_EXPORT QFontDialog* qt_widgets_c_QFontDialog_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_open(QFontDialog* this_ptr, QObject* receiver, const char* member);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QFontDialog_options(const QFontDialog* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFontDialog_qt_metacall(QFontDialog* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QFontDialog_qt_metacast(QFontDialog* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_selectedFont_to_output(const QFontDialog* this_ptr, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_setCurrentFont(QFontDialog* this_ptr, const QFont* font);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_setOption_option(QFontDialog* this_ptr, QFontDialog::FontDialogOption option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_setOption_option_on(QFontDialog* this_ptr, QFontDialog::FontDialogOption option, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_setOptions(QFontDialog* this_ptr, unsigned int options);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_setVisible(QFontDialog* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QFontDialog_testOption(const QFontDialog* this_ptr, QFontDialog::FontDialogOption option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontDialog_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QFONTDIALOG_H
