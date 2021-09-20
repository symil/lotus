use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}, rc::Rc};
use super::ProgramContext;

#[derive(Debug)]
pub struct GeneratedItemIndex<H, C> {
    pub entries: HashMap<u64, Entry<H, C>>
}

#[derive(Debug)]
pub struct Entry<H, C> {
    header: Rc<H>,
    content: Option<Rc<C>>
}

pub trait ItemGenerator<H, C> {
    fn get_id(&self) -> u64;
    fn generate_header(&self, id: u64) -> H;
    fn generate_content(&self, header: &Rc<H>, context: &mut ProgramContext) -> C;
}

impl<H, C> GeneratedItemIndex<H, C> {
    pub fn get_header<G : ItemGenerator<H, C>>(&mut self, generator: &G) -> (Rc<H>, bool) {
        let id = generator.get_id();

        match self.entries.get(&id) {
            Some(entry) => (Rc::clone(&entry.header), true),
            None => {
                let header = Rc::new(generator.generate_header(id));
                let entry = Entry {
                    header: Rc::clone(&header),
                    content: None,
                };

                self.entries.insert(id, entry);

                (header, false)
            }
        }
    }

    pub fn set_content<G : ItemGenerator<H, C>>(&mut self, generator: &G, content: C) {
        let mut entry = self.entries.get_mut(&generator.get_id()).unwrap();

        entry.content = Some(Rc::new(content));
    }
}

impl<H, C> Default for GeneratedItemIndex<H, C> {
    fn default() -> Self {
        Self { entries: Default::default() }
    }
}