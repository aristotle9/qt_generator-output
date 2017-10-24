#ifndef QT_WIDGETS_C_QERRORMESSAGE_H
#define QT_WIDGETS_C_QERRORMESSAGE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QErrorMessage* qt_widgets_c_QErrorMessage_G_dynamic_cast_QErrorMessage_ptr_QDialog(QDialog* ptr);
QT_WIDGETS_C_EXPORT QErrorMessage* qt_widgets_c_QErrorMessage_G_dynamic_cast_QErrorMessage_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QDialog* qt_widgets_c_QErrorMessage_G_static_cast_QDialog_ptr(QErrorMessage* ptr);
QT_WIDGETS_C_EXPORT QErrorMessage* qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QDialog(QDialog* ptr);
QT_WIDGETS_C_EXPORT QErrorMessage* qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QErrorMessage* qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QErrorMessage* qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QErrorMessage_G_static_cast_QObject_ptr(QErrorMessage* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QErrorMessage_G_static_cast_QPaintDevice_ptr(QErrorMessage* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QErrorMessage_G_static_cast_QWidget_ptr(QErrorMessage* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QErrorMessage_delete(QErrorMessage* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QErrorMessage_metaObject(const QErrorMessage* this_ptr);
QT_WIDGETS_C_EXPORT QErrorMessage* qt_widgets_c_QErrorMessage_new_no_args();
QT_WIDGETS_C_EXPORT QErrorMessage* qt_widgets_c_QErrorMessage_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QErrorMessage* qt_widgets_c_QErrorMessage_qtHandler();
QT_WIDGETS_C_EXPORT int qt_widgets_c_QErrorMessage_qt_metacall(QErrorMessage* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QErrorMessage_qt_metacast(QErrorMessage* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QErrorMessage_showMessage_message(QErrorMessage* this_ptr, const QString* message);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QErrorMessage_showMessage_message_type(QErrorMessage* this_ptr, const QString* message, const QString* type);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QErrorMessage_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QErrorMessage_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QERRORMESSAGE_H
