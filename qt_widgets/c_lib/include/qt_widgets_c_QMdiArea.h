#ifndef QT_WIDGETS_C_QMDIAREA_H
#define QT_WIDGETS_C_QMDIAREA_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QMdiArea* qt_widgets_c_QMdiArea_G_dynamic_cast_QMdiArea_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QMdiArea* qt_widgets_c_QMdiArea_G_dynamic_cast_QMdiArea_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QMdiArea* qt_widgets_c_QMdiArea_G_dynamic_cast_QMdiArea_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractScrollArea* qt_widgets_c_QMdiArea_G_static_cast_QAbstractScrollArea_ptr(QMdiArea* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QMdiArea_G_static_cast_QFrame_ptr(QMdiArea* ptr);
QT_WIDGETS_C_EXPORT QMdiArea* qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QMdiArea* qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QMdiArea* qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QMdiArea* qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QMdiArea* qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QMdiArea_G_static_cast_QObject_ptr(QMdiArea* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QMdiArea_G_static_cast_QPaintDevice_ptr(QMdiArea* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QMdiArea_G_static_cast_QWidget_ptr(QMdiArea* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_activateNextSubWindow(QMdiArea* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_activatePreviousSubWindow(QMdiArea* this_ptr);
QT_WIDGETS_C_EXPORT QMdiArea::WindowOrder qt_widgets_c_QMdiArea_activationOrder(const QMdiArea* this_ptr);
QT_WIDGETS_C_EXPORT QMdiSubWindow* qt_widgets_c_QMdiArea_activeSubWindow(const QMdiArea* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_background_to_output(const QMdiArea* this_ptr, QBrush* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_cascadeSubWindows(QMdiArea* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_closeActiveSubWindow(QMdiArea* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_closeAllSubWindows(QMdiArea* this_ptr);
QT_WIDGETS_C_EXPORT QMdiSubWindow* qt_widgets_c_QMdiArea_currentSubWindow(const QMdiArea* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_delete(QMdiArea* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMdiArea_documentMode(const QMdiArea* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QMdiArea_metaObject(const QMdiArea* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_minimumSizeHint_to_output(const QMdiArea* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QMdiArea* qt_widgets_c_QMdiArea_new_no_args();
QT_WIDGETS_C_EXPORT QMdiArea* qt_widgets_c_QMdiArea_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMdiArea_qt_metacall(QMdiArea* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QMdiArea_qt_metacast(QMdiArea* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_removeSubWindow(QMdiArea* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_setActivationOrder(QMdiArea* this_ptr, QMdiArea::WindowOrder order);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_setActiveSubWindow(QMdiArea* this_ptr, QMdiSubWindow* window);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_setBackground(QMdiArea* this_ptr, const QBrush* background);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_setDocumentMode(QMdiArea* this_ptr, bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_setOption_option(QMdiArea* this_ptr, QMdiArea::AreaOption option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_setOption_option_on(QMdiArea* this_ptr, QMdiArea::AreaOption option, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_setTabPosition(QMdiArea* this_ptr, const QTabWidget::TabPosition* position);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_setTabShape(QMdiArea* this_ptr, const QTabWidget::TabShape* shape);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_setTabsClosable(QMdiArea* this_ptr, bool closable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_setTabsMovable(QMdiArea* this_ptr, bool movable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_setViewMode(QMdiArea* this_ptr, QMdiArea::ViewMode mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_sizeHint_to_output(const QMdiArea* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_subWindowList_to_output_no_args(const QMdiArea* this_ptr, QList< QMdiSubWindow* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_subWindowList_to_output_order(const QMdiArea* this_ptr, QMdiArea::WindowOrder order, QList< QMdiSubWindow* >* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMdiArea_tabsClosable(const QMdiArea* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMdiArea_tabsMovable(const QMdiArea* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMdiArea_testOption(const QMdiArea* this_ptr, QMdiArea::AreaOption opton);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_tileSubWindows(QMdiArea* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiArea_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QMdiArea::ViewMode qt_widgets_c_QMdiArea_viewMode(const QMdiArea* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QMDIAREA_H
