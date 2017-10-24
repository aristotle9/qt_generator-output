#ifndef QT_WIDGETS_C_QMENUBAR_H
#define QT_WIDGETS_C_QMENUBAR_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QMenuBar* qt_widgets_c_QMenuBar_G_dynamic_cast_QMenuBar_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QMenuBar* qt_widgets_c_QMenuBar_G_static_cast_QMenuBar_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QMenuBar* qt_widgets_c_QMenuBar_G_static_cast_QMenuBar_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QMenuBar* qt_widgets_c_QMenuBar_G_static_cast_QMenuBar_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QMenuBar_G_static_cast_QObject_ptr(QMenuBar* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QMenuBar_G_static_cast_QPaintDevice_ptr(QMenuBar* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QMenuBar_G_static_cast_QWidget_ptr(QMenuBar* ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenuBar_actionAt(const QMenuBar* this_ptr, const QPoint* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenuBar_actionGeometry_to_output(const QMenuBar* this_ptr, QAction* arg1, QRect* output);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenuBar_activeAction(const QMenuBar* this_ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenuBar_addAction_text(QMenuBar* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenuBar_addAction_text_receiver_member(QMenuBar* this_ptr, const QString* text, const QObject* receiver, const char* member);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QMenuBar_addMenu_icon_title(QMenuBar* this_ptr, const QIcon* icon, const QString* title);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenuBar_addMenu_menu(QMenuBar* this_ptr, QMenu* menu);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QMenuBar_addMenu_title(QMenuBar* this_ptr, const QString* title);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenuBar_addSeparator(QMenuBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenuBar_clear(QMenuBar* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QMenuBar_cornerWidget_corner(const QMenuBar* this_ptr, const Qt::Corner* corner);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QMenuBar_cornerWidget_no_args(const QMenuBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenuBar_delete(QMenuBar* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMenuBar_heightForWidth(const QMenuBar* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenuBar_insertMenu(QMenuBar* this_ptr, QAction* before, QMenu* menu);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QMenuBar_insertSeparator(QMenuBar* this_ptr, QAction* before);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMenuBar_isDefaultUp(const QMenuBar* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMenuBar_isNativeMenuBar(const QMenuBar* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QMenuBar_metaObject(const QMenuBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenuBar_minimumSizeHint_to_output(const QMenuBar* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QMenuBar* qt_widgets_c_QMenuBar_new_no_args();
QT_WIDGETS_C_EXPORT QMenuBar* qt_widgets_c_QMenuBar_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMenuBar_qt_metacall(QMenuBar* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QMenuBar_qt_metacast(QMenuBar* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenuBar_setActiveAction(QMenuBar* this_ptr, QAction* action);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenuBar_setCornerWidget_w(QMenuBar* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenuBar_setCornerWidget_w_corner(QMenuBar* this_ptr, QWidget* w, const Qt::Corner* corner);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenuBar_setDefaultUp(QMenuBar* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenuBar_setNativeMenuBar(QMenuBar* this_ptr, bool nativeMenuBar);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenuBar_setVisible(QMenuBar* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenuBar_sizeHint_to_output(const QMenuBar* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenuBar_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMenuBar_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QMENUBAR_H
