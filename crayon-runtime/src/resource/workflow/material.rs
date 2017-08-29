use bincode;
use graphics;
use uuid;

use super::super::errors::*;
use super::super::{ResourceLoader, ResourceSystem, material, shader, texture};

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialSerializationPayload {
    pub shader: uuid::Uuid,
    pub textures: Vec<(String, Option<uuid::Uuid>)>,
    pub uniforms: Vec<(String, graphics::UniformVariable)>,
    pub priority: i32,
}

impl ResourceLoader for MaterialSerializationPayload {
    type Item = material::Material;

    fn load_from_memory(sys: &mut ResourceSystem, bytes: &[u8]) -> Result<Self::Item> {
        let data: MaterialSerializationPayload = bincode::deserialize(&bytes)?;
        let mut textures = Vec::new();

        let shader = sys.load_with_uuid::<shader::Shader>(data.shader)
            .chain_err(|| ErrorKind::ShaderNotFound)?;

        for (name, v) in data.textures {
            let texture = if let Some(uuid) = v {
                sys.load_with_uuid::<texture::Texture>(uuid).ok()
            } else {
                None
            };

            textures.push((name, texture));
        }

        Ok(material::Material::new(shader, textures, data.uniforms))
    }
}