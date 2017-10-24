#ifndef QT_GUI_C_QVECTOR3D_H
#define QT_GUI_C_QVECTOR3D_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDataStream* qt_gui_c_QVector3D_G_operator_shl(QDataStream* arg1, const QVector3D* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QVector3D_G_operator_shl_to_output(const QDebug* dbg, const QVector3D* vector, QDebug* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QVector3D_G_operator_shr(QDataStream* arg1, QVector3D* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QVector3D_convert_to_QVariant_to_output(const QVector3D* this_ptr, QVariant* output);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_crossProduct_as_ptr(const QVector3D* v1, const QVector3D* v2);
QT_GUI_C_EXPORT void qt_gui_c_QVector3D_delete(QVector3D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector3D_distanceToLine(const QVector3D* this_ptr, const QVector3D* point, const QVector3D* direction);
QT_GUI_C_EXPORT float qt_gui_c_QVector3D_distanceToPlane_plane1_plane2_plane3(const QVector3D* this_ptr, const QVector3D* plane1, const QVector3D* plane2, const QVector3D* plane3);
QT_GUI_C_EXPORT float qt_gui_c_QVector3D_distanceToPlane_plane_normal(const QVector3D* this_ptr, const QVector3D* plane, const QVector3D* normal);
QT_GUI_C_EXPORT float qt_gui_c_QVector3D_distanceToPoint(const QVector3D* this_ptr, const QVector3D* point);
QT_GUI_C_EXPORT float qt_gui_c_QVector3D_dotProduct(const QVector3D* v1, const QVector3D* v2);
QT_GUI_C_EXPORT bool qt_gui_c_QVector3D_isNull(const QVector3D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector3D_length(const QVector3D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector3D_lengthSquared(const QVector3D* this_ptr);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_new_QPoint(const QPoint* point);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_new_QPointF(const QPointF* point);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_new_QVector2D(const QVector2D* vector);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_new_QVector2D_float(const QVector2D* vector, float zpos);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_new_QVector4D(const QVector4D* vector);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_new_float_float_float(float xpos, float ypos, float zpos);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_new_no_args();
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_normal_as_ptr_v1_v2(const QVector3D* v1, const QVector3D* v2);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_normal_as_ptr_v1_v2_v3(const QVector3D* v1, const QVector3D* v2, const QVector3D* v3);
QT_GUI_C_EXPORT void qt_gui_c_QVector3D_normalize(QVector3D* this_ptr);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_normalized_as_ptr(const QVector3D* this_ptr);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_operator_add_assign(QVector3D* this_ptr, const QVector3D* vector);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_operator_div_assign_divisor(QVector3D* this_ptr, float divisor);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_operator_div_assign_vector(QVector3D* this_ptr, const QVector3D* vector);
QT_GUI_C_EXPORT float* qt_gui_c_QVector3D_operator_index(QVector3D* this_ptr, int i);
QT_GUI_C_EXPORT float qt_gui_c_QVector3D_operator_index_const(const QVector3D* this_ptr, int i);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_operator_mul_assign_factor(QVector3D* this_ptr, float factor);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_operator_mul_assign_vector(QVector3D* this_ptr, const QVector3D* vector);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_operator_sub_assign(QVector3D* this_ptr, const QVector3D* vector);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_project_as_ptr(const QVector3D* this_ptr, const QMatrix4x4* modelView, const QMatrix4x4* projection, const QRect* viewport);
QT_GUI_C_EXPORT void qt_gui_c_QVector3D_setX(QVector3D* this_ptr, float x);
QT_GUI_C_EXPORT void qt_gui_c_QVector3D_setY(QVector3D* this_ptr, float y);
QT_GUI_C_EXPORT void qt_gui_c_QVector3D_setZ(QVector3D* this_ptr, float z);
QT_GUI_C_EXPORT void qt_gui_c_QVector3D_toPointF_to_output(const QVector3D* this_ptr, QPointF* output);
QT_GUI_C_EXPORT void qt_gui_c_QVector3D_toPoint_to_output(const QVector3D* this_ptr, QPoint* output);
QT_GUI_C_EXPORT QVector2D* qt_gui_c_QVector3D_toVector2D_as_ptr(const QVector3D* this_ptr);
QT_GUI_C_EXPORT QVector4D* qt_gui_c_QVector3D_toVector4D_as_ptr(const QVector3D* this_ptr);
QT_GUI_C_EXPORT QVector3D* qt_gui_c_QVector3D_unproject_as_ptr(const QVector3D* this_ptr, const QMatrix4x4* modelView, const QMatrix4x4* projection, const QRect* viewport);
QT_GUI_C_EXPORT float qt_gui_c_QVector3D_x(const QVector3D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector3D_y(const QVector3D* this_ptr);
QT_GUI_C_EXPORT float qt_gui_c_QVector3D_z(const QVector3D* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QVECTOR3D_H
