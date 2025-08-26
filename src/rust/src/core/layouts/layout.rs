use uuid::Uuid;

pub trait Layout: crate::core::reflect::Reflect {
    fn name(&self) -> &str;
    fn set_name(&mut self, name: &str);

    fn layout(&self) -> &[Uuid];
    fn len(&self) -> usize;

    fn copy_to(&self, target: &mut dyn Layout, start_index: usize);

    fn add_id(&mut self, uuid: Uuid, index: usize) -> Result<(), String>;

    fn remove_id(&mut self, uuid: Uuid) -> Result<usize, String>;
    fn remove_id_by_index(&mut self, index: usize) -> Result<Uuid, String>;
    
    fn move_id(&mut self, uuid: Uuid, new_index: usize) -> Result<(), String>;
    fn move_id_by_index(&mut self, old_index: usize, new_index: usize) -> Result<(), String>;
}

