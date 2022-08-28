



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GenerateId {
    pub value:      usize,
    pub generation: usize
}


impl GenerateId {
    // valid id
    pub const VALID_GENERATE_ID: GenerateId = GenerateId {
        value:      usize::MAX,
        generation: usize::MAX
    };

    pub fn is_valid(&self) -> bool {
        self.value != usize::MAX && self.generation != usize::MAX
    }
}

pub struct Allocator<T> {
    storage:        Storage<T>,
    generation_map: Vec<usize>,
    last_generation:usize,
}

impl<T> Allocator<T>  {
    pub fn new() -> Self {
        Allocator {
            storage:        Storage::new(),
            generation_map: Vec::new(),
            last_generation:0
        }
    }

    pub fn alloc_id(&self) -> GenerateId {
        let id = GenerateId {
            value:      self.storage.len(), // todo maybe use free index
            generation: self.last_generation
        };
        id
    }

    pub fn allocate(&mut self, element:T) -> GenerateId {
        let id = GenerateId {
            value:      self.storage.len(), // todo maybe use free index
            generation: self.last_generation
        };
        self.last_generation += 1;
        self.generation_map.push(id.generation);
        self.storage.storage.push(element);
        id
    }

    pub fn get_elements(&self) -> &Vec<T> {
        &self.storage.storage
    }

    pub fn get_elements_mut(&mut self) -> &mut Vec<T> {
        &mut self.storage.storage
    }

    pub fn has_element(&self, id:&GenerateId) -> bool {
        if !id.is_valid() {
            return false;
        }
        self.generation_map.contains(&id.generation)
    }

    pub fn get_element_by_id(&self, id:GenerateId) -> Option<&T> {
        // todo check id is valid and add adjust id.value isn't in of free list
        if id.value < self.storage.len() {
            Some(&self.storage.storage[id.value])
        } else {
            None
        }
    }

    pub fn get_element_mut_by_id(&mut self, id:GenerateId) -> Option<&mut T> {
        // todo check id is valid and add adjust id.value isn't in of free list
        if id.value < self.storage.len() {
            Some(&mut self.storage.storage[id.value])
        } else {
            None
        }
    }

}

struct Storage<T> {
    pub storage: Vec<T>,
}

impl<T>  Storage<T>  {
    pub fn new()->Self {
        Storage {
            storage: Vec::new()
        }
    }
}

impl<T> std::ops::Deref for Storage<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}