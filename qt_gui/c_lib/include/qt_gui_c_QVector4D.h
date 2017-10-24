#ifndef QT_GUI_C_QVECTOR4D_H
#define QT_GUI_C_QVECTOR4D_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDataStream* qt_gui_c_QVector4D_G_operator_shl(QDataStream* arg1, const QVector4D* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QVector4D_G_operator_shl_to_output(const QDebug* dbg, const QVector4D* vector, QDebug* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QVector4D_G_operator_shr(QDataStream* arg1, QVector4D* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QVector4D_convert_to_QVariant_to_output(const QVector4D* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QVector4D_delete(QVector4D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector4D_dotProduct(const QVector4D* v1, const QVector4D* v2);
QT_GUI_C_EXPORT bool qt_gui_c_QVector4D_isNull(const QVector4D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector4D_length(const QVector4D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector4D_lengthSquared(const QVector4D* this_ptr);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_new_QPoint(const QPoint* point);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_new_QPointF(const QPointF* point);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_new_QVector2D(const QVector2D* vector);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_new_QVector2D_float_float(const QVector2D* vector, float zpos, float wpos);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_new_QVector3D(const QVector3D* vector);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_new_QVector3D_float(const QVector3D* vector, float wpos);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_new_float_float_float_float(float xpos, float ypos, float zpos, float wpos);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_new_no_args();
QT_GUI_C_EXPORT void qt_gui_c_QVector4D_normalize(QVector4D* this_ptr);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_normalized_as_ptr(const QVector4D* this_ptr);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_operator_add_assign(QVector4D* this_ptr, const QVector4D* vector);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_operator_div_assign_divisor(QVector4D* this_ptr, float divisor);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_operator_div_assign_vector(QVector4D* this_ptr, const QVector4D* vector);
QT_GUI_C_EXPORT float* qt_gui_c_QVector4D_operator_index(QVector4D* this_ptr, int i);
QT_GUI_C_EXPORT float qt_gui_c_QVector4D_operator_index_const(const QVector4D* this_ptr, int i);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_operator_mul_assign_factor(QVector4D* this_ptr, float factor);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_operator_mul_assign_vector(QVector4D* this_ptr, const QVector4D* vector);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector4D_operator_sub_assign(QVector4D* this_ptr, const QVector4D* vector);
QT_GUI_C_EXPORT void qt_gui_c_QVector4D_setW(QVector4D* this_ptr, float w);
QT_GUI_C_EXPORT void qt_gui_c_QVector4D_setX(QVector4D* this_ptr, float x);
QT_GUI_C_EXPORT void qt_gui_c_QVector4D_setY(QVector4D* this_ptr, float y);
QT_GUI_C_EXPORT void qt_gui_c_QVector4D_setZ(QVector4D* this_ptr, float z);
QT_GUI_C_EXPORT void qt_gui_c_QVector4D_toPointF_to_output(const QVector4D* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QVector4D_toPoint_to_output(const QVector4D* this_ptr, QPoint* output);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector4D_toVector2DAffine_as_ptr(const QVector4D* this_ptr);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector4D_toVector2D_as_ptr(const QVector4D* this_ptr);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector4D_toVector3DAffine_as_ptr(const QVector4D* this_ptr);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector4D_toVector3D_as_ptr(const QVector4D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector4D_w(const QVector4D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector4D_x(const QVector4D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector4D_y(const QVector4D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector4D_z(const QVector4D* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QVECTOR4D_H
