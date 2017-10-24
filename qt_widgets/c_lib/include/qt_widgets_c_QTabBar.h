#ifndef QT_WIDGETS_C_QTABBAR_H
#define QT_WIDGETS_C_QTABBAR_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QTabBar* qt_widgets_c_QTabBar_G_dynamic_cast_QTabBar_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QTabBar_G_static_cast_QObject_ptr(QTabBar* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QTabBar_G_static_cast_QPaintDevice_ptr(QTabBar* ptr);
QT_WIDGETS_C_EXPORT QTabBar* qt_widgets_c_QTabBar_G_static_cast_QTabBar_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QTabBar* qt_widgets_c_QTabBar_G_static_cast_QTabBar_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QTabBar* qt_widgets_c_QTabBar_G_static_cast_QTabBar_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QTabBar_G_static_cast_QWidget_ptr(QTabBar* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_accessibleTabName_to_output(const QTabBar* this_ptr, int index, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabBar_addTab_icon_text(QTabBar* this_ptr, const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabBar_addTab_text(QTabBar* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabBar_autoHide(const QTabBar* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabBar_changeCurrentOnDrag(const QTabBar* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabBar_count(const QTabBar* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabBar_currentIndex(const QTabBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_delete(QTabBar* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabBar_documentMode(const QTabBar* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabBar_drawBase(const QTabBar* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabBar_expanding(const QTabBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_iconSize_to_output(const QTabBar* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabBar_insertTab_index_icon_text(QTabBar* this_ptr, int index, const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabBar_insertTab_index_text(QTabBar* this_ptr, int index, const QString* text);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabBar_isMovable(const QTabBar* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabBar_isTabEnabled(const QTabBar* this_ptr, int index);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QTabBar_metaObject(const QTabBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_minimumSizeHint_to_output(const QTabBar* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_moveTab(QTabBar* this_ptr, int from, int to);
QT_WIDGETS_C_EXPORT QTabBar* qt_widgets_c_QTabBar_new_no_args();
QT_WIDGETS_C_EXPORT QTabBar* qt_widgets_c_QTabBar_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabBar_qt_metacall(QTabBar* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QTabBar_qt_metacast(QTabBar* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_removeTab(QTabBar* this_ptr, int index);
QT_WIDGETS_C_EXPORT QTabBar::SelectionBehavior qt_widgets_c_QTabBar_selectionBehaviorOnRemove(const QTabBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setAccessibleTabName(QTabBar* this_ptr, int index, const QString* name);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setAutoHide(QTabBar* this_ptr, bool hide);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setChangeCurrentOnDrag(QTabBar* this_ptr, bool change);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setCurrentIndex(QTabBar* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setDocumentMode(QTabBar* this_ptr, bool set);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setDrawBase(QTabBar* this_ptr, bool drawTheBase);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setElideMode(QTabBar* this_ptr, const Qt::TextElideMode* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setExpanding(QTabBar* this_ptr, bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setIconSize(QTabBar* this_ptr, const QSize* size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setMovable(QTabBar* this_ptr, bool movable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setSelectionBehaviorOnRemove(QTabBar* this_ptr, QTabBar::SelectionBehavior behavior);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setShape(QTabBar* this_ptr, QTabBar::Shape shape);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setTabButton(QTabBar* this_ptr, int index, QTabBar::ButtonPosition position, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setTabData(QTabBar* this_ptr, int index, const QVariant* data);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setTabEnabled(QTabBar* this_ptr, int index, bool arg2);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setTabIcon(QTabBar* this_ptr, int index, const QIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setTabText(QTabBar* this_ptr, int index, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setTabTextColor(QTabBar* this_ptr, int index, const QColor* color);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setTabToolTip(QTabBar* this_ptr, int index, const QString* tip);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setTabWhatsThis(QTabBar* this_ptr, int index, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setTabsClosable(QTabBar* this_ptr, bool closable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_setUsesScrollButtons(QTabBar* this_ptr, bool useButtons);
QT_WIDGETS_C_EXPORT QTabBar::Shape qt_widgets_c_QTabBar_shape(const QTabBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_sizeHint_to_output(const QTabBar* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTabBar_tabAt(const QTabBar* this_ptr, const QPoint* pos);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QTabBar_tabButton(const QTabBar* this_ptr, int index, QTabBar::ButtonPosition position);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_tabData_to_output(const QTabBar* this_ptr, int index, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_tabIcon_to_output(const QTabBar* this_ptr, int index, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_tabRect_to_output(const QTabBar* this_ptr, int index, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_tabTextColor_to_output(const QTabBar* this_ptr, int index, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_tabText_to_output(const QTabBar* this_ptr, int index, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_tabToolTip_to_output(const QTabBar* this_ptr, int index, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_tabWhatsThis_to_output(const QTabBar* this_ptr, int index, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabBar_tabsClosable(const QTabBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTabBar_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTabBar_usesScrollButtons(const QTabBar* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QTABBAR_H
