pub struct Mission;

pub struct MissionContext {
    mission: Mission,
}

impl MissionContext {
    pub fn new(mission: Mission) -> Self {
        MissionContext { mission }
    }

    pub fn mission(&self) -> &Mission {
        &self.mission
    }
}
