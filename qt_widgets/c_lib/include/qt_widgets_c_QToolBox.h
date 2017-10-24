#ifndef QT_WIDGETS_C_QTOOLBOX_H
#define QT_WIDGETS_C_QTOOLBOX_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QToolBox* qt_widgets_c_QToolBox_G_dynamic_cast_QToolBox_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QToolBox* qt_widgets_c_QToolBox_G_dynamic_cast_QToolBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QToolBox_G_static_cast_QFrame_ptr(QToolBox* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QToolBox_G_static_cast_QObject_ptr(QToolBox* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QToolBox_G_static_cast_QPaintDevice_ptr(QToolBox* ptr);
QT_WIDGETS_C_EXPORT QToolBox* qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QToolBox* qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QToolBox* qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QToolBox* qt_widgets_c_QToolBox_G_static_cast_QToolBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QToolBox_G_static_cast_QWidget_ptr(QToolBox* ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QToolBox_addItem_widget_icon_text(QToolBox* this_ptr, QWidget* widget, const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QToolBox_addItem_widget_text(QToolBox* this_ptr, QWidget* widget, const QString* text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QToolBox_count(const QToolBox* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QToolBox_currentIndex(const QToolBox* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QToolBox_currentWidget(const QToolBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBox_delete(QToolBox* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QToolBox_indexOf(const QToolBox* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QToolBox_insertItem_index_widget_icon_text(QToolBox* this_ptr, int index, QWidget* widget, const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QToolBox_insertItem_index_widget_text(QToolBox* this_ptr, int index, QWidget* widget, const QString* text);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QToolBox_isItemEnabled(const QToolBox* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBox_itemIcon_to_output(const QToolBox* this_ptr, int index, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBox_itemText_to_output(const QToolBox* this_ptr, int index, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBox_itemToolTip_to_output(const QToolBox* this_ptr, int index, QString* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QToolBox_metaObject(const QToolBox* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QToolBox_qt_metacall(QToolBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QToolBox_qt_metacast(QToolBox* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBox_removeItem(QToolBox* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBox_setCurrentIndex(QToolBox* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBox_setCurrentWidget(QToolBox* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBox_setItemEnabled(QToolBox* this_ptr, int index, bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBox_setItemIcon(QToolBox* this_ptr, int index, const QIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBox_setItemText(QToolBox* this_ptr, int index, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBox_setItemToolTip(QToolBox* this_ptr, int index, const QString* toolTip);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QToolBox_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QToolBox_widget(const QToolBox* this_ptr, int index);

} // extern "C"

#endif // QT_WIDGETS_C_QTOOLBOX_H
