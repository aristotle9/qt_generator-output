#ifndef QT_GUI_C_QVECTOR2D_H
#define QT_GUI_C_QVECTOR2D_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDataStream* qt_gui_c_QVector2D_G_operator_shl(QDataStream* arg1, const QVector2D* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QVector2D_G_operator_shl_to_output(const QDebug* dbg, const QVector2D* vector, QDebug* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QVector2D_G_operator_shr(QDataStream* arg1, QVector2D* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QVector2D_convert_to_QVariant_to_output(const QVector2D* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QVector2D_delete(QVector2D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector2D_distanceToLine(const QVector2D* this_ptr, const QVector2D* point, const QVector2D* direction);
QT_GUI_C_EXPORT float qt_gui_c_QVector2D_distanceToPoint(const QVector2D* this_ptr, const QVector2D* point);
QT_GUI_C_EXPORT float qt_gui_c_QVector2D_dotProduct(const QVector2D* v1, const QVector2D* v2);
QT_GUI_C_EXPORT bool qt_gui_c_QVector2D_isNull(const QVector2D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector2D_length(const QVector2D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector2D_lengthSquared(const QVector2D* this_ptr);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector2D_new_QPoint(const QPoint* point);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector2D_new_QPointF(const QPointF* point);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector2D_new_QVector3D(const QVector3D* vector);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector2D_new_QVector4D(const QVector4D* vector);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector2D_new_float_float(float xpos, float ypos);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector2D_new_no_args();
QT_GUI_C_EXPORT void qt_gui_c_QVector2D_normalize(QVector2D* this_ptr);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector2D_normalized_as_ptr(const QVector2D* this_ptr);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector2D_operator_add_assign(QVector2D* this_ptr, const QVector2D* vector);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector2D_operator_div_assign_divisor(QVector2D* this_ptr, float divisor);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector2D_operator_div_assign_vector(QVector2D* this_ptr, const QVector2D* vector);
QT_GUI_C_EXPORT float* qt_gui_c_QVector2D_operator_index(QVector2D* this_ptr, int i);
QT_GUI_C_EXPORT float qt_gui_c_QVector2D_operator_index_const(const QVector2D* this_ptr, int i);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector2D_operator_mul_assign_factor(QVector2D* this_ptr, float factor);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector2D_operator_mul_assign_vector(QVector2D* this_ptr, const QVector2D* vector);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector2D_operator_sub_assign(QVector2D* this_ptr, const QVector2D* vector);
QT_GUI_C_EXPORT void qt_gui_c_QVector2D_setX(QVector2D* this_ptr, float x);
QT_GUI_C_EXPORT void qt_gui_c_QVector2D_setY(QVector2D* this_ptr, float y);
QT_GUI_C_EXPORT void qt_gui_c_QVector2D_toPointF_to_output(const QVector2D* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QVector2D_toPoint_to_output(const QVector2D* this_ptr, QPoint* output);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector2D_toVector3D_as_ptr(const QVector2D* this_ptr);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector2D_toVector4D_as_ptr(const QVector2D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector2D_x(const QVector2D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector2D_y(const QVector2D* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QVECTOR2D_H
