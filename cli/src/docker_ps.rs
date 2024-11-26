use crate::*;

// Command, CreatedAt, ID, Image, Labels, LocalVolumes, Mounts,
// Names, Networks, Ports, RunningFor, Size, State, Status
#[derive(Deserialize)]
pub struct DockerPsLine {
    #[serde(alias = "Names")]
    pub name: String,

    #[serde(alias = "Image")]
    pub image: String,

    #[serde(alias = "ID")]
    pub id: String,

    #[serde(alias = "Status")]
    pub status: String,

    #[serde(alias = "CreatedAt")]
    pub created_at: String,

    #[serde(alias = "Ports")]
    pub ports: String,
}

impl DockerPsLine {
    pub fn short_id(&self, n: usize) -> &str {
        &self.id[0..self.id.len().min(n)]
    }

    pub fn ports(&self) -> Vec<&str> {
        let mut lines = vec![];
        let mut text = self.ports.as_str();
        while text.len() > 70 {
            // hope that there is a hit lol. Surely no port entry is that long.
            let (l, r) = text.split_at(text[..70].rfind(", ").unwrap() + 1);
            lines.push(l);
            text = &r[1..];
        }
        lines.push(text);
        lines
    }
}
