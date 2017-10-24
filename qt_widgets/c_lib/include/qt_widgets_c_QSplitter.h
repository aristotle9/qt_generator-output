#ifndef QT_WIDGETS_C_QSPLITTER_H
#define QT_WIDGETS_C_QSPLITTER_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QSplitter* qt_widgets_c_QSplitter_G_dynamic_cast_QSplitter_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QSplitter* qt_widgets_c_QSplitter_G_dynamic_cast_QSplitter_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QTextStream* qt_widgets_c_QSplitter_G_operator_shl(QTextStream* arg1, const QSplitter* arg2);
QT_WIDGETS_C_EXPORT QTextStream* qt_widgets_c_QSplitter_G_operator_shr(QTextStream* arg1, QSplitter* arg2);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QSplitter_G_static_cast_QFrame_ptr(QSplitter* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QSplitter_G_static_cast_QObject_ptr(QSplitter* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QSplitter_G_static_cast_QPaintDevice_ptr(QSplitter* ptr);
QT_WIDGETS_C_EXPORT QSplitter* qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QSplitter* qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QSplitter* qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QSplitter* qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QSplitter_G_static_cast_QWidget_ptr(QSplitter* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_addWidget(QSplitter* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSplitter_childrenCollapsible(const QSplitter* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSplitter_count(const QSplitter* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_delete(QSplitter* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_getRange(const QSplitter* this_ptr, int index, int* arg2, int* arg3);
QT_WIDGETS_C_EXPORT QSplitterHandle* qt_widgets_c_QSplitter_handle(const QSplitter* this_ptr, int index);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSplitter_handleWidth(const QSplitter* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSplitter_indexOf(const QSplitter* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_insertWidget(QSplitter* this_ptr, int index, QWidget* widget);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSplitter_isCollapsible(const QSplitter* this_ptr, int index);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QSplitter_metaObject(const QSplitter* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_minimumSizeHint_to_output(const QSplitter* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QSplitter* qt_widgets_c_QSplitter_new_arg1(const Qt::Orientation* arg1);
QT_WIDGETS_C_EXPORT QSplitter* qt_widgets_c_QSplitter_new_arg1_parent(const Qt::Orientation* arg1, QWidget* parent);
QT_WIDGETS_C_EXPORT QSplitter* qt_widgets_c_QSplitter_new_no_args();
QT_WIDGETS_C_EXPORT QSplitter* qt_widgets_c_QSplitter_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSplitter_opaqueResize(const QSplitter* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSplitter_qt_metacall(QSplitter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QSplitter_qt_metacast(QSplitter* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_refresh(QSplitter* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QSplitter_replaceWidget(QSplitter* this_ptr, int index, QWidget* widget);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSplitter_restoreState(QSplitter* this_ptr, const QByteArray* state);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_saveState_to_output(const QSplitter* this_ptr, QByteArray* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_setChildrenCollapsible(QSplitter* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_setCollapsible(QSplitter* this_ptr, int index, bool arg2);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_setHandleWidth(QSplitter* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_setOpaqueResize_no_args(QSplitter* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_setOpaqueResize_opaque(QSplitter* this_ptr, bool opaque);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_setOrientation(QSplitter* this_ptr, const Qt::Orientation* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_setSizes(QSplitter* this_ptr, const QList< int >* list);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_setStretchFactor(QSplitter* this_ptr, int index, int stretch);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_sizeHint_to_output(const QSplitter* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_sizes_to_output(const QSplitter* this_ptr, QList< int >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitter_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QSplitter_widget(const QSplitter* this_ptr, int index);

} // extern "C"

#endif // QT_WIDGETS_C_QSPLITTER_H
