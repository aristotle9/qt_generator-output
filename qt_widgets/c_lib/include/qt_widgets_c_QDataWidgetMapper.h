#ifndef QT_WIDGETS_C_QDATAWIDGETMAPPER_H
#define QT_WIDGETS_C_QDATAWIDGETMAPPER_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QDataWidgetMapper* qt_widgets_c_QDataWidgetMapper_G_static_cast_QDataWidgetMapper_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QDataWidgetMapper_G_static_cast_QObject_ptr(QDataWidgetMapper* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_addMapping_widget_section(QDataWidgetMapper* this_ptr, QWidget* widget, int section);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_addMapping_widget_section_propertyName(QDataWidgetMapper* this_ptr, QWidget* widget, int section, const QByteArray* propertyName);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_clearMapping(QDataWidgetMapper* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDataWidgetMapper_currentIndex(const QDataWidgetMapper* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_delete(QDataWidgetMapper* this_ptr);
QT_WIDGETS_C_EXPORT QAbstractItemDelegate* qt_widgets_c_QDataWidgetMapper_itemDelegate(const QDataWidgetMapper* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_mappedPropertyName_to_output(const QDataWidgetMapper* this_ptr, QWidget* widget, QByteArray* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDataWidgetMapper_mappedSection(const QDataWidgetMapper* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QDataWidgetMapper_mappedWidgetAt(const QDataWidgetMapper* this_ptr, int section);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QDataWidgetMapper_metaObject(const QDataWidgetMapper* this_ptr);
QT_WIDGETS_C_EXPORT QAbstractItemModel* qt_widgets_c_QDataWidgetMapper_model(const QDataWidgetMapper* this_ptr);
QT_WIDGETS_C_EXPORT QDataWidgetMapper* qt_widgets_c_QDataWidgetMapper_new_no_args();
QT_WIDGETS_C_EXPORT QDataWidgetMapper* qt_widgets_c_QDataWidgetMapper_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDataWidgetMapper_qt_metacall(QDataWidgetMapper* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QDataWidgetMapper_qt_metacast(QDataWidgetMapper* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_removeMapping(QDataWidgetMapper* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_revert(QDataWidgetMapper* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_rootIndex_to_output(const QDataWidgetMapper* this_ptr, QModelIndex* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_setCurrentIndex(QDataWidgetMapper* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_setCurrentModelIndex(QDataWidgetMapper* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_setItemDelegate(QDataWidgetMapper* this_ptr, QAbstractItemDelegate* delegate);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_setModel(QDataWidgetMapper* this_ptr, QAbstractItemModel* model);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_setOrientation(QDataWidgetMapper* this_ptr, const Qt::Orientation* aOrientation);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_setRootIndex(QDataWidgetMapper* this_ptr, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_setSubmitPolicy(QDataWidgetMapper* this_ptr, QDataWidgetMapper::SubmitPolicy policy);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDataWidgetMapper_submit(QDataWidgetMapper* this_ptr);
QT_WIDGETS_C_EXPORT QDataWidgetMapper::SubmitPolicy qt_widgets_c_QDataWidgetMapper_submitPolicy(const QDataWidgetMapper* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_toFirst(QDataWidgetMapper* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_toLast(QDataWidgetMapper* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_toNext(QDataWidgetMapper* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_toPrevious(QDataWidgetMapper* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDataWidgetMapper_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QDATAWIDGETMAPPER_H
