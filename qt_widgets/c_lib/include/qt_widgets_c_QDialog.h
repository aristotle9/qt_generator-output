#ifndef QT_WIDGETS_C_QDIALOG_H
#define QT_WIDGETS_C_QDIALOG_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QDialog* qt_widgets_c_QDialog_G_dynamic_cast_QDialog_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QDialog* qt_widgets_c_QDialog_G_static_cast_QDialog_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QDialog* qt_widgets_c_QDialog_G_static_cast_QDialog_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QDialog* qt_widgets_c_QDialog_G_static_cast_QDialog_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QDialog_G_static_cast_QObject_ptr(QDialog* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QDialog_G_static_cast_QPaintDevice_ptr(QDialog* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QDialog_G_static_cast_QWidget_ptr(QDialog* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_accept(QDialog* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_delete(QDialog* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_done(QDialog* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDialog_exec(QDialog* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QDialog_extension(const QDialog* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDialog_isSizeGripEnabled(const QDialog* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QDialog_metaObject(const QDialog* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_minimumSizeHint_to_output(const QDialog* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_open(QDialog* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDialog_qt_metacall(QDialog* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QDialog_qt_metacast(QDialog* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_reject(QDialog* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDialog_result(const QDialog* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_setExtension(QDialog* this_ptr, QWidget* extension);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_setModal(QDialog* this_ptr, bool modal);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_setOrientation(QDialog* this_ptr, const Qt::Orientation* orientation);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_setResult(QDialog* this_ptr, int r);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_setSizeGripEnabled(QDialog* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_setVisible(QDialog* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_showExtension(QDialog* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_sizeHint_to_output(const QDialog* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialog_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QDIALOG_H
