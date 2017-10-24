#include "qt_3d_render_c_QLevelOfDetailBoundingSphere.h"

QVector3D* qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_center_as_ptr(const Qt3DRender::QLevelOfDetailBoundingSphere* this_ptr) {
  return new QVector3D(this_ptr->center());
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_constructor_center(const QVector3D* center, Qt3DRender::QLevelOfDetailBoundingSphere* output) {
  new(output) Qt3DRender::QLevelOfDetailBoundingSphere(*center);
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_constructor_center_radius(const QVector3D* center, float radius, Qt3DRender::QLevelOfDetailBoundingSphere* output) {
  new(output) Qt3DRender::QLevelOfDetailBoundingSphere(*center, radius);
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_constructor_no_args(Qt3DRender::QLevelOfDetailBoundingSphere* output) {
  new(output) Qt3DRender::QLevelOfDetailBoundingSphere();
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_constructor_other(const Qt3DRender::QLevelOfDetailBoundingSphere* other, Qt3DRender::QLevelOfDetailBoundingSphere* output) {
  new(output) Qt3DRender::QLevelOfDetailBoundingSphere(*other);
}

void qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_destructor(Qt3DRender::QLevelOfDetailBoundingSphere* this_ptr) {
  qt_3d_render_c_call_destructor(this_ptr);
}

bool qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_isEmpty(const Qt3DRender::QLevelOfDetailBoundingSphere* this_ptr) {
  return this_ptr->isEmpty();
}

Qt3DRender::QLevelOfDetailBoundingSphere* qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_operator_assign(Qt3DRender::QLevelOfDetailBoundingSphere* this_ptr, const Qt3DRender::QLevelOfDetailBoundingSphere* other) {
  return &this_ptr->operator=(*other);
}

bool qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_operator_eq(const Qt3DRender::QLevelOfDetailBoundingSphere* this_ptr, const Qt3DRender::QLevelOfDetailBoundingSphere* other) {
  return this_ptr->operator==(*other);
}

bool qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_operator_neq(const Qt3DRender::QLevelOfDetailBoundingSphere* this_ptr, const Qt3DRender::QLevelOfDetailBoundingSphere* other) {
  return this_ptr->operator!=(*other);
}

float qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_radius(const Qt3DRender::QLevelOfDetailBoundingSphere* this_ptr) {
  return this_ptr->radius();
}

