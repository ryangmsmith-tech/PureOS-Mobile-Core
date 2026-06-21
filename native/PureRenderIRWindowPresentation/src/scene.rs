use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

pub const SCENE_JSON: &str = include_str!(
    "../../PureRenderIRPlazaFrame/assets/gold_ocean_city_plaza.pureir.scene.json"
);

#[derive(Debug, Clone, Deserialize)]
pub struct SceneIr {
    pub scene_version: String,
    pub scene_id: String,
    pub scene_name: String,
    pub width: u32,
    pub height: u32,
    pub clear_color: [f64; 4],
    pub camera: Camera,
    pub objects: Vec<SceneObject>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Camera {
    pub center: [f32; 2],
    pub half_height: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SceneObject {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub vertices: Vec<IrVertex>,
    pub indices: Vec<u16>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct IrVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl SceneIr {
    pub fn embedded() -> Result<Self> {
        let scene: Self = serde_json::from_str(SCENE_JSON).context("parse embedded plaza scene")?;
        scene.validate()?;
        Ok(scene)
    }

    pub fn validate(&self) -> Result<()> {
        if self.scene_version != "0.10" {
            bail!("unexpected scene version: {}", self.scene_version);
        }
        if self.scene_id.trim().is_empty() || self.scene_name.trim().is_empty() {
            bail!("scene identity is required");
        }
        if self.width == 0 || self.height == 0 || self.objects.len() < 9 {
            bail!("scene dimensions and nine-object minimum are required");
        }
        if !self.camera.half_height.is_finite() || self.camera.half_height <= 0.0 {
            bail!("camera half-height must be positive and finite");
        }
        for object in &self.objects {
            if object.vertices.len() < 3
                || object.indices.len() < 3
                || object.indices.len() % 3 != 0
            {
                bail!("invalid geometry for object {}", object.id);
            }
            let max_index = object.vertices.len() - 1;
            if object
                .indices
                .iter()
                .any(|index| usize::from(*index) > max_index)
            {
                bail!("object {} contains an out-of-range index", object.id);
            }
        }
        Ok(())
    }

    pub fn triangle_count(&self) -> usize {
        self.objects
            .iter()
            .map(|object| object.indices.len() / 3)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_scene_validates() {
        let scene = SceneIr::embedded().expect("embedded scene must parse");
        assert_eq!(scene.objects.len(), 9);
        assert!(scene.triangle_count() >= 20);
    }
}
