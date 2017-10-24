#ifndef QT_WIDGETS_C_QPUSHBUTTON_H
#define QT_WIDGETS_C_QPUSHBUTTON_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QPushButton_G_dynamic_cast_QPushButton_ptr_QAbstractButton(QAbstractButton* ptr);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QPushButton_G_dynamic_cast_QPushButton_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractButton* qt_widgets_c_QPushButton_G_static_cast_QAbstractButton_ptr(QPushButton* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QPushButton_G_static_cast_QObject_ptr(QPushButton* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QPushButton_G_static_cast_QPaintDevice_ptr(QPushButton* ptr);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QAbstractButton(QAbstractButton* ptr);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QPushButton_G_static_cast_QWidget_ptr(QPushButton* ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QPushButton_autoDefault(const QPushButton* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPushButton_delete(QPushButton* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QPushButton_isDefault(const QPushButton* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QPushButton_isFlat(const QPushButton* this_ptr);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QPushButton_menu(const QPushButton* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QPushButton_metaObject(const QPushButton* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPushButton_minimumSizeHint_to_output(const QPushButton* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QPushButton_new_icon_text(const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QPushButton_new_icon_text_parent(const QIcon* icon, const QString* text, QWidget* parent);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QPushButton_new_no_args();
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QPushButton_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QPushButton_new_text(const QString* text);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QPushButton_new_text_parent(const QString* text, QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QPushButton_qt_metacall(QPushButton* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QPushButton_qt_metacast(QPushButton* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPushButton_setAutoDefault(QPushButton* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPushButton_setDefault(QPushButton* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPushButton_setFlat(QPushButton* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPushButton_setMenu(QPushButton* this_ptr, QMenu* menu);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPushButton_showMenu(QPushButton* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPushButton_sizeHint_to_output(const QPushButton* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPushButton_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QPushButton_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QPUSHBUTTON_H
