use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}, rc::Rc};
use super::ProgramContext;

#[derive(Debug)]
pub struct GeneratedItemIndex<H, C> {
    pub headers: HashMap<u64, Rc<H>>,
    pub contents: HashMap<u64, Rc<C>>,
}

pub trait ItemGenerator<H, C> {
    fn get_id(&self) -> u64;
    fn generate_header(&self, id: u64) -> H;
    fn generate_content(&self, header: &Rc<H>, context: &mut ProgramContext) -> C;
}

impl<H, C> GeneratedItemIndex<H, C> {
    pub fn get_header<G : ItemGenerator<H, C>>(&mut self, generator: &G, context: &mut ProgramContext) -> Rc<H> {
        let id = generator.get_id();

        match self.headers.get(&id) {
            Some(header) => Rc::clone(header),
            None => {
                let header = Rc::new(generator.generate_header(id));

                self.headers.insert(id, Rc::clone(&header));
                self.contents.insert(id, Rc::new(generator.generate_content(&header, context)));

                header
            }
        }
    }

    pub fn get_content(&mut self, header: &H, context: &mut ProgramContext) -> Option<Rc<C>>
        where H : Hash
    {
        let mut state = DefaultHasher::new();
        header.hash(&mut state);
        let id = state.finish();

        self.contents.get(&id).cloned()
    }
}

impl<H, C> Default for GeneratedItemIndex<H, C> {
    fn default() -> Self {
        Self {
            headers: Default::default(),
            contents: Default::default()
        }
    }
}