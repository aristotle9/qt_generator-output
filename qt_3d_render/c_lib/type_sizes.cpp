#include <Qt3DRender>
#include <iostream>

int main() {
  std::cout << "pub const QT_3D_RENDER_TEXTURE_IMAGE_DATA_TEXTURE_IMAGE_DATA: usize = " << sizeof(Qt3DRender::QTextureImageData) << ";\n";
  std::cout << "pub const QT_3D_RENDER_LEVEL_OF_DETAIL_BOUNDING_SPHERE_LEVEL_OF_DETAIL_BOUNDING_SPHERE: usize = " << sizeof(Qt3DRender::QLevelOfDetailBoundingSphere) << ";\n";
  std::cout << "pub const QT_3D_RENDER_TEXTURE_DATA_TEXTURE_DATA: usize = " << sizeof(Qt3DRender::QTextureData) << ";\n";
  std::cout << "pub const QT_3D_RENDER_VECTOR_VECTOR_ABSTRACT_TEXTURE_IMAGE_MUT_PTR: usize = " << sizeof(QVector< Qt3DRender::QAbstractTextureImage* >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_VECTOR_VECTOR_PARAMETER_MUT_PTR: usize = " << sizeof(QVector< Qt3DRender::QParameter* >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_VECTOR_VECTOR_TECHNIQUE_MUT_PTR: usize = " << sizeof(QVector< Qt3DRender::QTechnique* >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_VECTOR_VECTOR_ATTRIBUTE_MUT_PTR: usize = " << sizeof(QVector< Qt3DRender::QAttribute* >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_VECTOR_VECTOR_LAYER_MUT_PTR: usize = " << sizeof(QVector< Qt3DRender::QLayer* >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_VECTOR_VECTOR_FILTER_KEY_MUT_PTR: usize = " << sizeof(QVector< Qt3DRender::QFilterKey* >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_VECTOR_VECTOR_RENDER_STATE_MUT_PTR: usize = " << sizeof(QVector< Qt3DRender::QRenderState* >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_VECTOR_VECTOR_RENDER_TARGET_OUTPUT_MUT_PTR: usize = " << sizeof(QVector< Qt3DRender::QRenderTargetOutput* >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_VECTOR_VECTOR_RENDER_TARGET_OUTPUT_ATTACHMENT_POINT: usize = " << sizeof(QVector< Qt3DRender::QRenderTargetOutput::AttachmentPoint >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_VECTOR_VECTOR_SORT_POLICY_SORT_TYPE: usize = " << sizeof(QVector< Qt3DRender::QSortPolicy::SortType >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_VECTOR_VECTOR_RENDER_PASS_MUT_PTR: usize = " << sizeof(QVector< Qt3DRender::QRenderPass* >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_SHARED_POINTER_SHARED_POINTER_TEXTURE_GENERATOR: usize = " << sizeof(QSharedPointer< Qt3DRender::QTextureGenerator >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_SHARED_POINTER_SHARED_POINTER_TEXTURE_IMAGE_DATA: usize = " << sizeof(QSharedPointer< Qt3DRender::QTextureImageData >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_SHARED_POINTER_SHARED_POINTER_TEXTURE_IMAGE_DATA_GENERATOR: usize = " << sizeof(QSharedPointer< Qt3DRender::QTextureImageDataGenerator >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_SHARED_POINTER_SHARED_POINTER_BUFFER_DATA_GENERATOR: usize = " << sizeof(QSharedPointer< Qt3DRender::QBufferDataGenerator >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_SHARED_POINTER_SHARED_POINTER_GEOMETRY_FACTORY: usize = " << sizeof(QSharedPointer< Qt3DRender::QGeometryFactory >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_SHARED_POINTER_SHARED_POINTER_SHADER_DATA_PROPERTY_READER_INTERFACE: usize = " << sizeof(QSharedPointer< Qt3DRender::PropertyReaderInterface >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_SHARED_POINTER_SHARED_POINTER_TEXTURE_DATA: usize = " << sizeof(QSharedPointer< Qt3DRender::QTextureData >) << ";\n";
  std::cout << "pub const QT_3D_RENDER_VECTOR_VECTOR_SHARED_POINTER_SHARED_POINTER_TEXTURE_IMAGE_DATA: usize = " << sizeof(QVector< QSharedPointer< Qt3DRender::QTextureImageData > >) << ";\n";
}
