#ifndef QT_CORE_C_QPAIR_H
#define QT_CORE_C_QPAIR_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QPair_QString_QString_constructor_no_args(QPair< QString, QString >* output);
QT_CORE_C_EXPORT void qt_core_c_QPair_QString_QString_constructor_t1_t2(const QString* t1, const QString* t2, QPair< QString, QString >* output);
QT_CORE_C_EXPORT void qt_core_c_QPair_QString_QString_destructor(QPair< QString, QString >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QPair_QString_QString_swap(QPair< QString, QString >* this_ptr, QPair< QString, QString >* other);
QT_CORE_C_EXPORT void qt_core_c_QPair_double_QVariant_constructor_no_args(QPair< double, QVariant >* output);
QT_CORE_C_EXPORT void qt_core_c_QPair_double_QVariant_constructor_t1_t2(const double* t1, const QVariant* t2, QPair< double, QVariant >* output);
QT_CORE_C_EXPORT void qt_core_c_QPair_double_QVariant_destructor(QPair< double, QVariant >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QPair_double_QVariant_swap(QPair< double, QVariant >* this_ptr, QPair< double, QVariant >* other);

} // extern "C"

#endif // QT_CORE_C_QPAIR_H
