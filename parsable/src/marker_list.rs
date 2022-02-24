pub struct MarkerList {
    markers: Vec<Marker>,
    counter: u64
}

struct Marker {
    name: &'static str,
    id: u64,
    value: bool
}

impl MarkerList {
    pub fn new() -> Self {
        Self {
            markers: vec![],
            counter: 1
        }
    }

    pub fn declare(&mut self, name: &'static str) -> u64 {
        let id = self.counter;
        let value = true;
        let marker = Marker { name, id, value, };

        self.counter += 1;
        self.markers.push(marker);

        id
    }

    pub fn remove(&mut self, id: u64) {
        let index = self.markers.iter().position(|marker| marker.id == id).unwrap();

        self.markers.remove(index);
    }

    fn get_by_name(&self, name: &'static str) -> Option<&Marker> {
        for marker in self.markers.iter().rev() {
            if marker.name == name {
                return Some(marker);
            }
        }

        None
    }

    fn get_by_name_mut(&mut self, name: &'static str) -> Option<&mut Marker> {
        for marker in self.markers.iter_mut().rev() {
            if marker.name == name {
                return Some(marker);
            }
        }

        None
    }

    pub fn get(&self, name: &'static str) -> bool {
        self.get_by_name(name).map(|marker| marker.value).unwrap_or(false)
    }

    pub fn set(&mut self, name: &'static str, value: bool) -> bool {
        if let Some(marker) = self.get_by_name_mut(name) {
            let prev_value = marker.value;
            marker.value = value;
            prev_value
        } else {
            panic!("marker {} does not exist", name);
        }
    }
}