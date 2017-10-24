#include "qt_3d_core_c_QNodeDestroyedChange.h"

Qt3DCore::QNodeDestroyedChange* qt_3d_core_c_QNodeDestroyedChange_G_dynamic_cast_Qt3DCore_QNodeDestroyedChange_ptr(Qt3DCore::QSceneChange* ptr) {
  return dynamic_cast<Qt3DCore::QNodeDestroyedChange*>(ptr);
}

Qt3DCore::QNodeDestroyedChange* qt_3d_core_c_QNodeDestroyedChange_G_static_cast_Qt3DCore_QNodeDestroyedChange_ptr(Qt3DCore::QSceneChange* ptr) {
  return static_cast<Qt3DCore::QNodeDestroyedChange*>(ptr);
}

Qt3DCore::QSceneChange* qt_3d_core_c_QNodeDestroyedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DCore::QNodeDestroyedChange* ptr) {
  return static_cast<Qt3DCore::QSceneChange*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QNodeDestroyedChange_delete(Qt3DCore::QNodeDestroyedChange* this_ptr) {
  delete this_ptr;
}

Qt3DCore::QNodeDestroyedChange* qt_3d_core_c_Qt3DCore_QNodeDestroyedChange_new(const Qt3DCore::QNode* node, const QVector< Qt3DCore::QNodeIdTypePair >* subtreeIdsAndTypes) {
  return new Qt3DCore::QNodeDestroyedChange(node, *subtreeIdsAndTypes);
}

void qt_3d_core_c_Qt3DCore_QNodeDestroyedChange_subtreeIdsAndTypes_to_output(const Qt3DCore::QNodeDestroyedChange* this_ptr, QVector< Qt3DCore::QNodeIdTypePair >* output) {
  new(output) QVector< Qt3DCore::QNodeIdTypePair >(this_ptr->subtreeIdsAndTypes());
}

