#ifndef QT_WIDGETS_C_QDIALOGBUTTONBOX_H
#define QT_WIDGETS_C_QDIALOGBUTTONBOX_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QDialogButtonBox* qt_widgets_c_QDialogButtonBox_G_dynamic_cast_QDialogButtonBox_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QDialogButtonBox* qt_widgets_c_QDialogButtonBox_G_static_cast_QDialogButtonBox_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QDialogButtonBox* qt_widgets_c_QDialogButtonBox_G_static_cast_QDialogButtonBox_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QDialogButtonBox* qt_widgets_c_QDialogButtonBox_G_static_cast_QDialogButtonBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QDialogButtonBox_G_static_cast_QObject_ptr(QDialogButtonBox* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QDialogButtonBox_G_static_cast_QPaintDevice_ptr(QDialogButtonBox* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QDialogButtonBox_G_static_cast_QWidget_ptr(QDialogButtonBox* ptr);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QDialogButtonBox_addButton_button(QDialogButtonBox* this_ptr, QDialogButtonBox::StandardButton button);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialogButtonBox_addButton_button_role(QDialogButtonBox* this_ptr, QAbstractButton* button, QDialogButtonBox::ButtonRole role);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QDialogButtonBox_addButton_text_role(QDialogButtonBox* this_ptr, const QString* text, QDialogButtonBox::ButtonRole role);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QDialogButtonBox_button(const QDialogButtonBox* this_ptr, QDialogButtonBox::StandardButton which);
QT_WIDGETS_C_EXPORT QDialogButtonBox::ButtonRole qt_widgets_c_QDialogButtonBox_buttonRole(const QDialogButtonBox* this_ptr, QAbstractButton* button);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialogButtonBox_buttons_to_output(const QDialogButtonBox* this_ptr, QList< QAbstractButton* >* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDialogButtonBox_centerButtons(const QDialogButtonBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialogButtonBox_clear(QDialogButtonBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialogButtonBox_delete(QDialogButtonBox* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QDialogButtonBox_metaObject(const QDialogButtonBox* this_ptr);
QT_WIDGETS_C_EXPORT QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_buttons(unsigned int buttons);
QT_WIDGETS_C_EXPORT QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_buttons_orientation(unsigned int buttons, const Qt::Orientation* orientation);
QT_WIDGETS_C_EXPORT QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_buttons_orientation_parent(unsigned int buttons, const Qt::Orientation* orientation, QWidget* parent);
QT_WIDGETS_C_EXPORT QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_buttons_parent(unsigned int buttons, QWidget* parent);
QT_WIDGETS_C_EXPORT QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_no_args();
QT_WIDGETS_C_EXPORT QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_orientation(const Qt::Orientation* orientation);
QT_WIDGETS_C_EXPORT QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_orientation_parent(const Qt::Orientation* orientation, QWidget* parent);
QT_WIDGETS_C_EXPORT QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDialogButtonBox_qt_metacall(QDialogButtonBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QDialogButtonBox_qt_metacast(QDialogButtonBox* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialogButtonBox_removeButton(QDialogButtonBox* this_ptr, QAbstractButton* button);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialogButtonBox_setCenterButtons(QDialogButtonBox* this_ptr, bool center);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialogButtonBox_setOrientation(QDialogButtonBox* this_ptr, const Qt::Orientation* orientation);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialogButtonBox_setStandardButtons(QDialogButtonBox* this_ptr, unsigned int buttons);
QT_WIDGETS_C_EXPORT QDialogButtonBox::StandardButton qt_widgets_c_QDialogButtonBox_standardButton(const QDialogButtonBox* this_ptr, QAbstractButton* button);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QDialogButtonBox_standardButtons(const QDialogButtonBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialogButtonBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDialogButtonBox_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QDIALOGBUTTONBOX_H
