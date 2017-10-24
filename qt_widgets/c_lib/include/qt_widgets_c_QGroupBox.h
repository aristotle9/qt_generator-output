#ifndef QT_WIDGETS_C_QGROUPBOX_H
#define QT_WIDGETS_C_QGROUPBOX_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGroupBox* qt_widgets_c_QGroupBox_G_dynamic_cast_QGroupBox_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QGroupBox* qt_widgets_c_QGroupBox_G_static_cast_QGroupBox_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QGroupBox* qt_widgets_c_QGroupBox_G_static_cast_QGroupBox_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QGroupBox* qt_widgets_c_QGroupBox_G_static_cast_QGroupBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGroupBox_G_static_cast_QObject_ptr(QGroupBox* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QGroupBox_G_static_cast_QPaintDevice_ptr(QGroupBox* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QGroupBox_G_static_cast_QWidget_ptr(QGroupBox* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGroupBox_delete(QGroupBox* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGroupBox_isCheckable(const QGroupBox* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGroupBox_isChecked(const QGroupBox* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGroupBox_isFlat(const QGroupBox* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGroupBox_metaObject(const QGroupBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGroupBox_minimumSizeHint_to_output(const QGroupBox* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QGroupBox* qt_widgets_c_QGroupBox_new_no_args();
QT_WIDGETS_C_EXPORT QGroupBox* qt_widgets_c_QGroupBox_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QGroupBox* qt_widgets_c_QGroupBox_new_title(const QString* title);
QT_WIDGETS_C_EXPORT QGroupBox* qt_widgets_c_QGroupBox_new_title_parent(const QString* title, QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGroupBox_qt_metacall(QGroupBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGroupBox_qt_metacast(QGroupBox* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGroupBox_setAlignment(QGroupBox* this_ptr, int alignment);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGroupBox_setCheckable(QGroupBox* this_ptr, bool checkable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGroupBox_setChecked(QGroupBox* this_ptr, bool checked);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGroupBox_setFlat(QGroupBox* this_ptr, bool flat);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGroupBox_setTitle(QGroupBox* this_ptr, const QString* title);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGroupBox_title_to_output(const QGroupBox* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGroupBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGroupBox_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QGROUPBOX_H
