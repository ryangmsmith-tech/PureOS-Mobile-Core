use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PureRenderIr {
    pub ir_version: String,
    pub object_id: String,
    pub object_name: String,
    pub width: u32,
    pub height: u32,
    pub clear_color: [f64; 4],
    pub vertices: Vec<IrVertex>,
    pub indices: Vec<u16>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct IrVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl PureRenderIr {
    pub fn validate(&self) -> Result<()> {
        if self.ir_version != "0.9" {
            bail!("unsupported PureRenderIR version: {}", self.ir_version);
        }
        if self.object_id.trim().is_empty() || self.object_name.trim().is_empty() {
            bail!("object identity is required");
        }
        if self.width == 0 || self.height == 0 || self.width > 4096 || self.height > 4096 {
            bail!("invalid frame dimensions");
        }
        if self.vertices.len() < 3 {
            bail!("at least three vertices are required");
        }
        if self.indices.len() < 3 || self.indices.len() % 3 != 0 {
            bail!("indices must contain complete triangles");
        }
        let max_index = self.vertices.len() - 1;
        if self.indices.iter().any(|index| usize::from(*index) > max_index) {
            bail!("index exceeds vertex array");
        }
        if self
            .vertices
            .iter()
            .flat_map(|vertex| vertex.position.iter().chain(vertex.color.iter()))
            .any(|value| !value.is_finite())
        {
            bail!("non-finite geometry value detected");
        }
        if self
            .clear_color
            .iter()
            .any(|channel| !channel.is_finite() || !(0.0..=1.0).contains(channel))
        {
            bail!("clear color must be finite and normalized");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_ir() -> PureRenderIr {
        PureRenderIr {
            ir_version: "0.9".to_owned(),
            object_id: "GOC_TEST".to_owned(),
            object_name: "Gold Ocean City test object".to_owned(),
            width: 64,
            height: 64,
            clear_color: [0.0, 0.0, 0.0, 1.0],
            vertices: vec![
                IrVertex {
                    position: [-0.5, -0.5],
                    color: [1.0, 0.0, 0.0, 1.0],
                },
                IrVertex {
                    position: [0.5, -0.5],
                    color: [0.0, 1.0, 0.0, 1.0],
                },
                IrVertex {
                    position: [0.0, 0.5],
                    color: [0.0, 0.0, 1.0, 1.0],
                },
            ],
            indices: vec![0, 1, 2],
        }
    }

    #[test]
    fn valid_geometry_passes() {
        assert!(valid_ir().validate().is_ok());
    }

    #[test]
    fn incomplete_triangle_fails() {
        let mut ir = valid_ir();
        ir.indices.pop();
        assert!(ir.validate().is_err());
    }

    #[test]
    fn out_of_range_index_fails() {
        let mut ir = valid_ir();
        ir.indices[2] = 99;
        assert!(ir.validate().is_err());
    }
}
