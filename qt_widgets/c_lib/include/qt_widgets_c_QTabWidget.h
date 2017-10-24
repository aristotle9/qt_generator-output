#ifndef QT_WIDGETS_C_QTABWIDGET_H
#define QT_WIDGETS_C_QTABWIDGET_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QTabWidget* qt_widgets_c_QTabWidget_G_dynamic_cast_QTabWidget_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QTabWidget_G_static_cast_QObject_ptr(QTabWidget* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QTabWidget_G_static_cast_QPaintDevice_ptr(QTabWidget* ptr);
QT_WIDGETS_C_EXPORT QTabWidget* qt_widgets_c_QTabWidget_G_static_cast_QTabWidget_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QTabWidget* qt_widgets_c_QTabWidget_G_static_cast_QTabWidget_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QTabWidget* qt_widgets_c_QTabWidget_G_static_cast_QTabWidget_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QTabWidget_G_static_cast_QWidget_ptr(QTabWidget* ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabWidget_addTab_widget_arg2(QTabWidget* this_ptr, QWidget* widget, const QString* arg2);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabWidget_addTab_widget_icon_label(QTabWidget* this_ptr, QWidget* widget, const QIcon* icon, const QString* label);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_clear(QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QTabWidget_cornerWidget_corner(const QTabWidget* this_ptr, const Qt::Corner* corner);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QTabWidget_cornerWidget_no_args(const QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabWidget_count(const QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabWidget_currentIndex(const QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QTabWidget_currentWidget(const QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_delete(QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabWidget_documentMode(const QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabWidget_hasHeightForWidth(const QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabWidget_heightForWidth(const QTabWidget* this_ptr, int width);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_iconSize_to_output(const QTabWidget* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabWidget_indexOf(const QTabWidget* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabWidget_insertTab_index_widget_arg3(QTabWidget* this_ptr, int index, QWidget* widget, const QString* arg3);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabWidget_insertTab_index_widget_icon_label(QTabWidget* this_ptr, int index, QWidget* widget, const QIcon* icon, const QString* label);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabWidget_isMovable(const QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabWidget_isTabEnabled(const QTabWidget* this_ptr, int index);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QTabWidget_metaObject(const QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_minimumSizeHint_to_output(const QTabWidget* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QTabWidget* qt_widgets_c_QTabWidget_new_no_args();
QT_WIDGETS_C_EXPORT QTabWidget* qt_widgets_c_QTabWidget_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabWidget_qt_metacall(QTabWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QTabWidget_qt_metacast(QTabWidget* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_removeTab(QTabWidget* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setCornerWidget_w(QTabWidget* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setCornerWidget_w_corner(QTabWidget* this_ptr, QWidget* w, const Qt::Corner* corner);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setCurrentIndex(QTabWidget* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setCurrentWidget(QTabWidget* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setDocumentMode(QTabWidget* this_ptr, bool set);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setElideMode(QTabWidget* this_ptr, const Qt::TextElideMode* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setIconSize(QTabWidget* this_ptr, const QSize* size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setMovable(QTabWidget* this_ptr, bool movable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setTabBarAutoHide(QTabWidget* this_ptr, bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setTabEnabled(QTabWidget* this_ptr, int index, bool arg2);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setTabIcon(QTabWidget* this_ptr, int index, const QIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setTabPosition(QTabWidget* this_ptr, QTabWidget::TabPosition arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setTabShape(QTabWidget* this_ptr, QTabWidget::TabShape s);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setTabText(QTabWidget* this_ptr, int index, const QString* arg2);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setTabToolTip(QTabWidget* this_ptr, int index, const QString* tip);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setTabWhatsThis(QTabWidget* this_ptr, int index, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setTabsClosable(QTabWidget* this_ptr, bool closeable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_setUsesScrollButtons(QTabWidget* this_ptr, bool useButtons);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_sizeHint_to_output(const QTabWidget* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QTabBar* qt_widgets_c_QTabWidget_tabBar(const QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabWidget_tabBarAutoHide(const QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_tabIcon_to_output(const QTabWidget* this_ptr, int index, QIcon* output);
QT_WIDGETS_C_EXPORT QTabWidget::TabPosition qt_widgets_c_QTabWidget_tabPosition(const QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT QTabWidget::TabShape qt_widgets_c_QTabWidget_tabShape(const QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_tabText_to_output(const QTabWidget* this_ptr, int index, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_tabToolTip_to_output(const QTabWidget* this_ptr, int index, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_tabWhatsThis_to_output(const QTabWidget* this_ptr, int index, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabWidget_tabsClosable(const QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabWidget_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabWidget_usesScrollButtons(const QTabWidget* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QTabWidget_widget(const QTabWidget* this_ptr, int index);

} // extern "C"

#endif // QT_WIDGETS_C_QTABWIDGET_H
