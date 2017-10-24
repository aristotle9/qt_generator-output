//! Bindings for [Qt3DRender](http://doc.qt.io/qt-5/qt3drender-module.html) library.
//!
//! This crate was generated using [cpp_to_rust](https://github.com/rust-qt/cpp_to_rust).

pub extern crate libc;
pub extern crate cpp_utils;

pub extern crate qt_core;

pub extern crate qt_gui;

pub extern crate qt_3d_core;

#[allow(dead_code)]
mod ffi {
  include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

mod type_sizes {
  include!(concat!(env!("OUT_DIR"), "/type_sizes.rs"));
}

/// Entities from `QAbstractFunctor` C++ header
pub mod abstract_functor;
/// Entities from `QAbstractLight` C++ header
pub mod abstract_light;
/// Entities from `QAbstractTexture` C++ header
pub mod abstract_texture;
/// Entities from `QAbstractTextureImage` C++ header
pub mod abstract_texture_image;
/// Entities from `QAlphaCoverage` C++ header
pub mod alpha_coverage;
/// Entities from `QAlphaTest` C++ header
pub mod alpha_test;
/// Entities from `QAttribute` C++ header
pub mod attribute;
/// Entities from `QBlendEquation` C++ header
pub mod blend_equation;
/// Entities from `QBlendEquationArguments` C++ header
pub mod blend_equation_arguments;
/// Entities from `QBuffer` C++ header
pub mod buffer;
/// Entities from `QBufferCapture` C++ header
pub mod buffer_capture;
/// Entities from `QBufferDataGenerator` C++ header
pub mod buffer_data_generator;
/// Entities from `QCamera` C++ header
pub mod camera;
/// Entities from `QCameraLens` C++ header
pub mod camera_lens;
/// Entities from `QCameraSelector` C++ header
pub mod camera_selector;
/// Entities from `QClearBuffers` C++ header
pub mod clear_buffers;
/// Entities from `QClipPlane` C++ header
pub mod clip_plane;
/// Entities from `QColorMask` C++ header
pub mod color_mask;
/// Entities from `QComputeCommand` C++ header
pub mod compute_command;
/// Entities from `QCullFace` C++ header
pub mod cull_face;
/// Entities from `QDepthTest` C++ header
pub mod depth_test;
/// Entities from `QDirectionalLight` C++ header
pub mod directional_light;
/// Entities from `QDispatchCompute` C++ header
pub mod dispatch_compute;
/// Entities from `QDithering` C++ header
pub mod dithering;
/// Entities from `QEffect` C++ header
pub mod effect;
/// Entities from `QEnvironmentLight` C++ header
pub mod environment_light;
/// Entities from `QFilterKey` C++ header
pub mod filter_key;
/// Entities from `QFrameGraphNode` C++ header
pub mod frame_graph_node;
/// Entities from `QFrameGraphNodeCreatedChange` C++ header
pub mod frame_graph_node_created_change;
/// Entities from `QFrontFace` C++ header
pub mod front_face;
/// Entities from `QFrustumCulling` C++ header
pub mod frustum_culling;
/// Entities from `QGeometry` C++ header
pub mod geometry;
/// Entities from `QGeometryFactory` C++ header
pub mod geometry_factory;
/// Entities from `QGeometryRenderer` C++ header
pub mod geometry_renderer;
/// Entities from `QGraphicsApiFilter` C++ header
pub mod graphics_api_filter;
/// Entities from `QLayer` C++ header
pub mod layer;
/// Entities from `QLayerFilter` C++ header
pub mod layer_filter;
/// Entities from `QLevelOfDetail` C++ header
pub mod level_of_detail;
/// Entities from `QLevelOfDetailBoundingSphere` C++ header
pub mod level_of_detail_bounding_sphere;
/// Entities from `QLevelOfDetailSwitch` C++ header
pub mod level_of_detail_switch;
/// Entities from `QMaterial` C++ header
pub mod material;
/// Entities from `QMemoryBarrier` C++ header
pub mod memory_barrier;
/// Entities from `QMesh` C++ header
pub mod mesh;
/// Entities from `QMultiSampleAntiAliasing` C++ header
pub mod multi_sample_anti_aliasing;
/// Entities from `QNoDepthMask` C++ header
pub mod no_depth_mask;
/// Entities from `QNoDraw` C++ header
pub mod no_draw;
/// Entities from `QObjectPicker` C++ header
pub mod object_picker;
/// Entities from `QPaintedTextureImage` C++ header
pub mod painted_texture_image;
/// Entities from `QParameter` C++ header
pub mod parameter;
/// Entities from `QPickEvent` C++ header
pub mod pick_event;
/// Entities from `QPickTriangleEvent` C++ header
pub mod pick_triangle_event;
/// Entities from `QPickingSettings` C++ header
pub mod picking_settings;
/// Entities from `QPointLight` C++ header
pub mod point_light;
/// Entities from `QPointSize` C++ header
pub mod point_size;
/// Entities from `QPolygonOffset` C++ header
pub mod polygon_offset;
/// Entities from `QRenderAspect` C++ header
pub mod render_aspect;
/// Entities from `QRenderPass` C++ header
pub mod render_pass;
/// Entities from `QRenderPassFilter` C++ header
pub mod render_pass_filter;
/// Entities from `QRenderSettings` C++ header
pub mod render_settings;
/// Entities from `QRenderState` C++ header
pub mod render_state;
/// Entities from `QRenderStateSet` C++ header
pub mod render_state_set;
/// Entities from `QRenderSurfaceSelector` C++ header
pub mod render_surface_selector;
/// Entities from `QRenderTarget` C++ header
pub mod render_target;
/// Entities from `QRenderTargetOutput` C++ header
pub mod render_target_output;
/// Entities from `QRenderTargetSelector` C++ header
pub mod render_target_selector;
/// Entities from `QSceneLoader` C++ header
pub mod scene_loader;
/// Entities from `QScissorTest` C++ header
pub mod scissor_test;
/// Entities from `QSeamlessCubemap` C++ header
pub mod seamless_cubemap;
/// Entities from `QShaderData` C++ header
pub mod shader_data;
/// Entities from `QShaderProgram` C++ header
pub mod shader_program;
/// Entities from `QSharedPointer` C++ header
pub mod shared_pointer;
/// Binding Qt signals to Rust closures or extern functions.
///
/// Types in this module allow to connect Qt signals with certain argument types to a Rust closure.
///
/// There is one slot type for each distinct set of argument types present in this crate. However, if argument types were present in a dependency crate, the corresponding slot type is located in the dependency's `slots` module.
pub mod slots;
/// Entities from `QSortPolicy` C++ header
pub mod sort_policy;
/// Entities from `QSpotLight` C++ header
pub mod spot_light;
/// Entities from `QStencilMask` C++ header
pub mod stencil_mask;
/// Entities from `QStencilOperation` C++ header
pub mod stencil_operation;
/// Entities from `QStencilOperationArguments` C++ header
pub mod stencil_operation_arguments;
/// Entities from `QStencilTest` C++ header
pub mod stencil_test;
/// Entities from `QStencilTestArguments` C++ header
pub mod stencil_test_arguments;
/// Entities from `QTechnique` C++ header
pub mod technique;
/// Entities from `QTechniqueFilter` C++ header
pub mod technique_filter;
/// Entities from `QTexture` C++ header
pub mod texture;
/// Entities from `QTextureData` C++ header
pub mod texture_data;
/// Entities from `QTextureGenerator` C++ header
pub mod texture_generator;
/// Entities from `QTextureImage` C++ header
pub mod texture_image;
/// Entities from `QTextureImageData` C++ header
pub mod texture_image_data;
/// Entities from `QTextureImageDataGenerator` C++ header
pub mod texture_image_data_generator;
/// Entities from `QTextureWrapMode` C++ header
pub mod texture_wrap_mode;
/// Entities from `QVector` C++ header
pub mod vector;
/// Entities from `QViewport` C++ header
pub mod viewport;
