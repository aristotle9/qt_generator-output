#ifndef QT_WIDGETS_C_QCOLORDIALOG_H
#define QT_WIDGETS_C_QCOLORDIALOG_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QColorDialog* qt_widgets_c_QColorDialog_G_dynamic_cast_QColorDialog_ptr_QDialog(QDialog* ptr);
QT_WIDGETS_C_EXPORT QColorDialog* qt_widgets_c_QColorDialog_G_dynamic_cast_QColorDialog_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QColorDialog* qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QDialog(QDialog* ptr);
QT_WIDGETS_C_EXPORT QColorDialog* qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QColorDialog* qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QColorDialog* qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QDialog* qt_widgets_c_QColorDialog_G_static_cast_QDialog_ptr(QColorDialog* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QColorDialog_G_static_cast_QObject_ptr(QColorDialog* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QColorDialog_G_static_cast_QPaintDevice_ptr(QColorDialog* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QColorDialog_G_static_cast_QWidget_ptr(QColorDialog* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_currentColor_to_output(const QColorDialog* this_ptr, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_customColor_to_output(int index, QColor* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QColorDialog_customCount();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_delete(QColorDialog* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_getColor_to_output_initial(const QColor* initial, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_getColor_to_output_initial_parent(const QColor* initial, QWidget* parent, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_getColor_to_output_initial_parent_title(const QColor* initial, QWidget* parent, const QString* title, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_getColor_to_output_initial_parent_title_options(const QColor* initial, QWidget* parent, const QString* title, unsigned int options, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_getColor_to_output_no_args(QColor* output);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QColorDialog_getRgba_no_args();
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QColorDialog_getRgba_rgba(unsigned int rgba);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QColorDialog_getRgba_rgba_ok(unsigned int rgba, bool* ok);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QColorDialog_getRgba_rgba_ok_parent(unsigned int rgba, bool* ok, QWidget* parent);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QColorDialog_metaObject(const QColorDialog* this_ptr);
QT_WIDGETS_C_EXPORT QColorDialog* qt_widgets_c_QColorDialog_new_initial(const QColor* initial);
QT_WIDGETS_C_EXPORT QColorDialog* qt_widgets_c_QColorDialog_new_initial_parent(const QColor* initial, QWidget* parent);
QT_WIDGETS_C_EXPORT QColorDialog* qt_widgets_c_QColorDialog_new_no_args();
QT_WIDGETS_C_EXPORT QColorDialog* qt_widgets_c_QColorDialog_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_open(QColorDialog* this_ptr, QObject* receiver, const char* member);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QColorDialog_options(const QColorDialog* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QColorDialog_qt_metacall(QColorDialog* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QColorDialog_qt_metacast(QColorDialog* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_selectedColor_to_output(const QColorDialog* this_ptr, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_setCurrentColor(QColorDialog* this_ptr, const QColor* color);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_setCustomColor(int index, const QColor* color);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_setOption_option(QColorDialog* this_ptr, QColorDialog::ColorDialogOption option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_setOption_option_on(QColorDialog* this_ptr, QColorDialog::ColorDialogOption option, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_setOptions(QColorDialog* this_ptr, unsigned int options);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_setStandardColor(int index, const QColor* color);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_setVisible(QColorDialog* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_standardColor_to_output(int index, QColor* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QColorDialog_testOption(const QColorDialog* this_ptr, QColorDialog::ColorDialogOption option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColorDialog_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QCOLORDIALOG_H
