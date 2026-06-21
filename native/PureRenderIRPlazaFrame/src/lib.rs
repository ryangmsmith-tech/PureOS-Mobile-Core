use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SceneObject {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub vertices: Vec<IrVertex>,
    pub indices: Vec<u16>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct IrVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl SceneIr {
    pub fn validate(&self) -> Result<()> {
        if self.scene_version != "0.10" {
            bail!("unsupported scene version: {}", self.scene_version);
        }
        if self.scene_id.trim().is_empty() || self.scene_name.trim().is_empty() {
            bail!("scene identity is required");
        }
        if self.width == 0 || self.height == 0 || self.width > 4096 || self.height > 4096 {
            bail!("invalid frame dimensions");
        }
        if !self.camera.half_height.is_finite() || self.camera.half_height <= 0.0 {
            bail!("camera half-height must be finite and positive");
        }
        if self.camera.center.iter().any(|value| !value.is_finite()) {
            bail!("camera center must be finite");
        }
        if self.objects.len() < 6 {
            bail!("the plaza proof requires at least six scene objects");
        }
        if self
            .clear_color
            .iter()
            .any(|channel| !channel.is_finite() || !(0.0..=1.0).contains(channel))
        {
            bail!("clear color must be finite and normalized");
        }

        let mut object_ids = HashSet::new();
        for object in &self.objects {
            if object.id.trim().is_empty() || object.name.trim().is_empty() || object.kind.trim().is_empty() {
                bail!("object identity fields are required");
            }
            if !object_ids.insert(object.id.as_str()) {
                bail!("duplicate object id: {}", object.id);
            }
            if object.vertices.len() < 3 {
                bail!("object {} requires at least three vertices", object.id);
            }
            if object.indices.len() < 3 || object.indices.len() % 3 != 0 {
                bail!("object {} indices must contain complete triangles", object.id);
            }
            let max_index = object.vertices.len() - 1;
            if object.indices.iter().any(|index| usize::from(*index) > max_index) {
                bail!("object {} index exceeds vertex array", object.id);
            }
            if object
                .vertices
                .iter()
                .flat_map(|vertex| vertex.position.iter().chain(vertex.color.iter()))
                .any(|value| !value.is_finite())
            {
                bail!("object {} contains non-finite geometry", object.id);
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn project_position(&self, position: [f32; 2]) -> [f32; 2] {
        let aspect = self.width as f32 / self.height as f32;
        [
            (position[0] - self.camera.center[0]) / (self.camera.half_height * aspect),
            (position[1] - self.camera.center[1]) / self.camera.half_height,
        ]
    }

    #[must_use]
    pub fn triangle_count(&self) -> usize {
        self.objects.iter().map(|object| object.indices.len() / 3).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn object(id: &str) -> SceneObject {
        SceneObject {
            id: id.to_owned(),
            name: id.to_owned(),
            kind: "test".to_owned(),
            vertices: vec![
                IrVertex { position: [-1.0, -1.0], color: [1.0, 0.0, 0.0, 1.0] },
                IrVertex { position: [1.0, -1.0], color: [0.0, 1.0, 0.0, 1.0] },
                IrVertex { position: [0.0, 1.0], color: [0.0, 0.0, 1.0, 1.0] },
            ],
            indices: vec![0, 1, 2],
        }
    }

    fn scene() -> SceneIr {
        SceneIr {
            scene_version: "0.10".to_owned(),
            scene_id: "GOC_TEST".to_owned(),
            scene_name: "Gold Ocean City test".to_owned(),
            width: 1280,
            height: 720,
            clear_color: [0.0, 0.0, 0.0, 1.0],
            camera: Camera { center: [0.0, 0.0], half_height: 9.0 },
            objects: (0..6).map(|index| object(&format!("object_{index}"))).collect(),
        }
    }

    #[test]
    fn valid_scene_passes() {
        assert!(scene().validate().is_ok());
    }

    #[test]
    fn duplicate_object_id_fails() {
        let mut scene = scene();
        scene.objects[1].id = scene.objects[0].id.clone();
        assert!(scene.validate().is_err());
    }

    #[test]
    fn camera_projection_uses_aspect_ratio() {
        let scene = scene();
        let projected = scene.project_position([16.0, 9.0]);
        assert!((projected[0] - 1.0).abs() < 0.001);
        assert!((projected[1] - 1.0).abs() < 0.001);
    }
}
