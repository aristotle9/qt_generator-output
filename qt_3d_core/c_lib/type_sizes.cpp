#include <Qt3DCore>
#include <iostream>

int main() {
  std::cout << "pub const QT_3D_CORE_NODE_ID_NODE_ID: usize = " << sizeof(Qt3DCore::QNodeId) << ";\n";
  std::cout << "pub const QT_3D_CORE_NODE_NODE_ID_TYPE_PAIR: usize = " << sizeof(Qt3DCore::QNodeIdTypePair) << ";\n";
  std::cout << "pub const QT_3D_CORE_SHARED_POINTER_SHARED_POINTER_BACKEND_NODE_BACKEND_NODE_MAPPER: usize = " << sizeof(QSharedPointer< Qt3DCore::QBackendNodeMapper >) << ";\n";
  std::cout << "pub const QT_3D_CORE_SHARED_POINTER_SHARED_POINTER_ASPECT_JOB: usize = " << sizeof(QSharedPointer< Qt3DCore::QAspectJob >) << ";\n";
  std::cout << "pub const QT_3D_CORE_SHARED_POINTER_SHARED_POINTER_ENTITY: usize = " << sizeof(QSharedPointer< Qt3DCore::QEntity >) << ";\n";
  std::cout << "pub const QT_3D_CORE_SHARED_POINTER_SHARED_POINTER_NODE_CREATED_CHANGE_NODE_CREATED_CHANGE_BASE: usize = " << sizeof(QSharedPointer< Qt3DCore::QNodeCreatedChangeBase >) << ";\n";
  std::cout << "pub const QT_3D_CORE_SHARED_POINTER_SHARED_POINTER_SCENE_CHANGE: usize = " << sizeof(QSharedPointer< Qt3DCore::QSceneChange >) << ";\n";
  std::cout << "pub const QT_3D_CORE_VECTOR_VECTOR_SHARED_POINTER_SHARED_POINTER_ASPECT_JOB: usize = " << sizeof(QVector< QSharedPointer< Qt3DCore::QAspectJob > >) << ";\n";
  std::cout << "pub const QT_3D_CORE_VECTOR_VECTOR_ABSTRACT_ASPECT_MUT_PTR: usize = " << sizeof(QVector< Qt3DCore::QAbstractAspect* >) << ";\n";
  std::cout << "pub const QT_3D_CORE_VECTOR_VECTOR_NODE_MUT_PTR: usize = " << sizeof(QVector< Qt3DCore::QNode* >) << ";\n";
  std::cout << "pub const QT_3D_CORE_VECTOR_VECTOR_NODE_ID: usize = " << sizeof(QVector< Qt3DCore::QNodeId >) << ";\n";
  std::cout << "pub const QT_3D_CORE_VECTOR_VECTOR_ENTITY_MUT_PTR: usize = " << sizeof(QVector< Qt3DCore::QEntity* >) << ";\n";
  std::cout << "pub const QT_3D_CORE_VECTOR_VECTOR_COMPONENT_MUT_PTR: usize = " << sizeof(QVector< Qt3DCore::QComponent* >) << ";\n";
  std::cout << "pub const QT_3D_CORE_VECTOR_VECTOR_NODE_NODE_ID_TYPE_PAIR: usize = " << sizeof(QVector< Qt3DCore::QNodeIdTypePair >) << ";\n";
  std::cout << "pub const QT_3D_CORE_WEAK_POINTER_WEAK_POINTER_ASPECT_JOB: usize = " << sizeof(QWeakPointer< Qt3DCore::QAspectJob >) << ";\n";
  std::cout << "pub const QT_3D_CORE_VECTOR_VECTOR_WEAK_POINTER_WEAK_POINTER_ASPECT_JOB: usize = " << sizeof(QVector< QWeakPointer< Qt3DCore::QAspectJob > >) << ";\n";
}
