#ifndef QT_WIDGETS_C_QITEMEDITORFACTORY_H
#define QT_WIDGETS_C_QITEMEDITORFACTORY_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QItemEditorFactory_createEditor(const QItemEditorFactory* this_ptr, int userType, QWidget* parent);
QT_WIDGETS_C_EXPORT const QItemEditorFactory* qt_widgets_c_QItemEditorFactory_defaultFactory();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QItemEditorFactory_delete(QItemEditorFactory* this_ptr);
QT_WIDGETS_C_EXPORT QItemEditorFactory* qt_widgets_c_QItemEditorFactory_new();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QItemEditorFactory_registerEditor(QItemEditorFactory* this_ptr, int userType, QItemEditorCreatorBase* creator);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QItemEditorFactory_setDefaultFactory(QItemEditorFactory* factory);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QItemEditorFactory_valuePropertyName_to_output(const QItemEditorFactory* this_ptr, int userType, QByteArray* output);

} // extern "C"

#endif // QT_WIDGETS_C_QITEMEDITORFACTORY_H
