#ifndef QT_WIDGETS_C_QCHECKBOX_H
#define QT_WIDGETS_C_QCHECKBOX_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QCheckBox* qt_widgets_c_QCheckBox_G_dynamic_cast_QCheckBox_ptr_QAbstractButton(QAbstractButton* ptr);
QT_WIDGETS_C_EXPORT QCheckBox* qt_widgets_c_QCheckBox_G_dynamic_cast_QCheckBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractButton* qt_widgets_c_QCheckBox_G_static_cast_QAbstractButton_ptr(QCheckBox* ptr);
QT_WIDGETS_C_EXPORT QCheckBox* qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QAbstractButton(QAbstractButton* ptr);
QT_WIDGETS_C_EXPORT QCheckBox* qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QCheckBox* qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QCheckBox* qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QCheckBox_G_static_cast_QObject_ptr(QCheckBox* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QCheckBox_G_static_cast_QPaintDevice_ptr(QCheckBox* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QCheckBox_G_static_cast_QWidget_ptr(QCheckBox* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCheckBox_delete(QCheckBox* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QCheckBox_isTristate(const QCheckBox* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QCheckBox_metaObject(const QCheckBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCheckBox_minimumSizeHint_to_output(const QCheckBox* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QCheckBox* qt_widgets_c_QCheckBox_new_no_args();
QT_WIDGETS_C_EXPORT QCheckBox* qt_widgets_c_QCheckBox_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QCheckBox* qt_widgets_c_QCheckBox_new_text(const QString* text);
QT_WIDGETS_C_EXPORT QCheckBox* qt_widgets_c_QCheckBox_new_text_parent(const QString* text, QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCheckBox_qt_metacall(QCheckBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QCheckBox_qt_metacast(QCheckBox* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCheckBox_setCheckState(QCheckBox* this_ptr, const Qt::CheckState* state);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCheckBox_setTristate_no_args(QCheckBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCheckBox_setTristate_y(QCheckBox* this_ptr, bool y);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCheckBox_sizeHint_to_output(const QCheckBox* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCheckBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCheckBox_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QCHECKBOX_H
