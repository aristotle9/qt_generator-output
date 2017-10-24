#ifndef QT_WIDGETS_C_QSTYLEFACTORY_H
#define QT_WIDGETS_C_QSTYLEFACTORY_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QStyle* qt_widgets_c_QStyleFactory_create(const QString* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleFactory_delete(QStyleFactory* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleFactory_keys_to_output(QStringList* output);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLEFACTORY_H
