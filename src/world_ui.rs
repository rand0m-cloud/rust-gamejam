use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::{Vec4, *},
    reflect::TypeUuid,
    render::{
        render_asset::*,
        render_resource::{std140::*, *},
        renderer::*,
        RenderApp, RenderStage,
    },
    sprite::*,
};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

pub struct BarMaterialPlugin;

impl Plugin for BarMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<BarMaterial>::default())
            .register_inspectable::<Percentage>();

        // Add all render world systems/resources
        app.sub_app_mut(RenderApp)
            .add_system_to_stage(RenderStage::Extract, extract_time)
            .add_system_to_stage(RenderStage::Extract, extract_health)
            .add_system_to_stage(RenderStage::Prepare, prepare_my_material);
    }
}

#[derive(TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct BarMaterial {
    percentage: f32,
    color_1: Color,
    color_2: Color,
}

// Holds the version of our data that can be sent to the graphics card (ie Color -> Vec4)
#[derive(Clone, AsStd140)]
pub struct BarMaterialUniformData {
    percentage: f32,
    color_1: Vec4,
    color_2: Vec4,
}

#[derive(Component, Clone, Copy, Inspectable)]
pub struct Percentage {
    #[inspectable(min = -1.0, max = 1.0)]
    pub value: f32,
}

pub fn spawn_quad(
    commands: &mut Commands,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    my_material_assets: &mut ResMut<Assets<BarMaterial>>,
) -> Entity {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            material: my_material_assets.add(BarMaterial {
                percentage: 0.5,
                color_1: Color::RED,
                color_2: Color::GREEN,
            }),
            transform: Transform {
                translation: bevy::prelude::Vec3::new(0.0, 0.15, 0.0),
                scale: bevy::prelude::Vec3::new(0.3, 0.05, 1.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(Percentage { value: 0.0 })
        .id()
}

struct ExtractedTime {
    #[allow(dead_code)]
    seconds_since_startup: f32,
}

fn extract_time(mut commands: Commands, time: Res<Time>) {
    commands.insert_resource(ExtractedTime {
        seconds_since_startup: time.seconds_since_startup() as f32,
    });
}

fn extract_health(
    mut commands: Commands,
    percent_query: Query<(Entity, &Percentage, &Handle<BarMaterial>)>,
) {
    for (entity, percentage, handle) in percent_query.iter() {
        commands
            .get_or_spawn(entity)
            .insert(*percentage)
            .insert(handle.clone());
    }
}

fn prepare_my_material(
    mut material_assets: ResMut<RenderAssets<BarMaterial>>,
    percent_query: Query<(&Percentage, &Handle<BarMaterial>)>,
    render_queue: Res<RenderQueue>,
) {
    for (percent, handle) in percent_query.iter() {
        if let Some(material) = material_assets.get_mut(handle) {
            material.uniform_data.percentage = percent.value;
        }
    }

    for material in material_assets.values_mut() {
        render_queue.write_buffer(
            &material.buffer,
            0,
            material.uniform_data.as_std140().as_bytes(),
        );
    }
}

// The PreparedAsset created from our material
pub struct MyMaterialGPU {
    bind_group: BindGroup,
    uniform_data: BarMaterialUniformData,
    buffer: Buffer,
}

impl Material2d for BarMaterial {
    fn bind_group(material: &MyMaterialGPU) -> &BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                // Our UniformData Buffer
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            BarMaterialUniformData::std140_size_static() as u64,
                        ),
                    },
                    count: None,
                },
            ],
        })
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        asset_server.watch_for_changes().unwrap();
        Some(asset_server.load("my_material.wgsl"))
    }
}

impl RenderAsset for BarMaterial {
    type ExtractedAsset = BarMaterial;
    type PreparedAsset = MyMaterialGPU;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<BarMaterial>>,
        SRes<RenderAssets<Image>>,
    );

    fn prepare_asset(
        extracted_asset: BarMaterial,
        (render_device, pipeline, _images): &mut SystemParamItem<Self::Param>,
    ) -> Result<MyMaterialGPU, PrepareAssetError<BarMaterial>> {
        let uniform_data = BarMaterialUniformData {
            percentage: extracted_asset.percentage,
            color_1: extracted_asset.color_1.as_linear_rgba_f32().into(),
            color_2: extracted_asset.color_2.as_linear_rgba_f32().into(),
        };

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: uniform_data.as_std140().as_bytes(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material2d_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        Ok(MyMaterialGPU {
            bind_group,
            uniform_data,
            buffer,
        })
    }

    fn extract_asset(&self) -> BarMaterial {
        self.clone()
    }
}
