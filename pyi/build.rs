use ssbh_data_py::adj_data::*;
use ssbh_data_py::anim_data::*;
use ssbh_data_py::matl_data::*;
use ssbh_data_py::mesh_data::*;
use ssbh_data_py::meshex_data::*;
use ssbh_data_py::modl_data::*;
use ssbh_data_py::skel_data::*;

use ssbh_data_py::Pyi;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn generate_pyi_file(file_path: &str, functions: &[&str], class_definitions: &[String]) {
    // Make sure the folder exists first.
    let file_path = Path::new(file_path);
    std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();

    let mut f = BufWriter::new(File::create(file_path).unwrap());

    writeln!(&mut f, "# File automatically generated by build.rs.").unwrap();
    writeln!(&mut f, "# Changes made to this file will not be saved.").unwrap();
    writeln!(
        &mut f,
        "from typing import List, Tuple, Any, Optional, Union, ClassVar"
    )
    .unwrap();
    writeln!(&mut f).unwrap();
    writeln!(&mut f).unwrap();
    for function in functions {
        writeln!(&mut f, "{}", function).unwrap();
        writeln!(&mut f).unwrap();
        writeln!(&mut f).unwrap();
    }
    writeln!(&mut f, "{}", class_definitions.join("\n\n\n")).unwrap();
}

fn main() {
    // TODO: When will this be rerun?
    // TODO: Rerun only if the rust file changes?
    // println!("cargo:rerun-if-changed=src/matl_data.rs");

    // TODO: Find a way to automatically register types?
    // This would avoid having to specify all types in each build.rs file.
    generate_pyi_file(
        "../ssbh_data_py/ssbh_data_py/matl_data.pyi",
        &["def read_matl(path: str) -> MatlData: ..."],
        &[
            MatlData::pyi(),
            MatlEntryData::pyi(),
            BlendStateParam::pyi(),
            FloatParam::pyi(),
            BooleanParam::pyi(),
            Vector4Param::pyi(),
            RasterizerStateParam::pyi(),
            SamplerParam::pyi(),
            TextureParam::pyi(),
            BlendStateData::pyi(),
            RasterizerStateData::pyi(),
            SamplerData::pyi(),
            ParamId::pyi(),
            FillMode::pyi(),
            CullMode::pyi(),
            BlendFactor::pyi(),
            WrapMode::pyi(),
            MinFilter::pyi(),
            MagFilter::pyi(),
            MaxAnisotropy::pyi(),
        ],
    );

    generate_pyi_file(
        "../ssbh_data_py/ssbh_data_py/modl_data.pyi",
        &["def read_modl(path: str) -> ModlData: ..."],
        &[ModlData::pyi(), ModlEntryData::pyi()],
    );

    generate_pyi_file(
        "../ssbh_data_py/ssbh_data_py/skel_data.pyi",
        &[
            "def read_skel(path: str) -> SkelData: ...",
            "def calculate_relative_transform(
    world_transform: list[list[float]],
    parent_world_transform: list[list[float]]) -> list[list[float]]: ...",
        ],
        &[SkelData::pyi(), BoneData::pyi()],
    );

    generate_pyi_file(
        "../ssbh_data_py/ssbh_data_py/mesh_data.pyi",
        &[
            "def read_mesh(path: str, use_numpy: bool = False) -> MeshData: ...",
            "def transform_points(
    points: list[list[float]], transform: list[list[float]]) -> list[list[float]]: ...",
            "def transform_vectors(
    points: list[list[float]], transform: list[list[float]]) -> list[list[float]]: ...",
            "def calculate_smooth_normals(
    positions: list[list[float]], vertex_indices: list[int]) -> list[list[float]]: ...", 
            "def calculate_tangents_vec4(
    positions: list[list[float]], normals: list[list[float]], uvs: list[list[float]], vertex_indices: list[int]) -> list[list[float]]: ..."
        ],
        &[
            MeshData::pyi(),
            MeshObjectData::pyi(),
            AttributeData::pyi(),
            BoneInfluence::pyi(),
            VertexWeight::pyi(),
        ],
    );

    generate_pyi_file(
        "../ssbh_data_py/ssbh_data_py/anim_data.pyi",
        &["def read_anim(path: str) -> AnimData: ..."],
        &[
            AnimData::pyi(),
            GroupData::pyi(),
            GroupType::pyi(),
            NodeData::pyi(),
            TrackData::pyi(),
            ScaleOptions::pyi(),
            Transform::pyi(),
            UvTransform::pyi(),
        ],
    );

    generate_pyi_file(
        "../ssbh_data_py/ssbh_data_py/adj_data.pyi",
        &["def read_adj(path: str) -> AdjData: ..."],
        &[AdjData::pyi(), AdjEntryData::pyi()],
    );

    generate_pyi_file(
        "../ssbh_data_py/ssbh_data_py/meshex_data.pyi",
        &["def read_meshex(path: str) -> MeshExData: ..."],
        &[
            MeshExData::pyi(),
            MeshObjectGroupData::pyi(),
            EntryFlags::pyi(),
        ],
    );
}
